use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{anyhow, ensure, Result};
use clap::{Args, CommandFactory, Parser, Subcommand};
use serde::Serialize;
use serde_json::json;
use tracing::info;

use crate::generation::{
    default_preset_dir, export_generated_midi, export_generated_wav, generate_composition,
    list_presets, load_preset, simulate_trajectory, summarize_trajectory, DEMO_PRESET_NAME,
};
use crate::governance::{
    add_preview_clip_to_deck, apply_transport_command, build_review_bundle, compare_runs,
    consume_approval_token, create_deck, create_harness_plan, create_preset_snapshot,
    create_session, default_approval_store_path, default_dataset_registry_path,
    default_daw_store_path, default_evaluation_store_path, default_harness_store_path,
    default_realtime_store_path, default_review_dir, default_runtime_dir,
    default_scheduler_store_path, default_session_store_path, default_snapshot_dir,
    execute_harness_action, export_review_bundle, inspect_dataset_record, inspect_deck,
    inspect_deck_transport, inspect_evaluation_record, inspect_harness_plan,
    inspect_realtime_adapter, inspect_run_manifest, inspect_scheduled_job, inspect_session,
    launch_deck_clip, list_dataset_records, list_decks, list_evaluation_records,
    list_harness_outcomes, list_realtime_adapters, list_run_manifests, list_scheduled_jobs,
    list_sessions, persist_action_record, queue_deck_clip, read_audit_events,
    register_dataset_record, render_session_preview, request_approval, resolve_approval,
    rollback_preset_snapshot, run_scheduled_job, schedule_job, send_preview_to_realtime_adapter,
    send_transport_to_realtime_adapter, snapshot_preset_hash, stop_deck, submit_evaluation_record,
    update_session, validate_scheduled_job, ActionAuditRef, ActionRisk, ActionStatus,
    ActionTransport, AddDeckPreviewRequest, ApprovalDecisionKind, ApprovedUseClass,
    CancelScheduledJobRequest, ChecksumEntry, EvaluationDecision, ExecuteHarnessActionRequest,
    HarnessRole, LaunchDeckClipRequest, ManifestArtifactInput, NewActionRecord, NewApprovalRequest,
    NewDatasetRecord, NewDeckRequest, NewEvaluationRecord, NewHarnessPlanRequest,
    NewRealtimeAdapterRequest, NewScheduledJobRequest, NewSessionRequest, PolicyStatus,
    QueueDeckClipRequest, RealtimeAdapterProtocol, RealtimeDispatchMode, ReviewBundleExportSummary,
    SchedulerBackend, SendRealtimePreviewRequest, SendRealtimeTransportRequest, SessionStatus,
    SessionTransportCommand, SessionTransportRequest, StopDeckRequest, UpdateSessionRequest,
    ValidateScheduledJobRequest,
};

/// Command-line interface for the state-space-music-box library.
#[derive(Parser, Default)]
#[command(name = "state-space-music-box")]
#[command(about = "Generate deterministic MIDI and WAV artifacts from state-space music presets", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Args, Debug)]
pub struct DatasetRegisterArgs {
    /// Dataset identifier
    #[arg(long)]
    dataset_id: String,

    /// Human-readable dataset name
    #[arg(long)]
    display_name: String,

    /// Canonical source URL
    #[arg(long)]
    source_url: String,

    /// Optional citation text
    #[arg(long)]
    citation: Option<String>,

    /// Dataset license
    #[arg(long)]
    license_name: String,

    /// Whether commercial use is allowed
    #[arg(long, value_enum)]
    commercial_use_status: PolicyStatus,

    /// Whether redistribution is allowed
    #[arg(long, value_enum)]
    redistribution_status: PolicyStatus,

    /// Approved use class in the local policy layer
    #[arg(long, value_enum)]
    approved_use_class: ApprovedUseClass,

    /// One or more checksum entries formatted as relative_path=sha256
    #[arg(long = "checksum")]
    checksum_manifest: Vec<String>,

    /// Local storage path for the dataset
    #[arg(long)]
    local_storage_path: PathBuf,

    /// Dataset version string
    #[arg(long)]
    dataset_version: String,

    /// Optional split policy description
    #[arg(long)]
    split_policy: Option<String>,

    /// Optional transform pipeline hash
    #[arg(long)]
    transform_pipeline_hash: Option<String>,

    /// Parent dataset ids
    #[arg(long = "parent-dataset")]
    parent_datasets: Vec<String>,

    /// Approval token authorizing dataset registration
    #[arg(long)]
    approval_token: String,

    /// Optional notes
    #[arg(long)]
    notes: Option<String>,

    /// Directory containing runtime state such as approvals and datasets
    #[arg(long, default_value_os_t = default_runtime_dir())]
    runtime_dir: PathBuf,
}

#[derive(Args, Debug)]
pub struct SessionUpdateArgs {
    /// Session identifier to update
    #[arg(long)]
    session_id: String,

    /// Actor applying the update
    #[arg(long)]
    actor_id: String,

    /// Optional display name override
    #[arg(long)]
    display_name: Option<String>,

    /// Optional preset name override
    #[arg(long)]
    preset_name: Option<String>,

    /// Optional deterministic seed override
    #[arg(long)]
    seed: Option<u64>,

    /// Optional tempo override
    #[arg(long)]
    tempo_bpm: Option<u16>,

    /// Optional session status override
    #[arg(long, value_enum)]
    status: Option<SessionStatus>,

    /// Directory containing user-defined presets
    #[arg(long, default_value_os_t = default_preset_dir())]
    preset_dir: PathBuf,

    /// Directory containing runtime state such as session records
    #[arg(long, default_value_os_t = default_runtime_dir())]
    runtime_dir: PathBuf,
}

#[derive(Args, Debug)]
pub struct EvaluationSubmitArgs {
    /// One or more run ids to score
    #[arg(long = "run-id")]
    run_ids: Vec<String>,

    /// Objective metric entries formatted as key=value
    #[arg(long = "metric")]
    objective_metrics: Vec<String>,

    /// Human score entries formatted as key=1..7
    #[arg(long = "human-score")]
    human_scores: Vec<String>,

    /// Reward weight entries formatted as key=value
    #[arg(long = "weight")]
    reward_weights: Vec<String>,

    /// Optional notes
    #[arg(long)]
    notes: Option<String>,

    /// Final decision for the reviewed run set
    #[arg(long, value_enum)]
    decision: EvaluationDecision,

    /// Actor creating the evaluation
    #[arg(long)]
    created_by: String,

