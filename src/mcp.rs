use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::{ensure, Context, Result};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, Json, ServerHandler, ServiceExt,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::info;

use crate::generation::{
    default_preset_dir, export_generated_midi, export_generated_wav, generate_composition,
    list_presets, load_preset, save_preset, simulate_trajectory, summarize_trajectory,
    RenderPreset, DEMO_PRESET_NAME,
};
use crate::governance::{
    add_preview_clip_to_deck, apply_transport_command, build_review_bundle, compare_runs,
    consume_approval_token, create_deck, create_harness_plan, create_preset_snapshot,
    create_session, default_approval_store_path, default_dataset_registry_path,
    default_daw_store_path, default_evaluation_store_path, default_harness_store_path,
    default_realtime_store_path, default_runtime_dir, default_scheduler_store_path,
    default_session_store_path, default_snapshot_dir, execute_harness_action,
    inspect_dataset_record, inspect_deck, inspect_deck_transport, inspect_evaluation_record,
    inspect_harness_plan, inspect_realtime_adapter, inspect_run_manifest, inspect_scheduled_job,
    inspect_session, launch_deck_clip, list_dataset_records, list_decks, list_evaluation_records,
    list_harness_outcomes, list_realtime_adapters, list_run_manifests, list_scheduled_jobs,
    list_sessions, persist_action_record, queue_deck_clip, read_audit_events,
    register_dataset_record, render_session_preview, request_approval, resolve_approval,
    rollback_preset_snapshot, run_scheduled_job, schedule_job, send_preview_to_realtime_adapter,
    send_transport_to_realtime_adapter, snapshot_preset_hash, stop_deck, submit_evaluation_record,
    update_session, validate_scheduled_job, ActionAuditRef, ActionStatus, ActionTransport,
    AddDeckPreviewRequest, ApprovalDecisionKind, ApprovalRequestRecord, ApprovalResolution,
    ApprovedUseClass, CancelScheduledJobRequest, ChecksumEntry, DeckRecord, DeckTransportSnapshot,
    EvaluationDecision, EvaluationRecord, ExecuteHarnessActionRequest, HarnessExecutionRecord,
    HarnessPlanRecord, HarnessRole, JobRunSummary, JobValidationResult, LaunchDeckClipRequest,
    ManifestArtifactInput, NewActionRecord, NewApprovalRequest, NewDatasetRecord, NewDeckRequest,
    NewEvaluationRecord, NewHarnessPlanRequest, NewRealtimeAdapterRequest, NewScheduledJobRequest,
    NewSessionRequest, PolicyStatus, PresetRollbackSummary, PresetSnapshotSummary,
    QueueDeckClipRequest, RealtimeAdapterProtocol, RealtimeAdapterRecord, RealtimeDispatchMode,
    RealtimeDispatchSummary, ReviewBundle, RunComparisonSummary, RunManifestRecord,
    ScheduledJobRecord, SchedulerBackend, SendRealtimePreviewRequest, SendRealtimeTransportRequest,
    SessionPreviewResult, SessionRecord, SessionStatus, SessionTransportCommand,
    SessionTransportRequest, StopDeckRequest, UpdateSessionRequest, ValidateScheduledJobRequest,
};
use crate::state_space::StateSpaceSystem;
use crate::vst_synthesizer::VstSynthesizer;

#[derive(Clone)]
pub struct MusicBoxMcpState {
    pub preset_dir: PathBuf,
    pub runtime_dir: PathBuf,
    pub state_space_systems: Arc<Mutex<HashMap<String, StateSpaceSystem>>>,
    pub vst_synthesizers: Arc<Mutex<HashMap<String, VstSynthesizer>>>,
}

