# Changelog

## [Unreleased]

## [0.2.0-beta] - 2026-04-20

### Added
- **HTTP API server**: `cargo run -- http --port 3001 --api-key <key>` exposes all tools via REST with bearer token auth, CORS, and consistent JSON response envelopes.
- 32 HTTP tool endpoints covering generation, sessions, decks, harness, realtime, evaluations, governance, scheduler, audit, and creative tools.
- `GET /api/health` and `GET /api/tools` endpoints for discovery.
- **Preset patch tool**: `preset_patch` applies diff-based parameter mutations (tempo, note range, scale, duration, etc.) with automatic snapshot for rollback.
- **Parameter sweep tool**: `parameter_sweep` runs N compositions across different seeds and ranks results by trajectory dynamics.
- **Next.js web dashboard** (`web/`): 11 pages covering every SDK surface -- Dashboard, Generation, Sessions, Decks, Evaluations, Harness, Scheduler, Realtime, Governance, Audit, Settings. Deployable to Vercel.
- **Agent integration guide**: `docs/AGENT_GUIDE.md` with complete tool reference, creative workflow cookbook, and governance invariants.
- **Hermes cron template**: `docs/HERMES_TEMPLATE.md` with real job configs for nightly evaluation and exploration.
- **OpenClaw cron template**: `docs/OPENCLAW_TEMPLATE.md` with real job configs and webhook delivery.
- Orchestrated realtime dispatch: harness and scheduler can plan and execute `realtime.send_preview` and `realtime.send_transport` through real OSC adapters.
- Orchestration policy module with configurable max actions per plan, max dispatches per job run, and recursive job prevention.
- Crate-level documentation with quick-start examples for both library and HTTP usage.
- Doc comments on public types and functions in `generation.rs` and `http_server.rs`.
- 4 runnable examples: `basic_generation`, `session_workflow`, `evaluation_loop`, `http_client`.
- End-to-end HTTP integration test covering the full creative workflow.
- CI parity script (`scripts/ci-check.sh`) running Rust and Next.js checks together.
- Apache-2.0 license text, Cargo.toml metadata for crates.io.

## [0.1.0] - 2026-04-20

### Added
- Deterministic state-space system with matrix validation, prediction, discretization, controllability, and observability.
- Finite state machine with event queue, priority transitions, and condition-based filtering.
- MIDI, instrument, and effect data models.
- Deterministic audio engine with offline mono rendering from state-space systems and MIDI clips.
- Preset-backed deterministic MIDI and WAV artifact generation with explicit seeding.
- Validated VST bundle reference boundary (metadata-only, no hosting).
- Real CLI with 40+ commands covering generation, sessions, decks, harness, scheduler, realtime, governance, and audit.
- Real stdio MCP server with 50+ tools backed by the same services as the CLI.
- Dataset registry with license, provenance, use-class, and checksum metadata.
- Approval request/resolve flow with single-use expiring tokens and scope checking.
- Preset snapshot creation with SHA256 hashing and exact content rollback.
- Durable run manifests and append-only JSONL audit log for all render and governance actions.
- Durable local sessions with preset identity, seed, tempo, status, transport control, and structured event history.
- Deterministic session preview rendering with MIDI and WAV artifact export.
- Evaluation records with raw metrics, human scores, reward weights, and aggregate scoring.
- Run comparison and side-by-side review bundle construction.
- DAW-agnostic deck adapter with preview-backed clips, queue/launch/stop transport, and transport snapshots.
- Constrained agent harness with persisted plans, bounded rule-based planning for 5 roles, mediated execution, and rollback payloads.
- Immutable unattended job store with config hashing, approval-gated scheduling/cancellation, local batch execution, and exported Hermes/OpenClaw-friendly bundles.
- Realtime OSC adapter store with live preview dispatch and live transport dispatch over UDP.

### Changed
- Recovered broken mainline from duplicate definitions and overstated docs into a green, truthful baseline.
- Replaced fake VST host shell with validated bundle reference boundary.
- Rewrote README, CONFIG, and sprint docs to match actual implementation.
