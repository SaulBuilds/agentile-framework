use std::path::Path;

use anyhow::{bail, ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::generation::{
    export_generated_midi, export_generated_wav, generate_composition, load_preset,
    MidiArtifactSummary, RenderPreset, WavArtifactSummary,
};

use super::{
    current_unix_seconds, default_preview_dir, new_runtime_id, read_json_or_default,
    snapshot_preset_hash, write_pretty_json,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Ready,
    Playing,
    Stopped,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionTransportCommand {
    Play,
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionEventRecord {
    pub event_id: String,
    pub created_at_unix_seconds: u64,
    pub actor_id: String,
    pub field_name: String,
    pub old_value: Value,
    pub new_value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionPreviewRecord {
    pub preview_id: String,
    pub created_at_unix_seconds: u64,
    pub created_by: String,
    pub midi: MidiArtifactSummary,
    pub wav: WavArtifactSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionRecord {
    pub session_id: String,
    pub display_name: String,
    pub preset_name: String,
    pub preset_hash: String,
    pub seed: u64,
    pub tempo_bpm: u16,
    pub status: SessionStatus,
    pub active_run_label: Option<String>,
    pub created_at_unix_seconds: u64,
    pub updated_at_unix_seconds: u64,
    pub previews: Vec<SessionPreviewRecord>,
    pub events: Vec<SessionEventRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewSessionRequest {
    pub display_name: String,
    pub preset_name: String,
    pub seed: u64,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct UpdateSessionRequest {
    pub actor_id: String,
    pub display_name: Option<String>,
    pub preset_name: Option<String>,
    pub seed: Option<u64>,
    pub tempo_bpm: Option<u16>,
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct SessionTransportRequest {
    pub actor_id: String,
    pub command: SessionTransportCommand,
    pub run_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionPreviewResult {
    pub session: SessionRecord,
    pub preview: SessionPreviewRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionPatchRequest {
    pub actor_id: String,
    pub display_name: Option<String>,
    pub seed: Option<u64>,
    pub tempo_bpm: Option<u16>,
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionPatchPreview {
    pub session_id: String,
    pub before: SessionRecord,
    pub after: SessionRecord,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct SessionPatchApplyResult {
    pub session: SessionRecord,
    pub rollback: SessionRecord,
    pub changed_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SessionStoreFile {
    version: u32,
    sessions: Vec<SessionRecord>,
}

pub fn create_session(
    store_path: &Path,
    preset_dir: &Path,
    request: NewSessionRequest,
) -> Result<SessionRecord> {
    validate_new_session_request(&request)?;
    let preset = load_preset(&request.preset_name, preset_dir)?;
    let preset_hash = snapshot_preset_hash(&preset)?;
    let now = current_unix_seconds();
    let record = SessionRecord {
        session_id: new_runtime_id("session"),
        display_name: request.display_name,
        preset_name: request.preset_name,
        preset_hash,
        seed: request.seed,
        tempo_bpm: preset.midi.tempo_bpm,
        status: SessionStatus::Ready,
        active_run_label: None,
        created_at_unix_seconds: now,
        updated_at_unix_seconds: now,
        previews: Vec::new(),
        events: Vec::new(),
    };

    let mut store = load_store(store_path)?;
    store.sessions.push(record.clone());
    save_store(store_path, &store)?;
    Ok(record)
}

pub fn inspect_session(store_path: &Path, session_id: &str) -> Result<SessionRecord> {
    ensure!(!session_id.trim().is_empty(), "session id cannot be empty");
    let store = load_store(store_path)?;
    store
        .sessions
        .into_iter()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| anyhow::anyhow!("session '{}' was not found", session_id))
}

pub fn list_sessions(store_path: &Path) -> Result<Vec<SessionRecord>> {
    let mut store = load_store(store_path)?;
    store.sessions.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
    });
    Ok(store.sessions)
}

pub fn update_session(
    store_path: &Path,
    preset_dir: &Path,
    session_id: &str,
    update: UpdateSessionRequest,
) -> Result<SessionRecord> {
    validate_update_session_request(&update)?;
    ensure!(!session_id.trim().is_empty(), "session id cannot be empty");

    let mut store = load_store(store_path)?;
    let session = store
        .sessions
        .iter_mut()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| anyhow::anyhow!("session '{}' was not found", session_id))?;

    if let Some(display_name) = update.display_name {
        ensure!(
            !display_name.trim().is_empty(),
            "display name cannot be empty"
        );
        push_event(
            session,
            &update.actor_id,
            "display_name",
            serde_json::to_value(&session.display_name)?,
            serde_json::to_value(&display_name)?,
        );
        session.display_name = display_name;
    }

    if let Some(preset_name) = update.preset_name {
        let preset = load_preset(&preset_name, preset_dir)?;
        let preset_hash = snapshot_preset_hash(&preset)?;
        push_event(
            session,
            &update.actor_id,
            "preset_name",
            serde_json::to_value(&session.preset_name)?,
            serde_json::to_value(&preset_name)?,
        );
        push_event(
            session,
            &update.actor_id,
            "preset_hash",
            serde_json::to_value(&session.preset_hash)?,
            serde_json::to_value(&preset_hash)?,
        );
        push_event(
            session,
            &update.actor_id,
            "tempo_bpm",
            serde_json::to_value(session.tempo_bpm)?,
            serde_json::to_value(preset.midi.tempo_bpm)?,
        );
        session.preset_name = preset_name;
        session.preset_hash = preset_hash;
        session.tempo_bpm = preset.midi.tempo_bpm;
    }

    if let Some(seed) = update.seed {
        push_event(
            session,
            &update.actor_id,
            "seed",
            serde_json::to_value(session.seed)?,
            serde_json::to_value(seed)?,
        );
        session.seed = seed;
    }

    if let Some(tempo_bpm) = update.tempo_bpm {
        ensure!(tempo_bpm > 0, "tempo_bpm must be greater than zero");
        push_event(
            session,
            &update.actor_id,
            "tempo_bpm",
            serde_json::to_value(session.tempo_bpm)?,
            serde_json::to_value(tempo_bpm)?,
        );
        session.tempo_bpm = tempo_bpm;
    }

    if let Some(status) = update.status {
        push_event(
            session,
            &update.actor_id,
            "status",
            serde_json::to_value(session.status)?,
            serde_json::to_value(status)?,
        );
        session.status = status;
        if !matches!(status, SessionStatus::Playing) {
            clear_active_run_label(session, &update.actor_id)?;
        }
    }

    ensure!(
        !session.events.is_empty(),
        "update request did not change any session fields"
    );
    session.updated_at_unix_seconds = current_unix_seconds();
    let updated = session.clone();
    save_store(store_path, &store)?;
    Ok(updated)
}

pub fn apply_transport_command(
    store_path: &Path,
    session_id: &str,
    request: SessionTransportRequest,
) -> Result<SessionRecord> {
    validate_transport_request(&request)?;
    ensure!(!session_id.trim().is_empty(), "session id cannot be empty");

    let mut store = load_store(store_path)?;
    let session = store
        .sessions
        .iter_mut()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| anyhow::anyhow!("session '{}' was not found", session_id))?;

    match request.command {
        SessionTransportCommand::Play => {
            if matches!(session.status, SessionStatus::Archived) {
                bail!("archived sessions cannot enter play state");
            }
            let run_label = request
                .run_label
                .clone()
                .unwrap_or_else(|| new_runtime_id("transport"));
            push_event(
                session,
                &request.actor_id,
                "status",
                serde_json::to_value(session.status)?,
                serde_json::to_value(SessionStatus::Playing)?,
            );
            push_event(
                session,
                &request.actor_id,
                "active_run_label",
                serde_json::to_value(&session.active_run_label)?,
                serde_json::to_value(&run_label)?,
            );
            session.status = SessionStatus::Playing;
            session.active_run_label = Some(run_label);
        }
        SessionTransportCommand::Stop => {
            if matches!(session.status, SessionStatus::Archived) {
                bail!("archived sessions cannot enter stop state");
            }
            push_event(
                session,
                &request.actor_id,
                "status",
                serde_json::to_value(session.status)?,
                serde_json::to_value(SessionStatus::Stopped)?,
            );
            session.status = SessionStatus::Stopped;
            clear_active_run_label(session, &request.actor_id)?;
        }
    }

    ensure!(
        !session.events.is_empty(),
        "transport command did not change session state"
    );
    session.updated_at_unix_seconds = current_unix_seconds();
    let updated = session.clone();
    save_store(store_path, &store)?;
    Ok(updated)
}

pub fn render_session_preview(
    store_path: &Path,
    preset_dir: &Path,
    runtime_dir: &Path,
    session_id: &str,
    actor_id: &str,
) -> Result<SessionPreviewResult> {
    ensure!(!actor_id.trim().is_empty(), "actor id cannot be empty");
    ensure!(!session_id.trim().is_empty(), "session id cannot be empty");

    let mut store = load_store(store_path)?;
    let session = store
        .sessions
        .iter_mut()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| anyhow::anyhow!("session '{}' was not found", session_id))?;

    let mut preset = load_preset(&session.preset_name, preset_dir)?;
    apply_session_overrides(&mut preset, session);
    let composition = generate_composition(preset, session.seed)?;

    let preview_id = new_runtime_id("preview");
    let preview_dir = default_preview_dir(runtime_dir).join(&session.session_id);
    let midi_path = preview_dir.join(format!("{preview_id}.mid"));
    let wav_path = preview_dir.join(format!("{preview_id}.wav"));
    let midi = export_generated_midi(&composition, &midi_path)?;
    let wav = export_generated_wav(&composition, &wav_path)?;

    let preview = SessionPreviewRecord {
        preview_id: preview_id.clone(),
        created_at_unix_seconds: current_unix_seconds(),
        created_by: actor_id.to_string(),
        midi,
        wav,
    };

    push_event(
        session,
        actor_id,
        "preview_id",
        session
            .previews
            .last()
            .map(|existing| serde_json::to_value(&existing.preview_id))
            .transpose()?
            .unwrap_or(Value::Null),
        serde_json::to_value(&preview.preview_id)?,
    );
    push_event(
        session,
        actor_id,
        "preview_count",
        serde_json::to_value(session.previews.len())?,
        serde_json::to_value(session.previews.len() + 1)?,
    );
    session.previews.push(preview.clone());
    session.updated_at_unix_seconds = current_unix_seconds();
    let updated = session.clone();
    save_store(store_path, &store)?;

    Ok(SessionPreviewResult {
        session: updated,
        preview,
    })
}

pub fn preview_session_patch(
    store_path: &Path,
    session_id: &str,
    patch: SessionPatchRequest,
) -> Result<SessionPatchPreview> {
    validate_patch_request(&patch)?;
    let before = inspect_session(store_path, session_id)?;
    let mut after = before.clone();
    let changed_fields = apply_patch_to_session(&mut after, &patch)?;
    ensure!(
        !changed_fields.is_empty(),
        "patch request did not change any session fields"
    );
    Ok(SessionPatchPreview {
        session_id: before.session_id.clone(),
        before,
        after,
        changed_fields,
    })
}

pub fn apply_session_patch(
    store_path: &Path,
    session_id: &str,
    patch: SessionPatchRequest,
) -> Result<SessionPatchApplyResult> {
    validate_patch_request(&patch)?;
    let mut store = load_store(store_path)?;
    let session = store
        .sessions
        .iter_mut()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| anyhow::anyhow!("session '{}' was not found", session_id))?;

    let rollback = session.clone();
    let changed_fields = apply_patch_to_session(session, &patch)?;
    ensure!(
        !changed_fields.is_empty(),
        "patch request did not change any session fields"
    );
    session.updated_at_unix_seconds = current_unix_seconds();
    let updated = session.clone();
    save_store(store_path, &store)?;
    Ok(SessionPatchApplyResult {
        session: updated,
        rollback,
        changed_fields,
    })
}

fn apply_session_overrides(preset: &mut RenderPreset, session: &SessionRecord) {
    preset.midi.tempo_bpm = session.tempo_bpm;
}

fn apply_patch_to_session(
    session: &mut SessionRecord,
    patch: &SessionPatchRequest,
) -> Result<Vec<String>> {
    let mut changed_fields = Vec::new();

    if let Some(display_name) = &patch.display_name {
        ensure!(
            !display_name.trim().is_empty(),
            "display name cannot be empty"
        );
        let old = serde_json::to_value(&session.display_name)?;
        let new = serde_json::to_value(display_name)?;
        if old != new {
            push_event(session, &patch.actor_id, "display_name", old, new);
            session.display_name = display_name.clone();
            changed_fields.push("display_name".to_string());
        }
    }

    if let Some(seed) = patch.seed {
        let old = serde_json::to_value(session.seed)?;
        let new = serde_json::to_value(seed)?;
        if old != new {
            push_event(session, &patch.actor_id, "seed", old, new);
            session.seed = seed;
            changed_fields.push("seed".to_string());
        }
    }

    if let Some(tempo_bpm) = patch.tempo_bpm {
        ensure!(tempo_bpm > 0, "tempo_bpm must be greater than zero");
        let old = serde_json::to_value(session.tempo_bpm)?;
        let new = serde_json::to_value(tempo_bpm)?;
        if old != new {
            push_event(session, &patch.actor_id, "tempo_bpm", old, new);
            session.tempo_bpm = tempo_bpm;
            changed_fields.push("tempo_bpm".to_string());
        }
    }

    if let Some(status) = patch.status {
        let old = serde_json::to_value(session.status)?;
        let new = serde_json::to_value(status)?;
        if old != new {
            push_event(session, &patch.actor_id, "status", old, new);
            session.status = status;
            changed_fields.push("status".to_string());
        }
        if !matches!(status, SessionStatus::Playing) {
            let old = serde_json::to_value(&session.active_run_label)?;
            if old != Value::Null {
                push_event(
                    session,
                    &patch.actor_id,
                    "active_run_label",
                    old,
                    Value::Null,
                );
                session.active_run_label = None;
                changed_fields.push("active_run_label".to_string());
            }
        }
    }

    Ok(changed_fields)
}

fn clear_active_run_label(session: &mut SessionRecord, actor_id: &str) -> Result<()> {
    push_event(
        session,
        actor_id,
        "active_run_label",
        serde_json::to_value(&session.active_run_label)?,
        Value::Null,
    );
    session.active_run_label = None;
    Ok(())
}

fn push_event(
    session: &mut SessionRecord,
    actor_id: &str,
    field_name: &str,
    old_value: Value,
    new_value: Value,
) {
    if old_value == new_value {
        return;
    }

    session.events.push(SessionEventRecord {
        event_id: new_runtime_id("session-event"),
        created_at_unix_seconds: current_unix_seconds(),
        actor_id: actor_id.to_string(),
        field_name: field_name.to_string(),
        old_value,
        new_value,
    });
}

fn validate_new_session_request(request: &NewSessionRequest) -> Result<()> {
    ensure!(
        !request.display_name.trim().is_empty(),
        "display name cannot be empty"
    );
    ensure!(
        !request.preset_name.trim().is_empty(),
        "preset name cannot be empty"
    );
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    Ok(())
}

fn validate_update_session_request(update: &UpdateSessionRequest) -> Result<()> {
    ensure!(
        !update.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    if update.display_name.is_none()
        && update.preset_name.is_none()
        && update.seed.is_none()
        && update.tempo_bpm.is_none()
        && update.status.is_none()
    {
        bail!("update request must change at least one field");
    }
    Ok(())
}

fn validate_transport_request(request: &SessionTransportRequest) -> Result<()> {
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    if let Some(run_label) = &request.run_label {
        ensure!(!run_label.trim().is_empty(), "run label cannot be empty");
    }
    Ok(())
}

fn validate_patch_request(request: &SessionPatchRequest) -> Result<()> {
    ensure!(
        !request.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    if request.display_name.is_none()
        && request.seed.is_none()
        && request.tempo_bpm.is_none()
        && request.status.is_none()
    {
        bail!("patch request must change at least one field");
    }
    Ok(())
}

fn load_store(store_path: &Path) -> Result<SessionStoreFile> {
    let mut store: SessionStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &SessionStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    use crate::generation::{demo_preset, save_preset};

    #[test]
    fn test_create_inspect_and_update_session() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let store_path = dir.path().join("sessions.json");

        let mut preset = demo_preset();
        preset.name = "session-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let created = create_session(
            &store_path,
            &preset_dir,
            NewSessionRequest {
                display_name: "My Session".to_string(),
                preset_name: "session-demo".to_string(),
                seed: 7,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let inspected = inspect_session(&store_path, &created.session_id).unwrap();
        assert_eq!(inspected.display_name, "My Session");

        let updated = update_session(
            &store_path,
            &preset_dir,
            &created.session_id,
            UpdateSessionRequest {
                actor_id: "tester".to_string(),
                display_name: Some("Renamed Session".to_string()),
                preset_name: None,
                seed: Some(11),
                tempo_bpm: Some(90),
                status: Some(SessionStatus::Playing),
            },
        )
        .unwrap();
        assert_eq!(updated.display_name, "Renamed Session");
        assert_eq!(updated.seed, 11);
        assert_eq!(updated.tempo_bpm, 90);
        assert_eq!(updated.status, SessionStatus::Playing);
        assert!(updated.events.len() >= 4);
        assert_eq!(updated.active_run_label, None);
    }

    #[test]
    fn test_update_session_rejects_missing_session() {
        let dir = tempdir().unwrap();
        let error = update_session(
            &dir.path().join("sessions.json"),
            dir.path(),
            "missing",
            UpdateSessionRequest {
                actor_id: "tester".to_string(),
                display_name: Some("Name".to_string()),
                preset_name: None,
                seed: None,
                tempo_bpm: None,
                status: None,
            },
        )
        .unwrap_err();
        assert!(error.to_string().contains("was not found"));
    }

    #[test]
    fn test_apply_transport_command_and_render_preview() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let store_path = runtime_dir.join("sessions.json");

        let mut preset = demo_preset();
        preset.name = "preview-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let created = create_session(
            &store_path,
            &preset_dir,
            NewSessionRequest {
                display_name: "Preview Session".to_string(),
                preset_name: "preview-demo".to_string(),
                seed: 5,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();

        let playing = apply_transport_command(
            &store_path,
            &created.session_id,
            SessionTransportRequest {
                actor_id: "tester".to_string(),
                command: SessionTransportCommand::Play,
                run_label: Some("live-pass".to_string()),
            },
        )
        .unwrap();
        assert_eq!(playing.status, SessionStatus::Playing);
        assert_eq!(playing.active_run_label.as_deref(), Some("live-pass"));

        let preview = render_session_preview(
            &store_path,
            &preset_dir,
            &runtime_dir,
            &created.session_id,
            "tester",
        )
        .unwrap();
        assert_eq!(preview.session.previews.len(), 1);
        assert!(preview.preview.midi.path.exists());
        assert!(preview.preview.wav.path.exists());

        let stopped = apply_transport_command(
            &store_path,
            &created.session_id,
            SessionTransportRequest {
                actor_id: "tester".to_string(),
                command: SessionTransportCommand::Stop,
                run_label: None,
            },
        )
        .unwrap();
        assert_eq!(stopped.status, SessionStatus::Stopped);
        assert_eq!(stopped.active_run_label, None);
    }

    #[test]
    fn test_preview_and_apply_session_patch() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let store_path = dir.path().join("sessions.json");

        let mut preset = demo_preset();
        preset.name = "patch-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let created = create_session(
            &store_path,
            &preset_dir,
            NewSessionRequest {
                display_name: "Patch Session".to_string(),
                preset_name: "patch-demo".to_string(),
                seed: 5,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();

        let preview = preview_session_patch(
            &store_path,
            &created.session_id,
            SessionPatchRequest {
                actor_id: "tester".to_string(),
                display_name: None,
                seed: Some(9),
                tempo_bpm: Some(132),
                status: None,
            },
        )
        .unwrap();
        assert_eq!(preview.after.seed, 9);
        assert_eq!(preview.after.tempo_bpm, 132);

        let applied = apply_session_patch(
            &store_path,
            &created.session_id,
            SessionPatchRequest {
                actor_id: "tester".to_string(),
                display_name: None,
                seed: Some(9),
                tempo_bpm: Some(132),
                status: None,
            },
        )
        .unwrap();
        assert_eq!(applied.session.seed, 9);
        assert_eq!(applied.rollback.seed, 5);
        assert!(applied.changed_fields.contains(&"seed".to_string()));
    }
}
