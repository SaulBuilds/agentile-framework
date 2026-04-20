# Agent Integration Guide

This guide tells Hermes, OpenClaw, and other AI agents everything they need to use the state-space-music-box HTTP API to creatively generate music, evaluate outputs, and refine parameters.

## Quick Start

```bash
# Start the server
cargo run -- http --port 3001 --api-key YOUR_KEY

# Every tool call follows this pattern:
curl -X POST http://localhost:3001/api/tools/{tool_name} \
  -H "Authorization: Bearer YOUR_KEY" \
  -H "Content-Type: application/json" \
  -d '{ ... params ... }'
```

All responses use a consistent envelope:
```json
{"success": true, "data": { ... }}
{"success": false, "error": "description"}
```

## Tool Reference

### Generation (low risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `list_presets` | List available presets | -- |
| `generate_demo` | Generate a composition from the built-in demo | `seed` (u64) |
| `generate_composition` | Generate from a named preset | `preset` (string), `seed` (u64) |

### Sessions (low-medium risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `session_create` | Create a session with a preset, seed, and tempo | `display_name`, `preset`, `seed` |
| `session_list` | List all sessions | -- |
| `session_inspect` | Get session details | `session_id` |
| `session_render_preview` | Render MIDI+WAV preview from session state | `session_id` |
| `session_play` | Start session transport | `session_id`, `run_label` (optional) |
| `session_stop` | Stop session transport | `session_id` |

### Creative Tools (low-medium risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `preset_patch` | Patch preset parameters with auto-snapshot | `preset`, `tempo_bpm`, `low_note`, `high_note`, `scale`, `duration_seconds`, etc. |
| `parameter_sweep` | Run N seeds, rank by dynamics | `preset`, `seeds` (array of u64) |
| `sweep_list` | List stored sweep results | -- |

### Evaluations (low risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `evaluation_list` | List evaluations | -- |
| `evaluation_inspect` | Get evaluation details | `evaluation_id` |

### Decks (low-medium risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `deck_create` | Create a deck bound to a session | `display_name`, `session_id` |
| `deck_list` | List all decks | -- |
| `deck_transport` | Get deck transport state | `deck_id` |

### Harness (medium risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `harness_plan` | Create a constrained agent plan | `role`, `prompt`, `session_id`, `deck_id`, `adapter_id` |
| `harness_execute` | Execute one action from a plan | `plan_id`, `action_id` |
| `harness_outcome_list` | List execution outcomes | -- |

### Realtime (medium risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `realtime_create` | Register an OSC adapter | `display_name`, `host`, `port` |
| `realtime_list` | List adapters | -- |
| `realtime_send_preview` | Dispatch preview over OSC | `adapter_id`, `session_id`, `preview_id` |

### Governance (low-high risk)

| Tool | Purpose | Key Params | Risk |
|------|---------|------------|------|
| `dataset_list` | List registered datasets | -- | low |
| `approval_request` | Create an approval request | `action_scope`, `target`, `reason` | low |
| `approval_resolve` | Approve/deny a request | `approval_id`, `decision` | high |
| `snapshot_create` | Snapshot a preset for rollback | `preset`, `reason` | medium |

### Scheduler (low-medium risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `job_validate` | Validate a job config | `backend`, `role`, `prompt`, `session_id` |
| `job_list` | List all jobs | -- |
| `job_run` | Execute a job locally | `job_id` |

### Audit (low risk)

| Tool | Purpose | Key Params |
|------|---------|------------|
| `run_list` | List run manifests | -- |
| `audit_list` | List audit events | -- |

## Creative Workflow Cookbook

### 1. Explore: Generate and Compare Seeds

```bash
# Sweep 10 seeds and see which is most dynamic
curl -X POST http://localhost:3001/api/tools/parameter_sweep \
  -H "Authorization: Bearer $KEY" -H "Content-Type: application/json" \
  -d '{"preset": "demo", "seeds": [1,2,3,4,5,6,7,8,9,10]}'

# Response includes ranked_seeds -- use the top seed
```

### 2. Create: Start a Session with the Best Seed

```bash
curl -X POST http://localhost:3001/api/tools/session_create \
  -H "Authorization: Bearer $KEY" -H "Content-Type: application/json" \
  -d '{"display_name": "Creative Session", "preset": "demo", "seed": 7}'
```

### 3. Render: Generate a Preview

```bash
curl -X POST http://localhost:3001/api/tools/session_render_preview \
  -H "Authorization: Bearer $KEY" -H "Content-Type: application/json" \
  -d '{"session_id": "SESSION_ID"}'
```

### 4. Adapt: Patch Parameters and Re-render

```bash
# Change the tempo and note range
curl -X POST http://localhost:3001/api/tools/preset_patch \
  -H "Authorization: Bearer $KEY" -H "Content-Type: application/json" \
  -d '{"preset": "demo", "tempo_bpm": 140, "low_note": 48, "high_note": 84}'

# Sweep again with the patched preset
curl -X POST http://localhost:3001/api/tools/parameter_sweep \
  -H "Authorization: Bearer $KEY" -H "Content-Type: application/json" \
  -d '{"preset": "demo", "seeds": [1,2,3,4,5]}'
```

### 5. Evaluate: Score and Decide

Use the evaluation tools to submit scores and compare runs programmatically.

### 6. Schedule: Automate Overnight Runs

Validate and schedule a batch job for unattended execution.

## Governance Invariants

These rules are enforced by the system -- agents cannot bypass them:

1. **Presets are snapshotted before mutation**. Every `preset_patch` call creates a snapshot first.
2. **Approval tokens are single-use and scoped**. A token for `jobs.schedule` cannot be used for `publish.execute`.
3. **Audit log is append-only**. Every tool call is recorded. Nothing is deleted.
4. **Run manifests are immutable**. Once a run is recorded, its manifest cannot be edited.
5. **Scheduled jobs have immutable configs**. Once scheduled, the job config cannot be changed (only cancelled with approval).
6. **Recursive scheduling is blocked**. A job running inside the scheduler cannot schedule new jobs.
7. **Orchestration policy limits apply**. Plans are capped at 10 actions, job runs at 20 dispatches (configurable).

## Error Handling

- `400 Bad Request` with `{"success": false, "error": "..."}` for invalid params or business rule violations.
- `401 Unauthorized` for missing or invalid API key.
- Errors are descriptive. Read the `error` field to understand what went wrong.
- If an approval is required, the error will say so -- request an approval and retry with the token.
