//! # HTTP Server
//!
//! An axum-based HTTP server that exposes all state-space-music-box tools
//! as a REST-like API. Agents (Hermes, OpenClaw, or any HTTP client) can
//! call any tool via:
//!
//! ```bash
//! curl -X POST http://localhost:3001/api/tools/list_presets \
//!   -H "Authorization: Bearer <api-key>" \
//!   -H "Content-Type: application/json" \
//!   -d '{}'
//! ```

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Json;
use axum::routing::{get, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::generation::{
    demo_preset, generate_composition, list_presets, GeneratedComposition, RenderPreset,
};
use crate::governance::*;

/// Shared state for the HTTP server, threaded into every handler via `Arc`.
#[derive(Clone)]
pub struct HttpServerState {
    /// Directory containing preset JSON files (e.g. `presets/lo-fi.json`).
    pub preset_dir: PathBuf,
    /// Runtime scratch directory for sessions, evaluations, manifests, and audit logs.
    pub runtime_dir: PathBuf,
    /// Bearer token required in the `Authorization` header of every request.
    pub api_key: String,
}

/// Consistent JSON response envelope returned by every endpoint.
///
/// On success `success` is `true` and `data` holds the result.
/// On failure `success` is `false` and `error` holds a human-readable message.
#[derive(Serialize)]
pub struct ApiResponse {
    /// `true` when the request succeeded; `false` otherwise.
    pub success: bool,
    /// Tool-specific result payload. Omitted from the JSON body on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    /// Human-readable error description. Omitted from the JSON body on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Tool metadata returned by `GET /api/tools`.
///
/// Agents can use this listing to discover available tools, their categories,
/// and risk levels before invoking them.
#[derive(Serialize)]
pub struct ToolInfo {
    /// Machine-readable tool name used as the `{tool_name}` path segment in
    /// `POST /api/tools/{tool_name}`.
    pub name: String,
    /// Logical grouping (e.g. `"generation"`, `"sessions"`, `"governance"`).
    pub category: String,
    /// Short human-readable description of what the tool does.
    pub description: String,
    /// Risk level: `"low"`, `"medium"`, or `"high"`.
    pub risk: String,
}

/// Start the axum HTTP server and block until it shuts down.
///
/// Binds to `0.0.0.0:{port}` with permissive CORS and HTTP tracing.
///
/// # Routes
///
/// | Method | Path                      | Description                 |
/// |--------|---------------------------|-----------------------------|
/// | GET    | `/api/health`             | Liveness / version check    |
/// | GET    | `/api/tools`              | List every available tool   |
/// | POST   | `/api/tools/{tool_name}`  | Execute a tool by name      |
///
/// Every `POST` request requires a `Bearer <api_key>` header.
pub async fn start_http_server(
    preset_dir: PathBuf,
    runtime_dir: PathBuf,
    api_key: String,
    port: u16,
) -> anyhow::Result<()> {
    let state = Arc::new(HttpServerState {
        preset_dir,
        runtime_dir,
        api_key,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/tools", get(list_tools))
        .route("/api/tools/{tool_name}", post(dispatch_tool))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("HTTP server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<ApiResponse> {
    Json(ApiResponse {
        success: true,
        data: Some(serde_json::json!({
            "status": "ok",
            "version": crate::VERSION,
        })),
        error: None,
    })
}

async fn list_tools() -> Json<Vec<ToolInfo>> {
    Json(all_tools())
}

async fn dispatch_tool(
    State(state): State<Arc<HttpServerState>>,
    Path(tool_name): Path<String>,
    headers: HeaderMap,
    Json(params): Json<Value>,
) -> (StatusCode, Json<ApiResponse>) {
    // Auth check
    let actor_id = match check_auth(&state.api_key, &headers) {
        Ok(actor) => actor,
        Err(msg) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    success: false,
                    data: None,
                    error: Some(msg),
                }),
            );
        }
    };

    match execute_tool(&state, &tool_name, &params, &actor_id) {
        Ok(data) => (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data: Some(data),
                error: None,
            }),
        ),
        Err(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(msg),
            }),
        ),
    }
}

