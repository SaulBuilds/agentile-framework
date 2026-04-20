use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{ensure, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{
    append_json_line, current_unix_seconds, default_audit_log_path, default_manifest_dir,
    new_runtime_id, read_json_lines, sha256_hex, write_pretty_json,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionTransport {
    Cli,
    Mcp,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ManifestArtifactRecord {
    pub kind: String,
    pub path: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ManifestArtifactInput {
    pub kind: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewActionRecord {
    pub action: String,
    pub actor_id: String,
    pub transport: ActionTransport,
    pub target: Option<String>,
    pub status: ActionStatus,
    pub input: Value,
    pub output: Option<Value>,
    pub metadata: Option<Value>,
    pub preset_name: Option<String>,
    pub preset_hash: Option<String>,
    pub seed: Option<u64>,
    pub approval_ids: Vec<String>,
    pub artifacts: Vec<ManifestArtifactInput>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RunManifestRecord {
    pub run_id: String,
    pub manifest_version: u32,
    pub created_at_unix_seconds: u64,
    pub action: String,
    pub actor_id: String,
    pub transport: ActionTransport,
    pub target: Option<String>,
    pub status: ActionStatus,
    pub preset_name: Option<String>,
    pub preset_hash: Option<String>,
    pub seed: Option<u64>,
    pub engine_version: String,
    pub tool_chain_version: String,
    pub approval_ids: Vec<String>,
    pub input: Value,
    pub output: Option<Value>,
    pub metadata: Value,
    pub artifacts: Vec<ManifestArtifactRecord>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AuditEventRecord {
    pub event_id: String,
    pub created_at_unix_seconds: u64,
    pub action: String,
    pub actor_id: String,
    pub transport: ActionTransport,
    pub target: Option<String>,
    pub status: ActionStatus,
    pub run_id: Option<String>,
    pub approval_ids: Vec<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ActionAuditRef {
    pub run_id: String,
    pub manifest_path: PathBuf,
    pub audit_event_id: String,
    pub audit_log_path: PathBuf,
}

pub fn persist_action_record(runtime_dir: &Path, input: NewActionRecord) -> Result<ActionAuditRef> {
    validate_action_record(&input)?;

    let manifest_dir = default_manifest_dir(runtime_dir);
    let audit_log_path = default_audit_log_path(runtime_dir);
    let run_id = new_runtime_id("run");
    let manifest_path = manifest_dir.join(format!("{run_id}.json"));
    let manifest = RunManifestRecord {
        run_id: run_id.clone(),
        manifest_version: 1,
        created_at_unix_seconds: current_unix_seconds(),
        action: input.action.clone(),
        actor_id: input.actor_id.clone(),
        transport: input.transport,
        target: input.target.clone(),
        status: input.status,
        preset_name: input.preset_name,
        preset_hash: input.preset_hash,
        seed: input.seed,
        engine_version: env!("CARGO_PKG_VERSION").to_string(),
        tool_chain_version: env!("CARGO_PKG_VERSION").to_string(),
        approval_ids: input.approval_ids.clone(),
        input: input.input,
        output: input.output,
        metadata: input.metadata.unwrap_or_else(|| json!({})),
        artifacts: collect_artifacts(&input.artifacts)?,
        error_message: input.error_message.clone(),
    };
    write_pretty_json(&manifest_path, &manifest)?;

    let audit = AuditEventRecord {
        event_id: new_runtime_id("audit"),
        created_at_unix_seconds: current_unix_seconds(),
        action: manifest.action.clone(),
        actor_id: manifest.actor_id.clone(),
        transport: manifest.transport,
        target: manifest.target.clone(),
        status: manifest.status,
        run_id: Some(manifest.run_id.clone()),
        approval_ids: manifest.approval_ids.clone(),
        message: manifest.error_message.clone(),
    };
    append_json_line(&audit_log_path, &audit)?;

    Ok(ActionAuditRef {
        run_id,
        manifest_path,
        audit_event_id: audit.event_id,
        audit_log_path,
    })
}

pub fn inspect_run_manifest(manifest_dir: &Path, run_id: &str) -> Result<RunManifestRecord> {
    ensure!(!run_id.trim().is_empty(), "run id cannot be empty");
    let manifest_path = manifest_dir.join(format!("{run_id}.json"));
    read_manifest_file(&manifest_path)
}

pub fn list_run_manifests(manifest_dir: &Path) -> Result<Vec<RunManifestRecord>> {
    let mut manifests = Vec::new();
    if !manifest_dir.exists() {
        return Ok(manifests);
    }

    for entry in fs::read_dir(manifest_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }

        let manifest = read_manifest_file(&path)?;
        manifests.push(manifest);
    }

    manifests.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
            .then(left.run_id.cmp(&right.run_id))
    });
    Ok(manifests)
}

pub fn read_audit_events(audit_log_path: &Path) -> Result<Vec<AuditEventRecord>> {
    read_json_lines(audit_log_path)
}

fn collect_artifacts(inputs: &[ManifestArtifactInput]) -> Result<Vec<ManifestArtifactRecord>> {
    let mut artifacts = Vec::with_capacity(inputs.len());
    for input in inputs {
        ensure!(
            !input.kind.trim().is_empty(),
            "artifact kind cannot be empty"
        );
        ensure!(
            input.path.exists(),
            "artifact path '{}' does not exist",
            input.path.display()
        );
        let bytes = fs::read(&input.path)?;
        let size_bytes = fs::metadata(&input.path)?.len();
        artifacts.push(ManifestArtifactRecord {
            kind: input.kind.clone(),
            path: input.path.clone(),
            sha256: sha256_hex(&bytes),
            size_bytes,
        });
    }

    Ok(artifacts)
}

fn read_manifest_file(path: &Path) -> Result<RunManifestRecord> {
    let raw = fs::read_to_string(path)?;
    let manifest = serde_json::from_str(&raw)?;
    Ok(manifest)
}

fn validate_action_record(input: &NewActionRecord) -> Result<()> {
    ensure!(!input.action.trim().is_empty(), "action cannot be empty");
    ensure!(
        !input.actor_id.trim().is_empty(),
        "actor id cannot be empty"
    );
    if matches!(input.status, ActionStatus::Failed | ActionStatus::Blocked) {
        ensure!(
            input
                .error_message
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty()),
            "failed or blocked actions require an error message"
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_persist_action_record_writes_manifest_and_audit() {
        let dir = tempdir().unwrap();
        let artifact_path = dir.path().join("artifact.mid");
        fs::write(&artifact_path, b"midi").unwrap();

        let audit = persist_action_record(
            dir.path(),
            NewActionRecord {
                action: "generate_midi".to_string(),
                actor_id: "local-cli".to_string(),
                transport: ActionTransport::Cli,
                target: Some("demo".to_string()),
                status: ActionStatus::Succeeded,
                input: json!({ "preset": "demo", "seed": 7 }),
                output: Some(json!({ "path": artifact_path })),
                metadata: Some(json!({ "kind": "render" })),
                preset_name: Some("demo".to_string()),
                preset_hash: Some("preset-hash".to_string()),
                seed: Some(7),
                approval_ids: Vec::new(),
                artifacts: vec![ManifestArtifactInput {
                    kind: "midi".to_string(),
                    path: artifact_path.clone(),
                }],
                error_message: None,
            },
        )
        .unwrap();

        let manifest =
            inspect_run_manifest(&default_manifest_dir(dir.path()), &audit.run_id).unwrap();
        let events = read_audit_events(&default_audit_log_path(dir.path())).unwrap();

        assert_eq!(manifest.status, ActionStatus::Succeeded);
        assert_eq!(manifest.artifacts.len(), 1);
        assert_eq!(manifest.artifacts[0].kind, "midi");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].run_id.as_deref(), Some(audit.run_id.as_str()));
    }

    #[test]
    fn test_read_audit_events_preserves_append_order() {
        let dir = tempdir().unwrap();
        persist_action_record(
            dir.path(),
            NewActionRecord {
                action: "first".to_string(),
                actor_id: "cli".to_string(),
                transport: ActionTransport::Cli,
                target: Some("one".to_string()),
                status: ActionStatus::Succeeded,
                input: json!({ "value": 1 }),
                output: Some(json!({ "ok": true })),
                metadata: None,
                preset_name: None,
                preset_hash: None,
                seed: None,
                approval_ids: Vec::new(),
                artifacts: Vec::new(),
                error_message: None,
            },
        )
        .unwrap();
        persist_action_record(
            dir.path(),
            NewActionRecord {
                action: "second".to_string(),
                actor_id: "cli".to_string(),
                transport: ActionTransport::Cli,
                target: Some("two".to_string()),
                status: ActionStatus::Blocked,
                input: json!({ "value": 2 }),
                output: None,
                metadata: None,
                preset_name: None,
                preset_hash: None,
                seed: None,
                approval_ids: Vec::new(),
                artifacts: Vec::new(),
                error_message: Some("blocked by policy".to_string()),
            },
        )
        .unwrap();

        let events = read_audit_events(&default_audit_log_path(dir.path())).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].action, "first");
        assert_eq!(events[1].action, "second");
        assert_eq!(events[1].status, ActionStatus::Blocked);
    }
}