    /// Directory containing runtime state such as evaluations and manifests
    #[arg(long, default_value_os_t = default_runtime_dir())]
    runtime_dir: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start the MCP server for agent integration over stdio
    #[command(name = "mcp")]
    Mcp {
        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Start the HTTP API server for agent and webapp access
    #[command(name = "http")]
    Http {
        /// Port to listen on
        #[arg(long, default_value_t = 3001)]
        port: u16,

        /// API key for bearer token authentication
        #[arg(long, env = "MUSIC_BOX_API_KEY")]
        api_key: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Generate both MIDI and WAV artifacts from the built-in demo preset
    #[command(name = "generate-demo")]
    GenerateDemo {
        /// Output path for the MIDI artifact
        #[arg(long)]
        midi: Option<PathBuf>,

        /// Output path for the WAV artifact
        #[arg(long)]
        wav: Option<PathBuf>,

        /// Deterministic seed for note mapping
        #[arg(long, default_value_t = 1)]
        seed: u64,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Generate a MIDI file from a preset
    #[command(name = "generate-midi")]
    GenerateMidi {
        /// Preset name to render
        #[arg(long, default_value = DEMO_PRESET_NAME)]
        preset: String,

        /// Output path for the MIDI artifact
        #[arg(long)]
        output: PathBuf,

        /// Deterministic seed for note mapping
        #[arg(long, default_value_t = 1)]
        seed: u64,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Generate a WAV file from a preset
    #[command(name = "generate-audio")]
    GenerateAudio {
        /// Preset name to render
        #[arg(long, default_value = DEMO_PRESET_NAME)]
        preset: String,

        /// Output path for the WAV artifact
        #[arg(long)]
        output: PathBuf,

        /// Deterministic seed for note mapping
        #[arg(long, default_value_t = 1)]
        seed: u64,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect the simulated state trajectory for a preset
    #[command(name = "inspect-trajectory")]
    InspectTrajectory {
        /// Preset name to inspect
        #[arg(long, default_value = DEMO_PRESET_NAME)]
        preset: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,
    },

    /// List available built-in and file-backed presets
    #[command(name = "list-presets")]
    ListPresets {
        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,
    },

    /// List persisted run manifests from the local runtime store
    #[command(name = "run-list")]
    RunList {
        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Compare two or more persisted runs
    #[command(name = "run-compare")]
    RunCompare {
        /// Run ids to compare
        #[arg(long = "run-id")]
        run_ids: Vec<String>,

        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one persisted run manifest by id
    #[command(name = "run-inspect")]
    RunInspect {
        /// Run identifier to inspect
        #[arg(long)]
        run_id: String,

        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// List audit events from the local runtime store
    #[command(name = "audit-list")]
    AuditList {
        /// Directory containing runtime state such as manifests and audit logs
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,

        /// Optional limit for the most recent events
        #[arg(long)]
        limit: Option<usize>,
    },

    /// List local sessions
    #[command(name = "session-list")]
    SessionList {
        /// Directory containing runtime state such as session records
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Create a local session record
    #[command(name = "session-create")]
    SessionCreate {
        /// Session display name
        #[arg(long)]
        display_name: String,

        /// Preset to load into the session
        #[arg(long, default_value = DEMO_PRESET_NAME)]
        preset: String,

        /// Initial deterministic seed
        #[arg(long, default_value_t = 1)]
        seed: u64,

        /// Actor creating the session
        #[arg(long)]
        actor_id: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as session records
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one local session
    #[command(name = "session-inspect")]
    SessionInspect {
        /// Session identifier to inspect
        #[arg(long)]
        session_id: String,

        /// Directory containing runtime state such as session records
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Update a local session
    #[command(name = "session-update")]
    SessionUpdate(Box<SessionUpdateArgs>),

    /// Mark a local session as actively playing
    #[command(name = "session-play")]
    SessionPlay {
        /// Session identifier to control
        #[arg(long)]
        session_id: String,

        /// Actor issuing the transport command
        #[arg(long)]
        actor_id: String,

        /// Optional run label for the active live pass
        #[arg(long)]
        run_label: Option<String>,

        /// Directory containing runtime state such as session records
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Mark a local session as stopped
    #[command(name = "session-stop")]
    SessionStop {
        /// Session identifier to control
        #[arg(long)]
        session_id: String,

        /// Actor issuing the transport command
        #[arg(long)]
        actor_id: String,

        /// Directory containing runtime state such as session records
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Render a deterministic preview bundle for a local session
    #[command(name = "session-render-preview")]
    SessionRenderPreview {
        /// Session identifier to preview
        #[arg(long)]
        session_id: String,

        /// Actor issuing the preview render
        #[arg(long)]
        actor_id: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as session records and previews
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Submit an evaluation record for one or more runs
    #[command(name = "evaluation-submit")]
    EvaluationSubmit(Box<EvaluationSubmitArgs>),

    /// List evaluation records
    #[command(name = "evaluation-list")]
    EvaluationList {
        /// Directory containing runtime state such as evaluations
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one evaluation record
    #[command(name = "evaluation-inspect")]
    EvaluationInspect {
        /// Evaluation identifier to inspect
        #[arg(long)]
        evaluation_id: String,

        /// Directory containing runtime state such as evaluations
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Build a side-by-side review bundle for two or more runs
    #[command(name = "review-build")]
    ReviewBuild {
        /// Run ids to include in the review bundle
        #[arg(long = "run-id")]
        run_ids: Vec<String>,

        /// Optional JSON export path for the generated review bundle
        #[arg(long)]
        output: Option<PathBuf>,

        /// Directory containing runtime state such as manifests, evaluations, and reviews
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// List local DAW-agnostic decks
    #[command(name = "deck-list")]
    DeckList {
        /// Directory containing runtime state such as decks
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Create a deck bound to one session
    #[command(name = "deck-create")]
    DeckCreate {
        /// Deck display name
        #[arg(long)]
        display_name: String,

        /// Session identifier backing this deck
        #[arg(long)]
        session_id: String,

        /// Actor creating the deck
        #[arg(long)]
        actor_id: String,

        /// Directory containing runtime state such as decks and sessions
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one deck
    #[command(name = "deck-inspect")]
    DeckInspect {
        /// Deck identifier to inspect
        #[arg(long)]
        deck_id: String,

        /// Directory containing runtime state such as decks
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Add one session preview as a deck clip
    #[command(name = "deck-add-preview")]
    DeckAddPreview {
        /// Deck identifier to update
        #[arg(long)]
        deck_id: String,

        /// Session identifier that owns the preview
        #[arg(long)]
        session_id: String,

        /// Preview identifier to import
        #[arg(long)]
        preview_id: String,

        /// Human-readable clip label
        #[arg(long)]
        label: String,

        /// Actor adding the clip
        #[arg(long)]
        actor_id: String,

        /// Directory containing runtime state such as decks and sessions
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Queue one deck clip for launch
    #[command(name = "deck-queue")]
    DeckQueue {
        /// Deck identifier to update
        #[arg(long)]
        deck_id: String,

        /// Clip identifier to queue
        #[arg(long)]
        clip_id: String,

        /// Actor queueing the clip
        #[arg(long)]
        actor_id: String,

        /// Directory containing runtime state such as decks
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Launch one deck clip and start transport
    #[command(name = "deck-launch")]
    DeckLaunch {
        /// Deck identifier to update
        #[arg(long)]
        deck_id: String,

        /// Clip identifier to launch
        #[arg(long)]
        clip_id: String,

        /// Actor launching the clip
        #[arg(long)]
        actor_id: String,

        /// Directory containing runtime state such as decks
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Stop one deck and clear the active clip
    #[command(name = "deck-stop")]
    DeckStop {
        /// Deck identifier to stop
        #[arg(long)]
        deck_id: String,

        /// Actor stopping the deck
        #[arg(long)]
        actor_id: String,

        /// Directory containing runtime state such as decks
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect current deck transport and loaded clips
    #[command(name = "deck-transport")]
    DeckTransport {
        /// Deck identifier to inspect
        #[arg(long)]
        deck_id: String,

        /// Directory containing runtime state such as decks
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Create a deterministic harness plan over the real backend tools
    #[command(name = "harness-plan")]
    HarnessPlan {
        /// Harness role
        #[arg(long, value_enum)]
        role: HarnessRole,

        /// Operator prompt or intent
        #[arg(long)]
        prompt: String,

        /// Optional session context
        #[arg(long)]
        session_id: Option<String>,

        /// Optional deck context
        #[arg(long)]
        deck_id: Option<String>,

        /// Optional realtime adapter context
        #[arg(long)]
        adapter_id: Option<String>,

        /// Optional run ids for evaluator plans
        #[arg(long = "run-id")]
        run_ids: Vec<String>,

        /// Maximum number of actions the plan may propose (policy override)
        #[arg(long)]
        max_actions: Option<usize>,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one harness plan
    #[command(name = "harness-plan-inspect")]
    HarnessPlanInspect {
        /// Plan identifier to inspect
        #[arg(long)]
        plan_id: String,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Execute one harness action from a stored plan
    #[command(name = "harness-execute")]
    HarnessExecute {
        /// Plan identifier to execute from
        #[arg(long)]
        plan_id: String,

        /// Action identifier to execute
        #[arg(long)]
        action_id: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// List recorded harness outcomes
    #[command(name = "harness-outcome-list")]
    HarnessOutcomeList {
        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Validate an immutable unattended job configuration
    #[command(name = "job-validate")]
    JobValidate {
        /// Scheduler backend target
        #[arg(long, value_enum)]
        backend: SchedulerBackend,

        /// Harness role used by the job
        #[arg(long, value_enum)]
        role: HarnessRole,

        /// Operator prompt or unattended instruction
        #[arg(long)]
        prompt: String,

        /// Optional session context
        #[arg(long)]
        session_id: Option<String>,

        /// Optional deck context
        #[arg(long)]
        deck_id: Option<String>,

        /// Optional run ids for review-oriented jobs
        #[arg(long = "run-id")]
        run_ids: Vec<String>,

        /// Maximum number of attempts for the immutable job config
        #[arg(long, default_value_t = 1)]
        retry_limit: u8,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Schedule an immutable unattended job after consuming an approval token
    #[command(name = "job-schedule")]
    JobSchedule {
        /// Human-readable job name
        #[arg(long)]
        job_name: String,

        /// Scheduler backend target
        #[arg(long, value_enum)]
        backend: SchedulerBackend,

        /// Harness role used by the job
        #[arg(long, value_enum)]
        role: HarnessRole,

        /// Operator prompt or unattended instruction
        #[arg(long)]
        prompt: String,

        /// Optional session context
        #[arg(long)]
        session_id: Option<String>,

        /// Optional deck context
        #[arg(long)]
        deck_id: Option<String>,

        /// Optional run ids for review-oriented jobs
        #[arg(long = "run-id")]
        run_ids: Vec<String>,

        /// Actor creating the job
        #[arg(long)]
        requested_by: String,

        /// Maximum number of attempts for the immutable job config
        #[arg(long, default_value_t = 1)]
        retry_limit: u8,

        /// Approval token authorizing `jobs.schedule` for this job name
        #[arg(long)]
        approval_token: String,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// List stored unattended jobs
    #[command(name = "job-list")]
    JobList {
        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one stored unattended job
    #[command(name = "job-inspect")]
    JobInspect {
        /// Job identifier to inspect
        #[arg(long)]
        job_id: String,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Run one stored unattended job through the local harness
    #[command(name = "job-run")]
    JobRun {
        /// Job identifier to execute
        #[arg(long)]
        job_id: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Cancel one stored unattended job after consuming an approval token
    #[command(name = "job-cancel")]
    JobCancel {
        /// Job identifier to cancel
        #[arg(long)]
        job_id: String,

        /// Actor cancelling the job
        #[arg(long)]
        requested_by: String,

        /// Approval token authorizing `jobs.cancel` for this job id
        #[arg(long)]
        approval_token: String,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Create a realtime adapter endpoint for local OSC dispatch
    #[command(name = "realtime-create")]
    RealtimeCreate {
        /// Human-readable adapter name
        #[arg(long)]
        display_name: String,

        /// Realtime protocol
        #[arg(long, value_enum, default_value_t = RealtimeAdapterProtocol::OscUdp)]
        protocol: RealtimeAdapterProtocol,

        /// Target host
        #[arg(long)]
        host: std::net::IpAddr,

        /// Target UDP port
        #[arg(long)]
        port: u16,

        /// OSC base path
        #[arg(long, default_value = "/state_space_music_box")]
        base_path: String,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// List configured realtime adapters
    #[command(name = "realtime-list")]
    RealtimeList {
        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one configured realtime adapter
    #[command(name = "realtime-inspect")]
    RealtimeInspect {
        /// Realtime adapter identifier
        #[arg(long)]
        adapter_id: String,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Send one stored session preview to a realtime adapter
    #[command(name = "realtime-send-preview")]
    RealtimeSendPreview {
        /// Realtime adapter identifier
        #[arg(long)]
        adapter_id: String,

        /// Session identifier
        #[arg(long)]
        session_id: String,

        /// Preview identifier
        #[arg(long)]
        preview_id: String,

        /// Actor performing the dispatch
        #[arg(long)]
        actor_id: String,

        /// Dispatch mode
        #[arg(long, value_enum, default_value_t = RealtimeDispatchMode::Timed)]
        dispatch_mode: RealtimeDispatchMode,

        /// Time scaling factor applied to MIDI event timing
        #[arg(long, default_value_t = 1.0)]
        time_scale: f64,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Send one deck transport snapshot to a realtime adapter
    #[command(name = "realtime-send-transport")]
    RealtimeSendTransport {
        /// Realtime adapter identifier
        #[arg(long)]
        adapter_id: String,

        /// Deck identifier
        #[arg(long)]
        deck_id: String,

        /// Actor performing the dispatch
        #[arg(long)]
        actor_id: String,

        /// Dispatch mode
        #[arg(long, value_enum, default_value_t = RealtimeDispatchMode::Immediate)]
        dispatch_mode: RealtimeDispatchMode,

        /// Time scaling factor applied to timed dispatch
        #[arg(long, default_value_t = 1.0)]
        time_scale: f64,

        /// Directory containing runtime state
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// List registered datasets from the local runtime registry
    #[command(name = "dataset-list")]
    DatasetList {
        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Inspect one dataset record from the local runtime registry
    #[command(name = "dataset-inspect")]
    DatasetInspect {
        /// Dataset identifier to inspect
        #[arg(long)]
        dataset_id: String,

        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Register a dataset record after consuming an approval token
    #[command(name = "dataset-register")]
    DatasetRegister(Box<DatasetRegisterArgs>),

    /// Create an approval request for a sensitive action
    #[command(name = "approval-request")]
    ApprovalRequest {
        /// Action scope, for example dataset.register
        #[arg(long)]
        action_scope: String,

        /// Action target, for example a dataset id
        #[arg(long)]
        target: String,

        /// Actor requesting the approval
        #[arg(long)]
        requested_by: String,

        /// Reason for the request
        #[arg(long)]
        reason: String,

        /// Risk level for the requested action
        #[arg(long, value_enum)]
        risk: ActionRisk,

        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Resolve an approval request and optionally issue a token
    #[command(name = "approval-resolve")]
    ApprovalResolve {
        /// Approval identifier to resolve
        #[arg(long)]
        approval_id: String,

        /// Operator resolving the approval
        #[arg(long)]
        operator_id: String,

        /// Decision to apply
        #[arg(long, value_enum)]
        decision: ApprovalDecisionKind,

        /// Reason for the decision
        #[arg(long)]
        reason: String,

        /// Token lifetime for approved actions
        #[arg(long, default_value_t = 3600)]
        expires_in_seconds: u64,

        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Create a snapshot for a file-backed preset
    #[command(name = "snapshot-create")]
    SnapshotCreate {
        /// Preset name to snapshot
        #[arg(long)]
        preset: String,

        /// Reason for the snapshot
        #[arg(long)]
        reason: String,

        /// Optional actor id creating the snapshot
        #[arg(long)]
        actor_id: Option<String>,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Roll back a preset from a stored snapshot
    #[command(name = "snapshot-rollback")]
    SnapshotRollback {
        /// Snapshot identifier to restore
        #[arg(long)]
        snapshot_id: String,

        /// Directory containing user-defined presets
        #[arg(long, default_value_os_t = default_preset_dir())]
        preset_dir: PathBuf,

        /// Directory containing runtime state such as approvals and datasets
        #[arg(long, default_value_os_t = default_runtime_dir())]
        runtime_dir: PathBuf,
    },

    /// Validate the library installation and deterministic generation backend
    #[command(name = "validate")]
    Validate,
}

#[derive(Debug, Serialize)]
struct CliAuditedResponse<T>
where
    T: Serialize,
{
    #[serde(flatten)]
    result: T,
    audit: ActionAuditRef,
}

#[derive(Debug, Serialize)]
struct ReviewBuildResponse<T>
where
    T: Serialize,
{
    #[serde(flatten)]
    review: T,
    export: Option<ReviewBundleExportSummary>,
}

fn base_cli_action(
    action: &str,
    target: Option<String>,
    input: serde_json::Value,
) -> NewActionRecord {
    NewActionRecord {
        action: action.to_string(),
        actor_id: "local-cli".to_string(),
        transport: ActionTransport::Cli,
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

fn record_cli_success<T>(
    runtime_dir: &std::path::Path,
    result: &T,
    mut action: NewActionRecord,
) -> Result<ActionAuditRef>
where
    T: Serialize,
{
    action.status = ActionStatus::Succeeded;
    action.output = Some(serde_json::to_value(result)?);
    action.error_message = None;
    persist_action_record(runtime_dir, action)
}

fn record_cli_failure(
    runtime_dir: &std::path::Path,
    error: &anyhow::Error,
    status: ActionStatus,
    mut action: NewActionRecord,
) -> Result<ActionAuditRef> {
    action.status = status;
    action.output = None;
    action.error_message = Some(error.to_string());
    persist_action_record(runtime_dir, action)
}

fn emit_cli_response<T>(result: T, audit: ActionAuditRef) -> Result<()>
where
    T: Serialize,
{
    println!(
        "{}",
        serde_json::to_string_pretty(&CliAuditedResponse { result, audit })?
    );
    Ok(())
}

fn merge_recording_error(
    action_error: anyhow::Error,
    record_error: anyhow::Error,
) -> anyhow::Error {
    anyhow!("{action_error}; additionally failed to persist manifest/audit records: {record_error}")
}

impl Cli {
    /// Execute the CLI command.
    pub fn execute(self) -> Result<()> {
        let _ = tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .try_init();

        match self.command {
            Some(Commands::Mcp {
                preset_dir,
                runtime_dir,
            }) => {
                info!("Starting MCP server");
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(crate::mcp::start_mcp_server(preset_dir, runtime_dir))?;
            }
            Some(Commands::Http {
                port,
                api_key,
                preset_dir,
                runtime_dir,
            }) => {
                info!("Starting HTTP server on port {port}");
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(crate::http_server::start_http_server(
                    preset_dir,
                    runtime_dir,
                    api_key,
                    port,
                ))?;
            }
            Some(Commands::GenerateDemo {
                midi,
                wav,
                seed,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({
                    "preset": DEMO_PRESET_NAME,
                    "seed": seed,
                    "midi_path": midi.as_ref().map(|path| path.display().to_string()),
                    "wav_path": wav.as_ref().map(|path| path.display().to_string()),
                });
                let mut action =
                    base_cli_action("generate_demo", Some(DEMO_PRESET_NAME.to_string()), input);
                action.preset_name = Some(DEMO_PRESET_NAME.to_string());
                action.seed = Some(seed);

                let result = (|| -> Result<_> {
                    ensure!(
                        midi.is_some() || wav.is_some(),
                        "at least one of --midi or --wav must be provided"
                    );

                    let preset = load_preset(DEMO_PRESET_NAME, &preset_dir)?;
                    let composition = generate_composition(preset, seed)?;
                    action.preset_hash = Some(snapshot_preset_hash(&composition.preset)?);

                    let midi_summary = match midi.as_deref() {
                        Some(path) => {
                            let summary = export_generated_midi(&composition, path)?;
                            action.artifacts.push(ManifestArtifactInput {
                                kind: "midi".to_string(),
                                path: summary.path.clone(),
                            });
                            Some(summary)
                        }
                        None => None,
                    };
                    let wav_summary = match wav.as_deref() {
                        Some(path) => {
                            let summary = export_generated_wav(&composition, path)?;
                            action.artifacts.push(ManifestArtifactInput {
                                kind: "wav".to_string(),
                                path: summary.path.clone(),
                            });
                            Some(summary)
                        }
                        None => None,
                    };

                    Ok(crate::generation::DemoArtifactSummary {
                        preset: composition.preset.name.clone(),
                        seed,
                        trajectory: composition.trajectory_summary.clone(),
                        midi: midi_summary,
                        wav: wav_summary,
                    })
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::GenerateMidi {
                preset,
                output,
                seed,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({
                    "preset": preset,
                    "seed": seed,
                    "output_path": output.display().to_string(),
                });
                let mut action = base_cli_action("generate_midi", Some(preset.clone()), input);
                action.preset_name = Some(preset.clone());
                action.seed = Some(seed);

                let result = (|| -> Result<_> {
                    let preset_config = load_preset(&preset, &preset_dir)?;
                    action.preset_hash = Some(snapshot_preset_hash(&preset_config)?);
                    let composition = generate_composition(preset_config, seed)?;
                    let summary = export_generated_midi(&composition, &output)?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "midi".to_string(),
                        path: summary.path.clone(),
                    });
                    Ok(summary)
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::GenerateAudio {
                preset,
                output,
                seed,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({
                    "preset": preset,
                    "seed": seed,
                    "output_path": output.display().to_string(),
                });
                let mut action = base_cli_action("generate_audio", Some(preset.clone()), input);
                action.preset_name = Some(preset.clone());
                action.seed = Some(seed);

                let result = (|| -> Result<_> {
                    let preset_config = load_preset(&preset, &preset_dir)?;
                    action.preset_hash = Some(snapshot_preset_hash(&preset_config)?);
                    let composition = generate_composition(preset_config, seed)?;
                    let summary = export_generated_wav(&composition, &output)?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "wav".to_string(),
                        path: summary.path.clone(),
                    });
                    Ok(summary)
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::InspectTrajectory { preset, preset_dir }) => {
                let preset = load_preset(&preset, &preset_dir)?;
                let system = preset.system.to_system()?;
                let trajectory = simulate_trajectory(&system, &preset.simulation)?;
                let summary = summarize_trajectory(&trajectory);
                println!("{}", serde_json::to_string_pretty(&summary)?);
            }
            Some(Commands::ListPresets { preset_dir }) => {
                let presets = list_presets(&preset_dir)?;
                println!("{}", serde_json::to_string_pretty(&presets)?);
            }
            Some(Commands::RunList { runtime_dir }) => {
                let manifests =
                    list_run_manifests(&crate::governance::default_manifest_dir(&runtime_dir))?;
                println!("{}", serde_json::to_string_pretty(&manifests)?);
            }
            Some(Commands::RunInspect {
                run_id,
                runtime_dir,
            }) => {
                let manifest = inspect_run_manifest(
                    &crate::governance::default_manifest_dir(&runtime_dir),
                    &run_id,
                )?;
                println!("{}", serde_json::to_string_pretty(&manifest)?);
            }
            Some(Commands::RunCompare {
                run_ids,
                runtime_dir,
            }) => {
                let comparison = compare_runs(
                    &crate::governance::default_manifest_dir(&runtime_dir),
                    &run_ids,
                )?;
                println!("{}", serde_json::to_string_pretty(&comparison)?);
            }
            Some(Commands::AuditList { runtime_dir, limit }) => {
                let mut events =
                    read_audit_events(&crate::governance::default_audit_log_path(&runtime_dir))?;
                if let Some(limit) = limit {
                    let start = events.len().saturating_sub(limit);
                    events = events.split_off(start);
                }
                println!("{}", serde_json::to_string_pretty(&events)?);
            }
            Some(Commands::SessionList { runtime_dir }) => {
                let sessions = list_sessions(&default_session_store_path(&runtime_dir))?;
                println!("{}", serde_json::to_string_pretty(&sessions)?);
            }
            Some(Commands::SessionCreate {
                display_name,
                preset,
                seed,
                actor_id,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({
                    "display_name": display_name,
                    "preset": preset,
                    "seed": seed,
                    "actor_id": actor_id,
                });
                let mut action = base_cli_action("session_create", None, input);
                action.actor_id = actor_id.clone();
                action.preset_name = Some(preset.clone());
                action.seed = Some(seed);

                let result = (|| -> Result<_> {
                    let session = create_session(
                        &default_session_store_path(&runtime_dir),
                        &preset_dir,
                        NewSessionRequest {
                            display_name,
                            preset_name: preset,
                            seed,
                            actor_id,
                        },
                    )?;
                    action.target = Some(session.session_id.clone());
                    action.preset_hash = Some(session.preset_hash.clone());
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "session-store".to_string(),
                        path: default_session_store_path(&runtime_dir),
                    });
                    Ok(session)
                })();

                match result {
                    Ok(session) => {
                        let audit = record_cli_success(&runtime_dir, &session, action)?;
                        emit_cli_response(session, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::SessionInspect {
                session_id,
                runtime_dir,
            }) => {
                let session =
                    inspect_session(&default_session_store_path(&runtime_dir), &session_id)?;
                println!("{}", serde_json::to_string_pretty(&session)?);
            }
            Some(Commands::SessionUpdate(args)) => {
                let SessionUpdateArgs {
                    session_id,
                    actor_id,
                    display_name,
                    preset_name,
                    seed,
                    tempo_bpm,
                    status,
                    preset_dir,
                    runtime_dir,
                } = *args;
                let input = json!({
                    "session_id": session_id,
                    "display_name": display_name,
                    "preset_name": preset_name,
                    "seed": seed,
                    "tempo_bpm": tempo_bpm,
                    "status": status,
                });
                let mut action = base_cli_action("session_update", Some(session_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let session = update_session(
                        &default_session_store_path(&runtime_dir),
                        &preset_dir,
                        &session_id,
                        UpdateSessionRequest {
                            actor_id,
                            display_name,
                            preset_name,
                            seed,
                            tempo_bpm,
                            status,
                        },
                    )?;
                    action.preset_name = Some(session.preset_name.clone());
                    action.preset_hash = Some(session.preset_hash.clone());
                    action.seed = Some(session.seed);
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "session-store".to_string(),
                        path: default_session_store_path(&runtime_dir),
                    });
                    Ok(session)
                })();

                match result {
                    Ok(session) => {
                        let audit = record_cli_success(&runtime_dir, &session, action)?;
                        emit_cli_response(session, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::SessionPlay {
                session_id,
                actor_id,
                run_label,
                runtime_dir,
            }) => {
                let input = json!({
                    "session_id": session_id,
                    "command": "play",
                    "run_label": run_label,
                });
                let mut action = base_cli_action("session_play", Some(session_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let session = apply_transport_command(
                        &default_session_store_path(&runtime_dir),
                        &session_id,
                        SessionTransportRequest {
                            actor_id,
                            command: SessionTransportCommand::Play,
                            run_label,
                        },
                    )?;
                    action.preset_name = Some(session.preset_name.clone());
                    action.preset_hash = Some(session.preset_hash.clone());
                    action.seed = Some(session.seed);
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "session-store".to_string(),
                        path: default_session_store_path(&runtime_dir),
                    });
                    Ok(session)
                })();

                match result {
                    Ok(session) => {
                        let audit = record_cli_success(&runtime_dir, &session, action)?;
                        emit_cli_response(session, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::SessionStop {
                session_id,
                actor_id,
                runtime_dir,
            }) => {
                let input = json!({
                    "session_id": session_id,
                    "command": "stop",
                });
                let mut action = base_cli_action("session_stop", Some(session_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let session = apply_transport_command(
                        &default_session_store_path(&runtime_dir),
                        &session_id,
                        SessionTransportRequest {
                            actor_id,
                            command: SessionTransportCommand::Stop,
                            run_label: None,
                        },
                    )?;
                    action.preset_name = Some(session.preset_name.clone());
                    action.preset_hash = Some(session.preset_hash.clone());
                    action.seed = Some(session.seed);
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "session-store".to_string(),
                        path: default_session_store_path(&runtime_dir),
                    });
                    Ok(session)
                })();

                match result {
                    Ok(session) => {
                        let audit = record_cli_success(&runtime_dir, &session, action)?;
                        emit_cli_response(session, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::SessionRenderPreview {
                session_id,
                actor_id,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({
                    "session_id": session_id,
                    "actor_id": actor_id,
                });
                let mut action =
                    base_cli_action("session_render_preview", Some(session_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let preview = render_session_preview(
                        &default_session_store_path(&runtime_dir),
                        &preset_dir,
                        &runtime_dir,
                        &session_id,
                        &actor_id,
                    )?;
                    action.preset_name = Some(preview.session.preset_name.clone());
                    action.preset_hash = Some(preview.session.preset_hash.clone());
                    action.seed = Some(preview.session.seed);
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "session-store".to_string(),
                        path: default_session_store_path(&runtime_dir),
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
                        let audit = record_cli_success(&runtime_dir, &preview, action)?;
                        emit_cli_response(preview, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::EvaluationSubmit(args)) => {
                let EvaluationSubmitArgs {
                    run_ids,
                    objective_metrics,
                    human_scores,
                    reward_weights,
                    notes,
                    decision,
                    created_by,
                    runtime_dir,
                } = *args;
                let input = json!({
                    "run_ids": run_ids,
                    "objective_metrics": objective_metrics,
                    "human_scores": human_scores,
                    "reward_weights": reward_weights,
                    "notes": notes,
                    "decision": decision,
                });
                let mut action = base_cli_action("evaluation_submit", None, input);
                action.actor_id = created_by.clone();

                let result = (|| -> Result<_> {
                    let evaluation = submit_evaluation_record(
                        &default_evaluation_store_path(&runtime_dir),
                        &crate::governance::default_manifest_dir(&runtime_dir),
                        NewEvaluationRecord {
                            run_ids,
                            objective_metrics: parse_f64_map_entries(&objective_metrics)?,
                            human_scores: parse_u8_map_entries(&human_scores)?,
                            reward_weights: parse_f64_map_entries(&reward_weights)?,
                            notes,
                            decision,
                            created_by,
                        },
                    )?;
                    action.target = Some(evaluation.evaluation_id.clone());
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "evaluation-store".to_string(),
                        path: default_evaluation_store_path(&runtime_dir),
                    });
                    Ok(evaluation)
                })();

                match result {
                    Ok(evaluation) => {
                        let audit = record_cli_success(&runtime_dir, &evaluation, action)?;
                        emit_cli_response(evaluation, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::EvaluationList { runtime_dir }) => {
                let evaluations =
                    list_evaluation_records(&default_evaluation_store_path(&runtime_dir))?;
                println!("{}", serde_json::to_string_pretty(&evaluations)?);
            }
            Some(Commands::EvaluationInspect {
                evaluation_id,
                runtime_dir,
            }) => {
                let evaluation = inspect_evaluation_record(
                    &default_evaluation_store_path(&runtime_dir),
                    &evaluation_id,
                )?;
                println!("{}", serde_json::to_string_pretty(&evaluation)?);
            }
            Some(Commands::ReviewBuild {
                run_ids,
                output,
                runtime_dir,
            }) => {
                let evaluation_store_path = default_evaluation_store_path(&runtime_dir);
                let manifest_dir = crate::governance::default_manifest_dir(&runtime_dir);
                let review = build_review_bundle(&evaluation_store_path, &manifest_dir, &run_ids)?;
                let export = match output {
                    Some(path) => {
                        let export_path = if path.is_absolute() {
                            path
                        } else {
                            default_review_dir(&runtime_dir).join(path)
                        };
                        Some(export_review_bundle(
                            &evaluation_store_path,
                            &manifest_dir,
                            &run_ids,
                            &export_path,
                        )?)
                    }
                    None => None,
                };
                println!(
                    "{}",
                    serde_json::to_string_pretty(&ReviewBuildResponse { review, export })?
                );
            }
            Some(Commands::DeckList { runtime_dir }) => {
                let decks = list_decks(&default_daw_store_path(&runtime_dir))?;
                println!("{}", serde_json::to_string_pretty(&decks)?);
            }
            Some(Commands::DeckCreate {
                display_name,
                session_id,
                actor_id,
                runtime_dir,
            }) => {
                let input = json!({
                    "display_name": display_name,
                    "session_id": session_id,
                });
                let mut action = base_cli_action("deck_create", Some(session_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let deck = create_deck(
                        &default_daw_store_path(&runtime_dir),
                        &default_session_store_path(&runtime_dir),
                        NewDeckRequest {
                            display_name,
                            session_id: session_id.clone(),
                            actor_id,
                        },
                    )?;
                    action.target = Some(deck.deck_id.clone());
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "deck-store".to_string(),
                        path: default_daw_store_path(&runtime_dir),
                    });
                    Ok(deck)
                })();

                match result {
                    Ok(deck) => {
                        let audit = record_cli_success(&runtime_dir, &deck, action)?;
                        emit_cli_response(deck, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::DeckInspect {
                deck_id,
                runtime_dir,
            }) => {
                let deck = inspect_deck(&default_daw_store_path(&runtime_dir), &deck_id)?;
                println!("{}", serde_json::to_string_pretty(&deck)?);
            }
            Some(Commands::DeckAddPreview {
                deck_id,
                session_id,
                preview_id,
                label,
                actor_id,
                runtime_dir,
            }) => {
                let input = json!({
                    "deck_id": deck_id,
                    "session_id": session_id,
                    "preview_id": preview_id,
                    "label": label,
                });
                let mut action = base_cli_action("deck_add_preview", Some(deck_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let deck = add_preview_clip_to_deck(
                        &default_daw_store_path(&runtime_dir),
                        &default_session_store_path(&runtime_dir),
                        &deck_id,
                        AddDeckPreviewRequest {
                            actor_id,
                            label,
                            session_id,
                            preview_id,
                        },
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "deck-store".to_string(),
                        path: default_daw_store_path(&runtime_dir),
                    });
                    Ok(deck)
                })();

                match result {
                    Ok(deck) => {
                        let audit = record_cli_success(&runtime_dir, &deck, action)?;
                        emit_cli_response(deck, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::DeckQueue {
                deck_id,
                clip_id,
                actor_id,
                runtime_dir,
            }) => {
                let input = json!({
                    "deck_id": deck_id,
                    "clip_id": clip_id,
                });
                let mut action = base_cli_action("deck_queue", Some(deck_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let deck = queue_deck_clip(
                        &default_daw_store_path(&runtime_dir),
                        &deck_id,
                        QueueDeckClipRequest { actor_id, clip_id },
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "deck-store".to_string(),
                        path: default_daw_store_path(&runtime_dir),
                    });
                    Ok(deck)
                })();

                match result {
                    Ok(deck) => {
                        let audit = record_cli_success(&runtime_dir, &deck, action)?;
                        emit_cli_response(deck, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::DeckLaunch {
                deck_id,
                clip_id,
                actor_id,
                runtime_dir,
            }) => {
                let input = json!({
                    "deck_id": deck_id,
                    "clip_id": clip_id,
                });
                let mut action = base_cli_action("deck_launch", Some(deck_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let snapshot = launch_deck_clip(
                        &default_daw_store_path(&runtime_dir),
                        &deck_id,
                        LaunchDeckClipRequest { actor_id, clip_id },
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "deck-store".to_string(),
                        path: default_daw_store_path(&runtime_dir),
                    });
                    Ok(snapshot)
                })();

                match result {
                    Ok(snapshot) => {
                        let audit = record_cli_success(&runtime_dir, &snapshot, action)?;
                        emit_cli_response(snapshot, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::DeckStop {
                deck_id,
                actor_id,
                runtime_dir,
            }) => {
                let input = json!({
                    "deck_id": deck_id,
                });
                let mut action = base_cli_action("deck_stop", Some(deck_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let snapshot = stop_deck(
                        &default_daw_store_path(&runtime_dir),
                        &deck_id,
                        StopDeckRequest { actor_id },
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "deck-store".to_string(),
                        path: default_daw_store_path(&runtime_dir),
                    });
                    Ok(snapshot)
                })();

                match result {
                    Ok(snapshot) => {
                        let audit = record_cli_success(&runtime_dir, &snapshot, action)?;
                        emit_cli_response(snapshot, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::DeckTransport {
                deck_id,
                runtime_dir,
            }) => {
                let snapshot =
                    inspect_deck_transport(&default_daw_store_path(&runtime_dir), &deck_id)?;
                println!("{}", serde_json::to_string_pretty(&snapshot)?);
            }
            Some(Commands::HarnessPlan {
                role,
                prompt,
                session_id,
                deck_id,
                adapter_id,
                run_ids,
                max_actions,
                runtime_dir,
            }) => {
                let plan = create_harness_plan(
                    &default_harness_store_path(&runtime_dir),
                    &runtime_dir,
                    NewHarnessPlanRequest {
                        role,
                        prompt,
                        session_id,
                        deck_id,
                        adapter_id,
                        run_ids,
                        max_actions,
                    },
                )?;
                println!("{}", serde_json::to_string_pretty(&plan)?);
            }
            Some(Commands::HarnessPlanInspect {
                plan_id,
                runtime_dir,
            }) => {
                let plan =
                    inspect_harness_plan(&default_harness_store_path(&runtime_dir), &plan_id)?;
                println!("{}", serde_json::to_string_pretty(&plan)?);
            }
            Some(Commands::HarnessExecute {
                plan_id,
                action_id,
                preset_dir,
                runtime_dir,
            }) => {
                let outcome = execute_harness_action(
                    &default_harness_store_path(&runtime_dir),
                    &runtime_dir,
                    &preset_dir,
                    ExecuteHarnessActionRequest { plan_id, action_id },
                )?;
                println!("{}", serde_json::to_string_pretty(&outcome)?);
            }
            Some(Commands::HarnessOutcomeList { runtime_dir }) => {
                let outcomes = list_harness_outcomes(&default_harness_store_path(&runtime_dir))?;
                println!("{}", serde_json::to_string_pretty(&outcomes)?);
            }
            Some(Commands::JobValidate {
                backend,
                role,
                prompt,
                session_id,
                deck_id,
                run_ids,
                retry_limit,
                runtime_dir,
            }) => {
                let validation = validate_scheduled_job(
                    &runtime_dir,
                    ValidateScheduledJobRequest {
                        backend,
                        role,
                        prompt,
                        session_id,
                        deck_id,
                        adapter_id: None,
                        run_ids,
                        retry_limit,
                        max_dispatches: None,
                    },
                )?;
                println!("{}", serde_json::to_string_pretty(&validation)?);
            }
            Some(Commands::JobSchedule {
                job_name,
                backend,
                role,
                prompt,
                session_id,
                deck_id,
                run_ids,
                requested_by,
                retry_limit,
                approval_token,
                runtime_dir,
            }) => {
                let input = json!({
                    "job_name": job_name,
                    "backend": backend,
                    "role": role,
                    "prompt": prompt,
                    "session_id": session_id,
                    "deck_id": deck_id,
                    "run_ids": run_ids,
                    "requested_by": requested_by,
                    "retry_limit": retry_limit,
                });
                let mut action = base_cli_action("job_schedule", Some(job_name.clone()), input);
                action.actor_id = requested_by.clone();

                let result = (|| -> Result<_> {
                    let job = schedule_job(
                        &runtime_dir,
                        NewScheduledJobRequest {
                            name: job_name,
                            backend,
                            role,
                            prompt,
                            session_id,
                            deck_id,
                            adapter_id: None,
                            run_ids,
                            requested_by,
                            retry_limit,
                            approval_token,
                            max_dispatches: None,
                        },
                    )?;
                    action.target = Some(job.job_id.clone());
                    action.approval_ids = vec![job.approval_id.clone()];
                    action.metadata = Some(json!({
                        "scheduler_backend": job.config.backend,
                        "config_hash": job.config_hash,
                    }));
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "scheduler-store".to_string(),
                        path: default_scheduler_store_path(&runtime_dir),
                    });
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "scheduler-export".to_string(),
                        path: job.export_path.clone(),
                    });
                    Ok(job)
                })();

                match result {
                    Ok(job) => {
                        let audit = record_cli_success(&runtime_dir, &job, action)?;
                        emit_cli_response(job, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Blocked, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::JobList { runtime_dir }) => {
                let jobs = list_scheduled_jobs(&runtime_dir)?;
                println!("{}", serde_json::to_string_pretty(&jobs)?);
            }
            Some(Commands::JobInspect {
                job_id,
                runtime_dir,
            }) => {
                let job = inspect_scheduled_job(&runtime_dir, &job_id)?;
                println!("{}", serde_json::to_string_pretty(&job)?);
            }
            Some(Commands::JobRun {
                job_id,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({ "job_id": job_id });
                let mut action = base_cli_action("job_run", Some(job_id.clone()), input);

                let result = (|| -> Result<_> {
                    let summary = run_scheduled_job(&runtime_dir, &preset_dir, &job_id)?;
                    action.metadata = Some(json!({
                        "plan_id": summary.plan_id,
                        "outcome_ids": summary.outcome_ids,
                        "scheduler_backend": summary.job.config.backend,
                    }));
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "scheduler-store".to_string(),
                        path: default_scheduler_store_path(&runtime_dir),
                    });
                    Ok(summary)
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::JobCancel {
                job_id,
                requested_by,
                approval_token,
                runtime_dir,
            }) => {
                let input = json!({
                    "job_id": job_id,
                    "requested_by": requested_by,
                });
                let mut action = base_cli_action("job_cancel", Some(job_id.clone()), input);
                action.actor_id = requested_by.clone();

                let result = (|| -> Result<_> {
                    let job = crate::governance::cancel_scheduled_job(
                        &runtime_dir,
                        CancelScheduledJobRequest {
                            job_id,
                            requested_by,
                            approval_token,
                        },
                    )?;
                    action.approval_ids = vec![job.approval_id.clone()];
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "scheduler-store".to_string(),
                        path: default_scheduler_store_path(&runtime_dir),
                    });
                    Ok(job)
                })();

                match result {
                    Ok(job) => {
                        let audit = record_cli_success(&runtime_dir, &job, action)?;
                        emit_cli_response(job, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Blocked, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::RealtimeCreate {
                display_name,
                protocol,
                host,
                port,
                base_path,
                runtime_dir,
            }) => {
                let input = json!({
                    "display_name": display_name,
                    "protocol": protocol,
                    "host": host,
                    "port": port,
                    "base_path": base_path,
                });
                let mut action =
                    base_cli_action("realtime_create", Some(display_name.clone()), input);

                let result = (|| -> Result<_> {
                    let adapter = crate::governance::create_realtime_adapter(
                        &default_realtime_store_path(&runtime_dir),
                        NewRealtimeAdapterRequest {
                            display_name,
                            protocol,
                            host,
                            port,
                            base_path,
                        },
                    )?;
                    action.target = Some(adapter.adapter_id.clone());
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "realtime-store".to_string(),
                        path: default_realtime_store_path(&runtime_dir),
                    });
                    Ok(adapter)
                })();

                match result {
                    Ok(adapter) => {
                        let audit = record_cli_success(&runtime_dir, &adapter, action)?;
                        emit_cli_response(adapter, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::RealtimeList { runtime_dir }) => {
                let adapters = list_realtime_adapters(&default_realtime_store_path(&runtime_dir))?;
                println!("{}", serde_json::to_string_pretty(&adapters)?);
            }
            Some(Commands::RealtimeInspect {
                adapter_id,
                runtime_dir,
            }) => {
                let adapter = inspect_realtime_adapter(
                    &default_realtime_store_path(&runtime_dir),
                    &adapter_id,
                )?;
                println!("{}", serde_json::to_string_pretty(&adapter)?);
            }
            Some(Commands::RealtimeSendPreview {
                adapter_id,
                session_id,
                preview_id,
                actor_id,
                dispatch_mode,
                time_scale,
                runtime_dir,
            }) => {
                let input = json!({
                    "adapter_id": adapter_id,
                    "session_id": session_id,
                    "preview_id": preview_id,
                    "dispatch_mode": dispatch_mode,
                    "time_scale": time_scale,
                });
                let mut action =
                    base_cli_action("realtime_send_preview", Some(adapter_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let summary = send_preview_to_realtime_adapter(
                        &default_realtime_store_path(&runtime_dir),
                        &default_session_store_path(&runtime_dir),
                        &adapter_id,
                        SendRealtimePreviewRequest {
                            actor_id,
                            session_id,
                            preview_id,
                            dispatch_mode,
                            time_scale,
                        },
                    )?;
                    action.metadata = Some(json!({
                        "dispatch_id": summary.dispatch.dispatch_id,
                        "message_count": summary.dispatch.message_count,
                        "dispatch_mode": summary.dispatch.dispatch_mode,
                    }));
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "realtime-store".to_string(),
                        path: default_realtime_store_path(&runtime_dir),
                    });
                    Ok(summary)
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::RealtimeSendTransport {
                adapter_id,
                deck_id,
                actor_id,
                dispatch_mode,
                time_scale,
                runtime_dir,
            }) => {
                let input = json!({
                    "adapter_id": adapter_id,
                    "deck_id": deck_id,
                    "dispatch_mode": dispatch_mode,
                    "time_scale": time_scale,
                });
                let mut action =
                    base_cli_action("realtime_send_transport", Some(adapter_id.clone()), input);
                action.actor_id = actor_id.clone();

                let result = (|| -> Result<_> {
                    let summary = send_transport_to_realtime_adapter(
                        &default_realtime_store_path(&runtime_dir),
                        &default_daw_store_path(&runtime_dir),
                        &adapter_id,
                        SendRealtimeTransportRequest {
                            actor_id,
                            deck_id,
                            dispatch_mode,
                            time_scale,
                        },
                    )?;
                    action.metadata = Some(json!({
                        "dispatch_id": summary.dispatch.dispatch_id,
                        "message_count": summary.dispatch.message_count,
                        "dispatch_mode": summary.dispatch.dispatch_mode,
                    }));
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "realtime-store".to_string(),
                        path: default_realtime_store_path(&runtime_dir),
                    });
                    Ok(summary)
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::DatasetList { runtime_dir }) => {
                let registry_path = default_dataset_registry_path(&runtime_dir);
                let datasets = list_dataset_records(&registry_path)?;
                println!("{}", serde_json::to_string_pretty(&datasets)?);
            }
            Some(Commands::DatasetInspect {
                dataset_id,
                runtime_dir,
            }) => {
                let registry_path = default_dataset_registry_path(&runtime_dir);
                let dataset = inspect_dataset_record(&registry_path, &dataset_id)?;
                println!("{}", serde_json::to_string_pretty(&dataset)?);
            }
            Some(Commands::DatasetRegister(args)) => {
                let DatasetRegisterArgs {
                    dataset_id,
                    display_name,
                    source_url,
                    citation,
                    license_name,
                    commercial_use_status,
                    redistribution_status,
                    approved_use_class,
                    checksum_manifest,
                    local_storage_path,
                    dataset_version,
                    split_policy,
                    transform_pipeline_hash,
                    parent_datasets,
                    approval_token,
                    notes,
                    runtime_dir,
                } = *args;
                let approval_store_path = default_approval_store_path(&runtime_dir);
                let registry_path = default_dataset_registry_path(&runtime_dir);
                let input = json!({
                    "dataset_id": dataset_id,
                    "display_name": display_name,
                    "source_url": source_url,
                    "license_name": license_name,
                    "commercial_use_status": commercial_use_status,
                    "redistribution_status": redistribution_status,
                    "approved_use_class": approved_use_class,
                    "checksum_manifest": checksum_manifest,
                    "local_storage_path": local_storage_path.display().to_string(),
                    "dataset_version": dataset_version,
                    "split_policy": split_policy,
                    "transform_pipeline_hash": transform_pipeline_hash,
                    "parent_datasets": parent_datasets,
                    "notes": notes,
                });
                let mut action =
                    base_cli_action("dataset_register", Some(dataset_id.clone()), input);

                let result = (|| -> Result<_> {
                    let approval = consume_approval_token(
                        &approval_store_path,
                        &approval_token,
                        "dataset.register",
                        &dataset_id,
                    )?;
                    action.approval_ids = vec![approval.approval_id.clone()];
                    let dataset = register_dataset_record(
                        &registry_path,
                        NewDatasetRecord {
                            dataset_id,
                            display_name,
                            source_url,
                            citation,
                            license_name,
                            commercial_use_status,
                            redistribution_status,
                            approved_use_class,
                            checksum_manifest: parse_checksum_entries(&checksum_manifest)?,
                            local_storage_path,
                            dataset_version,
                            split_policy,
                            transform_pipeline_hash,
                            parent_datasets,
                            operator_approval_id: approval.approval_id,
                            notes,
                        },
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "dataset-registry".to_string(),
                        path: registry_path.clone(),
                    });
                    Ok(dataset)
                })();

                match result {
                    Ok(dataset) => {
                        let audit = record_cli_success(&runtime_dir, &dataset, action)?;
                        emit_cli_response(dataset, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Blocked, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::ApprovalRequest {
                action_scope,
                target,
                requested_by,
                reason,
                risk,
                runtime_dir,
            }) => {
                let store_path = default_approval_store_path(&runtime_dir);
                let input = json!({
                    "action_scope": action_scope,
                    "target": target,
                    "requested_by": requested_by,
                    "reason": reason,
                    "risk": risk,
                });
                let mut action = base_cli_action("approval_request", Some(target.clone()), input);
                action.actor_id = requested_by.clone();

                let result = (|| -> Result<_> {
                    let approval = request_approval(
                        &store_path,
                        NewApprovalRequest {
                            action_scope,
                            target,
                            requested_by,
                            reason,
                            risk,
                        },
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "approval-store".to_string(),
                        path: store_path.clone(),
                    });
                    Ok(approval)
                })();

                match result {
                    Ok(approval) => {
                        let audit = record_cli_success(&runtime_dir, &approval, action)?;
                        emit_cli_response(approval, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::ApprovalResolve {
                approval_id,
                operator_id,
                decision,
                reason,
                expires_in_seconds,
                runtime_dir,
            }) => {
                let store_path = default_approval_store_path(&runtime_dir);
                let input = json!({
                    "approval_id": approval_id,
                    "decision": decision,
                    "reason": reason,
                    "expires_in_seconds": expires_in_seconds,
                });
                let mut action =
                    base_cli_action("approval_resolve", Some(approval_id.clone()), input);
                action.actor_id = operator_id.clone();

                let result = (|| -> Result<_> {
                    let resolution = resolve_approval(
                        &store_path,
                        &approval_id,
                        decision,
                        &operator_id,
                        &reason,
                        expires_in_seconds,
                    )?;
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "approval-store".to_string(),
                        path: store_path.clone(),
                    });
                    action.approval_ids = vec![approval_id.clone()];
                    Ok(resolution)
                })();

                match result {
                    Ok(resolution) => {
                        let audit = record_cli_success(&runtime_dir, &resolution, action)?;
                        emit_cli_response(resolution, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::SnapshotCreate {
                preset,
                reason,
                actor_id,
                preset_dir,
                runtime_dir,
            }) => {
                let snapshot_dir = default_snapshot_dir(&runtime_dir);
                let input = json!({
                    "preset": preset,
                    "reason": reason,
                    "actor_id": actor_id,
                });
                let mut action = base_cli_action("snapshot_create", Some(preset.clone()), input);
                action.actor_id = actor_id.clone().unwrap_or_else(|| "local-cli".to_string());
                action.preset_name = Some(preset.clone());

                let result = (|| -> Result<_> {
                    let summary = create_preset_snapshot(
                        &snapshot_dir,
                        &preset_dir,
                        &preset,
                        &reason,
                        actor_id.as_deref(),
                    )?;
                    action.preset_hash = Some(summary.preset_hash.clone());
                    action.artifacts.push(ManifestArtifactInput {
                        kind: "snapshot".to_string(),
                        path: summary.snapshot_path.clone(),
                    });
                    Ok(summary)
                })();

                match result {
                    Ok(summary) => {
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::SnapshotRollback {
                snapshot_id,
                preset_dir,
                runtime_dir,
            }) => {
                let input = json!({ "snapshot_id": snapshot_id });
                let mut action =
                    base_cli_action("snapshot_rollback", Some(snapshot_id.clone()), input);

                let result = (|| -> Result<_> {
                    let summary = rollback_preset_snapshot(
                        &default_snapshot_dir(&runtime_dir),
                        &preset_dir,
                        &snapshot_id,
                    )?;
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
                        let audit = record_cli_success(&runtime_dir, &summary, action)?;
                        emit_cli_response(summary, audit)?;
                    }
                    Err(error) => {
                        if let Err(record_error) =
                            record_cli_failure(&runtime_dir, &error, ActionStatus::Failed, action)
                        {
                            return Err(merge_recording_error(error, record_error));
                        }
                        return Err(error);
                    }
                }
            }
            Some(Commands::Validate) => {
                self.validate_installation()?;
            }
            None => {
                Cli::command().print_help()?;
                println!();
            }
        }

        Ok(())
    }

    fn validate_installation(&self) -> Result<()> {
        println!("Validating library installation...");

        let preset_dir = default_preset_dir();
        let presets = list_presets(&preset_dir)?;
        let preset = load_preset(DEMO_PRESET_NAME, &preset_dir)?;
        let composition = generate_composition(preset, 1)?;

        ensure!(!presets.is_empty(), "expected at least one preset");
        ensure!(
            !composition.midi_model.notes.is_empty(),
            "demo composition did not produce any notes"
        );
        ensure!(
            composition
                .audio_samples
                .iter()
                .any(|sample| sample.abs() > 0.0),
            "demo composition audio is silent"
        );

        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "preset_count": presets.len(),
                "note_count": composition.midi_model.notes.len(),
                "sample_count": composition.audio_samples.len(),
                "trajectory_frames": composition.trajectory.len(),
            }))?
        );
        println!("Library installation validated successfully.");
        Ok(())
    }
}

fn parse_checksum_entries(values: &[String]) -> Result<Vec<ChecksumEntry>> {
    ensure!(
        !values.is_empty(),
        "at least one --checksum entry is required"
    );

    values
        .iter()
        .map(|value| {
            let (relative_path, sha256) = value
                .split_once('=')
                .ok_or_else(|| anyhow!("checksum entry '{value}' must use relative_path=sha256"))?;
            ensure!(
                !relative_path.trim().is_empty(),
                "checksum relative_path cannot be empty"
            );
            ensure!(!sha256.trim().is_empty(), "checksum sha256 cannot be empty");
            Ok(ChecksumEntry {
                relative_path: relative_path.to_string(),
                sha256: sha256.to_string(),
            })
        })
        .collect()
}

fn parse_f64_map_entries(values: &[String]) -> Result<BTreeMap<String, f64>> {
    let mut map = BTreeMap::new();
    for value in values {
        let (key, raw) = value
            .split_once('=')
            .ok_or_else(|| anyhow!("map entry '{value}' must use key=value"))?;
        ensure!(!key.trim().is_empty(), "map entry key cannot be empty");
        let parsed: f64 = raw
            .parse()
            .map_err(|_| anyhow!("map entry '{value}' must have a numeric value"))?;
        ensure!(parsed.is_finite(), "map entry '{value}' must be finite");
        map.insert(key.to_string(), parsed);
    }
    Ok(map)
}

fn parse_u8_map_entries(values: &[String]) -> Result<BTreeMap<String, u8>> {
    let mut map = BTreeMap::new();
    for value in values {
        let (key, raw) = value
            .split_once('=')
            .ok_or_else(|| anyhow!("map entry '{value}' must use key=value"))?;
        ensure!(!key.trim().is_empty(), "map entry key cannot be empty");
        let parsed: u8 = raw
            .parse()
            .map_err(|_| anyhow!("map entry '{value}' must have an integer value"))?;
        map.insert(key.to_string(), parsed);
    }
    Ok(map)
}

impl TryFrom<Vec<String>> for Cli {
    type Error = anyhow::Error;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        Cli::try_parse_from(args).map_err(|err| anyhow!(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parses_generate_demo() {
        let cli = Cli::try_parse_from([
            "state-space-music-box",
            "generate-demo",
            "--midi",
            "out/demo.mid",
            "--seed",
            "2",
        ])
        .unwrap();

        match cli.command.unwrap() {
            Commands::GenerateDemo {
                midi, wav, seed, ..
            } => {
                assert_eq!(midi.unwrap(), PathBuf::from("out/demo.mid"));
                assert!(wav.is_none());
                assert_eq!(seed, 2);
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }

    #[test]
    fn test_cli_help_contains_real_commands() {
        let mut command = Cli::command();
        let help = command.render_long_help().to_string();

        assert!(help.contains("generate-demo"));
        assert!(help.contains("generate-midi"));
        assert!(help.contains("generate-audio"));
        assert!(help.contains("inspect-trajectory"));
        assert!(help.contains("list-presets"));
        assert!(help.contains("run-list"));
        assert!(help.contains("run-compare"));
        assert!(help.contains("run-inspect"));
        assert!(help.contains("audit-list"));
        assert!(help.contains("session-create"));
        assert!(help.contains("session-update"));
        assert!(help.contains("session-play"));
        assert!(help.contains("session-render-preview"));
        assert!(help.contains("evaluation-submit"));
        assert!(help.contains("evaluation-inspect"));
        assert!(help.contains("review-build"));
        assert!(help.contains("deck-create"));
        assert!(help.contains("deck-launch"));
        assert!(help.contains("deck-transport"));
        assert!(help.contains("harness-plan"));
        assert!(help.contains("harness-execute"));
        assert!(help.contains("job-validate"));
        assert!(help.contains("job-schedule"));
        assert!(help.contains("job-run"));
        assert!(help.contains("realtime-create"));
        assert!(help.contains("realtime-send-preview"));
        assert!(help.contains("realtime-send-transport"));
        assert!(help.contains("mcp"));
    }

    #[test]
    fn test_cli_parses_dataset_register() {
        let cli = Cli::try_parse_from([
            "state-space-music-box",
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
            "/datasets/pdmx",
            "--dataset-version",
            "v1",
            "--approval-token",
            "approval-token-1",
        ])
        .unwrap();

        match cli.command.unwrap() {
            Commands::DatasetRegister(args) => {
                assert_eq!(args.dataset_id, "pdmx");
            }
            other => panic!("unexpected command: {other:?}"),
        }
    }
}
