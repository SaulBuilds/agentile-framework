# state-space-music-box

`state-space-music-box` is a Rust library for turning linear state-space systems into deterministic trajectories, MIDI clips, and WAV audio artifacts.

The project is the mathematical and tooling foundation for a future "Agentic DJ" stack. The repository now ships a real deterministic generation path, a real CLI, a real stdio MCP server, an honest VST validation boundary, a local governance layer for datasets, approvals, and preset rollback, durable run manifests and audit logs for real actions, durable session-state and evaluation-record layers, honest live-control and review surfaces, a DAW-agnostic deck adapter, a constrained harness contract, an unattended scheduler layer, and a first local OSC bridge for realtime dispatch.

## Current Status

The repository currently verifies cleanly with:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

What works today:

- `StateSpaceSystem` creation, validation, prediction, output computation, discretization, controllability, and observability checks
- `MidiModel`, `InstrumentModel`, and `EffectModel` data structures
- `StateMachine` event and transition handling
- `AudioEngine` deterministic mono rendering from both state-space systems and generated MIDI clips
- preset-backed deterministic trajectory simulation, MIDI mapping, MIDI export, and WAV export
- persistent dataset registry records with license, provenance, use-class, and checksum metadata
- approval request, resolution, and single-use token consumption for gated actions
- preset snapshots with exact rollback for file-backed presets
- durable run manifests and append-only audit events for render and governance actions
- read-only runtime inspection through CLI commands and MCP tools for run manifests and audit events
- durable local sessions with preset identity, seed, tempo, status, and structured event history
- session play and stop control with active run labels persisted in the local session store
- deterministic session preview rendering that writes real preview MIDI and WAV artifacts into the runtime store
- durable evaluation records with raw metrics, raw human scores, reward weights, aggregate score, and final decision
- run comparison helpers over persisted manifests
- evaluation inspection and side-by-side review bundle construction over stored runs plus linked evaluations
- DAW-agnostic decks that can load preview clips, queue them, launch them, stop them, and report current transport state
- a persisted harness that can plan bounded actions, inspect those plans, execute one mediated action, and store the resulting outcome plus rollback payload
- immutable unattended jobs with approval-gated scheduling, approval-gated cancellation, local batch execution through the shared harness, and exported scheduler bundles for Hermes/OpenClaw-style runners
- persisted realtime adapter configs with OSC UDP endpoints plus live preview and transport dispatch logs
- orchestrated realtime dispatch through the harness so planned actions can include live OSC preview and transport sends
- orchestrated realtime dispatch through the scheduler so unattended jobs can include live OSC dispatch steps
- configurable orchestration policy with maximum actions per plan, maximum dispatches per job run, and recursive job prevention
- preset patch tool for diff-based parameter mutation with automatic snapshot and rollback
- parameter sweep tool for multi-seed generation with ranked comparison by trajectory dynamics
- HTTP API server exposing all tools via REST with bearer token auth and CORS
- CLI commands for `generate-demo`, `generate-midi`, `generate-audio`, `inspect-trajectory`, `list-presets`, `run-list`, `run-inspect`, `run-compare`, `audit-list`, `session-list`, `session-create`, `session-inspect`, `session-update`, `session-play`, `session-stop`, `session-render-preview`, `evaluation-list`, `evaluation-submit`, `evaluation-inspect`, `review-build`, `deck-list`, `deck-create`, `deck-inspect`, `deck-add-preview`, `deck-queue`, `deck-launch`, `deck-stop`, `deck-transport`, `harness-plan`, `harness-plan-inspect`, `harness-execute`, `harness-outcome-list`, `job-validate`, `job-schedule`, `job-list`, `job-inspect`, `job-run`, `job-cancel`, `realtime-create`, `realtime-list`, `realtime-inspect`, `realtime-send-preview`, `realtime-send-transport`, `validate`, `mcp`, `dataset-list`, `dataset-inspect`, `dataset-register`, `approval-request`, `approval-resolve`, `snapshot-create`, and `snapshot-rollback`
- stdio MCP tools for `create_system`, `create_preset`, `list_presets`, `run_list`, `run_inspect`, `run_compare`, `audit_list`, `session_list`, `session_create`, `session_inspect`, `session_update`, `session_play`, `session_stop`, `session_render_preview`, `evaluation_list`, `evaluation_submit`, `evaluation_inspect`, `review_build`, `deck_list`, `deck_create`, `deck_inspect`, `deck_add_preview`, `deck_queue`, `deck_launch`, `deck_stop`, `deck_transport`, `harness_plan`, `harness_plan_inspect`, `harness_execute`, `harness_outcome_list`, `job_validate`, `job_schedule`, `job_list`, `job_inspect`, `job_run`, `job_cancel`, `realtime_create`, `realtime_list`, `realtime_inspect`, `realtime_send_preview`, `realtime_send_transport`, `generate_midi`, `generate_audio`, `inspect_trajectory`, `dataset_list`, `dataset_inspect`, `dataset_register`, `approval_request`, `approval_resolve`, `snapshot_create`, and `snapshot_rollback`
- validated VST bundle references with real filesystem checks and metadata refresh

What is still partial or planned:

- Real VST3 hosting and audio processing
- native virtual MIDI-port output
- remote scheduler integration, publishing workflows, and deeper policy-aware adaptation loops for the Agentic DJ layer

## Quick Start

