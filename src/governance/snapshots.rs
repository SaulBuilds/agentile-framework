use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::generation::{RenderPreset, DEMO_PRESET_NAME};

use super::{current_unix_seconds, new_runtime_id, sha256_hex, write_pretty_json};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PresetSnapshotRecord {
    pub snapshot_id: String,
    pub preset_name: String,
    pub preset_hash: String,
    pub reason: String,
    pub actor_id: Option<String>,
    pub created_at_unix_seconds: u64,
    pub source_preset_path: PathBuf,
    pub serialized_preset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PresetSnapshotSummary {
    pub snapshot_id: String,
    pub preset_name: String,
    pub preset_hash: String,
    pub created_at_unix_seconds: u64,
    pub snapshot_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct PresetRollbackSummary {
    pub snapshot_id: String,
    pub preset_name: String,
    pub restored_preset_hash: String,
    pub output_path: PathBuf,
}

pub fn create_preset_snapshot(
    snapshot_dir: &Path,
    preset_dir: &Path,
    preset_name: &str,
    reason: &str,
    actor_id: Option<&str>,
) -> Result<PresetSnapshotSummary> {
    ensure!(
        preset_name != DEMO_PRESET_NAME,
        "built-in preset '{}' cannot be snapshotted because it is not file-backed",
        DEMO_PRESET_NAME
    );
    ensure!(
        !preset_name.trim().is_empty(),
        "preset name cannot be empty"
    );
    ensure!(!reason.trim().is_empty(), "snapshot reason cannot be empty");

    let preset_path = preset_dir.join(format!("{preset_name}.json"));
    let raw = fs::read_to_string(&preset_path)
        .with_context(|| format!("failed to read preset file {}", preset_path.display()))?;
    let preset: RenderPreset = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse preset file {}", preset_path.display()))?;
    ensure!(
        preset.name == preset_name,
        "preset file '{}' does not match requested preset '{}'",
        preset.name,
        preset_name
    );

    fs::create_dir_all(snapshot_dir).with_context(|| {
        format!(
            "failed to create snapshot directory {}",
            snapshot_dir.display()
        )
    })?;
    let snapshot_id = new_runtime_id("snapshot");
    let snapshot_path = snapshot_dir.join(format!("{snapshot_id}.json"));
    let record = PresetSnapshotRecord {
        snapshot_id: snapshot_id.clone(),
        preset_name: preset_name.to_string(),
        preset_hash: sha256_hex(raw.as_bytes()),
        reason: reason.to_string(),
        actor_id: actor_id.map(str::to_string),
        created_at_unix_seconds: current_unix_seconds(),
        source_preset_path: preset_path,
        serialized_preset: raw,
    };
    write_pretty_json(&snapshot_path, &record)?;

    Ok(PresetSnapshotSummary {
        snapshot_id: record.snapshot_id,
        preset_name: record.preset_name,
        preset_hash: record.preset_hash,
        created_at_unix_seconds: record.created_at_unix_seconds,
        snapshot_path,
    })
}

pub fn rollback_preset_snapshot(
    snapshot_dir: &Path,
    preset_dir: &Path,
    snapshot_id: &str,
) -> Result<PresetRollbackSummary> {
    ensure!(
        !snapshot_id.trim().is_empty(),
        "snapshot id cannot be empty"
    );
    let snapshot_path = snapshot_dir.join(format!("{snapshot_id}.json"));
    let raw = fs::read_to_string(&snapshot_path)
        .with_context(|| format!("failed to read snapshot {}", snapshot_path.display()))?;
    let record: PresetSnapshotRecord = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse snapshot {}", snapshot_path.display()))?;
    serde_json::from_str::<RenderPreset>(&record.serialized_preset).with_context(|| {
        format!(
            "stored preset content in snapshot '{}' is not valid JSON",
            snapshot_id
        )
    })?;

    fs::create_dir_all(preset_dir)
        .with_context(|| format!("failed to create preset directory {}", preset_dir.display()))?;
    let output_path = preset_dir.join(format!("{}.json", record.preset_name));
    fs::write(&output_path, &record.serialized_preset)
        .with_context(|| format!("failed to restore preset file {}", output_path.display()))?;

    Ok(PresetRollbackSummary {
        snapshot_id: record.snapshot_id,
        preset_name: record.preset_name,
        restored_preset_hash: sha256_hex(record.serialized_preset.as_bytes()),
        output_path,
    })
}

pub fn default_snapshot_target_dir(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("snapshots")
}

pub fn snapshot_preset_hash(preset: &RenderPreset) -> Result<String> {
    let raw = serde_json::to_vec(preset)?;
    Ok(sha256_hex(&raw))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    use crate::generation::{default_preset_dir, demo_preset, load_preset, save_preset};

    #[test]
    fn test_create_and_rollback_snapshot() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let snapshot_dir = dir.path().join("snapshots");

        let mut preset = demo_preset();
        preset.name = "custom-demo".to_string();
        preset.description = "custom".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let snapshot = create_preset_snapshot(
            &snapshot_dir,
            &preset_dir,
            "custom-demo",
            "before update",
            Some("tester"),
        )
        .unwrap();

        let mut changed = load_preset("custom-demo", &preset_dir).unwrap();
        changed.midi.tempo_bpm = 90;
        save_preset(&changed, &preset_dir).unwrap();

        let rollback =
            rollback_preset_snapshot(&snapshot_dir, &preset_dir, &snapshot.snapshot_id).unwrap();
        let restored = load_preset("custom-demo", &preset_dir).unwrap();

        assert_eq!(rollback.preset_name, "custom-demo");
        assert_eq!(restored.midi.tempo_bpm, preset.midi.tempo_bpm);
    }

    #[test]
    fn test_rollback_fails_for_invalid_snapshot_payload() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let snapshot_dir = dir.path().join("snapshots");
        fs::create_dir_all(&snapshot_dir).unwrap();

        let record = PresetSnapshotRecord {
            snapshot_id: "snapshot-bad".to_string(),
            preset_name: "broken".to_string(),
            preset_hash: "hash".to_string(),
            reason: "broken".to_string(),
            actor_id: None,
            created_at_unix_seconds: 1,
            source_preset_path: default_preset_dir().join("broken.json"),
            serialized_preset: "{not-json}".to_string(),
        };
        write_pretty_json(&snapshot_dir.join("snapshot-bad.json"), &record).unwrap();

        let error =
            rollback_preset_snapshot(&snapshot_dir, &preset_dir, "snapshot-bad").unwrap_err();
        assert!(error.to_string().contains("stored preset content"));
    }
}