impl MusicBoxMcpState {
    pub fn new(preset_dir: PathBuf, runtime_dir: PathBuf) -> Self {
        Self {
            preset_dir,
            runtime_dir,
            state_space_systems: Arc::new(Mutex::new(HashMap::new())),
            vst_synthesizers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for MusicBoxMcpState {
    fn default() -> Self {
        Self::new(default_preset_dir(), default_runtime_dir())
    }
}

#[derive(Clone)]
pub struct MusicBoxMcpServer {
    state: MusicBoxMcpState,
    tool_router: ToolRouter<Self>,
}

impl MusicBoxMcpServer {
    pub fn new(preset_dir: PathBuf, runtime_dir: PathBuf) -> Self {
        Self {
            state: MusicBoxMcpState::new(preset_dir, runtime_dir),
            tool_router: Self::tool_router(),
        }
    }

    #[cfg(test)]
    fn with_state(state: MusicBoxMcpState) -> Self {
        Self {
            state,
            tool_router: Self::tool_router(),
        }
    }

    fn system_from_request(
        &self,
        name: &str,
        request: &CreateSystemRequest,
    ) -> Result<StateSpaceSystem, String> {
        let system = StateSpaceSystem::new(
            request.a.to_matrix().map_err(|err| err.to_string())?,
            request.b.to_matrix().map_err(|err| err.to_string())?,
            request.c.to_matrix().map_err(|err| err.to_string())?,
            request.d.to_matrix().map_err(|err| err.to_string())?,
            request.dt,
        )
        .map_err(|err| err.to_string())?;

        let mut systems = self
            .state
            .state_space_systems
            .lock()
            .map_err(|_| "failed to lock state-space system store".to_string())?;
        systems.insert(name.to_string(), system.clone());
        Ok(system)
    }

    fn system_from_name(&self, name: &str) -> Result<StateSpaceSystem, String> {
        let systems = self
            .state
            .state_space_systems
            .lock()
            .map_err(|_| "failed to lock state-space system store".to_string())?;
        systems
            .get(name)
            .cloned()
            .ok_or_else(|| format!("state-space system '{name}' was not found"))
    }

    fn base_action(&self, action: &str, target: Option<String>, input: Value) -> NewActionRecord {
        NewActionRecord {
            action: action.to_string(),
            actor_id: "mcp-client".to_string(),
            transport: ActionTransport::Mcp,
            target,
            status: ActionStatus::Succeeded,
            input,
            output: None,
            metadata: None,
            preset_name: None,
            preset_hash: None,
            seed: None,
            approval_ids: Vec::new(),
            artifacts: Vec::new(),
            error_message: None,
        }
    }

    fn record_success<T>(
        &self,
        result: &T,
        mut action: NewActionRecord,
    ) -> Result<ActionAuditRef, String>
    where
        T: Serialize,
    {
        action.status = ActionStatus::Succeeded;
        action.output = Some(serde_json::to_value(result).map_err(|err| err.to_string())?);
        action.error_message = None;
        persist_action_record(&self.state.runtime_dir, action).map_err(|err| err.to_string())
    }

    fn record_failure(
        &self,
        error: &str,
        status: ActionStatus,
        mut action: NewActionRecord,
    ) -> Result<ActionAuditRef, String> {
        action.status = status;
        action.output = None;
        action.error_message = Some(error.to_string());
        persist_action_record(&self.state.runtime_dir, action).map_err(|err| err.to_string())
    }

    fn merge_record_error(action_error: String, record_error: String) -> String {
        format!(
            "{action_error}; additionally failed to persist manifest/audit records: {record_error}"
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MatrixInput {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl MatrixInput {
    fn to_matrix(&self) -> Result<nalgebra::DMatrix<f64>> {
        ensure!(
            self.data.len() == self.rows * self.cols,
            "matrix data length {} does not match {}x{}",
            self.data.len(),
            self.rows,
            self.cols
        );

        Ok(nalgebra::DMatrix::from_row_slice(
            self.rows, self.cols, &self.data,
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateSystemRequest {
    pub name: String,
    pub a: MatrixInput,
    pub b: MatrixInput,
    pub c: MatrixInput,
    pub d: MatrixInput,
    pub dt: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SystemSummary {
    pub name: String,
    pub state_dim: usize,
    pub input_dim: usize,
    pub output_dim: usize,
    pub dt: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePresetRequest {
    pub name: String,
    pub system_name: String,
    pub description: Option<String>,
    pub duration_seconds: Option<f64>,
    pub trajectory_sample_rate: Option<u32>,
    pub tempo_bpm: Option<u16>,
    pub audio_sample_rate: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePresetResponse {
    pub name: String,
    pub path: String,
    pub source_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListPresetsResponse {
    pub presets: Vec<PresetEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PresetEntry {
    pub name: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GenerateArtifactRequest {
    pub preset_name: String,
    pub output_path: String,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MidiArtifactResponse {
    pub path: String,
    pub note_count: usize,
    pub duration_beats: f64,
    pub tempo_bpm: u16,
    pub bytes_written: u64,
    pub artifact_hash: String,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WavArtifactResponse {
    pub path: String,
    pub sample_count: usize,
    pub duration_seconds: f64,
    pub peak_amplitude: f32,
    pub sample_rate: u32,
    pub artifact_hash: String,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InspectTrajectoryRequest {
    pub preset_name: Option<String>,
    pub system_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TrajectorySummaryResponse {
    pub frame_count: usize,
    pub duration_seconds: f64,
    pub min_output: f64,
    pub max_output: f64,
    pub mean_abs_output: f64,
    pub peak_abs_output: f64,
    pub preview: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DatasetListResponse {
    pub datasets: Vec<crate::governance::DatasetRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DatasetInspectRequest {
    pub dataset_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DatasetRegisterRequest {
    pub dataset_id: String,
    pub display_name: String,
    pub source_url: String,
    pub citation: Option<String>,
    pub license_name: String,
    pub commercial_use_status: PolicyStatus,
    pub redistribution_status: PolicyStatus,
    pub approved_use_class: ApprovedUseClass,
    pub checksum_manifest: Vec<ChecksumEntry>,
    pub local_storage_path: PathBuf,
    pub dataset_version: String,
    pub split_policy: Option<String>,
    pub transform_pipeline_hash: Option<String>,
    pub parent_datasets: Vec<String>,
    pub approval_token: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ApprovalResolveRequest {
    pub approval_id: String,
    pub operator_id: String,
    pub decision: ApprovalDecisionKind,
    pub reason: String,
    pub expires_in_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SnapshotCreateRequest {
    pub preset_name: String,
    pub reason: String,
    pub actor_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SnapshotRollbackRequest {
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RunInspectRequest {
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RunListResponse {
    pub runs: Vec<RunManifestRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditListRequest {
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditListResponse {
    pub events: Vec<crate::governance::AuditEventRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionCreateRequest {
    pub display_name: String,
    pub preset_name: String,
    pub seed: Option<u64>,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionInspectRequest {
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionUpdateRequest {
    pub session_id: String,
    pub actor_id: String,
    pub display_name: Option<String>,
    pub preset_name: Option<String>,
    pub seed: Option<u64>,
    pub tempo_bpm: Option<u16>,
    pub status: Option<SessionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionListResponse {
    pub sessions: Vec<SessionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionTransportToolRequest {
    pub session_id: String,
    pub actor_id: String,
    pub run_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SessionRenderPreviewRequest {
    pub session_id: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RunCompareRequest {
    pub run_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvaluationSubmitRequest {
    pub run_ids: Vec<String>,
    pub objective_metrics: std::collections::BTreeMap<String, f64>,
    pub human_scores: std::collections::BTreeMap<String, u8>,
    pub reward_weights: std::collections::BTreeMap<String, f64>,
    pub notes: Option<String>,
    pub decision: EvaluationDecision,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvaluationListResponse {
    pub evaluations: Vec<EvaluationRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EvaluationInspectRequest {
    pub evaluation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReviewBundleRequest {
    pub run_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckCreateRequest {
    pub display_name: String,
    pub session_id: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckInspectRequest {
    pub deck_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckListResponse {
    pub decks: Vec<DeckRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckAddPreviewToolRequest {
    pub deck_id: String,
    pub session_id: String,
    pub preview_id: String,
    pub label: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckQueueRequest {
    pub deck_id: String,
    pub clip_id: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckLaunchRequest {
    pub deck_id: String,
    pub clip_id: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeckStopRequest {
    pub deck_id: String,
    pub actor_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HarnessPlanRequest {
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub adapter_id: Option<String>,
    #[serde(default)]
    pub run_ids: Vec<String>,
    pub max_actions: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HarnessPlanInspectRequest {
    pub plan_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HarnessExecuteRequest {
    pub plan_id: String,
    pub action_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HarnessOutcomeListResponse {
    pub outcomes: Vec<HarnessExecutionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JobValidateRequest {
    pub backend: SchedulerBackend,
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    #[serde(default)]
    pub run_ids: Vec<String>,
    pub retry_limit: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JobScheduleRequest {
    pub job_name: String,
    pub backend: SchedulerBackend,
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    #[serde(default)]
    pub run_ids: Vec<String>,
    pub requested_by: String,
    pub retry_limit: Option<u8>,
    pub approval_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JobInspectRequest {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JobRunRequest {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JobCancelRequest {
    pub job_id: String,
    pub requested_by: String,
    pub approval_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JobListResponse {
    pub jobs: Vec<ScheduledJobRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RealtimeCreateRequest {
    pub display_name: String,
    pub protocol: RealtimeAdapterProtocol,
    pub host: std::net::IpAddr,
    pub port: u16,
    pub base_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RealtimeInspectRequest {
    pub adapter_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RealtimeSendPreviewToolRequest {
    pub adapter_id: String,
    pub session_id: String,
    pub preview_id: String,
    pub actor_id: String,
    pub dispatch_mode: Option<RealtimeDispatchMode>,
    pub time_scale: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RealtimeSendTransportToolRequest {
    pub adapter_id: String,
    pub deck_id: String,
    pub actor_id: String,
    pub dispatch_mode: Option<RealtimeDispatchMode>,
    pub time_scale: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RealtimeListResponse {
    pub adapters: Vec<RealtimeAdapterRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedSessionResponse {
    #[serde(flatten)]
    pub session: SessionRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedEvaluationResponse {
    #[serde(flatten)]
    pub evaluation: EvaluationRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedSessionPreviewResponse {
    #[serde(flatten)]
    pub preview: SessionPreviewResult,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedDeckResponse {
    #[serde(flatten)]
    pub deck: DeckRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedDeckTransportResponse {
    #[serde(flatten)]
    pub snapshot: DeckTransportSnapshot,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HarnessExecuteResponse {
    #[serde(flatten)]
    pub outcome: HarnessExecutionRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedJobResponse {
    #[serde(flatten)]
    pub job: ScheduledJobRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedJobRunResponse {
    #[serde(flatten)]
    pub summary: JobRunSummary,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedRealtimeAdapterResponse {
    #[serde(flatten)]
    pub adapter: RealtimeAdapterRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedRealtimeDispatchResponse {
    #[serde(flatten)]
    pub summary: RealtimeDispatchSummary,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedDatasetRecordResponse {
    #[serde(flatten)]
    pub dataset: crate::governance::DatasetRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedApprovalRequestResponse {
    #[serde(flatten)]
    pub approval: ApprovalRequestRecord,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedApprovalResolutionResponse {
    #[serde(flatten)]
    pub resolution: ApprovalResolution,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedSnapshotCreateResponse {
    #[serde(flatten)]
    pub snapshot: PresetSnapshotSummary,
    pub audit: ActionAuditRef,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AuditedSnapshotRollbackResponse {
    #[serde(flatten)]
    pub rollback: PresetRollbackSummary,
    pub audit: ActionAuditRef,
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for MusicBoxMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "Generate deterministic MIDI and WAV artifacts from state-space presets.",
        )
    }
}

#[tool_router]
impl MusicBoxMcpServer {
    #[tool(description = "Create and store a named state-space system for this MCP session.")]
    async fn create_system(
        &self,
        Parameters(request): Parameters<CreateSystemRequest>,
    ) -> Result<Json<SystemSummary>, String> {
        let system = self.system_from_request(&request.name, &request)?;
        Ok(Json(SystemSummary {
            name: request.name,
            state_dim: system.a.nrows(),
            input_dim: system.b.ncols(),
            output_dim: system.c.nrows(),
            dt: system.dt,
        }))
    }

    #[tool(description = "Create and persist a preset from a stored session system.")]
    async fn create_preset(
        &self,
        Parameters(request): Parameters<CreatePresetRequest>,
    ) -> Result<Json<CreatePresetResponse>, String> {
        let system = self.system_from_name(&request.system_name)?;
        let mut preset = RenderPreset::from_system(
            request.name.clone(),
            request
                .description
                .clone()
                .unwrap_or_else(|| format!("Preset derived from {}", request.system_name)),
            &system,
        );

        if let Some(duration_seconds) = request.duration_seconds {
            preset.simulation.duration_seconds = duration_seconds;
        }
        if let Some(sample_rate) = request.trajectory_sample_rate {
            preset.simulation.trajectory_sample_rate = sample_rate;
        }
        if let Some(tempo_bpm) = request.tempo_bpm {
            preset.midi.tempo_bpm = tempo_bpm;
        }
        if let Some(audio_sample_rate) = request.audio_sample_rate {
            preset.audio.sample_rate = audio_sample_rate;
        }

        let path = save_preset(&preset, &self.state.preset_dir).map_err(|err| err.to_string())?;
        Ok(Json(CreatePresetResponse {
            name: preset.name,
            path: path.display().to_string(),
            source_system: request.system_name,
        }))
    }

    #[tool(description = "List built-in and file-backed presets available to the server.")]
    async fn list_presets(&self) -> Result<Json<ListPresetsResponse>, String> {
        let presets = list_presets(&self.state.preset_dir)
            .map_err(|err| err.to_string())?
            .into_iter()
            .map(|preset| PresetEntry {
                name: preset.name,
                source: preset.source,
            })
            .collect();

        Ok(Json(ListPresetsResponse { presets }))
    }

    #[tool(description = "List persisted run manifests from the local runtime store.")]
    async fn run_list(&self) -> Result<Json<RunListResponse>, String> {
        let runs = list_run_manifests(&crate::governance::default_manifest_dir(
            &self.state.runtime_dir,
        ))
        .map_err(|err| err.to_string())?;
        Ok(Json(RunListResponse { runs }))
    }

    #[tool(description = "Inspect one persisted run manifest by id.")]
    async fn run_inspect(
        &self,
        Parameters(request): Parameters<RunInspectRequest>,
    ) -> Result<Json<RunManifestRecord>, String> {
        let manifest = inspect_run_manifest(
            &crate::governance::default_manifest_dir(&self.state.runtime_dir),
            &request.run_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(manifest))
    }

    #[tool(description = "List audit events from the local runtime store.")]
    async fn audit_list(
        &self,
        Parameters(request): Parameters<AuditListRequest>,
    ) -> Result<Json<AuditListResponse>, String> {
        let mut events = read_audit_events(&crate::governance::default_audit_log_path(
            &self.state.runtime_dir,
        ))
        .map_err(|err| err.to_string())?;
        if let Some(limit) = request.limit {
            let start = events.len().saturating_sub(limit);
            events = events.split_off(start);
        }
        Ok(Json(AuditListResponse { events }))
    }

    #[tool(description = "List local session records.")]
    async fn session_list(&self) -> Result<Json<SessionListResponse>, String> {
        let sessions = list_sessions(&default_session_store_path(&self.state.runtime_dir))
            .map_err(|err| err.to_string())?;
        Ok(Json(SessionListResponse { sessions }))
    }

    #[tool(description = "Create a local session record for a preset.")]
    async fn session_create(
        &self,
        Parameters(request): Parameters<SessionCreateRequest>,
    ) -> Result<Json<AuditedSessionResponse>, String> {
        let input = json!({
            "display_name": request.display_name,
            "preset_name": request.preset_name,
            "seed": request.seed,
            "actor_id": request.actor_id,
        });
        let mut action = self.base_action("session_create", None, input);
        action.actor_id = request.actor_id.clone();
        action.preset_name = Some(request.preset_name.clone());
        action.seed = request.seed;

        let result = (|| -> Result<_, String> {
            let session = create_session(
                &default_session_store_path(&self.state.runtime_dir),
                &self.state.preset_dir,
                NewSessionRequest {
                    display_name: request.display_name,
                    preset_name: request.preset_name,
                    seed: request.seed.unwrap_or(1),
                    actor_id: request.actor_id,
                },
            )
            .map_err(|err| err.to_string())?;
            action.target = Some(session.session_id.clone());
            action.preset_hash = Some(session.preset_hash.clone());
            action.artifacts.push(ManifestArtifactInput {
                kind: "session-store".to_string(),
                path: default_session_store_path(&self.state.runtime_dir),
            });
            Ok(session)
        })();

        match result {
            Ok(session) => {
                let audit = self.record_success(&session, action)?;
                Ok(Json(AuditedSessionResponse { session, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Inspect one local session record.")]
    async fn session_inspect(
        &self,
        Parameters(request): Parameters<SessionInspectRequest>,
    ) -> Result<Json<SessionRecord>, String> {
        let session = inspect_session(
            &default_session_store_path(&self.state.runtime_dir),
            &request.session_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(session))
    }

    #[tool(description = "Update a local session record.")]
    async fn session_update(
        &self,
        Parameters(request): Parameters<SessionUpdateRequest>,
    ) -> Result<Json<AuditedSessionResponse>, String> {
        let input = json!({
            "session_id": request.session_id,
            "display_name": request.display_name,
            "preset_name": request.preset_name,
            "seed": request.seed,
            "tempo_bpm": request.tempo_bpm,
            "status": request.status,
        });
        let mut action =
            self.base_action("session_update", Some(request.session_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let session = update_session(
                &default_session_store_path(&self.state.runtime_dir),
                &self.state.preset_dir,
                &request.session_id,
                UpdateSessionRequest {
                    actor_id: request.actor_id,
                    display_name: request.display_name,
                    preset_name: request.preset_name,
                    seed: request.seed,
                    tempo_bpm: request.tempo_bpm,
                    status: request.status,
                },
            )
            .map_err(|err| err.to_string())?;
            action.preset_name = Some(session.preset_name.clone());
            action.preset_hash = Some(session.preset_hash.clone());
            action.seed = Some(session.seed);
            action.artifacts.push(ManifestArtifactInput {
                kind: "session-store".to_string(),
                path: default_session_store_path(&self.state.runtime_dir),
            });
            Ok(session)
        })();

        match result {
            Ok(session) => {
                let audit = self.record_success(&session, action)?;
                Ok(Json(AuditedSessionResponse { session, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Mark a local session as actively playing.")]
    async fn session_play(
        &self,
        Parameters(request): Parameters<SessionTransportToolRequest>,
    ) -> Result<Json<AuditedSessionResponse>, String> {
        let input = json!({
            "session_id": request.session_id,
            "command": "play",
            "run_label": request.run_label,
        });
        let mut action = self.base_action("session_play", Some(request.session_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let session = apply_transport_command(
                &default_session_store_path(&self.state.runtime_dir),
                &request.session_id,
                SessionTransportRequest {
                    actor_id: request.actor_id,
                    command: SessionTransportCommand::Play,
                    run_label: request.run_label,
                },
            )
            .map_err(|err| err.to_string())?;
            action.preset_name = Some(session.preset_name.clone());
            action.preset_hash = Some(session.preset_hash.clone());
            action.seed = Some(session.seed);
            action.artifacts.push(ManifestArtifactInput {
                kind: "session-store".to_string(),
                path: default_session_store_path(&self.state.runtime_dir),
            });
            Ok(session)
        })();

        match result {
            Ok(session) => {
                let audit = self.record_success(&session, action)?;
                Ok(Json(AuditedSessionResponse { session, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Mark a local session as stopped.")]
    async fn session_stop(
        &self,
        Parameters(request): Parameters<SessionTransportToolRequest>,
    ) -> Result<Json<AuditedSessionResponse>, String> {
        let input = json!({
            "session_id": request.session_id,
            "command": "stop",
        });
        let mut action = self.base_action("session_stop", Some(request.session_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let session = apply_transport_command(
                &default_session_store_path(&self.state.runtime_dir),
                &request.session_id,
                SessionTransportRequest {
                    actor_id: request.actor_id,
                    command: SessionTransportCommand::Stop,
                    run_label: None,
                },
            )
            .map_err(|err| err.to_string())?;
            action.preset_name = Some(session.preset_name.clone());
            action.preset_hash = Some(session.preset_hash.clone());
            action.seed = Some(session.seed);
            action.artifacts.push(ManifestArtifactInput {
                kind: "session-store".to_string(),
                path: default_session_store_path(&self.state.runtime_dir),
            });
            Ok(session)
        })();

        match result {
            Ok(session) => {
                let audit = self.record_success(&session, action)?;
                Ok(Json(AuditedSessionResponse { session, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Render a deterministic MIDI and WAV preview bundle for a local session.")]
    async fn session_render_preview(
        &self,
        Parameters(request): Parameters<SessionRenderPreviewRequest>,
    ) -> Result<Json<AuditedSessionPreviewResponse>, String> {
        let input = json!({
            "session_id": request.session_id,
            "actor_id": request.actor_id,
        });
        let mut action = self.base_action(
            "session_render_preview",
            Some(request.session_id.clone()),
            input,
        );
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let preview = render_session_preview(
                &default_session_store_path(&self.state.runtime_dir),
                &self.state.preset_dir,
                &self.state.runtime_dir,
                &request.session_id,
                &request.actor_id,
            )
            .map_err(|err| err.to_string())?;
            action.preset_name = Some(preview.session.preset_name.clone());
            action.preset_hash = Some(preview.session.preset_hash.clone());
            action.seed = Some(preview.session.seed);
            action.artifacts.push(ManifestArtifactInput {
                kind: "session-store".to_string(),
                path: default_session_store_path(&self.state.runtime_dir),
            });
            action.artifacts.push(ManifestArtifactInput {
                kind: "preview-midi".to_string(),
                path: preview.preview.midi.path.clone(),
            });
            action.artifacts.push(ManifestArtifactInput {
                kind: "preview-wav".to_string(),
                path: preview.preview.wav.path.clone(),
            });
            Ok(preview)
        })();

        match result {
            Ok(preview) => {
                let audit = self.record_success(&preview, action)?;
                Ok(Json(AuditedSessionPreviewResponse { preview, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Compare two or more persisted run manifests.")]
    async fn run_compare(
        &self,
        Parameters(request): Parameters<RunCompareRequest>,
    ) -> Result<Json<RunComparisonSummary>, String> {
        let comparison = compare_runs(
            &crate::governance::default_manifest_dir(&self.state.runtime_dir),
            &request.run_ids,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(comparison))
    }

    #[tool(description = "Submit an evaluation record for one or more runs.")]
    async fn evaluation_submit(
        &self,
        Parameters(request): Parameters<EvaluationSubmitRequest>,
    ) -> Result<Json<AuditedEvaluationResponse>, String> {
        let input = json!({
            "run_ids": request.run_ids,
            "objective_metrics": request.objective_metrics,
            "human_scores": request.human_scores,
            "reward_weights": request.reward_weights,
            "notes": request.notes,
            "decision": request.decision,
        });
        let mut action = self.base_action("evaluation_submit", None, input);
        action.actor_id = request.created_by.clone();

        let result = (|| -> Result<_, String> {
            let evaluation = submit_evaluation_record(
                &default_evaluation_store_path(&self.state.runtime_dir),
                &crate::governance::default_manifest_dir(&self.state.runtime_dir),
                NewEvaluationRecord {
                    run_ids: request.run_ids,
                    objective_metrics: request.objective_metrics,
                    human_scores: request.human_scores,
                    reward_weights: request.reward_weights,
                    notes: request.notes,
                    decision: request.decision,
                    created_by: request.created_by,
                },
            )
            .map_err(|err| err.to_string())?;
            action.target = Some(evaluation.evaluation_id.clone());
            action.artifacts.push(ManifestArtifactInput {
                kind: "evaluation-store".to_string(),
                path: default_evaluation_store_path(&self.state.runtime_dir),
            });
            Ok(evaluation)
        })();

        match result {
            Ok(evaluation) => {
                let audit = self.record_success(&evaluation, action)?;
                Ok(Json(AuditedEvaluationResponse { evaluation, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "List stored evaluation records.")]
    async fn evaluation_list(&self) -> Result<Json<EvaluationListResponse>, String> {
        let evaluations =
            list_evaluation_records(&default_evaluation_store_path(&self.state.runtime_dir))
                .map_err(|err| err.to_string())?;
        Ok(Json(EvaluationListResponse { evaluations }))
    }

    #[tool(description = "Inspect one stored evaluation record.")]
    async fn evaluation_inspect(
        &self,
        Parameters(request): Parameters<EvaluationInspectRequest>,
    ) -> Result<Json<EvaluationRecord>, String> {
        let evaluation = inspect_evaluation_record(
            &default_evaluation_store_path(&self.state.runtime_dir),
            &request.evaluation_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(evaluation))
    }

    #[tool(description = "Build a side-by-side review bundle for two or more runs.")]
    async fn review_build(
        &self,
        Parameters(request): Parameters<ReviewBundleRequest>,
    ) -> Result<Json<ReviewBundle>, String> {
        let review = build_review_bundle(
            &default_evaluation_store_path(&self.state.runtime_dir),
            &crate::governance::default_manifest_dir(&self.state.runtime_dir),
            &request.run_ids,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(review))
    }

    #[tool(description = "List local DAW-agnostic decks.")]
    async fn deck_list(&self) -> Result<Json<DeckListResponse>, String> {
        let decks = list_decks(&default_daw_store_path(&self.state.runtime_dir))
            .map_err(|err| err.to_string())?;
        Ok(Json(DeckListResponse { decks }))
    }

    #[tool(description = "Create a deck bound to one session.")]
    async fn deck_create(
        &self,
        Parameters(request): Parameters<DeckCreateRequest>,
    ) -> Result<Json<AuditedDeckResponse>, String> {
        let input = json!({
            "display_name": request.display_name,
            "session_id": request.session_id,
        });
        let mut action = self.base_action("deck_create", Some(request.session_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let deck = create_deck(
                &default_daw_store_path(&self.state.runtime_dir),
                &default_session_store_path(&self.state.runtime_dir),
                NewDeckRequest {
                    display_name: request.display_name,
                    session_id: request.session_id,
                    actor_id: request.actor_id,
                },
            )
            .map_err(|err| err.to_string())?;
            action.target = Some(deck.deck_id.clone());
            action.artifacts.push(ManifestArtifactInput {
                kind: "deck-store".to_string(),
                path: default_daw_store_path(&self.state.runtime_dir),
            });
            Ok(deck)
        })();

        match result {
            Ok(deck) => {
                let audit = self.record_success(&deck, action)?;
                Ok(Json(AuditedDeckResponse { deck, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Inspect one local deck.")]
    async fn deck_inspect(
        &self,
        Parameters(request): Parameters<DeckInspectRequest>,
    ) -> Result<Json<DeckRecord>, String> {
        let deck = inspect_deck(
            &default_daw_store_path(&self.state.runtime_dir),
            &request.deck_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(deck))
    }

    #[tool(description = "Add a session preview as a clip on one deck.")]
    async fn deck_add_preview(
        &self,
        Parameters(request): Parameters<DeckAddPreviewToolRequest>,
    ) -> Result<Json<AuditedDeckResponse>, String> {
        let input = json!({
            "deck_id": request.deck_id,
            "session_id": request.session_id,
            "preview_id": request.preview_id,
            "label": request.label,
        });
        let mut action = self.base_action("deck_add_preview", Some(request.deck_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let deck = add_preview_clip_to_deck(
                &default_daw_store_path(&self.state.runtime_dir),
                &default_session_store_path(&self.state.runtime_dir),
                &request.deck_id,
                AddDeckPreviewRequest {
                    actor_id: request.actor_id,
                    label: request.label,
                    session_id: request.session_id,
                    preview_id: request.preview_id,
                },
            )
            .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "deck-store".to_string(),
                path: default_daw_store_path(&self.state.runtime_dir),
            });
            Ok(deck)
        })();

        match result {
            Ok(deck) => {
                let audit = self.record_success(&deck, action)?;
                Ok(Json(AuditedDeckResponse { deck, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Queue one clip on a deck without launching it yet.")]
    async fn deck_queue(
        &self,
        Parameters(request): Parameters<DeckQueueRequest>,
    ) -> Result<Json<AuditedDeckResponse>, String> {
        let input = json!({
            "deck_id": request.deck_id,
            "clip_id": request.clip_id,
        });
        let mut action = self.base_action("deck_queue", Some(request.deck_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let deck = queue_deck_clip(
                &default_daw_store_path(&self.state.runtime_dir),
                &request.deck_id,
                QueueDeckClipRequest {
                    actor_id: request.actor_id,
                    clip_id: request.clip_id,
                },
            )
            .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "deck-store".to_string(),
                path: default_daw_store_path(&self.state.runtime_dir),
            });
            Ok(deck)
        })();

        match result {
            Ok(deck) => {
                let audit = self.record_success(&deck, action)?;
                Ok(Json(AuditedDeckResponse { deck, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Launch one deck clip and move the deck into playing state.")]
    async fn deck_launch(
        &self,
        Parameters(request): Parameters<DeckLaunchRequest>,
    ) -> Result<Json<AuditedDeckTransportResponse>, String> {
        let input = json!({
            "deck_id": request.deck_id,
            "clip_id": request.clip_id,
        });
        let mut action = self.base_action("deck_launch", Some(request.deck_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let snapshot = launch_deck_clip(
                &default_daw_store_path(&self.state.runtime_dir),
                &request.deck_id,
                LaunchDeckClipRequest {
                    actor_id: request.actor_id,
                    clip_id: request.clip_id,
                },
            )
            .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "deck-store".to_string(),
                path: default_daw_store_path(&self.state.runtime_dir),
            });
            Ok(snapshot)
        })();

        match result {
            Ok(snapshot) => {
                let audit = self.record_success(&snapshot, action)?;
                Ok(Json(AuditedDeckTransportResponse { snapshot, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Stop one deck and clear the active clip.")]
    async fn deck_stop(
        &self,
        Parameters(request): Parameters<DeckStopRequest>,
    ) -> Result<Json<AuditedDeckTransportResponse>, String> {
        let input = json!({
            "deck_id": request.deck_id,
        });
        let mut action = self.base_action("deck_stop", Some(request.deck_id.clone()), input);
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let snapshot = stop_deck(
                &default_daw_store_path(&self.state.runtime_dir),
                &request.deck_id,
                StopDeckRequest {
                    actor_id: request.actor_id,
                },
            )
            .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "deck-store".to_string(),
                path: default_daw_store_path(&self.state.runtime_dir),
            });
            Ok(snapshot)
        })();

        match result {
            Ok(snapshot) => {
                let audit = self.record_success(&snapshot, action)?;
                Ok(Json(AuditedDeckTransportResponse { snapshot, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Inspect the current transport snapshot for one deck.")]
    async fn deck_transport(
        &self,
        Parameters(request): Parameters<DeckInspectRequest>,
    ) -> Result<Json<DeckTransportSnapshot>, String> {
        let snapshot = inspect_deck_transport(
            &default_daw_store_path(&self.state.runtime_dir),
            &request.deck_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(snapshot))
    }

    #[tool(description = "Create a deterministic harness plan over the real backend tools.")]
    async fn harness_plan(
        &self,
        Parameters(request): Parameters<HarnessPlanRequest>,
    ) -> Result<Json<HarnessPlanRecord>, String> {
        let plan = create_harness_plan(
            &default_harness_store_path(&self.state.runtime_dir),
            &self.state.runtime_dir,
            NewHarnessPlanRequest {
                role: request.role,
                prompt: request.prompt,
                session_id: request.session_id,
                deck_id: request.deck_id,
                adapter_id: request.adapter_id,
                run_ids: request.run_ids,
                max_actions: request.max_actions,
            },
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(plan))
    }

    #[tool(description = "Inspect one stored harness plan.")]
    async fn harness_plan_inspect(
        &self,
        Parameters(request): Parameters<HarnessPlanInspectRequest>,
    ) -> Result<Json<HarnessPlanRecord>, String> {
        let plan = inspect_harness_plan(
            &default_harness_store_path(&self.state.runtime_dir),
            &request.plan_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(plan))
    }

    #[tool(description = "Execute one action from a stored harness plan.")]
    async fn harness_execute(
        &self,
        Parameters(request): Parameters<HarnessExecuteRequest>,
    ) -> Result<Json<HarnessExecuteResponse>, String> {
        let outcome = execute_harness_action(
            &default_harness_store_path(&self.state.runtime_dir),
            &self.state.runtime_dir,
            &self.state.preset_dir,
            ExecuteHarnessActionRequest {
                plan_id: request.plan_id,
                action_id: request.action_id,
            },
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(HarnessExecuteResponse { outcome }))
    }

    #[tool(description = "List stored harness execution outcomes.")]
    async fn harness_outcome_list(&self) -> Result<Json<HarnessOutcomeListResponse>, String> {
        let outcomes = list_harness_outcomes(&default_harness_store_path(&self.state.runtime_dir))
            .map_err(|err| err.to_string())?;
        Ok(Json(HarnessOutcomeListResponse { outcomes }))
    }

    #[tool(
        description = "Validate an immutable unattended job configuration over the real harness context."
    )]
    async fn job_validate(
        &self,
        Parameters(request): Parameters<JobValidateRequest>,
    ) -> Result<Json<JobValidationResult>, String> {
        let validation = validate_scheduled_job(
            &self.state.runtime_dir,
            ValidateScheduledJobRequest {
                backend: request.backend,
                role: request.role,
                prompt: request.prompt,
                session_id: request.session_id,
                deck_id: request.deck_id,
                adapter_id: None,
                run_ids: request.run_ids,
                retry_limit: request.retry_limit.unwrap_or(1),
                max_dispatches: None,
            },
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(validation))
    }

    #[tool(
        description = "Schedule an immutable unattended job after consuming a matching approval token."
    )]
    async fn job_schedule(
        &self,
        Parameters(request): Parameters<JobScheduleRequest>,
    ) -> Result<Json<AuditedJobResponse>, String> {
        let input = json!({
            "job_name": request.job_name,
            "backend": request.backend,
            "role": request.role,
            "prompt": request.prompt,
            "session_id": request.session_id,
            "deck_id": request.deck_id,
            "run_ids": request.run_ids,
            "requested_by": request.requested_by,
            "retry_limit": request.retry_limit.unwrap_or(1),
        });
        let mut action = self.base_action("job_schedule", Some(request.job_name.clone()), input);
        action.actor_id = request.requested_by.clone();

        let result = (|| -> Result<_, String> {
            let job = schedule_job(
                &self.state.runtime_dir,
                NewScheduledJobRequest {
                    name: request.job_name,
                    backend: request.backend,
                    role: request.role,
                    prompt: request.prompt,
                    session_id: request.session_id,
                    deck_id: request.deck_id,
                    adapter_id: None,
                    run_ids: request.run_ids,
                    requested_by: request.requested_by,
                    retry_limit: request.retry_limit.unwrap_or(1),
                    approval_token: request.approval_token,
                    max_dispatches: None,
                },
            )
            .map_err(|err| err.to_string())?;
            action.target = Some(job.job_id.clone());
            action.approval_ids = vec![job.approval_id.clone()];
            action.metadata = Some(json!({
                "scheduler_backend": job.config.backend,
                "config_hash": job.config_hash,
            }));
            action.artifacts.push(ManifestArtifactInput {
                kind: "scheduler-store".to_string(),
                path: default_scheduler_store_path(&self.state.runtime_dir),
            });
            action.artifacts.push(ManifestArtifactInput {
                kind: "scheduler-export".to_string(),
                path: job.export_path.clone(),
            });
            Ok(job)
        })();

        match result {
            Ok(job) => {
                let audit = self.record_success(&job, action)?;
                Ok(Json(AuditedJobResponse { job, audit }))
            }
            Err(error) => {
                if let Err(record_error) =
                    self.record_failure(&error, ActionStatus::Blocked, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "List stored unattended job definitions.")]
    async fn job_list(&self) -> Result<Json<JobListResponse>, String> {
        let jobs = list_scheduled_jobs(&self.state.runtime_dir).map_err(|err| err.to_string())?;
        Ok(Json(JobListResponse { jobs }))
    }

    #[tool(description = "Inspect one stored unattended job definition.")]
    async fn job_inspect(
        &self,
        Parameters(request): Parameters<JobInspectRequest>,
    ) -> Result<Json<ScheduledJobRecord>, String> {
        let job = inspect_scheduled_job(&self.state.runtime_dir, &request.job_id)
            .map_err(|err| err.to_string())?;
        Ok(Json(job))
    }

    #[tool(
        description = "Run one stored unattended job locally through the shared harness backend."
    )]
    async fn job_run(
        &self,
        Parameters(request): Parameters<JobRunRequest>,
    ) -> Result<Json<AuditedJobRunResponse>, String> {
        let input = json!({ "job_id": request.job_id });
        let mut action = self.base_action("job_run", Some(request.job_id.clone()), input);

        let result = (|| -> Result<_, String> {
            let summary = run_scheduled_job(
                &self.state.runtime_dir,
                &self.state.preset_dir,
                &request.job_id,
            )
            .map_err(|err| err.to_string())?;
            action.metadata = Some(json!({
                "plan_id": summary.plan_id,
                "outcome_ids": summary.outcome_ids,
                "scheduler_backend": summary.job.config.backend,
            }));
            action.artifacts.push(ManifestArtifactInput {
                kind: "scheduler-store".to_string(),
                path: default_scheduler_store_path(&self.state.runtime_dir),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(AuditedJobRunResponse { summary, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(
        description = "Cancel one stored unattended job after consuming a matching approval token."
    )]
    async fn job_cancel(
        &self,
        Parameters(request): Parameters<JobCancelRequest>,
    ) -> Result<Json<AuditedJobResponse>, String> {
        let input = json!({
            "job_id": request.job_id,
            "requested_by": request.requested_by,
        });
        let mut action = self.base_action("job_cancel", Some(request.job_id.clone()), input);
        action.actor_id = request.requested_by.clone();

        let result = (|| -> Result<_, String> {
            let job = crate::governance::cancel_scheduled_job(
                &self.state.runtime_dir,
                CancelScheduledJobRequest {
                    job_id: request.job_id,
                    requested_by: request.requested_by,
                    approval_token: request.approval_token,
                },
            )
            .map_err(|err| err.to_string())?;
            action.approval_ids = vec![job.approval_id.clone()];
            action.artifacts.push(ManifestArtifactInput {
                kind: "scheduler-store".to_string(),
                path: default_scheduler_store_path(&self.state.runtime_dir),
            });
            Ok(job)
        })();

        match result {
            Ok(job) => {
                let audit = self.record_success(&job, action)?;
                Ok(Json(AuditedJobResponse { job, audit }))
            }
            Err(error) => {
                if let Err(record_error) =
                    self.record_failure(&error, ActionStatus::Blocked, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Create a realtime OSC adapter for local live dispatch.")]
    async fn realtime_create(
        &self,
        Parameters(request): Parameters<RealtimeCreateRequest>,
    ) -> Result<Json<AuditedRealtimeAdapterResponse>, String> {
        let input = json!({
            "display_name": request.display_name,
            "protocol": request.protocol,
            "host": request.host,
            "port": request.port,
            "base_path": request.base_path,
        });
        let mut action =
            self.base_action("realtime_create", Some(request.display_name.clone()), input);

        let result = (|| -> Result<_, String> {
            let adapter = crate::governance::create_realtime_adapter(
                &default_realtime_store_path(&self.state.runtime_dir),
                NewRealtimeAdapterRequest {
                    display_name: request.display_name,
                    protocol: request.protocol,
                    host: request.host,
                    port: request.port,
                    base_path: request
                        .base_path
                        .unwrap_or_else(|| "/state_space_music_box".to_string()),
                },
            )
            .map_err(|err| err.to_string())?;
            action.target = Some(adapter.adapter_id.clone());
            action.artifacts.push(ManifestArtifactInput {
                kind: "realtime-store".to_string(),
                path: default_realtime_store_path(&self.state.runtime_dir),
            });
            Ok(adapter)
        })();

        match result {
            Ok(adapter) => {
                let audit = self.record_success(&adapter, action)?;
                Ok(Json(AuditedRealtimeAdapterResponse { adapter, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "List configured realtime adapters.")]
    async fn realtime_list(&self) -> Result<Json<RealtimeListResponse>, String> {
        let adapters =
            list_realtime_adapters(&default_realtime_store_path(&self.state.runtime_dir))
                .map_err(|err| err.to_string())?;
        Ok(Json(RealtimeListResponse { adapters }))
    }

    #[tool(description = "Inspect one configured realtime adapter.")]
    async fn realtime_inspect(
        &self,
        Parameters(request): Parameters<RealtimeInspectRequest>,
    ) -> Result<Json<RealtimeAdapterRecord>, String> {
        let adapter = inspect_realtime_adapter(
            &default_realtime_store_path(&self.state.runtime_dir),
            &request.adapter_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(adapter))
    }

    #[tool(description = "Send a stored session preview to a realtime OSC adapter.")]
    async fn realtime_send_preview(
        &self,
        Parameters(request): Parameters<RealtimeSendPreviewToolRequest>,
    ) -> Result<Json<AuditedRealtimeDispatchResponse>, String> {
        let input = json!({
            "adapter_id": request.adapter_id,
            "session_id": request.session_id,
            "preview_id": request.preview_id,
            "dispatch_mode": request.dispatch_mode.unwrap_or(RealtimeDispatchMode::Timed),
            "time_scale": request.time_scale.unwrap_or(1.0),
        });
        let mut action = self.base_action(
            "realtime_send_preview",
            Some(request.adapter_id.clone()),
            input,
        );
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let summary = send_preview_to_realtime_adapter(
                &default_realtime_store_path(&self.state.runtime_dir),
                &default_session_store_path(&self.state.runtime_dir),
                &request.adapter_id,
                SendRealtimePreviewRequest {
                    actor_id: request.actor_id,
                    session_id: request.session_id,
                    preview_id: request.preview_id,
                    dispatch_mode: request.dispatch_mode.unwrap_or(RealtimeDispatchMode::Timed),
                    time_scale: request.time_scale.unwrap_or(1.0),
                },
            )
            .map_err(|err| err.to_string())?;
            action.metadata = Some(json!({
                "dispatch_id": summary.dispatch.dispatch_id,
                "message_count": summary.dispatch.message_count,
            }));
            action.artifacts.push(ManifestArtifactInput {
                kind: "realtime-store".to_string(),
                path: default_realtime_store_path(&self.state.runtime_dir),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(AuditedRealtimeDispatchResponse { summary, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Send the current deck transport snapshot to a realtime OSC adapter.")]
    async fn realtime_send_transport(
        &self,
        Parameters(request): Parameters<RealtimeSendTransportToolRequest>,
    ) -> Result<Json<AuditedRealtimeDispatchResponse>, String> {
        let input = json!({
            "adapter_id": request.adapter_id,
            "deck_id": request.deck_id,
            "dispatch_mode": request.dispatch_mode.unwrap_or(RealtimeDispatchMode::Immediate),
            "time_scale": request.time_scale.unwrap_or(1.0),
        });
        let mut action = self.base_action(
            "realtime_send_transport",
            Some(request.adapter_id.clone()),
            input,
        );
        action.actor_id = request.actor_id.clone();

        let result = (|| -> Result<_, String> {
            let summary = send_transport_to_realtime_adapter(
                &default_realtime_store_path(&self.state.runtime_dir),
                &default_daw_store_path(&self.state.runtime_dir),
                &request.adapter_id,
                SendRealtimeTransportRequest {
                    actor_id: request.actor_id,
                    deck_id: request.deck_id,
                    dispatch_mode: request
                        .dispatch_mode
                        .unwrap_or(RealtimeDispatchMode::Immediate),
                    time_scale: request.time_scale.unwrap_or(1.0),
                },
            )
            .map_err(|err| err.to_string())?;
            action.metadata = Some(json!({
                "dispatch_id": summary.dispatch.dispatch_id,
                "message_count": summary.dispatch.message_count,
            }));
            action.artifacts.push(ManifestArtifactInput {
                kind: "realtime-store".to_string(),
                path: default_realtime_store_path(&self.state.runtime_dir),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(AuditedRealtimeDispatchResponse { summary, audit }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "List dataset records from the local runtime registry.")]
    async fn dataset_list(&self) -> Result<Json<DatasetListResponse>, String> {
        let datasets =
            list_dataset_records(&default_dataset_registry_path(&self.state.runtime_dir))
                .map_err(|err| err.to_string())?;
        Ok(Json(DatasetListResponse { datasets }))
    }

    #[tool(description = "Inspect one dataset record from the local runtime registry.")]
    async fn dataset_inspect(
        &self,
        Parameters(request): Parameters<DatasetInspectRequest>,
    ) -> Result<Json<crate::governance::DatasetRecord>, String> {
        let dataset = inspect_dataset_record(
            &default_dataset_registry_path(&self.state.runtime_dir),
            &request.dataset_id,
        )
        .map_err(|err| err.to_string())?;
        Ok(Json(dataset))
    }

    #[tool(description = "Register a dataset after consuming a matching approval token.")]
    async fn dataset_register(
        &self,
        Parameters(request): Parameters<DatasetRegisterRequest>,
    ) -> Result<Json<AuditedDatasetRecordResponse>, String> {
        let registry_path = default_dataset_registry_path(&self.state.runtime_dir);
        let approval_store_path = default_approval_store_path(&self.state.runtime_dir);
        let input = json!({
            "dataset_id": request.dataset_id,
            "display_name": request.display_name,
            "source_url": request.source_url,
            "citation": request.citation,
            "license_name": request.license_name,
            "commercial_use_status": request.commercial_use_status,
            "redistribution_status": request.redistribution_status,
            "approved_use_class": request.approved_use_class,
            "checksum_manifest": request.checksum_manifest,
            "local_storage_path": request.local_storage_path,
            "dataset_version": request.dataset_version,
            "split_policy": request.split_policy,
            "transform_pipeline_hash": request.transform_pipeline_hash,
            "parent_datasets": request.parent_datasets,
            "notes": request.notes,
        });
        let mut action =
            self.base_action("dataset_register", Some(request.dataset_id.clone()), input);

        let result = (|| -> Result<_, String> {
            let approval = consume_approval_token(
                &approval_store_path,
                &request.approval_token,
                "dataset.register",
                &request.dataset_id,
            )
            .map_err(|err| err.to_string())?;
            action.approval_ids = vec![approval.approval_id.clone()];
            let record = register_dataset_record(
                &registry_path,
                NewDatasetRecord {
                    dataset_id: request.dataset_id,
                    display_name: request.display_name,
                    source_url: request.source_url,
                    citation: request.citation,
                    license_name: request.license_name,
                    commercial_use_status: request.commercial_use_status,
                    redistribution_status: request.redistribution_status,
                    approved_use_class: request.approved_use_class,
                    checksum_manifest: request.checksum_manifest,
                    local_storage_path: request.local_storage_path,
                    dataset_version: request.dataset_version,
                    split_policy: request.split_policy,
                    transform_pipeline_hash: request.transform_pipeline_hash,
                    parent_datasets: request.parent_datasets,
                    operator_approval_id: approval.approval_id,
                    notes: request.notes,
                },
            )
            .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "dataset-registry".to_string(),
                path: registry_path,
            });
            Ok(record)
        })();

        match result {
            Ok(record) => {
                let audit = self.record_success(&record, action)?;
                Ok(Json(AuditedDatasetRecordResponse {
                    dataset: record,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) =
                    self.record_failure(&error, ActionStatus::Blocked, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Generate a MIDI file from a preset.")]
    async fn generate_midi(
        &self,
        Parameters(request): Parameters<GenerateArtifactRequest>,
    ) -> Result<Json<MidiArtifactResponse>, String> {
        let seed = request.seed.unwrap_or(1);
        let input = json!({
            "preset_name": request.preset_name,
            "output_path": request.output_path,
            "seed": seed,
        });
        let mut action =
            self.base_action("generate_midi", Some(request.preset_name.clone()), input);
        action.preset_name = Some(request.preset_name.clone());
        action.seed = Some(seed);

        let result = (|| -> Result<_, String> {
            let preset = load_preset(&request.preset_name, &self.state.preset_dir)
                .map_err(|err| err.to_string())?;
            action.preset_hash =
                Some(snapshot_preset_hash(&preset).map_err(|err| err.to_string())?);
            let composition = generate_composition(preset, seed).map_err(|err| err.to_string())?;
            let summary =
                export_generated_midi(&composition, PathBuf::from(&request.output_path).as_path())
                    .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "midi".to_string(),
                path: summary.path.clone(),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(MidiArtifactResponse {
                    path: summary.path.display().to_string(),
                    note_count: summary.note_count,
                    duration_beats: summary.duration_beats,
                    tempo_bpm: summary.tempo_bpm,
                    bytes_written: summary.bytes_written,
                    artifact_hash: summary.artifact_hash,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Generate a WAV file from a preset.")]
    async fn generate_audio(
        &self,
        Parameters(request): Parameters<GenerateArtifactRequest>,
    ) -> Result<Json<WavArtifactResponse>, String> {
        let seed = request.seed.unwrap_or(1);
        let input = json!({
            "preset_name": request.preset_name,
            "output_path": request.output_path,
            "seed": seed,
        });
        let mut action =
            self.base_action("generate_audio", Some(request.preset_name.clone()), input);
        action.preset_name = Some(request.preset_name.clone());
        action.seed = Some(seed);

        let result = (|| -> Result<_, String> {
            let preset = load_preset(&request.preset_name, &self.state.preset_dir)
                .map_err(|err| err.to_string())?;
            action.preset_hash =
                Some(snapshot_preset_hash(&preset).map_err(|err| err.to_string())?);
            let composition = generate_composition(preset, seed).map_err(|err| err.to_string())?;
            let summary =
                export_generated_wav(&composition, PathBuf::from(&request.output_path).as_path())
                    .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "wav".to_string(),
                path: summary.path.clone(),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(WavArtifactResponse {
                    path: summary.path.display().to_string(),
                    sample_count: summary.sample_count,
                    duration_seconds: summary.duration_seconds,
                    peak_amplitude: summary.peak_amplitude,
                    sample_rate: summary.sample_rate,
                    artifact_hash: summary.artifact_hash,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Create an approval request for a gated action.")]
    async fn approval_request(
        &self,
        Parameters(request): Parameters<NewApprovalRequest>,
    ) -> Result<Json<AuditedApprovalRequestResponse>, String> {
        let store_path = default_approval_store_path(&self.state.runtime_dir);
        let input = json!({
            "action_scope": request.action_scope,
            "target": request.target,
            "requested_by": request.requested_by,
            "reason": request.reason,
            "risk": request.risk,
        });
        let mut action = self.base_action("approval_request", Some(request.target.clone()), input);
        action.actor_id = request.requested_by.clone();

        let result = (|| -> Result<_, String> {
            let record = request_approval(&store_path, request).map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "approval-store".to_string(),
                path: store_path,
            });
            Ok(record)
        })();

        match result {
            Ok(record) => {
                let audit = self.record_success(&record, action)?;
                Ok(Json(AuditedApprovalRequestResponse {
                    approval: record,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Resolve an approval request and optionally issue a token.")]
    async fn approval_resolve(
        &self,
        Parameters(request): Parameters<ApprovalResolveRequest>,
    ) -> Result<Json<AuditedApprovalResolutionResponse>, String> {
        let store_path = default_approval_store_path(&self.state.runtime_dir);
        let input = json!({
            "approval_id": request.approval_id,
            "decision": request.decision,
            "reason": request.reason,
            "expires_in_seconds": request.expires_in_seconds,
        });
        let mut action =
            self.base_action("approval_resolve", Some(request.approval_id.clone()), input);
        action.actor_id = request.operator_id.clone();
        action.approval_ids = vec![request.approval_id.clone()];

        let result = (|| -> Result<_, String> {
            let resolution = resolve_approval(
                &store_path,
                &request.approval_id,
                request.decision,
                &request.operator_id,
                &request.reason,
                request.expires_in_seconds.unwrap_or(3600),
            )
            .map_err(|err| err.to_string())?;
            action.artifacts.push(ManifestArtifactInput {
                kind: "approval-store".to_string(),
                path: store_path,
            });
            Ok(resolution)
        })();

        match result {
            Ok(resolution) => {
                let audit = self.record_success(&resolution, action)?;
                Ok(Json(AuditedApprovalResolutionResponse {
                    resolution,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Create a snapshot of a file-backed preset.")]
    async fn snapshot_create(
        &self,
        Parameters(request): Parameters<SnapshotCreateRequest>,
    ) -> Result<Json<AuditedSnapshotCreateResponse>, String> {
        let snapshot_dir = default_snapshot_dir(&self.state.runtime_dir);
        let input = json!({
            "preset_name": request.preset_name,
            "reason": request.reason,
            "actor_id": request.actor_id,
        });
        let mut action =
            self.base_action("snapshot_create", Some(request.preset_name.clone()), input);
        action.actor_id = request
            .actor_id
            .clone()
            .unwrap_or_else(|| "mcp-client".to_string());
        action.preset_name = Some(request.preset_name.clone());

        let result = (|| -> Result<_, String> {
            let summary = create_preset_snapshot(
                &snapshot_dir,
                &self.state.preset_dir,
                &request.preset_name,
                &request.reason,
                request.actor_id.as_deref(),
            )
            .map_err(|err| err.to_string())?;
            action.preset_hash = Some(summary.preset_hash.clone());
            action.artifacts.push(ManifestArtifactInput {
                kind: "snapshot".to_string(),
                path: summary.snapshot_path.clone(),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(AuditedSnapshotCreateResponse {
                    snapshot: summary,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(description = "Roll back a preset from a stored snapshot.")]
    async fn snapshot_rollback(
        &self,
        Parameters(request): Parameters<SnapshotRollbackRequest>,
    ) -> Result<Json<AuditedSnapshotRollbackResponse>, String> {
        let input = json!({ "snapshot_id": request.snapshot_id });
        let mut action = self.base_action(
            "snapshot_rollback",
            Some(request.snapshot_id.clone()),
            input,
        );

        let result = (|| -> Result<_, String> {
            let summary = rollback_preset_snapshot(
                &default_snapshot_dir(&self.state.runtime_dir),
                &self.state.preset_dir,
                &request.snapshot_id,
            )
            .map_err(|err| err.to_string())?;
            action.preset_name = Some(summary.preset_name.clone());
            action.preset_hash = Some(summary.restored_preset_hash.clone());
            action.artifacts.push(ManifestArtifactInput {
                kind: "preset".to_string(),
                path: summary.output_path.clone(),
            });
            Ok(summary)
        })();

        match result {
            Ok(summary) => {
                let audit = self.record_success(&summary, action)?;
                Ok(Json(AuditedSnapshotRollbackResponse {
                    rollback: summary,
                    audit,
                }))
            }
            Err(error) => {
                if let Err(record_error) = self.record_failure(&error, ActionStatus::Failed, action)
                {
                    return Err(Self::merge_record_error(error, record_error));
                }
                Err(error)
            }
        }
    }

    #[tool(
        description = "Inspect a deterministic trajectory summary from a preset or stored system."
    )]
    async fn inspect_trajectory(
        &self,
        Parameters(request): Parameters<InspectTrajectoryRequest>,
    ) -> Result<Json<TrajectorySummaryResponse>, String> {
        let preset = match (
            request.preset_name.as_deref(),
            request.system_name.as_deref(),
        ) {
            (Some(preset_name), None) => {
                load_preset(preset_name, &self.state.preset_dir).map_err(|err| err.to_string())?
            }
            (None, Some(system_name)) => {
                let system = self.system_from_name(system_name)?;
                RenderPreset::from_system(
                    system_name.to_string(),
                    format!("Session-scoped preset from {system_name}"),
                    &system,
                )
            }
            (None, None) => load_preset(DEMO_PRESET_NAME, &self.state.preset_dir)
                .map_err(|err| err.to_string())?,
            (Some(_), Some(_)) => {
                return Err("provide either preset_name or system_name, not both".to_string());
            }
        };

        let system = preset.system.to_system().map_err(|err| err.to_string())?;
        let summary = summarize_trajectory(
            &simulate_trajectory(&system, &preset.simulation).map_err(|err| err.to_string())?,
        );

        Ok(Json(TrajectorySummaryResponse {
            frame_count: summary.frame_count,
            duration_seconds: summary.duration_seconds,
            min_output: summary.min_output,
            max_output: summary.max_output,
            mean_abs_output: summary.mean_abs_output,
            peak_abs_output: summary.peak_abs_output,
            preview: summary.preview,
        }))
    }
}

pub async fn start_mcp_server(preset_dir: PathBuf, runtime_dir: PathBuf) -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .try_init();
    info!("Starting state-space-music-box MCP server");

    let server = MusicBoxMcpServer::new(preset_dir, runtime_dir);
    server
        .serve(rmcp::transport::stdio())
        .await
        .context("failed to start MCP stdio server")?
        .waiting()
        .await
        .context("MCP server exited with an error")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use midly::Smf;
    use rmcp::{model::CallToolRequestParams, ServiceExt};
    use tempfile::tempdir;

    use super::*;
    use crate::generation::{demo_preset, save_preset};

    fn test_server(preset_dir: PathBuf, runtime_dir: PathBuf) -> MusicBoxMcpServer {
        MusicBoxMcpServer::with_state(MusicBoxMcpState::new(preset_dir, runtime_dir))
    }

    #[tokio::test]
    async fn test_mcp_tools_list_and_generate_midi() {
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("demo.mid");
        let server = test_server(dir.path().join("presets"), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");
        let tools = client.peer().list_tools(Default::default()).await.unwrap();
        assert!(tools.tools.iter().any(|tool| tool.name == "generate_midi"));
        assert!(tools.tools.iter().any(|tool| tool.name == "run_list"));
        assert!(tools.tools.iter().any(|tool| tool.name == "audit_list"));
        assert!(tools.tools.iter().any(|tool| tool.name == "session_create"));
        assert!(tools.tools.iter().any(|tool| tool.name == "session_play"));
        assert!(tools
            .tools
            .iter()
            .any(|tool| tool.name == "session_render_preview"));
        assert!(tools
            .tools
            .iter()
            .any(|tool| tool.name == "evaluation_submit"));
        assert!(tools.tools.iter().any(|tool| tool.name == "review_build"));
        assert!(tools.tools.iter().any(|tool| tool.name == "deck_create"));
        assert!(tools.tools.iter().any(|tool| tool.name == "deck_launch"));
        assert!(tools.tools.iter().any(|tool| tool.name == "deck_transport"));
        assert!(tools.tools.iter().any(|tool| tool.name == "harness_plan"));
        assert!(tools
            .tools
            .iter()
            .any(|tool| tool.name == "harness_execute"));
        assert!(tools.tools.iter().any(|tool| tool.name == "job_validate"));
        assert!(tools.tools.iter().any(|tool| tool.name == "job_schedule"));
        assert!(tools.tools.iter().any(|tool| tool.name == "job_run"));
        assert!(tools
            .tools
            .iter()
            .any(|tool| tool.name == "realtime_create"));
        assert!(tools
            .tools
            .iter()
            .any(|tool| tool.name == "realtime_send_preview"));
        assert!(tools
            .tools
            .iter()
            .any(|tool| tool.name == "realtime_send_transport"));

        let args = serde_json::json!({
            "preset_name": DEMO_PRESET_NAME,
            "output_path": output_path,
            "seed": 2
        });
        let result = client
            .call_tool(
                CallToolRequestParams::new("generate_midi")
                    .with_arguments(args.as_object().unwrap().clone()),
            )
            .await
            .unwrap();
        let typed: MidiArtifactResponse = result.into_typed().unwrap();

        assert!(PathBuf::from(&typed.path).exists());
        let bytes = fs::read(&typed.path).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        assert_eq!(smf.tracks.len(), 1);

        let runs: RunListResponse = client
            .call_tool(CallToolRequestParams::new("run_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(runs.runs.len(), 1);

        let inspect_args = serde_json::json!({
            "run_id": typed.audit.run_id
        });
        let manifest: RunManifestRecord = client
            .call_tool(
                CallToolRequestParams::new("run_inspect")
                    .with_arguments(inspect_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(manifest.action, "generate_midi");

        let audits: AuditListResponse = client
            .call_tool(
                CallToolRequestParams::new("audit_list").with_arguments(
                    serde_json::json!({ "limit": 10 })
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(audits.events.len(), 1);

        server_task.abort();
    }

    #[tokio::test]
    async fn test_mcp_generate_audio_error_path() {
        let dir = tempdir().unwrap();
        let server = test_server(default_preset_dir(), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");
        let args = serde_json::json!({
            "preset_name": "missing",
            "output_path": "out/missing.wav",
            "seed": 1
        });
        let result = client
            .call_tool(
                CallToolRequestParams::new("generate_audio")
                    .with_arguments(args.as_object().unwrap().clone()),
            )
            .await
            .unwrap();

        assert_eq!(result.is_error, Some(true));
        server_task.abort();
    }

    #[tokio::test]
    async fn test_create_system_and_inspect_trajectory() {
        let dir = tempdir().unwrap();
        let server = test_server(default_preset_dir(), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");
        let create_args = serde_json::json!({
            "name": "session-oscillator",
            "a": { "rows": 2, "cols": 2, "data": [0.0, 1.0, -1.0, -0.2] },
            "b": { "rows": 2, "cols": 0, "data": [] },
            "c": { "rows": 1, "cols": 2, "data": [1.0, 0.0] },
            "d": { "rows": 1, "cols": 0, "data": [] },
            "dt": null
        });
        let summary: SystemSummary = client
            .call_tool(
                CallToolRequestParams::new("create_system")
                    .with_arguments(create_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(summary.name, "session-oscillator");

        let inspect_args = serde_json::json!({
            "system_name": "session-oscillator"
        });
        let trajectory: TrajectorySummaryResponse = client
            .call_tool(
                CallToolRequestParams::new("inspect_trajectory")
                    .with_arguments(inspect_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(trajectory.frame_count > 0);
        assert!(trajectory.preview.iter().all(|value| value.is_finite()));

        server_task.abort();
    }

    #[tokio::test]
    async fn test_mcp_approval_and_dataset_register_round_trip() {
        let dir = tempdir().unwrap();
        let server = test_server(dir.path().join("presets"), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");
        let approval_args = serde_json::json!({
            "action_scope": "dataset.register",
            "target": "pdmx",
            "requested_by": "tester",
            "reason": "register dataset",
            "risk": "approval_required"
        });
        let approval: AuditedApprovalRequestResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_request")
                    .with_arguments(approval_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let resolve_args = serde_json::json!({
            "approval_id": approval.approval.approval_id,
            "operator_id": "approver",
            "decision": "approve",
            "reason": "approved",
            "expires_in_seconds": 600
        });
        let resolution: AuditedApprovalResolutionResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_resolve")
                    .with_arguments(resolve_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let register_args = serde_json::json!({
            "dataset_id": "pdmx",
            "display_name": "PDMX",
            "source_url": "https://example.com/pdmx",
            "citation": "Example citation",
            "license_name": "CC-BY-4.0",
            "commercial_use_status": "allowed",
            "redistribution_status": "allowed",
            "approved_use_class": "production_allowed",
            "checksum_manifest": [
                {
                    "relative_path": "archive.tar.gz",
                    "sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                }
            ],
            "local_storage_path": dir.path().join("datasets/pdmx"),
            "dataset_version": "v1",
            "split_policy": "train/valid/test",
            "transform_pipeline_hash": "pipeline-hash",
            "parent_datasets": [],
            "approval_token": resolution.resolution.approval_token,
            "notes": "ready"
        });
        let dataset: AuditedDatasetRecordResponse = client
            .call_tool(
                CallToolRequestParams::new("dataset_register")
                    .with_arguments(register_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(dataset.dataset.dataset_id, "pdmx");

        let list: DatasetListResponse = client
            .call_tool(CallToolRequestParams::new("dataset_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(list.datasets.len(), 1);

        server_task.abort();
    }

    #[tokio::test]
    async fn test_mcp_snapshot_create_and_rollback() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");

        let mut preset = demo_preset();
        preset.name = "session-preset".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let server = test_server(preset_dir.clone(), runtime_dir);
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");
        let create_args = serde_json::json!({
            "preset_name": "session-preset",
            "reason": "before mutation",
            "actor_id": "tester"
        });
        let snapshot: AuditedSnapshotCreateResponse = client
            .call_tool(
                CallToolRequestParams::new("snapshot_create")
                    .with_arguments(create_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let mut changed = load_preset("session-preset", &preset_dir).unwrap();
        changed.midi.tempo_bpm = 90;
        save_preset(&changed, &preset_dir).unwrap();

        let rollback_args = serde_json::json!({
            "snapshot_id": snapshot.snapshot.snapshot_id
        });
        let rollback: AuditedSnapshotRollbackResponse = client
            .call_tool(
                CallToolRequestParams::new("snapshot_rollback")
                    .with_arguments(rollback_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let restored = load_preset("session-preset", &preset_dir).unwrap();
        assert_eq!(rollback.rollback.preset_name, "session-preset");
        assert_eq!(restored.midi.tempo_bpm, preset.midi.tempo_bpm);

        server_task.abort();
    }

    #[tokio::test]
    async fn test_mcp_session_and_evaluation_round_trip() {
        let dir = tempdir().unwrap();
        let server = test_server(dir.path().join("presets"), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");

        let generate_args = serde_json::json!({
            "preset_name": DEMO_PRESET_NAME,
            "output_path": dir.path().join("demo.mid"),
            "seed": 4
        });
        let rendered: MidiArtifactResponse = client
            .call_tool(
                CallToolRequestParams::new("generate_midi")
                    .with_arguments(generate_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let session_create_args = serde_json::json!({
            "display_name": "MCP Session",
            "preset_name": DEMO_PRESET_NAME,
            "seed": 4,
            "actor_id": "tester"
        });
        let session: AuditedSessionResponse = client
            .call_tool(
                CallToolRequestParams::new("session_create")
                    .with_arguments(session_create_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(session.session.display_name, "MCP Session");

        let session_update_args = serde_json::json!({
            "session_id": session.session.session_id,
            "actor_id": "tester",
            "tempo_bpm": 90,
            "status": "playing"
        });
        let updated: AuditedSessionResponse = client
            .call_tool(
                CallToolRequestParams::new("session_update")
                    .with_arguments(session_update_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(updated.session.tempo_bpm, 90);

        let play_args = serde_json::json!({
            "session_id": session.session.session_id,
            "actor_id": "tester",
            "run_label": "live-pass"
        });
        let playing: AuditedSessionResponse = client
            .call_tool(
                CallToolRequestParams::new("session_play")
                    .with_arguments(play_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            playing.session.active_run_label.as_deref(),
            Some("live-pass")
        );

        let preview_args = serde_json::json!({
            "session_id": session.session.session_id,
            "actor_id": "tester"
        });
        let preview: AuditedSessionPreviewResponse = client
            .call_tool(
                CallToolRequestParams::new("session_render_preview")
                    .with_arguments(preview_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(preview.preview.preview.midi.path.exists());
        assert!(preview.preview.preview.wav.path.exists());

        let stop_args = serde_json::json!({
            "session_id": session.session.session_id,
            "actor_id": "tester"
        });
        let stopped: AuditedSessionResponse = client
            .call_tool(
                CallToolRequestParams::new("session_stop")
                    .with_arguments(stop_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(stopped.session.status, SessionStatus::Stopped);

        let deck_create_args = serde_json::json!({
            "display_name": "Main Deck",
            "session_id": session.session.session_id,
            "actor_id": "tester"
        });
        let deck: AuditedDeckResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_create")
                    .with_arguments(deck_create_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(deck.deck.display_name, "Main Deck");

        let deck_add_args = serde_json::json!({
            "deck_id": deck.deck.deck_id,
            "session_id": session.session.session_id,
            "preview_id": preview.preview.preview.preview_id,
            "label": "Clip One",
            "actor_id": "tester"
        });
        let deck_with_clip: AuditedDeckResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_add_preview")
                    .with_arguments(deck_add_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(deck_with_clip.deck.clips.len(), 1);

        let queue_args = serde_json::json!({
            "deck_id": deck_with_clip.deck.deck_id,
            "clip_id": deck_with_clip.deck.clips[0].clip_id,
            "actor_id": "tester"
        });
        let queued: AuditedDeckResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_queue")
                    .with_arguments(queue_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(queued.deck.queued_clip_id.is_some());

        let launch_args = serde_json::json!({
            "deck_id": queued.deck.deck_id,
            "clip_id": queued.deck.clips[0].clip_id,
            "actor_id": "tester"
        });
        let launched: AuditedDeckTransportResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_launch")
                    .with_arguments(launch_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            launched.snapshot.deck.transport_state,
            crate::governance::DeckTransportState::Playing
        );

        let transport_args = serde_json::json!({
            "deck_id": queued.deck.deck_id
        });
        let transport: DeckTransportSnapshot = client
            .call_tool(
                CallToolRequestParams::new("deck_transport")
                    .with_arguments(transport_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(transport.active_clip.is_some());

        let deck_stop_args = serde_json::json!({
            "deck_id": queued.deck.deck_id,
            "actor_id": "tester"
        });
        let deck_stopped: AuditedDeckTransportResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_stop")
                    .with_arguments(deck_stop_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            deck_stopped.snapshot.deck.transport_state,
            crate::governance::DeckTransportState::Stopped
        );

        let harness_plan_args = serde_json::json!({
            "role": "session_dj",
            "prompt": "set tempo to 132 and render a preview",
            "session_id": session.session.session_id
        });
        let harness_plan: HarnessPlanRecord = client
            .call_tool(
                CallToolRequestParams::new("harness_plan")
                    .with_arguments(harness_plan_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(harness_plan
            .proposed_actions
            .iter()
            .any(|action| action.tool_name == "live.apply_patch"));

        let apply_action = harness_plan
            .proposed_actions
            .iter()
            .find(|action| action.tool_name == "live.apply_patch")
            .unwrap();
        let harness_execute_args = serde_json::json!({
            "plan_id": harness_plan.plan_id,
            "action_id": apply_action.action_id
        });
        let harness_outcome: HarnessExecuteResponse = client
            .call_tool(
                CallToolRequestParams::new("harness_execute")
                    .with_arguments(harness_execute_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            harness_outcome.outcome.status,
            crate::governance::HarnessOutcomeStatus::Succeeded
        );

        let comparison_args = serde_json::json!({
            "run_ids": [rendered.audit.run_id.clone(), session.audit.run_id.clone()]
        });
        let comparison: RunComparisonSummary = client
            .call_tool(
                CallToolRequestParams::new("run_compare")
                    .with_arguments(comparison_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(comparison.runs.len(), 2);

        let evaluation_args = serde_json::json!({
            "run_ids": [rendered.audit.run_id.clone()],
            "objective_metrics": { "note_density": 0.8 },
            "human_scores": { "musicality": 6 },
            "reward_weights": { "musicality": 1.0 },
            "notes": "strong run",
            "decision": "promote",
            "created_by": "tester"
        });
        let evaluation: AuditedEvaluationResponse = client
            .call_tool(
                CallToolRequestParams::new("evaluation_submit")
                    .with_arguments(evaluation_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(evaluation.evaluation.aggregate_score, 6.0);

        let evaluation_inspect_args = serde_json::json!({
            "evaluation_id": evaluation.evaluation.evaluation_id
        });
        let inspected_evaluation: EvaluationRecord = client
            .call_tool(
                CallToolRequestParams::new("evaluation_inspect")
                    .with_arguments(evaluation_inspect_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(inspected_evaluation.aggregate_score, 6.0);

        let review_args = serde_json::json!({
            "run_ids": [rendered.audit.run_id.clone(), playing.audit.run_id.clone()]
        });
        let review: ReviewBundle = client
            .call_tool(
                CallToolRequestParams::new("review_build")
                    .with_arguments(review_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(review.comparison.runs.len(), 2);

        let evaluations: EvaluationListResponse = client
            .call_tool(CallToolRequestParams::new("evaluation_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(evaluations.evaluations.len(), 1);

        let sessions: SessionListResponse = client
            .call_tool(CallToolRequestParams::new("session_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(sessions.sessions.len(), 1);

        let harness_outcomes: HarnessOutcomeListResponse = client
            .call_tool(CallToolRequestParams::new("harness_outcome_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(harness_outcomes.outcomes.len(), 1);

        server_task.abort();
    }

    #[tokio::test]
    async fn test_mcp_scheduler_tools_validate_schedule_run_and_cancel() {
        let dir = tempdir().unwrap();
        let server = test_server(default_preset_dir(), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");

        let session_args = serde_json::json!({
            "display_name": "Scheduled Session",
            "preset_name": DEMO_PRESET_NAME,
            "actor_id": "tester",
            "seed": 7
        });
        let session: AuditedSessionResponse = client
            .call_tool(
                CallToolRequestParams::new("session_create")
                    .with_arguments(session_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let validate_args = serde_json::json!({
            "backend": "local_cli",
            "role": "session_dj",
            "prompt": "set tempo to 132 and render a preview",
            "session_id": session.session.session_id,
            "retry_limit": 1
        });
        let validation: JobValidationResult = client
            .call_tool(
                CallToolRequestParams::new("job_validate")
                    .with_arguments(validate_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(validation.allowed);

        let schedule_approval_args = serde_json::json!({
            "action_scope": "jobs.schedule",
            "target": "nightly-preview",
            "requested_by": "tester",
            "reason": "schedule unattended run",
            "risk": "approval_required"
        });
        let schedule_approval: AuditedApprovalRequestResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_request")
                    .with_arguments(schedule_approval_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let schedule_resolve_args = serde_json::json!({
            "approval_id": schedule_approval.approval.approval_id,
            "operator_id": "approver",
            "decision": "approve",
            "reason": "approved",
            "expires_in_seconds": 600
        });
        let schedule_resolution: AuditedApprovalResolutionResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_resolve")
                    .with_arguments(schedule_resolve_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let schedule_args = serde_json::json!({
            "job_name": "nightly-preview",
            "backend": "local_cli",
            "role": "session_dj",
            "prompt": "set tempo to 132 and render a preview",
            "session_id": session.session.session_id,
            "requested_by": "tester",
            "retry_limit": 1,
            "approval_token": schedule_resolution.resolution.approval_token
        });
        let scheduled: AuditedJobResponse = client
            .call_tool(
                CallToolRequestParams::new("job_schedule")
                    .with_arguments(schedule_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            scheduled.job.status,
            crate::governance::ScheduledJobStatus::Scheduled
        );

        let jobs: JobListResponse = client
            .call_tool(CallToolRequestParams::new("job_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(jobs.jobs.len(), 1);

        let inspect_args = serde_json::json!({
            "job_id": scheduled.job.job_id
        });
        let inspected: ScheduledJobRecord = client
            .call_tool(
                CallToolRequestParams::new("job_inspect")
                    .with_arguments(inspect_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(inspected.job_id, scheduled.job.job_id);

        let run_args = serde_json::json!({
            "job_id": scheduled.job.job_id
        });
        let run_summary: AuditedJobRunResponse = client
            .call_tool(
                CallToolRequestParams::new("job_run")
                    .with_arguments(run_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            run_summary.summary.job.status,
            crate::governance::ScheduledJobStatus::Completed
        );
        assert_eq!(run_summary.summary.job.runs.len(), 1);

        let schedule_cancel_approval_args = serde_json::json!({
            "action_scope": "jobs.schedule",
            "target": "cancel-me",
            "requested_by": "tester",
            "reason": "schedule cancellation candidate",
            "risk": "approval_required"
        });
        let schedule_cancel_approval: AuditedApprovalRequestResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_request")
                    .with_arguments(schedule_cancel_approval_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let schedule_cancel_resolve_args = serde_json::json!({
            "approval_id": schedule_cancel_approval.approval.approval_id,
            "operator_id": "approver",
            "decision": "approve",
            "reason": "approved",
            "expires_in_seconds": 600
        });
        let schedule_cancel_resolution: AuditedApprovalResolutionResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_resolve")
                    .with_arguments(schedule_cancel_resolve_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let cancel_candidate_schedule_args = serde_json::json!({
            "job_name": "cancel-me",
            "backend": "local_cli",
            "role": "session_dj",
            "prompt": "render a preview",
            "session_id": session.session.session_id,
            "requested_by": "tester",
            "retry_limit": 1,
            "approval_token": schedule_cancel_resolution.resolution.approval_token
        });
        let cancel_candidate: AuditedJobResponse = client
            .call_tool(
                CallToolRequestParams::new("job_schedule")
                    .with_arguments(cancel_candidate_schedule_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let cancel_approval_args = serde_json::json!({
            "action_scope": "jobs.cancel",
            "target": cancel_candidate.job.job_id,
            "requested_by": "tester",
            "reason": "cancel scheduled job before execution",
            "risk": "approval_required"
        });
        let cancel_approval: AuditedApprovalRequestResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_request")
                    .with_arguments(cancel_approval_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let cancel_resolve_args = serde_json::json!({
            "approval_id": cancel_approval.approval.approval_id,
            "operator_id": "approver",
            "decision": "approve",
            "reason": "approved",
            "expires_in_seconds": 600
        });
        let cancel_resolution: AuditedApprovalResolutionResponse = client
            .call_tool(
                CallToolRequestParams::new("approval_resolve")
                    .with_arguments(cancel_resolve_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let cancel_args = serde_json::json!({
            "job_id": cancel_candidate.job.job_id,
            "requested_by": "tester",
            "approval_token": cancel_resolution.resolution.approval_token
        });
        let cancelled: AuditedJobResponse = client
            .call_tool(
                CallToolRequestParams::new("job_cancel")
                    .with_arguments(cancel_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(
            cancelled.job.status,
            crate::governance::ScheduledJobStatus::Cancelled
        );

        server_task.abort();
    }

    #[tokio::test]
    async fn test_mcp_realtime_tools_create_and_dispatch() {
        let dir = tempdir().unwrap();
        let server = test_server(default_preset_dir(), dir.path().join("runtime"));
        let (server_transport, client_transport) = tokio::io::duplex(8_192);

        let server_task = tokio::spawn(async move {
            let running = server
                .serve(server_transport)
                .await
                .expect("server should initialize");
            let _ = running.waiting().await.expect("server should keep running");
        });

        let client = ().serve(client_transport).await.expect("MCP client should initialize");
        let listener = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
        listener
            .set_read_timeout(Some(std::time::Duration::from_millis(250)))
            .unwrap();
        let port = listener.local_addr().unwrap().port();

        let create_args = serde_json::json!({
            "display_name": "Loopback",
            "protocol": "osc_udp",
            "host": "127.0.0.1",
            "port": port,
            "base_path": "/agentic_dj"
        });
        let adapter: AuditedRealtimeAdapterResponse = client
            .call_tool(
                CallToolRequestParams::new("realtime_create")
                    .with_arguments(create_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let session_args = serde_json::json!({
            "display_name": "Realtime Session",
            "preset_name": DEMO_PRESET_NAME,
            "actor_id": "tester",
            "seed": 6
        });
        let session: AuditedSessionResponse = client
            .call_tool(
                CallToolRequestParams::new("session_create")
                    .with_arguments(session_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let preview_args = serde_json::json!({
            "session_id": session.session.session_id,
            "actor_id": "tester"
        });
        let preview: AuditedSessionPreviewResponse = client
            .call_tool(
                CallToolRequestParams::new("session_render_preview")
                    .with_arguments(preview_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let send_preview_args = serde_json::json!({
            "adapter_id": adapter.adapter.adapter_id,
            "session_id": session.session.session_id,
            "preview_id": preview.preview.preview.preview_id,
            "actor_id": "tester",
            "dispatch_mode": "immediate",
            "time_scale": 0.0
        });
        let preview_dispatch: AuditedRealtimeDispatchResponse = client
            .call_tool(
                CallToolRequestParams::new("realtime_send_preview")
                    .with_arguments(send_preview_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(preview_dispatch.summary.dispatch.message_count >= 3);

        let deck_args = serde_json::json!({
            "display_name": "Realtime Deck",
            "session_id": session.session.session_id,
            "actor_id": "tester"
        });
        let deck: AuditedDeckResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_create")
                    .with_arguments(deck_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let add_args = serde_json::json!({
            "deck_id": deck.deck.deck_id,
            "session_id": session.session.session_id,
            "preview_id": preview.preview.preview.preview_id,
            "label": "Clip One",
            "actor_id": "tester"
        });
        let deck_with_clip: AuditedDeckResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_add_preview")
                    .with_arguments(add_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let launch_args = serde_json::json!({
            "deck_id": deck_with_clip.deck.deck_id,
            "clip_id": deck_with_clip.deck.clips[0].clip_id,
            "actor_id": "tester"
        });
        let _launched: AuditedDeckTransportResponse = client
            .call_tool(
                CallToolRequestParams::new("deck_launch")
                    .with_arguments(launch_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();

        let send_transport_args = serde_json::json!({
            "adapter_id": adapter.adapter.adapter_id,
            "deck_id": deck_with_clip.deck.deck_id,
            "actor_id": "tester",
            "dispatch_mode": "immediate",
            "time_scale": 0.0
        });
        let transport_dispatch: AuditedRealtimeDispatchResponse = client
            .call_tool(
                CallToolRequestParams::new("realtime_send_transport")
                    .with_arguments(send_transport_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(transport_dispatch.summary.dispatch.message_count >= 1);

        let list: RealtimeListResponse = client
            .call_tool(CallToolRequestParams::new("realtime_list"))
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert_eq!(list.adapters.len(), 1);

        let inspect_args = serde_json::json!({
            "adapter_id": adapter.adapter.adapter_id
        });
        let inspected: RealtimeAdapterRecord = client
            .call_tool(
                CallToolRequestParams::new("realtime_inspect")
                    .with_arguments(inspect_args.as_object().unwrap().clone()),
            )
            .await
            .unwrap()
            .into_typed()
            .unwrap();
        assert!(inspected.dispatches.len() >= 2);

        let mut buf = [0u8; 2048];
        let (size, _) = listener.recv_from(&mut buf).unwrap();
        let packet = rosc::decoder::decode_udp(&buf[..size]).unwrap().1;
        match packet {
            rosc::OscPacket::Message(message) => assert!(message.addr.starts_with("/agentic_dj/")),
            other => panic!("unexpected OSC packet: {other:?}"),
        }

        server_task.abort();
    }
}
