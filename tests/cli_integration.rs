use std::fs;
use std::process::Command;

use hound::WavReader;
use midly::Smf;
use serde_json::Value;
use state_space_music_box::{
    default_audit_log_path, default_manifest_dir, list_run_manifests, read_audit_events,
    ActionStatus,
};
use tempfile::tempdir;

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_state-space-music-box")
}

#[test]
fn generate_demo_command_writes_valid_artifacts() {
    let dir = tempdir().unwrap();
    let midi_path = dir.path().join("artifacts/demo.mid");
    let wav_path = dir.path().join("artifacts/demo.wav");
    let runtime_dir = dir.path().join("runtime");

    let output = Command::new(binary())
        .args([
            "generate-demo",
            "--midi",
            midi_path.to_str().unwrap(),
            "--wav",
            wav_path.to_str().unwrap(),
            "--seed",
            "3",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let midi_bytes = fs::read(&midi_path).unwrap();
    let midi = Smf::parse(&midi_bytes).unwrap();
    let wav = WavReader::open(&wav_path).unwrap();

    assert_eq!(midi.tracks.len(), 1);
    assert!(wav.duration() > 0);

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    let audit_events = read_audit_events(&default_audit_log_path(&runtime_dir)).unwrap();
    assert_eq!(manifests.len(), 1);
    assert_eq!(audit_events.len(), 1);
    assert_eq!(manifests[0].status, ActionStatus::Succeeded);
    assert_eq!(manifests[0].artifacts.len(), 2);
    assert_eq!(audit_events[0].status, ActionStatus::Succeeded);
}

#[test]
fn generate_midi_command_fails_for_directory_output_path() {
    let dir = tempdir().unwrap();
    let output_dir = dir.path().join("directory-output");
    let runtime_dir = dir.path().join("runtime");
    fs::create_dir(&output_dir).unwrap();

    let output = Command::new(binary())
        .args([
            "generate-midi",
            "--preset",
            "demo",
            "--output",
            output_dir.to_str().unwrap(),
            "--seed",
            "1",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    let audit_events = read_audit_events(&default_audit_log_path(&runtime_dir)).unwrap();
    assert_eq!(manifests.len(), 1);
    assert_eq!(audit_events.len(), 1);
    assert_eq!(manifests[0].status, ActionStatus::Failed);
    assert_eq!(audit_events[0].status, ActionStatus::Failed);
}

#[test]
fn dataset_register_command_consumes_approval_token() {
    let dir = tempdir().unwrap();
    let runtime_dir = dir.path().join("runtime");

    let request_output = Command::new(binary())
        .args([
            "approval-request",
            "--action-scope",
            "dataset.register",
            "--target",
            "pdmx",
            "--requested-by",
            "tester",
            "--reason",
            "register dataset",
            "--risk",
            "approval-required",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(request_output.status.success());
    let approval: Value = serde_json::from_slice(&request_output.stdout).unwrap();

    let approval_id = approval["approval_id"].as_str().unwrap();
    let resolve_output = Command::new(binary())
        .args([
            "approval-resolve",
            "--approval-id",
            approval_id,
            "--operator-id",
            "approver",
            "--decision",
            "approve",
            "--reason",
            "approved",
            "--expires-in-seconds",
            "600",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(resolve_output.status.success());
    let resolution: Value = serde_json::from_slice(&resolve_output.stdout).unwrap();
    let token = resolution["approval_token"].as_str().unwrap();

    let dataset_dir = dir.path().join("datasets/pdmx");
    let register_output = Command::new(binary())
        .args([
            "dataset-register",
            "--dataset-id",
            "pdmx",
            "--display-name",
            "PDMX",
            "--source-url",
            "https://example.com/pdmx",
            "--license-name",
            "CC-BY-4.0",
            "--commercial-use-status",
            "allowed",
            "--redistribution-status",
            "allowed",
            "--approved-use-class",
            "production-allowed",
            "--checksum",
            "archive.tar.gz=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "--local-storage-path",
            dataset_dir.to_str().unwrap(),
            "--dataset-version",
            "v1",
            "--approval-token",
            token,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        register_output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&register_output.stdout),
        String::from_utf8_lossy(&register_output.stderr)
    );

    let registered: Value = serde_json::from_slice(&register_output.stdout).unwrap();
    assert_eq!(registered["dataset_id"].as_str().unwrap(), "pdmx");

    let second_register = Command::new(binary())
        .args([
            "dataset-register",
            "--dataset-id",
            "pdmx-second",
            "--display-name",
            "PDMX Second",
            "--source-url",
            "https://example.com/pdmx2",
            "--license-name",
            "CC-BY-4.0",
            "--commercial-use-status",
            "allowed",
            "--redistribution-status",
            "allowed",
            "--approved-use-class",
            "production-allowed",
            "--checksum",
            "archive.tar.gz=0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "--local-storage-path",
            dataset_dir.to_str().unwrap(),
            "--dataset-version",
            "v1",
            "--approval-token",
            token,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(!second_register.status.success());

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    let audit_events = read_audit_events(&default_audit_log_path(&runtime_dir)).unwrap();
    assert_eq!(manifests.len(), 4);
    assert_eq!(audit_events.len(), 4);
    assert_eq!(
        manifests
            .iter()
            .filter(|manifest| manifest.action == "dataset_register"
                && manifest.status == ActionStatus::Succeeded)
            .count(),
        1
    );
    assert_eq!(
        manifests
            .iter()
            .filter(|manifest| manifest.action == "dataset_register"
                && manifest.status == ActionStatus::Blocked)
            .count(),
        1
    );
}

#[test]
fn snapshot_commands_restore_previous_preset_contents() {
    let dir = tempdir().unwrap();
    let preset_dir = dir.path().join("presets");
    let runtime_dir = dir.path().join("runtime");
    fs::create_dir_all(&preset_dir).unwrap();

    let preset_path = preset_dir.join("custom.json");
    fs::write(
        &preset_path,
        r#"{
  "name": "custom",
  "description": "custom preset",
  "system": {
    "a": { "rows": 2, "cols": 2, "data": [0.0, 1.0, -1.0, -0.15] },
    "b": { "rows": 2, "cols": 0, "data": [] },
    "c": { "rows": 1, "cols": 2, "data": [1.0, 0.0] },
    "d": { "rows": 1, "cols": 0, "data": [] },
    "dt": null
  },
  "simulation": {
    "duration_seconds": 8.0,
    "trajectory_sample_rate": 256,
    "initial_state": [1.0, 0.0],
    "input": []
  },
  "midi": {
    "tempo_bpm": 120,
    "ticks_per_beat": 480,
    "channel": 0,
    "default_velocity": 96,
    "low_note": 48,
    "high_note": 84,
    "step_beats": 0.5,
    "root_note": 60,
    "scale": [0, 2, 4, 7, 9],
    "seed_variation_semitones": 4
  },
  "audio": {
    "sample_rate": 44100,
    "peak_limit": 0.85,
    "attack_seconds": 0.01,
    "release_seconds": 0.05
  }
}"#,
    )
    .unwrap();

    let snapshot_output = Command::new(binary())
        .args([
            "snapshot-create",
            "--preset",
            "custom",
            "--reason",
            "before mutation",
            "--preset-dir",
            preset_dir.to_str().unwrap(),
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(snapshot_output.status.success());
    let snapshot: Value = serde_json::from_slice(&snapshot_output.stdout).unwrap();
    let snapshot_id = snapshot["snapshot_id"].as_str().unwrap();

    let mutated = fs::read_to_string(&preset_path)
        .unwrap()
        .replace("\"tempo_bpm\": 120", "\"tempo_bpm\": 90");
    fs::write(&preset_path, mutated).unwrap();

    let rollback_output = Command::new(binary())
        .args([
            "snapshot-rollback",
            "--snapshot-id",
            snapshot_id,
            "--preset-dir",
            preset_dir.to_str().unwrap(),
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(rollback_output.status.success());

    let restored = fs::read_to_string(&preset_path).unwrap();
    assert!(restored.contains("\"tempo_bpm\": 120"));

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    let audit_events = read_audit_events(&default_audit_log_path(&runtime_dir)).unwrap();
    assert_eq!(manifests.len(), 2);
    assert_eq!(audit_events.len(), 2);
    assert!(manifests
        .iter()
        .all(|manifest| manifest.status == ActionStatus::Succeeded));
}

#[test]
fn session_and_evaluation_commands_persist_records() {
    let dir = tempdir().unwrap();
    let runtime_dir = dir.path().join("runtime");
    let midi_path = dir.path().join("artifacts/demo.mid");

    let render_output = Command::new(binary())
        .args([
            "generate-midi",
            "--preset",
            "demo",
            "--output",
            midi_path.to_str().unwrap(),
            "--seed",
            "5",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(render_output.status.success());
    let render_json: Value = serde_json::from_slice(&render_output.stdout).unwrap();
    let run_id = render_json["audit"]["run_id"].as_str().unwrap().to_string();

    let session_create = Command::new(binary())
        .args([
            "session-create",
            "--display-name",
            "Local Eval Session",
            "--preset",
            "demo",
            "--seed",
            "5",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(session_create.status.success());
    let session_json: Value = serde_json::from_slice(&session_create.stdout).unwrap();
    let session_id = session_json["session_id"].as_str().unwrap();

    let session_update = Command::new(binary())
        .args([
            "session-update",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--tempo-bpm",
            "90",
            "--status",
            "playing",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(session_update.status.success());

    let evaluation_submit = Command::new(binary())
        .args([
            "evaluation-submit",
            "--run-id",
            &run_id,
            "--metric",
            "note_density=0.8",
            "--human-score",
            "musicality=6",
            "--weight",
            "musicality=1.0",
            "--decision",
            "promote",
            "--created-by",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(evaluation_submit.status.success());

    let evaluations = Command::new(binary())
        .args([
            "evaluation-list",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(evaluations.status.success());
    let evaluations_json: Value = serde_json::from_slice(&evaluations.stdout).unwrap();
    assert_eq!(evaluations_json.as_array().unwrap().len(), 1);

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    let audit_events = read_audit_events(&default_audit_log_path(&runtime_dir)).unwrap();
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "session_create"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "session_update"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "evaluation_submit"));
    assert!(audit_events
        .iter()
        .any(|event| event.action == "evaluation_submit"));
}

#[test]
fn session_preview_and_review_commands_emit_real_artifacts() {
    let dir = tempdir().unwrap();
    let runtime_dir = dir.path().join("runtime");
    let midi_path_a = dir.path().join("artifacts/a.mid");
    let midi_path_b = dir.path().join("artifacts/b.mid");

    let render_a = Command::new(binary())
        .args([
            "generate-midi",
            "--preset",
            "demo",
            "--output",
            midi_path_a.to_str().unwrap(),
            "--seed",
            "5",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(render_a.status.success());
    let render_a_json: Value = serde_json::from_slice(&render_a.stdout).unwrap();
    let run_a = render_a_json["audit"]["run_id"]
        .as_str()
        .unwrap()
        .to_string();

    let render_b = Command::new(binary())
        .args([
            "generate-midi",
            "--preset",
            "demo",
            "--output",
            midi_path_b.to_str().unwrap(),
            "--seed",
            "8",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(render_b.status.success());
    let render_b_json: Value = serde_json::from_slice(&render_b.stdout).unwrap();
    let run_b = render_b_json["audit"]["run_id"]
        .as_str()
        .unwrap()
        .to_string();

    let session_create = Command::new(binary())
        .args([
            "session-create",
            "--display-name",
            "Preview Session",
            "--preset",
            "demo",
            "--seed",
            "5",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(session_create.status.success());
    let session_json: Value = serde_json::from_slice(&session_create.stdout).unwrap();
    let session_id = session_json["session_id"].as_str().unwrap();

    let play = Command::new(binary())
        .args([
            "session-play",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--run-label",
            "pass-1",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(play.status.success());

    let preview = Command::new(binary())
        .args([
            "session-render-preview",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(preview.status.success());
    let preview_json: Value = serde_json::from_slice(&preview.stdout).unwrap();
    assert!(preview_json["preview"]["preview_id"].as_str().is_some());
    let preview_midi = preview_json["preview"]["midi"]["path"].as_str().unwrap();
    let preview_wav = preview_json["preview"]["wav"]["path"].as_str().unwrap();
    assert!(std::path::Path::new(preview_midi).exists());
    assert!(std::path::Path::new(preview_wav).exists());

    let stop = Command::new(binary())
        .args([
            "session-stop",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(stop.status.success());

    let deck_create = Command::new(binary())
        .args([
            "deck-create",
            "--display-name",
            "Deck A",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_create.status.success());
    let deck_create_json: Value = serde_json::from_slice(&deck_create.stdout).unwrap();
    let deck_id = deck_create_json["deck_id"].as_str().unwrap();

    let deck_add = Command::new(binary())
        .args([
            "deck-add-preview",
            "--deck-id",
            deck_id,
            "--session-id",
            session_id,
            "--preview-id",
            preview_json["preview"]["preview_id"].as_str().unwrap(),
            "--label",
            "Intro Clip",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_add.status.success());
    let deck_add_json: Value = serde_json::from_slice(&deck_add.stdout).unwrap();
    let clip_id = deck_add_json["clips"][0]["clip_id"].as_str().unwrap();

    let deck_queue = Command::new(binary())
        .args([
            "deck-queue",
            "--deck-id",
            deck_id,
            "--clip-id",
            clip_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_queue.status.success());

    let deck_launch = Command::new(binary())
        .args([
            "deck-launch",
            "--deck-id",
            deck_id,
            "--clip-id",
            clip_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_launch.status.success());

    let deck_transport = Command::new(binary())
        .args([
            "deck-transport",
            "--deck-id",
            deck_id,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_transport.status.success());
    let deck_transport_json: Value = serde_json::from_slice(&deck_transport.stdout).unwrap();
    assert_eq!(
        deck_transport_json["deck"]["transport_state"]
            .as_str()
            .unwrap(),
        "playing"
    );

    let review = Command::new(binary())
        .args([
            "review-build",
            "--run-id",
            &run_a,
            "--run-id",
            &run_b,
            "--output",
            "comparison.json",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(review.status.success());
    let review_json: Value = serde_json::from_slice(&review.stdout).unwrap();
    assert_eq!(review_json["runs"].as_array().unwrap().len(), 2);
    assert!(review_json["export"]["path"]
        .as_str()
        .unwrap()
        .ends_with("comparison.json"));

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "session_play"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "session_render_preview"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "session_stop"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "deck_create"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "deck_add_preview"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "deck_launch"));
}

#[test]
fn harness_commands_plan_and_execute_real_backend_actions() {
    let dir = tempdir().unwrap();
    let runtime_dir = dir.path().join("runtime");

    let session_create = Command::new(binary())
        .args([
            "session-create",
            "--display-name",
            "Harness Session",
            "--preset",
            "demo",
            "--seed",
            "5",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(session_create.status.success());
    let session_json: Value = serde_json::from_slice(&session_create.stdout).unwrap();
    let session_id = session_json["session_id"].as_str().unwrap();

    let plan = Command::new(binary())
        .args([
            "harness-plan",
            "--role",
            "session-dj",
            "--prompt",
            "set tempo to 132 and render a preview",
            "--session-id",
            session_id,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(plan.status.success());
    let plan_json: Value = serde_json::from_slice(&plan.stdout).unwrap();
    let plan_id = plan_json["plan_id"].as_str().unwrap();
    let action_id = plan_json["proposed_actions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["tool_name"] == "live.apply_patch")
        .unwrap()["action_id"]
        .as_str()
        .unwrap();

    let execute = Command::new(binary())
        .args([
            "harness-execute",
            "--plan-id",
            plan_id,
            "--action-id",
            action_id,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(execute.status.success());
    let outcome_json: Value = serde_json::from_slice(&execute.stdout).unwrap();
    assert_eq!(outcome_json["status"].as_str().unwrap(), "succeeded");
    assert!(outcome_json["rollback_handle"].is_object());

    let outcomes = Command::new(binary())
        .args([
            "harness-outcome-list",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(outcomes.status.success());
    let outcomes_json: Value = serde_json::from_slice(&outcomes.stdout).unwrap();
    assert_eq!(outcomes_json.as_array().unwrap().len(), 1);
}

#[test]
fn scheduler_commands_validate_schedule_run_and_cancel_jobs() {
    let dir = tempdir().unwrap();
    let runtime_dir = dir.path().join("runtime");

    let session_create = Command::new(binary())
        .args([
            "session-create",
            "--display-name",
            "Scheduled Session",
            "--preset",
            "demo",
            "--seed",
            "9",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(session_create.status.success());
    let session_json: Value = serde_json::from_slice(&session_create.stdout).unwrap();
    let session_id = session_json["session_id"].as_str().unwrap();

    let validate = Command::new(binary())
        .args([
            "job-validate",
            "--backend",
            "local-cli",
            "--role",
            "session-dj",
            "--prompt",
            "set tempo to 132 and render a preview",
            "--session-id",
            session_id,
            "--retry-limit",
            "1",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(validate.status.success());
    let validation_json: Value = serde_json::from_slice(&validate.stdout).unwrap();
    assert!(validation_json["allowed"].as_bool().unwrap());

    let schedule_request = Command::new(binary())
        .args([
            "approval-request",
            "--action-scope",
            "jobs.schedule",
            "--target",
            "nightly-preview",
            "--requested-by",
            "tester",
            "--reason",
            "schedule unattended run",
            "--risk",
            "approval-required",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(schedule_request.status.success());
    let schedule_approval: Value = serde_json::from_slice(&schedule_request.stdout).unwrap();

    let schedule_resolve = Command::new(binary())
        .args([
            "approval-resolve",
            "--approval-id",
            schedule_approval["approval_id"].as_str().unwrap(),
            "--operator-id",
            "approver",
            "--decision",
            "approve",
            "--reason",
            "approved",
            "--expires-in-seconds",
            "600",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(schedule_resolve.status.success());
    let schedule_resolution: Value = serde_json::from_slice(&schedule_resolve.stdout).unwrap();

    let schedule = Command::new(binary())
        .args([
            "job-schedule",
            "--job-name",
            "nightly-preview",
            "--backend",
            "local-cli",
            "--role",
            "session-dj",
            "--prompt",
            "set tempo to 132 and render a preview",
            "--session-id",
            session_id,
            "--requested-by",
            "tester",
            "--retry-limit",
            "1",
            "--approval-token",
            schedule_resolution["approval_token"].as_str().unwrap(),
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(schedule.status.success());
    let scheduled_job: Value = serde_json::from_slice(&schedule.stdout).unwrap();
    let job_id = scheduled_job["job_id"].as_str().unwrap();
    assert_eq!(scheduled_job["status"].as_str().unwrap(), "scheduled");
    assert!(scheduled_job["export_path"]
        .as_str()
        .unwrap()
        .ends_with(".json"));

    let run = Command::new(binary())
        .args([
            "job-run",
            "--job-id",
            job_id,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(run.status.success());
    let run_json: Value = serde_json::from_slice(&run.stdout).unwrap();
    assert_eq!(run_json["job"]["status"].as_str().unwrap(), "completed");
    assert_eq!(run_json["job"]["runs"].as_array().unwrap().len(), 1);
    assert!(!run_json["outcome_ids"].as_array().unwrap().is_empty());

    let jobs = Command::new(binary())
        .args(["job-list", "--runtime-dir", runtime_dir.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(jobs.status.success());
    let jobs_json: Value = serde_json::from_slice(&jobs.stdout).unwrap();
    assert_eq!(jobs_json.as_array().unwrap().len(), 1);

    let inspect = Command::new(binary())
        .args([
            "job-inspect",
            "--job-id",
            job_id,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(inspect.status.success());
    let inspect_json: Value = serde_json::from_slice(&inspect.stdout).unwrap();
    assert_eq!(inspect_json["job_id"].as_str().unwrap(), job_id);

    let schedule_cancel_request = Command::new(binary())
        .args([
            "approval-request",
            "--action-scope",
            "jobs.schedule",
            "--target",
            "cancel-me",
            "--requested-by",
            "tester",
            "--reason",
            "schedule cancellation candidate",
            "--risk",
            "approval-required",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(schedule_cancel_request.status.success());
    let schedule_cancel_approval: Value =
        serde_json::from_slice(&schedule_cancel_request.stdout).unwrap();

    let schedule_cancel_resolve = Command::new(binary())
        .args([
            "approval-resolve",
            "--approval-id",
            schedule_cancel_approval["approval_id"].as_str().unwrap(),
            "--operator-id",
            "approver",
            "--decision",
            "approve",
            "--reason",
            "approved",
            "--expires-in-seconds",
            "600",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(schedule_cancel_resolve.status.success());
    let schedule_cancel_resolution: Value =
        serde_json::from_slice(&schedule_cancel_resolve.stdout).unwrap();

    let cancel_candidate = Command::new(binary())
        .args([
            "job-schedule",
            "--job-name",
            "cancel-me",
            "--backend",
            "local-cli",
            "--role",
            "session-dj",
            "--prompt",
            "render a preview",
            "--session-id",
            session_id,
            "--requested-by",
            "tester",
            "--retry-limit",
            "1",
            "--approval-token",
            schedule_cancel_resolution["approval_token"]
                .as_str()
                .unwrap(),
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(cancel_candidate.status.success());
    let cancel_candidate_json: Value = serde_json::from_slice(&cancel_candidate.stdout).unwrap();
    let cancel_job_id = cancel_candidate_json["job_id"].as_str().unwrap();

    let cancel_request = Command::new(binary())
        .args([
            "approval-request",
            "--action-scope",
            "jobs.cancel",
            "--target",
            cancel_job_id,
            "--requested-by",
            "tester",
            "--reason",
            "cancel scheduled job before execution",
            "--risk",
            "approval-required",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(cancel_request.status.success());
    let cancel_approval: Value = serde_json::from_slice(&cancel_request.stdout).unwrap();

    let cancel_resolve = Command::new(binary())
        .args([
            "approval-resolve",
            "--approval-id",
            cancel_approval["approval_id"].as_str().unwrap(),
            "--operator-id",
            "approver",
            "--decision",
            "approve",
            "--reason",
            "approved",
            "--expires-in-seconds",
            "600",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(cancel_resolve.status.success());
    let cancel_resolution: Value = serde_json::from_slice(&cancel_resolve.stdout).unwrap();

    let cancel = Command::new(binary())
        .args([
            "job-cancel",
            "--job-id",
            cancel_job_id,
            "--requested-by",
            "tester",
            "--approval-token",
            cancel_resolution["approval_token"].as_str().unwrap(),
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(cancel.status.success());
    let cancel_json: Value = serde_json::from_slice(&cancel.stdout).unwrap();
    assert_eq!(cancel_json["status"].as_str().unwrap(), "cancelled");

    let manifests = list_run_manifests(&default_manifest_dir(&runtime_dir)).unwrap();
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "job_schedule"));
    assert!(
        manifests
            .iter()
            .filter(|manifest| manifest.action == "job_schedule")
            .count()
            >= 2
    );
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "job_run"));
    assert!(manifests
        .iter()
        .any(|manifest| manifest.action == "job_cancel"));
}

#[test]
fn realtime_commands_create_adapter_and_dispatch_preview_and_transport() {
    let dir = tempdir().unwrap();
    let runtime_dir = dir.path().join("runtime");
    let listener = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
    listener
        .set_read_timeout(Some(std::time::Duration::from_millis(250)))
        .unwrap();
    let port = listener.local_addr().unwrap().port();

    let adapter_create = Command::new(binary())
        .args([
            "realtime-create",
            "--display-name",
            "Loopback",
            "--host",
            "127.0.0.1",
            "--port",
            &port.to_string(),
            "--base-path",
            "/agentic_dj",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(adapter_create.status.success());
    let adapter_json: Value = serde_json::from_slice(&adapter_create.stdout).unwrap();
    let adapter_id = adapter_json["adapter_id"].as_str().unwrap();

    let session_create = Command::new(binary())
        .args([
            "session-create",
            "--display-name",
            "Realtime Session",
            "--preset",
            "demo",
            "--seed",
            "4",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(session_create.status.success());
    let session_json: Value = serde_json::from_slice(&session_create.stdout).unwrap();
    let session_id = session_json["session_id"].as_str().unwrap();

    let preview_render = Command::new(binary())
        .args([
            "session-render-preview",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(preview_render.status.success());
    let preview_json: Value = serde_json::from_slice(&preview_render.stdout).unwrap();
    let preview_id = preview_json["preview"]["preview_id"].as_str().unwrap();

    let preview_send = Command::new(binary())
        .args([
            "realtime-send-preview",
            "--adapter-id",
            adapter_id,
            "--session-id",
            session_id,
            "--preview-id",
            preview_id,
            "--actor-id",
            "tester",
            "--dispatch-mode",
            "immediate",
            "--time-scale",
            "0",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(preview_send.status.success());
    let preview_dispatch: Value = serde_json::from_slice(&preview_send.stdout).unwrap();
    assert!(
        preview_dispatch["dispatch"]["message_count"]
            .as_u64()
            .unwrap()
            >= 3
    );

    let deck_create = Command::new(binary())
        .args([
            "deck-create",
            "--display-name",
            "Realtime Deck",
            "--session-id",
            session_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_create.status.success());
    let deck_json: Value = serde_json::from_slice(&deck_create.stdout).unwrap();
    let deck_id = deck_json["deck_id"].as_str().unwrap();

    let deck_add = Command::new(binary())
        .args([
            "deck-add-preview",
            "--deck-id",
            deck_id,
            "--session-id",
            session_id,
            "--preview-id",
            preview_id,
            "--label",
            "Clip One",
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_add.status.success());
    let deck_add_json: Value = serde_json::from_slice(&deck_add.stdout).unwrap();
    let clip_id = deck_add_json["clips"][0]["clip_id"].as_str().unwrap();

    let deck_launch = Command::new(binary())
        .args([
            "deck-launch",
            "--deck-id",
            deck_id,
            "--clip-id",
            clip_id,
            "--actor-id",
            "tester",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(deck_launch.status.success());

    let transport_send = Command::new(binary())
        .args([
            "realtime-send-transport",
            "--adapter-id",
            adapter_id,
            "--deck-id",
            deck_id,
            "--actor-id",
            "tester",
            "--dispatch-mode",
            "immediate",
            "--time-scale",
            "0",
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(transport_send.status.success());
    let transport_dispatch: Value = serde_json::from_slice(&transport_send.stdout).unwrap();
    assert!(
        transport_dispatch["dispatch"]["message_count"]
            .as_u64()
            .unwrap()
            >= 1
    );

    let inspect = Command::new(binary())
        .args([
            "realtime-inspect",
            "--adapter-id",
            adapter_id,
            "--runtime-dir",
            runtime_dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(inspect.status.success());
    let inspect_json: Value = serde_json::from_slice(&inspect.stdout).unwrap();
    assert!(inspect_json["dispatches"].as_array().unwrap().len() >= 2);

    let mut buf = [0u8; 2048];
    let (size, _) = listener.recv_from(&mut buf).unwrap();
    let packet = rosc::decoder::decode_udp(&buf[..size]).unwrap().1;
    match packet {
        rosc::OscPacket::Message(message) => assert!(message.addr.starts_with("/agentic_dj/")),
        other => panic!("unexpected OSC packet: {other:?}"),
    }
}
