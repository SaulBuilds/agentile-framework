use std::path::Path;

use anyhow::{bail, ensure, Result};
use clap::ValueEnum;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::generation::list_presets;

use super::{
    apply_session_patch, build_review_bundle, create_preset_snapshot, current_unix_seconds,
    default_manifest_dir, default_review_dir, inspect_deck_transport, inspect_realtime_adapter,
    inspect_run_manifest, inspect_session, list_realtime_adapters, new_runtime_id,
    preview_session_patch, read_json_or_default, render_session_preview,
    send_preview_to_realtime_adapter, send_transport_to_realtime_adapter, write_pretty_json,
    OrchestrationPolicy, RealtimeDispatchMode, SendRealtimePreviewRequest,
    SendRealtimeTransportRequest, SessionPatchRequest,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HarnessRole {
    SessionDj,
    Evaluator,
    Librarian,
    Publisher,
    Scheduler,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HarnessRiskLevel {
    Low,
    Medium,
    ApprovalRequired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ValueEnum, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HarnessOutcomeStatus {
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct HarnessContextRef {
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub adapter_id: Option<String>,
    pub run_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct HarnessActionProposal {
    pub action_id: String,
    pub tool_name: String,
    pub risk_level: HarnessRiskLevel,
    pub requires_approval: bool,
    pub justification: String,
    pub expected_effect: String,
    pub rollback_strategy: String,
    pub arguments: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct HarnessPlanRecord {
    pub plan_id: String,
    pub role: HarnessRole,
    pub prompt: String,
    pub system_prompt: String,
    pub context: HarnessContextRef,
    pub deterministic_signature: String,
    pub proposed_actions: Vec<HarnessActionProposal>,
    pub created_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct HarnessExecutionRecord {
    pub outcome_id: String,
    pub plan_id: String,
    pub action_id: String,
    pub tool_name: String,
    pub status: HarnessOutcomeStatus,
    pub result: Option<Value>,
    pub rollback_handle: Option<Value>,
    pub error_message: Option<String>,
    pub created_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct NewHarnessPlanRequest {
    pub role: HarnessRole,
    pub prompt: String,
    pub session_id: Option<String>,
    pub deck_id: Option<String>,
    pub adapter_id: Option<String>,
    pub run_ids: Vec<String>,
    #[serde(default)]
    pub max_actions: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ExecuteHarnessActionRequest {
    pub plan_id: String,
    pub action_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct HarnessPlanSummary {
    pub plan: HarnessPlanRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct HarnessOutcomeSummary {
    pub outcome: HarnessExecutionRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct HarnessStoreFile {
    version: u32,
    plans: Vec<HarnessPlanRecord>,
    outcomes: Vec<HarnessExecutionRecord>,
}

pub fn create_harness_plan(
    store_path: &Path,
    runtime_dir: &Path,
    request: NewHarnessPlanRequest,
) -> Result<HarnessPlanRecord> {
    create_harness_plan_with_policy(
        store_path,
        runtime_dir,
        request,
        OrchestrationPolicy::default(),
    )
}

pub fn create_harness_plan_with_policy(
    store_path: &Path,
    runtime_dir: &Path,
    request: NewHarnessPlanRequest,
    policy: OrchestrationPolicy,
) -> Result<HarnessPlanRecord> {
    validate_plan_request(&request)?;
    validate_context(runtime_dir, &request)?;

    let max_actions = request.max_actions.unwrap_or(policy.max_actions_per_plan);
    let actions = derive_actions(runtime_dir, &request)?;

    let effective_policy = OrchestrationPolicy {
        max_actions_per_plan: max_actions,
        ..policy
    };
    effective_policy
        .validate_plan_action_count(actions.len())
        .map_err(|violation| anyhow::anyhow!("{}", violation))?;

    let plan = HarnessPlanRecord {
        plan_id: new_runtime_id("harness-plan"),
        role: request.role,
        prompt: request.prompt.clone(),
        system_prompt: system_prompt_for_role(request.role).to_string(),
        deterministic_signature: deterministic_signature(&request),
        context: HarnessContextRef {
            session_id: request.session_id,
            deck_id: request.deck_id,
            adapter_id: request.adapter_id,
            run_ids: request.run_ids,
        },
        proposed_actions: actions,
        created_at_unix_seconds: current_unix_seconds(),
    };

    let mut store = load_store(store_path)?;
    store.plans.push(plan.clone());
    save_store(store_path, &store)?;
    Ok(plan)
}

pub fn inspect_harness_plan(store_path: &Path, plan_id: &str) -> Result<HarnessPlanRecord> {
    ensure!(!plan_id.trim().is_empty(), "plan id cannot be empty");
    let store = load_store(store_path)?;
    store
        .plans
        .into_iter()
        .find(|plan| plan.plan_id == plan_id)
        .ok_or_else(|| anyhow::anyhow!("harness plan '{}' was not found", plan_id))
}

pub fn list_harness_outcomes(store_path: &Path) -> Result<Vec<HarnessExecutionRecord>> {
    let mut store = load_store(store_path)?;
    store.outcomes.sort_by(|left, right| {
        left.created_at_unix_seconds
            .cmp(&right.created_at_unix_seconds)
    });
    Ok(store.outcomes)
}

pub fn execute_harness_action(
    store_path: &Path,
    runtime_dir: &Path,
    preset_dir: &Path,
    request: ExecuteHarnessActionRequest,
) -> Result<HarnessExecutionRecord> {
    ensure!(
        !request.plan_id.trim().is_empty(),
        "plan id cannot be empty"
    );
    ensure!(
        !request.action_id.trim().is_empty(),
        "action id cannot be empty"
    );

    let mut store = load_store(store_path)?;
    let plan = store
        .plans
        .iter()
        .find(|plan| plan.plan_id == request.plan_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("harness plan '{}' was not found", request.plan_id))?;
    let action = plan
        .proposed_actions
        .iter()
        .find(|action| action.action_id == request.action_id)
        .cloned()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "action '{}' was not found on plan '{}'",
                request.action_id,
                request.plan_id
            )
        })?;
    ensure!(
        role_allows_tool(plan.role, &action.tool_name),
        "role '{:?}' is not allowed to execute '{}'",
        plan.role,
        action.tool_name
    );

    let outcome = match run_tool(runtime_dir, preset_dir, &plan, &action) {
        Ok((result, rollback_handle)) => HarnessExecutionRecord {
            outcome_id: new_runtime_id("harness-outcome"),
            plan_id: plan.plan_id.clone(),
            action_id: action.action_id.clone(),
            tool_name: action.tool_name.clone(),
            status: HarnessOutcomeStatus::Succeeded,
            result: Some(result),
            rollback_handle,
            error_message: None,
            created_at_unix_seconds: current_unix_seconds(),
        },
        Err(error) => HarnessExecutionRecord {
            outcome_id: new_runtime_id("harness-outcome"),
            plan_id: plan.plan_id.clone(),
            action_id: action.action_id.clone(),
            tool_name: action.tool_name.clone(),
            status: classify_error(&error),
            result: None,
            rollback_handle: None,
            error_message: Some(error.to_string()),
            created_at_unix_seconds: current_unix_seconds(),
        },
    };

    store.outcomes.push(outcome.clone());
    save_store(store_path, &store)?;
    Ok(outcome)
}

fn derive_actions(
    runtime_dir: &Path,
    request: &NewHarnessPlanRequest,
) -> Result<Vec<HarnessActionProposal>> {
    let prompt = request.prompt.to_ascii_lowercase();
    let mut actions = Vec::new();

    if (prompt.contains("compare") || prompt.contains("review")) && request.run_ids.len() >= 2 {
        actions.push(new_action(
            "eval.compare_candidates",
            HarnessRiskLevel::Low,
            false,
            "Compare the requested candidate runs using stored manifests and linked evaluations.",
            "Produce a side-by-side review bundle over the referenced runs.",
            "Read-only analysis; no rollback needed.",
            json!({ "run_ids": request.run_ids }),
        ));
    }

    if prompt.contains("preview") || prompt.contains("render") {
        if let Some(session_id) = &request.session_id {
            actions.push(new_action(
                "live.preview_render",
                HarnessRiskLevel::Low,
                false,
                "Render a deterministic preview from the current session state before applying further changes.",
                "Produce new preview MIDI and WAV artifacts for audition and deck loading.",
                "Read-only render against persisted session state; no rollback needed.",
                json!({ "session_id": session_id }),
            ));
        }
    }

    if prompt.contains("play") || prompt.contains("launch") {
        if let Some(deck_id) = &request.deck_id {
            let transport =
                inspect_deck_transport(&super::default_daw_store_path(runtime_dir), deck_id)?;
            if let Some(clip) = transport
                .queued_clip
                .clone()
                .or_else(|| transport.deck.clips.first().cloned())
            {
                actions.push(new_action(
                    "live.launch_clip",
                    HarnessRiskLevel::Medium,
                    false,
                    "Launch a queued or first available deck clip through the shared deck transport layer.",
                    "Move the deck into playing state with an active clip.",
                    "Stop the deck to clear the active clip and return to a known transport state.",
                    json!({ "deck_id": deck_id, "clip_id": clip.clip_id }),
                ));
            }
        } else if let Some(session_id) = &request.session_id {
            actions.push(new_action(
                "session.get_status",
                HarnessRiskLevel::Low,
                false,
                "Inspect the session before proposing play transport without a deck binding.",
                "Return the current session state for operator review.",
                "Read-only inspection; no rollback needed.",
                json!({ "session_id": session_id }),
            ));
        }
    }

    if prompt.contains("stop") {
        if let Some(deck_id) = &request.deck_id {
            actions.push(new_action(
                "live.stop_transport",
                HarnessRiskLevel::Medium,
                false,
                "Stop the current deck transport through the shared control layer.",
                "Clear the active clip and set the deck transport to stopped.",
                "Launch a clip again to resume transport.",
                json!({ "deck_id": deck_id }),
            ));
        }
    }

    if let Some(session_id) = &request.session_id {
        if let Some(tempo_bpm) = extract_tempo_bpm(&prompt) {
            let patch = SessionPatchRequest {
                actor_id: "harness".to_string(),
                display_name: None,
                seed: None,
                tempo_bpm: Some(tempo_bpm),
                status: None,
            };
            let preview = preview_session_patch(
                &super::default_session_store_path(runtime_dir),
                session_id,
                patch.clone(),
            )?;
            actions.push(new_action(
                "live.preview_patch",
                HarnessRiskLevel::Low,
                false,
                "Preview the requested session mutation before changing persisted state.",
                "Show the before/after session diff for the proposed live patch.",
                "Read-only patch preview; no rollback needed.",
                json!({
                    "session_id": session_id,
                    "patch": patch,
                    "changed_fields": preview.changed_fields,
                }),
            ));
            actions.push(new_action(
                "live.apply_patch",
                HarnessRiskLevel::Medium,
                false,
                "Apply the requested live session mutation through the shared session layer with a captured rollback state.",
                "Persist the requested session change and return a rollback handle.",
                "Use the returned rollback session payload to restore the previous state.",
                json!({ "session_id": session_id, "patch": patch }),
            ));
        }
    }

    if prompt.contains("send")
        || prompt.contains("dispatch")
        || prompt.contains("osc")
        || prompt.contains("broadcast")
        || prompt.contains("stream")
    {
        if let Some(adapter_id) = &request.adapter_id {
            let realtime_store = super::default_realtime_store_path(runtime_dir);
            if let Ok(adapter) = inspect_realtime_adapter(&realtime_store, adapter_id) {
                if let Some(session_id) = &request.session_id {
                    let session_store = super::default_session_store_path(runtime_dir);
                    if let Ok(session) = inspect_session(&session_store, session_id) {
                        if let Some(preview) = session.previews.last() {
                            actions.push(new_action(
                                "realtime.send_preview",
                                HarnessRiskLevel::Medium,
                                false,
                                "Dispatch the latest session preview to the realtime adapter as OSC packets.",
                                "Send MIDI note events from the preview to the configured OSC endpoint.",
                                "No persistent state mutation; dispatch is fire-and-forget over UDP.",
                                json!({
                                    "adapter_id": adapter.adapter_id,
                                    "session_id": session_id,
                                    "preview_id": preview.preview_id,
                                    "dispatch_mode": "immediate",
                                    "time_scale": 0.0
                                }),
                            ));
                        }
                    }
                }
                if let Some(deck_id) = &request.deck_id {
                    actions.push(new_action(
                        "realtime.send_transport",
                        HarnessRiskLevel::Medium,
                        false,
                        "Dispatch the current deck transport state to the realtime adapter as OSC packets.",
                        "Send transport play/stop and active clip metadata to the configured OSC endpoint.",
                        "No persistent state mutation; dispatch is fire-and-forget over UDP.",
                        json!({
                            "adapter_id": adapter.adapter_id,
                            "deck_id": deck_id,
                            "dispatch_mode": "immediate",
                            "time_scale": 0.0
                        }),
                    ));
                }
            }
        } else {
            let realtime_store = super::default_realtime_store_path(runtime_dir);
            if let Ok(adapters) = list_realtime_adapters(&realtime_store) {
                if let Some(adapter) = adapters.first() {
                    if let Some(session_id) = &request.session_id {
                        let session_store = super::default_session_store_path(runtime_dir);
                        if let Ok(session) = inspect_session(&session_store, session_id) {
                            if let Some(preview) = session.previews.last() {
                                actions.push(new_action(
                                    "realtime.send_preview",
                                    HarnessRiskLevel::Medium,
                                    false,
                                    "Dispatch the latest session preview to the first available realtime adapter.",
                                    "Send MIDI note events from the preview to the configured OSC endpoint.",
                                    "No persistent state mutation; dispatch is fire-and-forget over UDP.",
                                    json!({
                                        "adapter_id": adapter.adapter_id,
                                        "session_id": session_id,
                                        "preview_id": preview.preview_id,
                                        "dispatch_mode": "immediate",
                                        "time_scale": 0.0
                                    }),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    if actions.is_empty() {
        if let Some(session_id) = &request.session_id {
            actions.push(new_action(
                "session.get_status",
                HarnessRiskLevel::Low,
                false,
                "No explicit mutation was requested, so inspect the current session state first.",
                "Return the current persisted session state and recent event history.",
                "Read-only inspection; no rollback needed.",
                json!({ "session_id": session_id }),
            ));
        } else {
            actions.push(new_action(
                "preset.list",
                HarnessRiskLevel::Low,
                false,
                "No direct session context was provided, so enumerate available presets first.",
                "Return the available preset list for follow-up selection.",
                "Read-only listing; no rollback needed.",
                json!({}),
            ));
        }
    }

    Ok(actions)
}

fn run_tool(
    runtime_dir: &Path,
    preset_dir: &Path,
    plan: &HarnessPlanRecord,
    action: &HarnessActionProposal,
) -> Result<(Value, Option<Value>)> {
    match action.tool_name.as_str() {
        "session.get_status" => {
            let session_id = required_str(&action.arguments, "session_id")?;
            let session =
                inspect_session(&super::default_session_store_path(runtime_dir), session_id)?;
            Ok((serde_json::to_value(session)?, None))
        }
        "preset.list" => {
            let presets = list_presets(preset_dir)?;
            Ok((serde_json::to_value(presets)?, None))
        }
        "eval.compare_candidates" => {
            let run_ids = required_string_list(&action.arguments, "run_ids")?;
            let review = build_review_bundle(
                &super::default_evaluation_store_path(runtime_dir),
                &default_manifest_dir(runtime_dir),
                &run_ids,
            )?;
            let export_path =
                default_review_dir(runtime_dir).join(format!("{}.json", action.action_id));
            write_pretty_json(&export_path, &review)?;
            Ok((
                json!({
                    "review": review,
                    "export_path": export_path,
                }),
                None,
            ))
        }
        "live.preview_render" => {
            let session_id = required_str(&action.arguments, "session_id")?;
            let preview = render_session_preview(
                &super::default_session_store_path(runtime_dir),
                preset_dir,
                runtime_dir,
                session_id,
                "harness",
            )?;
            Ok((serde_json::to_value(preview)?, None))
        }
        "live.preview_patch" => {
            let session_id = required_str(&action.arguments, "session_id")?;
            let patch: SessionPatchRequest =
                serde_json::from_value(required_value(&action.arguments, "patch")?.clone())?;
            let preview = preview_session_patch(
                &super::default_session_store_path(runtime_dir),
                session_id,
                patch,
            )?;
            Ok((serde_json::to_value(preview)?, None))
        }
        "live.apply_patch" => {
            let session_id = required_str(&action.arguments, "session_id")?;
            let patch: SessionPatchRequest =
                serde_json::from_value(required_value(&action.arguments, "patch")?.clone())?;
            if let Some(preset_name) = &plan.context.session_id.as_ref().and_then(|id| {
                inspect_session(&super::default_session_store_path(runtime_dir), id)
                    .ok()
                    .map(|session| session.preset_name)
            }) {
                let _ = create_preset_snapshot(
                    &super::default_snapshot_dir(runtime_dir),
                    preset_dir,
                    preset_name,
                    "harness live apply patch",
                    Some("harness"),
                );
            }
            let applied = apply_session_patch(
                &super::default_session_store_path(runtime_dir),
                session_id,
                patch,
            )?;
            Ok((
                serde_json::to_value(&applied.session)?,
                Some(json!({
                    "rollback_session": applied.rollback,
                    "changed_fields": applied.changed_fields,
                })),
            ))
        }
        "live.launch_clip" => {
            let deck_id = required_str(&action.arguments, "deck_id")?;
            let clip_id = required_str(&action.arguments, "clip_id")?;
            let snapshot = super::launch_deck_clip(
                &super::default_daw_store_path(runtime_dir),
                deck_id,
                super::LaunchDeckClipRequest {
                    actor_id: "harness".to_string(),
                    clip_id: clip_id.to_string(),
                },
            )?;
            Ok((
                serde_json::to_value(snapshot)?,
                Some(json!({ "deck_id": deck_id, "rollback_action": "live.stop_transport" })),
            ))
        }
        "live.stop_transport" => {
            let deck_id = required_str(&action.arguments, "deck_id")?;
            let snapshot = super::stop_deck(
                &super::default_daw_store_path(runtime_dir),
                deck_id,
                super::StopDeckRequest {
                    actor_id: "harness".to_string(),
                },
            )?;
            Ok((serde_json::to_value(snapshot)?, None))
        }
        "realtime.send_preview" => {
            let adapter_id = required_str(&action.arguments, "adapter_id")?;
            let session_id = required_str(&action.arguments, "session_id")?;
            let preview_id = required_str(&action.arguments, "preview_id")?;
            let mode = action
                .arguments
                .get("dispatch_mode")
                .and_then(Value::as_str)
                .unwrap_or("immediate");
            let dispatch_mode = if mode == "timed" {
                RealtimeDispatchMode::Timed
            } else {
                RealtimeDispatchMode::Immediate
            };
            let time_scale = action
                .arguments
                .get("time_scale")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
            let summary = send_preview_to_realtime_adapter(
                &super::default_realtime_store_path(runtime_dir),
                &super::default_session_store_path(runtime_dir),
                adapter_id,
                SendRealtimePreviewRequest {
                    actor_id: "harness".to_string(),
                    session_id: session_id.to_string(),
                    preview_id: preview_id.to_string(),
                    dispatch_mode,
                    time_scale,
                },
            )?;
            Ok((
                json!({
                    "adapter_id": summary.adapter.adapter_id,
                    "protocol": summary.adapter.protocol,
                    "message_count": summary.dispatch.message_count,
                    "dispatch_mode": summary.dispatch.dispatch_mode,
                }),
                None,
            ))
        }
        "realtime.send_transport" => {
            let adapter_id = required_str(&action.arguments, "adapter_id")?;
            let deck_id = required_str(&action.arguments, "deck_id")?;
            let mode = action
                .arguments
                .get("dispatch_mode")
                .and_then(Value::as_str)
                .unwrap_or("immediate");
            let dispatch_mode = if mode == "timed" {
                RealtimeDispatchMode::Timed
            } else {
                RealtimeDispatchMode::Immediate
            };
            let time_scale = action
                .arguments
                .get("time_scale")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
            let summary = send_transport_to_realtime_adapter(
                &super::default_realtime_store_path(runtime_dir),
                &super::default_daw_store_path(runtime_dir),
                adapter_id,
                SendRealtimeTransportRequest {
                    actor_id: "harness".to_string(),
                    deck_id: deck_id.to_string(),
                    dispatch_mode,
                    time_scale,
                },
            )?;
            Ok((
                json!({
                    "adapter_id": summary.adapter.adapter_id,
                    "protocol": summary.adapter.protocol,
                    "message_count": summary.dispatch.message_count,
                    "dispatch_mode": summary.dispatch.dispatch_mode,
                }),
                None,
            ))
        }
        _ => bail!(
            "tool '{}' is not implemented by the harness executor",
            action.tool_name
        ),
    }
}

fn classify_error(error: &anyhow::Error) -> HarnessOutcomeStatus {
    if error.to_string().contains("not allowed")
        || error.to_string().contains("approval")
        || error.to_string().contains("not implemented")
    {
        HarnessOutcomeStatus::Blocked
    } else {
        HarnessOutcomeStatus::Failed
    }
}

fn validate_plan_request(request: &NewHarnessPlanRequest) -> Result<()> {
    ensure!(!request.prompt.trim().is_empty(), "prompt cannot be empty");
    Ok(())
}

fn validate_context(runtime_dir: &Path, request: &NewHarnessPlanRequest) -> Result<()> {
    if let Some(session_id) = &request.session_id {
        inspect_session(&super::default_session_store_path(runtime_dir), session_id)?;
    }
    if let Some(deck_id) = &request.deck_id {
        inspect_deck_transport(&super::default_daw_store_path(runtime_dir), deck_id)?;
    }
    if let Some(adapter_id) = &request.adapter_id {
        inspect_realtime_adapter(&super::default_realtime_store_path(runtime_dir), adapter_id)?;
    }
    for run_id in &request.run_ids {
        inspect_run_manifest(&default_manifest_dir(runtime_dir), run_id)?;
    }
    Ok(())
}

fn deterministic_signature(request: &NewHarnessPlanRequest) -> String {
    format!(
        "{:?}|{}|{:?}|{:?}|{:?}|{:?}",
        request.role,
        request.prompt,
        request.session_id,
        request.deck_id,
        request.adapter_id,
        request.run_ids
    )
}

fn new_action(
    tool_name: &str,
    risk_level: HarnessRiskLevel,
    requires_approval: bool,
    justification: &str,
    expected_effect: &str,
    rollback_strategy: &str,
    arguments: Value,
) -> HarnessActionProposal {
    HarnessActionProposal {
        action_id: new_runtime_id("harness-action"),
        tool_name: tool_name.to_string(),
        risk_level,
        requires_approval,
        justification: justification.to_string(),
        expected_effect: expected_effect.to_string(),
        rollback_strategy: rollback_strategy.to_string(),
        arguments,
    }
}

fn role_allows_tool(role: HarnessRole, tool_name: &str) -> bool {
    match role {
        HarnessRole::SessionDj => matches!(
            tool_name,
            "session.get_status"
                | "preset.list"
                | "eval.compare_candidates"
                | "live.preview_render"
                | "live.preview_patch"
                | "live.apply_patch"
                | "live.launch_clip"
                | "live.stop_transport"
                | "realtime.send_preview"
                | "realtime.send_transport"
        ),
        HarnessRole::Evaluator => matches!(tool_name, "eval.compare_candidates"),
        HarnessRole::Librarian => matches!(tool_name, "preset.list"),
        HarnessRole::Publisher | HarnessRole::Scheduler => false,
    }
}

fn system_prompt_for_role(role: HarnessRole) -> &'static str {
    match role {
        HarnessRole::SessionDj => "You are Session DJ, a constrained music-control agent.",
        HarnessRole::Evaluator => "You are Evaluator, a scoring and analysis agent.",
        HarnessRole::Librarian => "You are Librarian, the provenance and preset curation agent.",
        HarnessRole::Publisher => "You are Publisher, a high-risk gated agent.",
        HarnessRole::Scheduler => "You are Scheduler, the unattended-run planning agent.",
    }
}

fn extract_tempo_bpm(prompt: &str) -> Option<u16> {
    let tokens = prompt
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|token| !token.is_empty());
    for token in tokens {
        if let Ok(value) = token.parse::<u16>() {
            if (20..=320).contains(&value) {
                return Some(value);
            }
        }
    }
    None
}

fn required_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("missing string field '{key}'"))
}

fn required_value<'a>(value: &'a Value, key: &str) -> Result<&'a Value> {
    value
        .get(key)
        .ok_or_else(|| anyhow::anyhow!("missing field '{key}'"))
}

fn required_string_list(value: &Value, key: &str) -> Result<Vec<String>> {
    let Some(array) = value.get(key).and_then(Value::as_array) else {
        bail!("missing string array field '{key}'");
    };
    array
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| anyhow::anyhow!("field '{key}' must contain strings"))
        })
        .collect()
}

fn load_store(store_path: &Path) -> Result<HarnessStoreFile> {
    let mut store: HarnessStoreFile = read_json_or_default(store_path)?;
    if store.version == 0 {
        store.version = 1;
    }
    Ok(store)
}

fn save_store(store_path: &Path, store: &HarnessStoreFile) -> Result<()> {
    write_pretty_json(store_path, store)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::generation::{demo_preset, save_preset};
    use crate::governance::{
        create_deck, create_session, render_session_preview, NewDeckRequest, NewSessionRequest,
    };

    #[test]
    fn test_create_and_execute_session_dj_plan() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let harness_store = runtime_dir.join("harness.json");

        let mut preset = demo_preset();
        preset.name = "harness-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let session = create_session(
            &super::super::default_session_store_path(&runtime_dir),
            &preset_dir,
            NewSessionRequest {
                display_name: "Harness Session".to_string(),
                preset_name: "harness-demo".to_string(),
                seed: 7,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let preview = render_session_preview(
            &super::super::default_session_store_path(&runtime_dir),
            &preset_dir,
            &runtime_dir,
            &session.session_id,
            "tester",
        )
        .unwrap();
        let deck = create_deck(
            &super::super::default_daw_store_path(&runtime_dir),
            &super::super::default_session_store_path(&runtime_dir),
            NewDeckRequest {
                display_name: "Deck".to_string(),
                session_id: session.session_id.clone(),
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        super::super::add_preview_clip_to_deck(
            &super::super::default_daw_store_path(&runtime_dir),
            &super::super::default_session_store_path(&runtime_dir),
            &deck.deck_id,
            super::super::AddDeckPreviewRequest {
                actor_id: "tester".to_string(),
                label: "Clip".to_string(),
                session_id: session.session_id.clone(),
                preview_id: preview.preview.preview_id.clone(),
            },
        )
        .unwrap();

        let plan = create_harness_plan(
            &harness_store,
            &runtime_dir,
            NewHarnessPlanRequest {
                role: HarnessRole::SessionDj,
                prompt: "set tempo to 132 and render a preview".to_string(),
                session_id: Some(session.session_id.clone()),
                deck_id: Some(deck.deck_id.clone()),
                adapter_id: None,
                run_ids: Vec::new(),
                max_actions: None,
            },
        )
        .unwrap();
        assert!(plan
            .proposed_actions
            .iter()
            .any(|action| action.tool_name == "live.preview_patch"));
        let apply_action = plan
            .proposed_actions
            .iter()
            .find(|action| action.tool_name == "live.apply_patch")
            .unwrap();

        let outcome = execute_harness_action(
            &harness_store,
            &runtime_dir,
            &preset_dir,
            ExecuteHarnessActionRequest {
                plan_id: plan.plan_id,
                action_id: apply_action.action_id.clone(),
            },
        )
        .unwrap();
        assert_eq!(outcome.status, HarnessOutcomeStatus::Succeeded);
        assert!(outcome.rollback_handle.is_some());
    }

    #[test]
    fn test_harness_realtime_dispatch_through_adapter() {
        use std::net::UdpSocket;
        use std::time::Duration;

        use crate::governance::{
            add_preview_clip_to_deck, create_realtime_adapter, AddDeckPreviewRequest,
            NewRealtimeAdapterRequest, RealtimeAdapterProtocol,
        };

        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let harness_store = runtime_dir.join("harness.json");

        let mut preset = demo_preset();
        preset.name = "dispatch-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let session = create_session(
            &super::super::default_session_store_path(&runtime_dir),
            &preset_dir,
            NewSessionRequest {
                display_name: "Dispatch Session".to_string(),
                preset_name: "dispatch-demo".to_string(),
                seed: 9,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        let preview = render_session_preview(
            &super::super::default_session_store_path(&runtime_dir),
            &preset_dir,
            &runtime_dir,
            &session.session_id,
            "tester",
        )
        .unwrap();
        let deck = create_deck(
            &super::super::default_daw_store_path(&runtime_dir),
            &super::super::default_session_store_path(&runtime_dir),
            NewDeckRequest {
                display_name: "Dispatch Deck".to_string(),
                session_id: session.session_id.clone(),
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();
        add_preview_clip_to_deck(
            &super::super::default_daw_store_path(&runtime_dir),
            &super::super::default_session_store_path(&runtime_dir),
            &deck.deck_id,
            AddDeckPreviewRequest {
                actor_id: "tester".to_string(),
                label: "Clip".to_string(),
                session_id: session.session_id.clone(),
                preview_id: preview.preview.preview_id.clone(),
            },
        )
        .unwrap();

        let listener = UdpSocket::bind("127.0.0.1:0").unwrap();
        listener
            .set_read_timeout(Some(Duration::from_millis(250)))
            .unwrap();
        let port = listener.local_addr().unwrap().port();

        let adapter = create_realtime_adapter(
            &super::super::default_realtime_store_path(&runtime_dir),
            NewRealtimeAdapterRequest {
                display_name: "Harness Loopback".to_string(),
                protocol: RealtimeAdapterProtocol::OscUdp,
                host: "127.0.0.1".parse().unwrap(),
                port,
                base_path: "/harness_test".to_string(),
            },
        )
        .unwrap();

        let plan = create_harness_plan(
            &harness_store,
            &runtime_dir,
            NewHarnessPlanRequest {
                role: HarnessRole::SessionDj,
                prompt: "send the latest preview to the osc adapter".to_string(),
                session_id: Some(session.session_id.clone()),
                deck_id: Some(deck.deck_id.clone()),
                adapter_id: Some(adapter.adapter_id.clone()),
                run_ids: Vec::new(),
                max_actions: None,
            },
        )
        .unwrap();

        assert!(plan
            .proposed_actions
            .iter()
            .any(|action| action.tool_name == "realtime.send_preview"));

        let send_action = plan
            .proposed_actions
            .iter()
            .find(|action| action.tool_name == "realtime.send_preview")
            .unwrap();

        let outcome = execute_harness_action(
            &harness_store,
            &runtime_dir,
            &preset_dir,
            ExecuteHarnessActionRequest {
                plan_id: plan.plan_id,
                action_id: send_action.action_id.clone(),
            },
        )
        .unwrap();
        assert_eq!(outcome.status, HarnessOutcomeStatus::Succeeded);
        let result = outcome.result.unwrap();
        assert!(result.get("message_count").unwrap().as_u64().unwrap() >= 3);

        let mut buf = [0u8; 2048];
        let (size, _) = listener.recv_from(&mut buf).unwrap();
        let packet = rosc::decoder::decode_udp(&buf[..size]).unwrap().1;
        match packet {
            rosc::OscPacket::Message(msg) => {
                assert!(msg.addr.starts_with("/harness_test/"));
            }
            other => panic!("unexpected packet: {other:?}"),
        }
    }

    #[test]
    fn test_policy_rejects_plan_with_too_many_actions() {
        let dir = tempdir().unwrap();
        let preset_dir = dir.path().join("presets");
        let runtime_dir = dir.path().join("runtime");
        let harness_store = runtime_dir.join("harness.json");

        let mut preset = demo_preset();
        preset.name = "policy-demo".to_string();
        save_preset(&preset, &preset_dir).unwrap();

        let session = create_session(
            &super::super::default_session_store_path(&runtime_dir),
            &preset_dir,
            NewSessionRequest {
                display_name: "Policy Session".to_string(),
                preset_name: "policy-demo".to_string(),
                seed: 5,
                actor_id: "tester".to_string(),
            },
        )
        .unwrap();

        let result = create_harness_plan_with_policy(
            &harness_store,
            &runtime_dir,
            NewHarnessPlanRequest {
                role: HarnessRole::SessionDj,
                prompt: "set tempo to 132 and render a preview".to_string(),
                session_id: Some(session.session_id.clone()),
                deck_id: None,
                adapter_id: None,
                run_ids: Vec::new(),
                max_actions: Some(1),
            },
            OrchestrationPolicy {
                max_actions_per_plan: 1,
                ..Default::default()
            },
        );
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("policy allows at most 1"));
    }
}