fn check_auth(expected_key: &str, headers: &HeaderMap) -> Result<String, String> {
    let auth = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    if token.is_empty() {
        return Err("missing Authorization header".to_string());
    }
    if token != expected_key {
        return Err("invalid API key".to_string());
    }
    Ok("http-agent".to_string())
}

fn str_param(params: &Value, key: &str) -> Option<String> {
    params.get(key).and_then(Value::as_str).map(str::to_string)
}

fn str_required(params: &Value, key: &str) -> Result<String, String> {
    str_param(params, key).ok_or_else(|| format!("missing required field '{key}'"))
}

fn u64_param(params: &Value, key: &str) -> Option<u64> {
    params.get(key).and_then(Value::as_u64)
}

fn execute_tool(
    state: &HttpServerState,
    tool_name: &str,
    params: &Value,
    actor_id: &str,
) -> Result<Value, String> {
    match tool_name {
        // ── Generation ───────────────────────────────────────────────
        "list_presets" => {
            let presets = list_presets(&state.preset_dir).map_err(|e| e.to_string())?;
            serde_json::to_value(presets).map_err(|e| e.to_string())
        }
        "generate_composition" => {
            let preset_name = str_required(params, "preset")?;
            let seed = u64_param(params, "seed").unwrap_or(1);
            let preset_path = state.preset_dir.join(format!("{preset_name}.json"));
            let raw = std::fs::read_to_string(&preset_path)
                .map_err(|_| format!("preset '{preset_name}' not found"))?;
            let preset: RenderPreset = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
            let comp = generate_composition(preset, seed).map_err(|e| e.to_string())?;
            serde_json::to_value(CompositionSummary::from(&comp)).map_err(|e| e.to_string())
        }
        "generate_demo" => {
            let seed = u64_param(params, "seed").unwrap_or(1);
            let preset = demo_preset();
            let comp = generate_composition(preset, seed).map_err(|e| e.to_string())?;
            serde_json::to_value(CompositionSummary::from(&comp)).map_err(|e| e.to_string())
        }

        // ── Sessions ─────────────────────────────────────────────────
        "session_create" => {
            let session = create_session(
                &default_session_store_path(&state.runtime_dir),
                &state.preset_dir,
                NewSessionRequest {
                    display_name: str_required(params, "display_name")?,
                    preset_name: str_required(params, "preset")?,
                    seed: u64_param(params, "seed").unwrap_or(1),
                    actor_id: actor_id.to_string(),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(session).map_err(|e| e.to_string())
        }
        "session_list" => {
            let sessions = list_sessions(&default_session_store_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(sessions).map_err(|e| e.to_string())
        }
        "session_inspect" => {
            let id = str_required(params, "session_id")?;
            let session = inspect_session(&default_session_store_path(&state.runtime_dir), &id)
                .map_err(|e| e.to_string())?;
            serde_json::to_value(session).map_err(|e| e.to_string())
        }
        "session_render_preview" => {
            let id = str_required(params, "session_id")?;
            let preview = render_session_preview(
                &default_session_store_path(&state.runtime_dir),
                &state.preset_dir,
                &state.runtime_dir,
                &id,
                actor_id,
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(preview).map_err(|e| e.to_string())
        }
        "session_play" => {
            let id = str_required(params, "session_id")?;
            let run_label = str_param(params, "run_label").unwrap_or_default();
            let session = apply_transport_command(
                &default_session_store_path(&state.runtime_dir),
                &id,
                SessionTransportRequest {
                    actor_id: actor_id.to_string(),
                    command: SessionTransportCommand::Play,
                    run_label: if run_label.is_empty() {
                        None
                    } else {
                        Some(run_label)
                    },
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(session).map_err(|e| e.to_string())
        }
        "session_stop" => {
            let id = str_required(params, "session_id")?;
            let session = apply_transport_command(
                &default_session_store_path(&state.runtime_dir),
                &id,
                SessionTransportRequest {
                    actor_id: actor_id.to_string(),
                    command: SessionTransportCommand::Stop,
                    run_label: None,
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(session).map_err(|e| e.to_string())
        }

        // ── Evaluations ──────────────────────────────────────────────
        "evaluation_list" => {
            let evals = list_evaluation_records(&default_evaluation_store_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(evals).map_err(|e| e.to_string())
        }
        "evaluation_inspect" => {
            let id = str_required(params, "evaluation_id")?;
            let eval =
                inspect_evaluation_record(&default_evaluation_store_path(&state.runtime_dir), &id)
                    .map_err(|e| e.to_string())?;
            serde_json::to_value(eval).map_err(|e| e.to_string())
        }

        // ── Runs & Audit ─────────────────────────────────────────────
        "run_list" => {
            let runs = list_run_manifests(&default_manifest_dir(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(runs).map_err(|e| e.to_string())
        }
        "audit_list" => {
            let events = read_audit_events(&default_audit_log_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(events).map_err(|e| e.to_string())
        }

        // ── Decks ────────────────────────────────────────────────────
        "deck_create" => {
            let deck = create_deck(
                &default_daw_store_path(&state.runtime_dir),
                &default_session_store_path(&state.runtime_dir),
                NewDeckRequest {
                    display_name: str_required(params, "display_name")?,
                    session_id: str_required(params, "session_id")?,
                    actor_id: actor_id.to_string(),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(deck).map_err(|e| e.to_string())
        }
        "deck_list" => {
            let decks = list_decks(&default_daw_store_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(decks).map_err(|e| e.to_string())
        }
        "deck_transport" => {
            let id = str_required(params, "deck_id")?;
            let snap = inspect_deck_transport(&default_daw_store_path(&state.runtime_dir), &id)
                .map_err(|e| e.to_string())?;
            serde_json::to_value(snap).map_err(|e| e.to_string())
        }

        // ── Harness ──────────────────────────────────────────────────
        "harness_plan" => {
            let plan = create_harness_plan(
                &default_harness_store_path(&state.runtime_dir),
                &state.runtime_dir,
                NewHarnessPlanRequest {
                    role: serde_json::from_value(
                        params
                            .get("role")
                            .cloned()
                            .unwrap_or(serde_json::json!("session_dj")),
                    )
                    .map_err(|e| e.to_string())?,
                    prompt: str_required(params, "prompt")?,
                    session_id: str_param(params, "session_id"),
                    deck_id: str_param(params, "deck_id"),
                    adapter_id: str_param(params, "adapter_id"),
                    run_ids: params
                        .get("run_ids")
                        .and_then(Value::as_array)
                        .map(|a| {
                            a.iter()
                                .filter_map(Value::as_str)
                                .map(str::to_string)
                                .collect()
                        })
                        .unwrap_or_default(),
                    max_actions: params
                        .get("max_actions")
                        .and_then(Value::as_u64)
                        .map(|n| n as usize),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(plan).map_err(|e| e.to_string())
        }
        "harness_execute" => {
            let outcome = execute_harness_action(
                &default_harness_store_path(&state.runtime_dir),
                &state.runtime_dir,
                &state.preset_dir,
                ExecuteHarnessActionRequest {
                    plan_id: str_required(params, "plan_id")?,
                    action_id: str_required(params, "action_id")?,
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(outcome).map_err(|e| e.to_string())
        }
        "harness_outcome_list" => {
            let outcomes = list_harness_outcomes(&default_harness_store_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(outcomes).map_err(|e| e.to_string())
        }

        // ── Realtime ─────────────────────────────────────────────────
        "realtime_create" => {
            let adapter = create_realtime_adapter(
                &default_realtime_store_path(&state.runtime_dir),
                NewRealtimeAdapterRequest {
                    display_name: str_required(params, "display_name")?,
                    protocol: RealtimeAdapterProtocol::OscUdp,
                    host: str_required(params, "host")?
                        .parse()
                        .map_err(|e: std::net::AddrParseError| e.to_string())?,
                    port: params.get("port").and_then(Value::as_u64).unwrap_or(9000) as u16,
                    base_path: str_param(params, "base_path")
                        .unwrap_or_else(|| "/agentic_dj".to_string()),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(adapter).map_err(|e| e.to_string())
        }
        "realtime_list" => {
            let adapters = list_realtime_adapters(&default_realtime_store_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(adapters).map_err(|e| e.to_string())
        }
        "realtime_send_preview" => {
            let summary = send_preview_to_realtime_adapter(
                &default_realtime_store_path(&state.runtime_dir),
                &default_session_store_path(&state.runtime_dir),
                &str_required(params, "adapter_id")?,
                SendRealtimePreviewRequest {
                    actor_id: actor_id.to_string(),
                    session_id: str_required(params, "session_id")?,
                    preview_id: str_required(params, "preview_id")?,
                    dispatch_mode: if str_param(params, "dispatch_mode").unwrap_or_default()
                        == "timed"
                    {
                        RealtimeDispatchMode::Timed
                    } else {
                        RealtimeDispatchMode::Immediate
                    },
                    time_scale: params
                        .get("time_scale")
                        .and_then(Value::as_f64)
                        .unwrap_or(0.0),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(summary).map_err(|e| e.to_string())
        }

        // ── Governance ───────────────────────────────────────────────
        "dataset_list" => {
            let datasets = list_dataset_records(&default_dataset_registry_path(&state.runtime_dir))
                .map_err(|e| e.to_string())?;
            serde_json::to_value(datasets).map_err(|e| e.to_string())
        }
        "approval_request" => {
            let approval = request_approval(
                &default_approval_store_path(&state.runtime_dir),
                NewApprovalRequest {
                    action_scope: str_required(params, "action_scope")?,
                    target: str_required(params, "target")?,
                    requested_by: actor_id.to_string(),
                    reason: str_required(params, "reason")?,
                    risk: serde_json::from_value(
                        params
                            .get("risk")
                            .cloned()
                            .unwrap_or(serde_json::json!("medium")),
                    )
                    .map_err(|e| e.to_string())?,
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(approval).map_err(|e| e.to_string())
        }
        "approval_resolve" => {
            let resolution = resolve_approval(
                &default_approval_store_path(&state.runtime_dir),
                &str_required(params, "approval_id")?,
                serde_json::from_value(
                    params
                        .get("decision")
                        .cloned()
                        .unwrap_or(serde_json::json!("approve")),
                )
                .map_err(|e| e.to_string())?,
                actor_id,
                &str_param(params, "reason").unwrap_or_default(),
                u64_param(params, "token_ttl_seconds").unwrap_or(600),
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(resolution).map_err(|e| e.to_string())
        }
        "snapshot_create" => {
            let snap = create_preset_snapshot(
                &default_snapshot_dir(&state.runtime_dir),
                &state.preset_dir,
                &str_required(params, "preset")?,
                &str_param(params, "reason").unwrap_or_default(),
                Some(actor_id),
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(snap).map_err(|e| e.to_string())
        }

        // ── Scheduler ────────────────────────────────────────────────
        "job_validate" => {
            let validation = validate_scheduled_job(
                &state.runtime_dir,
                ValidateScheduledJobRequest {
                    backend: serde_json::from_value(
                        params
                            .get("backend")
                            .cloned()
                            .unwrap_or(serde_json::json!("local_cli")),
                    )
                    .map_err(|e| e.to_string())?,
                    role: serde_json::from_value(
                        params
                            .get("role")
                            .cloned()
                            .unwrap_or(serde_json::json!("session_dj")),
                    )
                    .map_err(|e| e.to_string())?,
                    prompt: str_required(params, "prompt")?,
                    session_id: str_param(params, "session_id"),
                    deck_id: str_param(params, "deck_id"),
                    adapter_id: str_param(params, "adapter_id"),
                    run_ids: params
                        .get("run_ids")
                        .and_then(Value::as_array)
                        .map(|a| {
                            a.iter()
                                .filter_map(Value::as_str)
                                .map(str::to_string)
                                .collect()
                        })
                        .unwrap_or_default(),
                    retry_limit: params
                        .get("retry_limit")
                        .and_then(Value::as_u64)
                        .unwrap_or(1) as u8,
                    max_dispatches: params
                        .get("max_dispatches")
                        .and_then(Value::as_u64)
                        .map(|n| n as usize),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(validation).map_err(|e| e.to_string())
        }
        "job_list" => {
            let jobs = list_scheduled_jobs(&state.runtime_dir).map_err(|e| e.to_string())?;
            serde_json::to_value(jobs).map_err(|e| e.to_string())
        }
        "job_run" => {
            let summary = run_scheduled_job(
                &state.runtime_dir,
                &state.preset_dir,
                &str_required(params, "job_id")?,
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(summary).map_err(|e| e.to_string())
        }

        // ── Creative Tools ─────────────────────────────────────────
        "preset_patch" => {
            let result = apply_preset_patch(
                &state.preset_dir,
                &state.runtime_dir,
                PresetPatchRequest {
                    preset_name: str_required(params, "preset")?,
                    actor_id: actor_id.to_string(),
                    reason: str_param(params, "reason").unwrap_or_else(|| "http patch".to_string()),
                    tempo_bpm: params
                        .get("tempo_bpm")
                        .and_then(Value::as_u64)
                        .map(|n| n as u16),
                    seed_variation_semitones: params
                        .get("seed_variation_semitones")
                        .and_then(Value::as_u64)
                        .map(|n| n as u8),
                    low_note: params
                        .get("low_note")
                        .and_then(Value::as_u64)
                        .map(|n| n as u8),
                    high_note: params
                        .get("high_note")
                        .and_then(Value::as_u64)
                        .map(|n| n as u8),
                    step_beats: params.get("step_beats").and_then(Value::as_f64),
                    duration_seconds: params.get("duration_seconds").and_then(Value::as_f64),
                    peak_limit: params
                        .get("peak_limit")
                        .and_then(Value::as_f64)
                        .map(|n| n as f32),
                    root_note: params
                        .get("root_note")
                        .and_then(Value::as_u64)
                        .map(|n| n as u8),
                    scale: params.get("scale").and_then(Value::as_array).map(|a| {
                        a.iter()
                            .filter_map(Value::as_u64)
                            .map(|n| n as u8)
                            .collect()
                    }),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(result).map_err(|e| e.to_string())
        }
        "parameter_sweep" => {
            let result = run_parameter_sweep(
                &state.preset_dir,
                &state.runtime_dir,
                ParameterSweepRequest {
                    preset_name: str_required(params, "preset")?,
                    seeds: params
                        .get("seeds")
                        .and_then(Value::as_array)
                        .map(|a| a.iter().filter_map(Value::as_u64).collect())
                        .unwrap_or_else(|| (1..=10).collect()),
                    actor_id: actor_id.to_string(),
                },
            )
            .map_err(|e| e.to_string())?;
            serde_json::to_value(result).map_err(|e| e.to_string())
        }
        "sweep_list" => {
            let sweeps = list_sweeps(&state.runtime_dir).map_err(|e| e.to_string())?;
            serde_json::to_value(sweeps).map_err(|e| e.to_string())
        }

        _ => Err(format!("unknown tool '{tool_name}'")),
    }
}

/// Lightweight composition summary for HTTP responses (avoids sending raw audio buffers).
#[derive(Serialize, Deserialize)]
struct CompositionSummary {
    preset_name: String,
    seed: u64,
    note_count: usize,
    trajectory_frames: usize,
    duration_seconds: f64,
    audio_sample_count: usize,
}

impl From<&GeneratedComposition> for CompositionSummary {
    fn from(comp: &GeneratedComposition) -> Self {
        Self {
            preset_name: comp.preset.name.clone(),
            seed: comp.seed,
            note_count: comp.midi_model.notes().len(),
            trajectory_frames: comp.trajectory_summary.frame_count,
            duration_seconds: comp.trajectory_summary.duration_seconds,
            audio_sample_count: comp.audio_samples.len(),
        }
    }
}

fn all_tools() -> Vec<ToolInfo> {
    vec![
        tool(
            "list_presets",
            "generation",
            "List available presets",
            "low",
        ),
        tool(
            "generate_composition",
            "generation",
            "Generate a composition from a preset (returns summary, not raw audio)",
            "low",
        ),
        tool(
            "generate_demo",
            "generation",
            "Generate a demo composition with the built-in preset",
            "low",
        ),
        tool(
            "session_create",
            "sessions",
            "Create a new session with a preset, seed, and tempo",
            "low",
        ),
        tool("session_list", "sessions", "List all sessions", "low"),
        tool(
            "session_inspect",
            "sessions",
            "Inspect a session by ID",
            "low",
        ),
        tool(
            "session_render_preview",
            "sessions",
            "Render a deterministic MIDI/WAV preview from session state",
            "low",
        ),
        tool(
            "session_play",
            "sessions",
            "Start session transport",
            "medium",
        ),
        tool(
            "session_stop",
            "sessions",
            "Stop session transport",
            "medium",
        ),
        tool(
            "evaluation_list",
            "evaluations",
            "List all evaluation records",
            "low",
        ),
        tool(
            "evaluation_inspect",
            "evaluations",
            "Inspect an evaluation by ID",
            "low",
        ),
        tool("run_list", "audit", "List all run manifests", "low"),
        tool("audit_list", "audit", "List all audit events", "low"),
        tool(
            "deck_create",
            "decks",
            "Create a new deck bound to a session",
            "low",
        ),
        tool("deck_list", "decks", "List all decks", "low"),
        tool(
            "deck_transport",
            "decks",
            "Inspect deck transport state",
            "low",
        ),
        tool(
            "harness_plan",
            "harness",
            "Create a constrained agent plan from a prompt",
            "medium",
        ),
        tool(
            "harness_execute",
            "harness",
            "Execute one action from a harness plan",
            "medium",
        ),
        tool(
            "harness_outcome_list",
            "harness",
            "List all harness execution outcomes",
            "low",
        ),
        tool(
            "realtime_create",
            "realtime",
            "Create an OSC realtime adapter",
            "medium",
        ),
        tool(
            "realtime_list",
            "realtime",
            "List all realtime adapters",
            "low",
        ),
        tool(
            "realtime_send_preview",
            "realtime",
            "Dispatch a session preview to an OSC adapter",
            "medium",
        ),
        tool(
            "dataset_list",
            "governance",
            "List registered datasets",
            "low",
        ),
        tool(
            "approval_request",
            "governance",
            "Create an approval request",
            "low",
        ),
        tool(
            "approval_resolve",
            "governance",
            "Resolve (approve/deny) a pending approval",
            "high",
        ),
        tool(
            "snapshot_create",
            "governance",
            "Create a preset snapshot for rollback",
            "medium",
        ),
        tool(
            "job_validate",
            "scheduler",
            "Validate a scheduled job config",
            "low",
        ),
        tool("job_list", "scheduler", "List all scheduled jobs", "low"),
        tool(
            "job_run",
            "scheduler",
            "Execute a scheduled job locally",
            "medium",
        ),
        tool(
            "preset_patch",
            "creative",
            "Apply a diff-based patch to a preset (e.g. change tempo, note range, scale) with automatic snapshot for rollback",
            "medium",
        ),
        tool(
            "parameter_sweep",
            "creative",
            "Run compositions across multiple seeds and rank by trajectory dynamics",
            "low",
        ),
        tool(
            "sweep_list",
            "creative",
            "List stored parameter sweep results",
            "low",
        ),
    ]
}

fn tool(name: &str, category: &str, description: &str, risk: &str) -> ToolInfo {
    ToolInfo {
        name: name.to_string(),
        category: category.to_string(),
        description: description.to_string(),
        risk: risk.to_string(),
    }
}