```bash
cargo build
cargo test
cargo run -- validate
cargo run -- generate-demo --midi out/demo.mid --wav out/demo.wav --seed 1
cargo run -- run-list
cargo run -- session-create --display-name "First Session" --preset demo --actor-id local-dev
cargo run -- session-play --session-id session-123 --actor-id local-dev --run-label rehearsal-pass
cargo run -- session-render-preview --session-id session-123 --actor-id local-dev
cargo run -- deck-create --display-name "Deck A" --session-id session-123 --actor-id local-dev
cargo run -- harness-plan --role session-dj --prompt "set tempo to 132 and render a preview" --session-id session-123
cargo run -- job-validate --backend local-cli --role session-dj --prompt "set tempo to 132 and render a preview" --session-id session-123
cargo run -- realtime-create --display-name "Loopback" --host 127.0.0.1 --port 9000 --base-path /agentic_dj
cargo run -- review-build --run-id run-123 --run-id run-456 --output comparison.json
cargo run -- approval-request --action-scope dataset.register --target pdmx-demo --requested-by local-dev --reason "register production dataset"
```

## CLI

```bash
cargo run -- generate-midi --preset demo --output out/demo.mid --seed 1
cargo run -- generate-audio --preset demo --output out/demo.wav --seed 1
cargo run -- inspect-trajectory --preset demo
cargo run -- list-presets
cargo run -- run-list
cargo run -- run-inspect --run-id run-123
cargo run -- run-compare --run-id run-123 --run-id run-456
cargo run -- audit-list --limit 10
cargo run -- session-list
cargo run -- session-inspect --session-id session-123
cargo run -- session-play --session-id session-123 --actor-id local-dev
cargo run -- session-render-preview --session-id session-123 --actor-id local-dev
cargo run -- deck-transport --deck-id deck-123
cargo run -- harness-outcome-list
cargo run -- job-list
cargo run -- job-inspect --job-id job-123
cargo run -- realtime-send-preview --adapter-id adapter-123 --session-id session-123 --preview-id preview-123 --actor-id local-dev --dispatch-mode immediate --time-scale 0
cargo run -- evaluation-list
cargo run -- evaluation-inspect --evaluation-id evaluation-123
cargo run -- review-build --run-id run-123 --run-id run-456
cargo run -- dataset-list
cargo run -- dataset-inspect --dataset-id pdmx-demo
cargo run -- snapshot-create --preset my-preset --reason "before retune"
cargo run -- mcp --runtime-dir .agentile/runtime
```

## Example

```rust
use nalgebra::DMatrix;
use state_space_music_box::{AudioEngine, StateSpaceSystem};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = DMatrix::from_row_slice(2, 2, &[1.0, 0.01, -0.01, 0.999]);
    let b = DMatrix::zeros(2, 0);
    let c = DMatrix::from_row_slice(1, 2, &[1.0, 0.0]);
    let d = DMatrix::zeros(1, 0);

    let system = StateSpaceSystem::new(a, b, c, d, Some(0.01))?;
    let mut engine = AudioEngine::new();
    let buffer = engine.generate_audio_from_state_space(&system, 1.0, 44_100)?;

    println!("generated {} samples", buffer.len());
    Ok(())
}
```

## Repository Layout

- `src/state_space.rs`: state-space system math and validation
- `src/midi_model.rs`: MIDI note and clip data models
- `src/instrument_model.rs`: instrument parameter models
- `src/effect_model.rs`: effect parameter models
- `src/state_machine.rs`: event-driven state machine
- `src/generation.rs`: preset-backed trajectory simulation, MIDI mapping, and artifact orchestration
- `src/audio_engine.rs`: deterministic offline audio buffer generation and WAV writing
- `src/cli.rs`: real command-line artifact and inspection workflows
- `src/mcp.rs`: stdio MCP server and tool surface
- `src/vst_synthesizer.rs`: validated VST bundle reference and metadata boundary
- `src/governance/`: dataset registry, approval flow, preset snapshot/rollback, run manifest, audit-trail, session, evaluation, and DAW-agnostic deck primitives
- `src/governance/harness.rs`: persisted harness plans, persisted outcomes, deterministic planning, and mediated execution
- `src/governance/scheduler.rs`: immutable unattended jobs, local batch execution, and exported scheduler adapter bundles
- `src/governance/realtime.rs`: persisted OSC adapters plus live preview and transport dispatch
- `src/governance/policy.rs`: orchestration policy enforcement for harness plans and scheduled job runs
- `src/governance/creative.rs`: preset patch and parameter sweep tools for agent-driven music exploration
- `src/http_server.rs`: axum HTTP server exposing all tools via REST
- `docs/AGENT_GUIDE.md`: complete agent integration guide with tool reference and creative workflow cookbook
- `docs/HERMES_TEMPLATE.md`: Hermes cron job templates for nightly evaluation and exploration
- `docs/OPENCLAW_TEMPLATE.md`: OpenClaw cron job templates with webhook delivery
- `.agentile/`: sprint records, rules, audits, and implementation plans

## Web Dashboard

A Next.js webapp under `web/` provides a visual control plane for every SDK surface.

```bash
# Set up
cp web/.env.local.example web/.env.local
# Edit web/.env.local with your API URL and key

# Development
cd web && npm install && npm run dev

# Production build
cd web && npm run build && npm start
```

**Deploy to Vercel:** Set the root directory to `web/` in your Vercel project settings. Add `NEXT_PUBLIC_API_URL` and `NEXT_PUBLIC_API_KEY` as environment variables.

Pages: Dashboard, Generation, Sessions, Decks, Evaluations, Harness, Scheduler, Realtime, Governance, Audit, Settings.

## Near-Term Roadmap

- Add native MIDI-port output on top of the verified orchestrated OSC layer
- Add remote scheduler handoff, stronger orchestration policy, and cloud deployment on top of the verified core
- Add publishing flows, scheduler integrations, and stronger policy enforcement on external actions
- Decide whether real VST hosting belongs in this repo or in a separate host adapter

## License

This project is licensed under the MIT OR Apache-2.0 licenses. See [LICENSE](LICENSE) for details.
