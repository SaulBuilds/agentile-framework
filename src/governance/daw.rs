use std::path::Path;

use anyhow::{ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    current_unix_seconds, inspect_session, new_runtime_id, read_json_or_default, write_pretty_json,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeckTransportState {
    Ready,
    Playing,
    Stopped,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeckSourceKind {
    SessionPreview,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DeckEventRecord {
    pub event_id: String,
    pub created_at_unix_seconds: u64,
    pub actor_id: String,
    pub field_name: String,
    pub old_value: Value,
    pub new_value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DeckClipRecord {
    pub clip_id: String,
    pub created_at_unix_seconds: u64,
    pub label: String,
    pub source_kind: DeckSourceKind,
    pub session_id: String,
    pub preview_id: String,
    pub midi_path: std::path::PathBuf,
    pub wav_path: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DeckRecord {
    pub deck_id: String,
    pub display_name: String,
    pub session_id: String,
    pub transport_state: DeckTransportState,
    pub active_clip_id: Option<String>,
    pub queued_clip_id: Option<String>,
    pub created_at_unix_seconds: u64,
    pub updated_at_unix_seconds: u64,
    pub clips: Vec<DeckClipRecord>,
    pub events: Vec<DeckEventRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewDeckRequest {
    pub display_name: String,
    pub session_id: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AddDeckPreviewRequest {
    pub actor_id: String,
    pub label: String,
    pub session_id: String,
    pub preview_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct QueueDeckClipRequest {
    pub actor_id: String,
    pub clip_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct LaunchDeckClipRequest {
    pub actor_id: String,
    pub clip_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct StopDeckRequest {
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct DeckTransportSnapshot {
    pub deck: DeckRecord,
    pub active_clip: Option<DeckClipRecord>,
    pub queued_clip: Option<DeckClipRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct DeckStoreFile {
    version: u32,
    decks: Vec<DeckRecord>,
}

pub fn create_deck(
    store_path: &Path,
    session_store_path: &Path,
    request: NewDeckRequest,
) -> Result<DeckRecord> {
    validate_new_deck_request(&request)?;
    inspect_session(session_store_path, &request.session_id)?;
    let now = current_unix_seconds();
    let deck = DeckRecord {
        deck_id: new_runtime_id("deck"),
        display_name: request.display_name,
        session_id: request.session_id,
        transport_state: DeckTransportState::Ready,
        active_clip_id: None,
        queued_clip_id: None,
        created_at_unix_seconds: now,
        updated_at_unix_seconds: now,
        clips: Vec::new(),
        events: Vec::new(),
    };

    let mut store = load_store(store_path)?;
    store.decks.push(deck.clone());
    save_store(store_path, &store)?;
    Ok(deck)
}

pub fn inspect_deck(store_path: &Path, deck_id: &str) -> Result<DeckRecord> {
    ensure!(!deck_id.trim().is_empty(), "deck id cannot be empty");
    let store = load_store(store_path)?;
    store
        .decks
        .into_iter()
        .find(|deck| deck.deck_id == deck_id)
        .ok_or_else(|| anyhow::anyhow!("deck '{}' was not found", deck_id))
}

pub fn list_decks(store_path: &Path) -> Result<Vec<DeckRecord>> {
    let mut store = load_store(store_path)?;
    store.decks.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
    });
    Ok(store.decks)
}

pub fn add_preview_clip_to_deck(
    store_path: &Path,
    session_store_path: &Path,
    deck_id: &str,
    request: AddDeckPreviewRequest,
) -> Result<DeckRecord> {
    validate_add_preview_request(&request)?;
    ensure!(!deck_id.trim().is_empty(), "deck id cannot be empty");
    let session = inspect_session(session_store_path, &request.session_id)?;

    let preview = session
        .previews
        .iter()
        .find(|preview| preview.preview_id == request.preview_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("preview '{}' was not found", request.preview_id))?;

    let mut store = load_store(store_path)?;
    let deck = store
        .decks
        .iter_mut()
        .find(|deck| deck.deck_id == deck_id)
        .ok_or_else(|| anyhow::anyhow!("deck '{}' was not found", deck_id))?;
    ensure!(
        deck.session_id == request.session_id,
        "deck '{}' is bound to session '{}', not '{}'",
        deck_id,
        deck.session_id,
        request.session_id
    );

    let clip = DeckClipRecord {
        clip_id: new_runtime_id("clip"),
        created_at_unix_seconds: current_unix_seconds(),
        label: request.label,
        source_kind: DeckSourceKind::SessionPreview,
        session_id: request.session_id,
        preview_id: request.preview_id,
        midi_path: preview.midi.path,
        wav_path: preview.wav.path,
    };
    push_event(
        deck,
        &request.actor_id,
        "clip_count",
        serde_json::to_value(deck.clips.len())?,
        serde_json::to_value(deck.clips.len() + 1)?,
    );
    push_event(
        deck,
        &request.actor_id,
        "last_clip_id",
        deck.clips
            .last()
            .map(|existing| serde_json::to_value(&existing.clip_id))
            .transpose()?
            .unwrap_or(Value::Null),
        serde_json::to_value(&clip.clip_id)?,
    );
    deck.clips.push(clip);
    deck.updated_at_unix_seconds = current_unix_seconds();
    let updated = deck.clone();
    save_store(store_path, &store)?;
    Ok(updated)
}

pub fn queue_deck_clip(
    store_path: &Path,
    deck_id: &str,
    request: QueueDeckClipRequest,
) -> Result<DeckRecord> {
    ensure!(!deck_id.trim().is_empty(), "deck id cannot be empty");
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    ensure!(
        !request.clip_id.trim().is_empty(),
        "clip id cannot be empty"
    );

    let mut store = load_store(store_path)?;
    let deck = store
        .decks
        .iter_mut()
        .find(|deck| deck.deck_id == deck_id)
        .ok_or_else(|| anyhow::anyhow!("deck '{}' was not found", deck_id))?;
    ensure!(
        deck.clips
            .iter()
            .any(|clip| clip.clip_id == request.clip_id),
        "clip '{}' was not found on deck '{}'",
        request.clip_id,
        deck_id
    );
    push_event(
        deck,
        &request.actor_id,
        "queued_clip_id",
        serde_json::to_value(&deck.queued_clip_id)?,
        serde_json::to_value(&request.clip_id)?,
    );
    deck.queued_clip_id = Some(request.clip_id);
    deck.updated_at_unix_seconds = current_unix_seconds();
    let updated = deck.clone();
    save_store(store_path, &store)?;
    Ok(updated)
}

pub fn launch_deck_clip(
    store_path: &Path,
    deck_id: &str,
    request: LaunchDeckClipRequest,
) -> Result<DeckTransportSnapshot> {
    ensure!(!deck_id.trim().is_empty(), "deck id cannot be empty");
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    ensure!(
        !request.clip_id.trim().is_empty(),
        "clip id cannot be empty"
    );

    let mut store = load_store(store_path)?;
    let deck = store
        .decks
        .iter_mut()
        .find(|deck| deck.deck_id == deck_id)
        .ok_or_else(|| anyhow::anyhow!("deck '{}' was not found", deck_id))?;
    let clip = deck
        .clips
        .iter()
        .find(|clip| clip.clip_id == request.clip_id)
        .cloned()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "clip '{}' was not found on deck '{}'",
                request.clip_id,
                deck_id
            )
        })?;

    push_event(
        deck,
        &request.actor_id,
        "active_clip_id",
        serde_json::to_value(&deck.active_clip_id)?,
        serde_json::to_value(&request.clip_id)?,
    );
    push_event(
        deck,
        &request.actor_id,
        "transport_state",
        serde_json::to_value(deck.transport_state)?,
        serde_json::to_value(DeckTransportState::Playing)?,
    );
    push_event(
        deck,
        &request.actor_id,
        "queued_clip_id",
        serde_json::to_value(&deck.queued_clip_id)?,
        Value::Null,
    );
    deck.active_clip_id = Some(request.clip_id);
    deck.queued_clip_id = None;
    deck.transport_state = DeckTransportState::Playing;
    deck.updated_at_unix_seconds = current_unix_seconds();
    let snapshot = deck_transport_snapshot(deck.clone());
    let _ = clip;
    save_store(store_path, &store)?;
    Ok(snapshot)
}

pub fn stop_deck(
    store_path: &Path,
    deck_id: &str,
    request: StopDeckRequest,
) -> Result<DeckTransportSnapshot> {
    ensure!(!deck_id.trim().is_empty(), "deck id cannot be empty");
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );

    let mut store = load_store(store_path)?;
    let deck = store
        .decks
        .iter_mut()
        .find(|deck| deck.deck_id == deck_id)
        .ok_or_else(|| anyhow::anyhow!("deck '{}' was not found", deck_id))?;
    push_event(
        deck,
        &request.actor_id,
        "transport_state",
        serde_json::to_value(deck.transport_state)?,
        serde_json::to_value(DeckTransportState::Stopped)?,
    );
    push_event(
        deck,
        &request.actor_id,
        "active_clip_id",
        serde_json::to_value(&deck.active_clip_id)?,
        Value::Null,
    );
    deck.transport_state = DeckTransportState::Stopped;
    deck.active_clip_id = None;
    deck.updated_at_unix_seconds = current_unix_seconds();
    let snapshot = deck_transport_snapshot(deck.clone());
    save_store(store_path, &store)?;
    Ok(snapshot)
}

pub fn inspect_deck_transport(store_path: &Path, deck_id: &str) -> Result<DeckTransportSnapshot> {
    Ok(deck_transport_snapshot(inspect_deck(store_path, deck_id)?))
}

fn deck_transport_snapshot(deck: DeckRecord) -> DeckTransportSnapshot {
    let active_clip = deck
        .active_clip_id
        .as_ref()
        .and_then(|clip_id| deck.clips.iter().find(|clip| &clip.clip_id == clip_id))
        .cloned();
    let queued_clip = deck
        .queued_clip_id
        .as_ref()
        .and_then(|clip_id| deck.clips.iter().find(|clip| &clip.clip_id == clip_id))
        .cloned();
    DeckTransportSnapshot {
        deck,
        active_clip,
        queued_clip,
    }
}

fn push_event(
    deck: &mut DeckRecord,
    actor_id: &str,
    field_name: &str,
    old_value: Value,
    new_value: Value,
) {
    if old_value == new_value {
        return;
    }
    deck.events.push(DeckEventRecord {
        event_id: new_runtime_id("deck-event"),
        created_at_unix_seconds: current_unix_seconds(),
        actor_id: actor_id.to_string(),
        field_name: field_name.to_string(),
        old_value,
        new_value,
    });
}

fn validate_new_deck_request(request: &NewDeckRequest) -> Result<()> {
    ensure!(
        !request.display_name.trim().is_empty(),
        "display name cannot be empty"
    );
    ensure!(
        !request.session_id.trim().is_empty(),
        "session id cannot be empty"
    );
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    Ok(())
}

fn validate_add_preview_request(request: &AddDeckPreviewRequest) -> Result<()> {
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    ensure!(!request.label.trim().is_empty(), "label cannot be empty");
    ensure!(
        !request.session_id.trim().is_empty(),
        "session id cannot be empty"
    );
    ensure!(
        !request.preview_id.trim().is_empty(),
        "preview id cannot be empty"
    );
    Ok(())
}

fn load_store(store_path: &Path) -> Result<DeckStoreFile> {
    let mut store: DeckStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &DeckStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::generation::{demo_preset, save_preset};
    use crate::governance::{create_session, render_session_preview, NewSessionRequest};

    #[test]
    fn test_create_add_queue_launch_and_stop_deck() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let session_store_path = runtime_dir.join("sessions.json");
        let deck_store_path = runtime_dir.join("decks.json");

        let mut preset = demo_preset();
        preset.name = "deck-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let session = create_session(
            &session_store_path,
            &preset_dir,
            NewSessionRequest {
                display_name: "Deck Session".to_string(),
                preset_name: "deck-demo".to_string(),
                seed: 3,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let preview = render_session_preview(
            &session_store_path,
            &preset_dir,
            &runtime_dir,
            &session.session_id,
            "tester",
        )
        .unwrap();

        let deck = create_deck(
            &deck_store_path,
            &session_store_path,
            NewDeckRequest {
                display_name: "Main Deck".to_string(),
                session_id: session.session_id.clone(),
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let deck = add_preview_clip_to_deck(
            &deck_store_path,
            &session_store_path,
            &deck.deck_id,
            AddDeckPreviewRequest {
                actor_id: "tester".to_string(),
                label: "Take 1".to_string(),
                session_id: session.session_id.clone(),
                preview_id: preview.preview.preview_id.clone(),
            },
        )
        .unwrap();
        assert_eq!(deck.clips.len(), 1);

        let deck = queue_deck_clip(
            &deck_store_path,
            &deck.deck_id,
            QueueDeckClipRequest {
                actor_id: "tester".to_string(),
                clip_id: deck.clips[0].clip_id.clone(),
            },
        )
        .unwrap();
        assert_eq!(
            deck.queued_clip_id.as_deref(),
            Some(deck.clips[0].clip_id.as_str())
        );

        let launched = launch_deck_clip(
            &deck_store_path,
            &deck.deck_id,
            LaunchDeckClipRequest {
                actor_id: "tester".to_string(),
                clip_id: deck.clips[0].clip_id.clone(),
            },
        )
        .unwrap();
        assert_eq!(launched.deck.transport_state, DeckTransportState::Playing);
        assert_eq!(
            launched
                .active_clip
                .as_ref()
                .map(|clip| clip.label.as_str()),
            Some("Take 1")
        );

        let stopped = stop_deck(
            &deck_store_path,
            &deck.deck_id,
            StopDeckRequest {
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        assert_eq!(stopped.deck.transport_state, DeckTransportState::Stopped);
        assert!(stopped.active_clip.is_none());
    }
}
