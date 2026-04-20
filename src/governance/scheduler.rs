use std::path::{Path, PathBuf};

use anyhow::{ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    consume_approval_token, create_harness_plan_with_policy, current_unix_seconds,
    default_approval_store_path, default_harness_store_path, default_scheduler_export_dir,
    default_scheduler_store_path, execute_harness_action, new_runtime_id, read_json_or_default,
    sha256_hex, write_pretty_json, ExecuteHarnessActionRequest, HarnessOutcomeStatus, HarnessRole,
    NewHarnessPlanRequest, OrchestrationPolicy,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerBackend {
    LocalCli,
    Hermes,
    Openclaw,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduledJobStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewScheduledJobRequest {
    pub name: String,
    pub backend: SchedulerBackend,
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub adapter_id: Option<String>,
    pub run_ids: Vec<String>,
    pub requested_by: String,
    pub retry_limit: u8,
    pub approval_token: String,
    #[serde(default)]
    pub max_dispatches: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ValidateScheduledJobRequest {
    pub backend: SchedulerBackend,
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub adapter_id: Option<String>,
    pub run_ids: Vec<String>,
    pub retry_limit: u8,
    #[serde(default)]
    pub max_dispatches: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct CancelScheduledJobRequest {
    pub job_id: String,
    pub requested_by: String,
    pub approval_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScheduledJobConfig {
    pub backend: SchedulerBackend,
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub adapter_id: Option<String>,
    pub run_ids: Vec<String>,
    pub retry_limit: u8,
    #[serde(default)]
    pub max_dispatches: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct SchedulerAdapterBundle {
    pub backend: SchedulerBackend,
    pub config_hash: String,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct JobRunRecord {
    pub run_id: String,
    pub started_at_unix_seconds: u64,
    pub finished_at_unix_seconds: u64,
    pub status: ScheduledJobStatus,
    pub plan_id: String,
    pub outcome_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScheduledJobRecord {
    pub job_id: String,
    pub name: String,
    pub requested_by: String,
    pub status: ScheduledJobStatus,
    pub created_at_unix_seconds: u64,
    pub updated_at_unix_seconds: u64,
    pub approval_id: String,
    pub config_hash: String,
    pub config: ScheduledJobConfig,
    pub export_path: PathBuf,
    pub adapter_bundle: SchedulerAdapterBundle,
    pub runs: Vec<JobRunRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct JobValidationResult {
    pub allowed: bool,
    pub warnings: Vec<String>,
    pub config_hash: String,
    pub backend: SchedulerBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct JobRunSummary {
    pub job: ScheduledJobRecord,
    pub plan_id: String,
    pub outcome_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SchedulerStoreFile {
    version: u32,
    jobs: Vec<ScheduledJobRecord>,
}

pub fn validate_scheduled_job(
    runtime_dir: &Path,
    request: ValidateScheduledJobRequest,
) -> Result<JobValidationResult> {
    let config = ScheduledJobConfig {
        backend: request.backend,
        role: request.role,
        prompt: request.prompt,
        session_id: request.session_id,
        deck_id: request.deck_id,
        adapter_id: request.adapter_id,
        run_ids: request.run_ids,
        retry_limit: request.retry_limit,
        max_dispatches: request.max_dispatches,
    };
    validate_config(runtime_dir, &config)?;
    Ok(JobValidationResult {
        allowed: true,
        warnings: warnings_for_config(&config),
        config_hash: config_hash(&config)?,
        backend: config.backend,
    })
}

pub fn schedule_job(
    runtime_dir: &Path,
    request: NewScheduledJobRequest,
) -> Result<ScheduledJobRecord> {
    validate_new_job_request(runtime_dir, &request)?;
    let config = ScheduledJobConfig {
        backend: request.backend,
        role: request.role,
        prompt: request.prompt,
        session_id: request.session_id,
        deck_id: request.deck_id,
        adapter_id: request.adapter_id,
        run_ids: request.run_ids,
        retry_limit: request.retry_limit,
        max_dispatches: request.max_dispatches,
    };
    let config_hash = config_hash(&config)?;
    let job_id = new_runtime_id("job");
    let approval = consume_approval_token(
        &default_approval_store_path(runtime_dir),
        &request.approval_token,
        "jobs.schedule",
        &request.name,
    )?;
    let export_dir = default_scheduler_export_dir(runtime_dir);
    let export_path = export_dir.join(format!("{job_id}.json"));
    let adapter_bundle = adapter_bundle(runtime_dir, &job_id, &config_hash, config.backend);
    write_pretty_json(
        &export_path,
        &json!({
            "job_id": job_id,
            "name": request.name,
            "config_hash": config_hash,
            "adapter_bundle": adapter_bundle,
            "config": config,
        }),
    )?;

    let record = ScheduledJobRecord {
        job_id: job_id.clone(),
        name: request.name,
        requested_by: request.requested_by,
        status: ScheduledJobStatus::Scheduled,
        created_at_unix_seconds: current_unix_seconds(),
        updated_at_unix_seconds: current_unix_seconds(),
        approval_id: approval.approval_id,
        config_hash,
        config,
        export_path,
        adapter_bundle,
        runs: Vec::new(),
    };

    let store_path = default_scheduler_store_path(runtime_dir);
    let mut store = load_store(&store_path)?;
    store.jobs.push(record.clone());
    save_store(&store_path, &store)?;
    Ok(record)
}

pub fn inspect_scheduled_job(runtime_dir: &Path, job_id: &str) -> Result<ScheduledJobRecord> {
    ensure!(!job_id.trim().is_empty(), "job id cannot be empty");
    let store = load_store(&default_scheduler_store_path(runtime_dir))?;
    store
        .jobs
        .into_iter()
        .find(|job| job.job_id == job_id)
        .ok_or_else(|| anyhow::anyhow!("scheduled job '{}' was not found", job_id))
}

pub fn list_scheduled_jobs(runtime_dir: &Path) -> Result<Vec<ScheduledJobRecord>> {
    let mut store = load_store(&default_scheduler_store_path(runtime_dir))?;
    store.jobs.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
    });
    Ok(store.jobs)
}

pub fn cancel_scheduled_job(
    runtime_dir: &Path,
    request: CancelScheduledJobRequest,
) -> Result<ScheduledJobRecord> {
    ensure!(!request.job_id.trim().is_empty(), "job id cannot be empty");
    ensure!(
        !request.requested_by.trim().is_empty(),
        "requested_by cannot be empty"
    );
    let store_path = default_scheduler_store_path(runtime_dir);
    let mut store = load_store(&store_path)?;
    let job = store
        .jobs
        .iter_mut()
        .find(|job| job.job_id == request.job_id)
        .ok_or_else(|| anyhow::anyhow!("scheduled job '{}' was not found", request.job_id))?;
    ensure!(
        !matches!(
            job.status,
            ScheduledJobStatus::Completed | ScheduledJobStatus::Cancelled
        ),
        "scheduled job '{}' cannot be cancelled from state '{:?}'",
        request.job_id,
        job.status
    );
    consume_approval_token(
        &default_approval_store_path(runtime_dir),
        &request.approval_token,
        "jobs.cancel",
        &job.job_id,
    )?;
    job.status = ScheduledJobStatus::Cancelled;
    job.updated_at_unix_seconds = current_unix_seconds();
    let updated = job.clone();
    save_store(&store_path, &store)?;
    Ok(updated)
}

pub fn run_scheduled_job(
    runtime_dir: &Path,
    preset_dir: &Path,
    job_id: &str,
) -> Result<JobRunSummary> {
    ensure!(!job_id.trim().is_empty(), "job id cannot be empty");
    let store_path = default_scheduler_store_path(runtime_dir);
    let mut store = load_store(&store_path)?;
    let job = store
        .jobs
        .iter_mut()
        .find(|job| job.job_id == job_id)
        .ok_or_else(|| anyhow::anyhow!("scheduled job '{}' was not found", job_id))?;
    ensure!(
        !matches!(job.status, ScheduledJobStatus::Cancelled),
        "scheduled job '{}' is cancelled",
        job_id
    );
    ensure!(
        job.runs.len() <= usize::from(job.config.retry_limit),
        "scheduled job '{}' exceeded retry limit",
        job_id
    );

    let started_at = current_unix_seconds();
    job.status = ScheduledJobStatus::Running;
    job.updated_at_unix_seconds = started_at;

    let mut policy = OrchestrationPolicy::for_scheduled_job();
    if let Some(max_dispatches) = job.config.max_dispatches {
        policy.max_dispatches_per_job_run = max_dispatches;
    }

    let plan = create_harness_plan_with_policy(
        &default_harness_store_path(runtime_dir),
        runtime_dir,
        NewHarnessPlanRequest {
            role: job.config.role,
            prompt: job.config.prompt.clone(),
            session_id: job.config.session_id.clone(),
            deck_id: job.config.deck_id.clone(),
            adapter_id: job.config.adapter_id.clone(),
            run_ids: job.config.run_ids.clone(),
            max_actions: None,
        },
        policy.clone(),
    )?;

    let mut outcome_ids = Vec::new();
    let mut dispatch_count = 0usize;
    let mut final_status = ScheduledJobStatus::Completed;
    for action in &plan.proposed_actions {
        if action.tool_name.starts_with("realtime.") {
            dispatch_count += 1;
            if let Err(violation) = policy.validate_dispatch_count(dispatch_count) {
                final_status = ScheduledJobStatus::Failed;
                outcome_ids.push(format!("policy-violation: {}", violation));
                break;
            }
        }
        let outcome = execute_harness_action(
            &default_harness_store_path(runtime_dir),
            runtime_dir,
            preset_dir,
            ExecuteHarnessActionRequest {
                plan_id: plan.plan_id.clone(),
                action_id: action.action_id.clone(),
            },
        )?;
        outcome_ids.push(outcome.outcome_id.clone());
        if !matches!(outcome.status, HarnessOutcomeStatus::Succeeded) {
            final_status = ScheduledJobStatus::Failed;
            break;
        }
    }

    let finished_at = current_unix_seconds();
    let run_record = JobRunRecord {
        run_id: new_runtime_id("job-run"),
        started_at_unix_seconds: started_at,
        finished_at_unix_seconds: finished_at,
        status: final_status,
        plan_id: plan.plan_id.clone(),
        outcome_ids: outcome_ids.clone(),
    };
    job.status = final_status;
    job.updated_at_unix_seconds = current_unix_seconds();
    job.runs.push(run_record);
    let updated = job.clone();
    save_store(&store_path, &store)?;

    Ok(JobRunSummary {
        job: updated,
        plan_id: plan.plan_id,
        outcome_ids,
    })
}

fn validate_new_job_request(runtime_dir: &Path, request: &NewScheduledJobRequest) -> Result<()> {
    ensure!(!request.name.trim().is_empty(), "job name cannot be empty");
    ensure!(
        !request.requested_by.trim().is_empty(),
        "requested_by cannot be empty"
    );
    ensure!(
        !request.approval_token.trim().is_empty(),
        "approval token cannot be empty"
    );
    validate_config(
        runtime_dir,
        &ScheduledJobConfig {
            backend: request.backend,
            role: request.role,
            prompt: request.prompt.clone(),
            session_id: request.session_id.clone(),
            deck_id: request.deck_id.clone(),
            adapter_id: request.adapter_id.clone(),
            run_ids: request.run_ids.clone(),
            retry_limit: request.retry_limit,
            max_dispatches: request.max_dispatches,
        },
    )
}

fn validate_config(runtime_dir: &Path, config: &ScheduledJobConfig) -> Result<()> {
    ensure!(!config.prompt.trim().is_empty(), "prompt cannot be empty");
    ensure!(config.retry_limit > 0, "retry_limit must be at least one");
    if let Some(session_id) = &config.session_id {
        super::inspect_session(&super::default_session_store_path(runtime_dir), session_id)?;
    }
    if let Some(deck_id) = &config.deck_id {
        super::inspect_deck_transport(&super::default_daw_store_path(runtime_dir), deck_id)?;
    }
    for run_id in &config.run_ids {
        super::inspect_run_manifest(&super::default_manifest_dir(runtime_dir), run_id)?;
    }
    Ok(())
}

fn warnings_for_config(config: &ScheduledJobConfig) -> Vec<String> {
    let mut warnings = Vec::new();
    if matches!(config.role, HarnessRole::Publisher) {
        warnings.push("publisher jobs remain blocked without a publish backend".to_string());
    }
    if matches!(
        config.backend,
        SchedulerBackend::Hermes | SchedulerBackend::Openclaw
    ) {
        warnings.push(
            "exported scheduler bundles are adapter-friendly manifests; they do not create remote jobs by themselves".to_string(),
        );
    }
    warnings
}

fn config_hash(config: &ScheduledJobConfig) -> Result<String> {
    let raw = serde_json::to_vec(config)?;
    Ok(sha256_hex(&raw))
}

fn adapter_bundle(
    runtime_dir: &Path,
    job_id: &str,
    config_hash: &str,
    backend: SchedulerBackend,
) -> SchedulerAdapterBundle {
    SchedulerAdapterBundle {
        backend,
        config_hash: config_hash.to_string(),
        command: "cargo".to_string(),
        args: vec![
            "run".to_string(),
            "--".to_string(),
            "job-run".to_string(),
            "--job-id".to_string(),
            job_id.to_string(),
            "--runtime-dir".to_string(),
            runtime_dir.display().to_string(),
        ],
    }
}

fn load_store(store_path: &Path) -> Result<SchedulerStoreFile> {
    let mut store: SchedulerStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &SchedulerStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::generation::{demo_preset, save_preset};
    use crate::governance::{
        create_session, request_approval, resolve_approval, ActionRisk, ApprovalDecisionKind,
        NewApprovalRequest, NewSessionRequest,
    };

    #[test]
    fn test_validate_schedule_and_run_job() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let approval_store = default_approval_store_path(&runtime_dir);

        let mut preset = demo_preset();
        preset.name = "job-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let session = create_session(
            &super::super::default_session_store_path(&runtime_dir),
            &preset_dir,
            NewSessionRequest {
                display_name: "Job Session".to_string(),
                preset_name: "job-demo".to_string(),
                seed: 4,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();

        let validation = validate_scheduled_job(
            &runtime_dir,
            ValidateScheduledJobRequest {
                backend: SchedulerBackend::LocalCli,
                role: HarnessRole::SessionDj,
                prompt: "set tempo to 132 and render a preview".to_string(),
                session_id: Some(session.session_id.clone()),
                deck_id: None,
                adapter_id: None,
                run_ids: Vec::new(),
                retry_limit: 1,
                max_dispatches: None,
            },
        )
        .unwrap();
        assert!(validation.allowed);

        let approval = request_approval(
            &approval_store,
            NewApprovalRequest {
                action_scope: "jobs.schedule".to_string(),
                target: "nightly-preview".to_string(),
                requested_by: "tester".to_string(),
                reason: "schedule unattended run".to_string(),
                risk: ActionRisk::ApprovalRequired,
            },
        )
        .unwrap();
        let resolution = resolve_approval(
            &approval_store,
            &approval.approval_id,
            ApprovalDecisionKind::Approve,
            "approver",
            "approved",
            600,
        )
        .unwrap();

        let job = schedule_job(
            &runtime_dir,
            NewScheduledJobRequest {
                name: "nightly-preview".to_string(),
                backend: SchedulerBackend::LocalCli,
                role: HarnessRole::SessionDj,
                prompt: "set tempo to 132 and render a preview".to_string(),
                session_id: Some(session.session_id.clone()),
                deck_id: None,
                adapter_id: None,
                run_ids: Vec::new(),
                requested_by: "tester".to_string(),
                retry_limit: 1,
                approval_token: resolution.approval_token.unwrap(),
                max_dispatches: None,
            },
        )
        .unwrap();
        assert!(job.export_path.exists());

        let run = run_scheduled_job(&runtime_dir, &preset_dir, &job.job_id).unwrap();
        assert_eq!(run.job.runs.len(), 1);
        assert!(!run.outcome_ids.is_empty());
    }
}
