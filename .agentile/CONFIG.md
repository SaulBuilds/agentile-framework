---
created: 2026-04-19T16:17:14Z
branch: main
author: codex
status: active
---

# Project Configuration

This file is the canonical source for repo-specific facts. When commands, names, paths, or lifecycle assumptions disagree, this file wins.

## Identity

| Key | Value |
|-----|-------|
| Project Name | state-space-music-box |
| Repository Purpose | Build a Rust library and local-first tooling stack that turns state-space models into musical artifacts and eventually powers Agentic DJ workflows |
| Primary Domain | SDK / local-first music tooling prototype |
| Current Phase | BETA_RELEASED |
| Primary Users | Rust developers, creative tool builders, and future human-in-the-loop agent operators |

## Collaboration Context

| Key | Value |
|-----|-------|
| Human Counterparts | Saul and collaborating coding agents working through Agentile sprint artifacts |
| Preferred Work Mode | PLANNED_SPRINTS |
| Definition Of Done | Code is build-green, tests/clippy/fmt pass, sprint artifacts are updated, user-facing docs are truthful, and no new scaffolding is introduced without an explicit follow-up plan |
| Review Standard | Changes touching security, approvals, remote execution, publishing, or training/adaptation logic require explicit review before merge |

## Technology Stack

| Layer | Technology |
|-------|------------|
| Primary Languages | Rust |
| Frameworks / Runtimes | `nalgebra`, `serde`, `clap`, `tracing`, `tokio`, `rmcp`, `midly`, `hound` |
| Package / Build Tooling | Cargo |
| Storage / State | In-memory model state and JSON-serializable data structures |
| Test Tooling | `cargo test`, `proptest`, `approx` |
| Lint / Static Analysis | `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check` |
| Formal Verification | Invariant and regression tests today; formal specs are planned for critical agent/state-machine, approval, and agent-policy flows |

## Core Commands

```bash
# Bootstrap / install
cargo build

# Build
cargo build

# Test
cargo test

# Lint / typecheck
cargo clippy --all-targets --all-features -- -D warnings

# Format
cargo fmt --check

# Run locally
cargo run -- generate-demo --midi out/demo.mid --wav out/demo.wav --seed 1

# Request and resolve a local approval
cargo run -- approval-request --action-scope dataset.register --target pdmx-demo --requested-by local-dev --reason "register dataset"

# Inspect recent runtime records
cargo run -- run-list
cargo run -- audit-list --limit 10

# Create and score a local session
cargo run -- session-create --display-name "First Session" --preset demo --actor-id local-dev
cargo run -- session-play --session-id session-123 --actor-id local-dev --run-label rehearsal-pass
cargo run -- session-render-preview --session-id session-123 --actor-id local-dev
cargo run -- deck-create --display-name "Deck A" --session-id session-123 --actor-id local-dev
cargo run -- deck-transport --deck-id deck-123
cargo run -- harness-plan --role session-dj --prompt "set tempo to 132 and render a preview" --session-id session-123
cargo run -- realtime-create --display-name "Loopback" --host 127.0.0.1 --port 9000 --base-path /agentic_dj
cargo run -- realtime-send-preview --adapter-id adapter-123 --session-id session-123 --preview-id preview-123 --actor-id local-dev --dispatch-mode immediate --time-scale 0
cargo run -- review-build --run-id run-123 --run-id run-456 --output comparison.json

# CI parity command
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test
```

## Workspace Map

| Area | Path | Purpose |
|------|------|---------|
| Core library | `src/` | State-space math, preset-backed generation, audio rendering, CLI, MCP server, governance services, runtime manifests, audit logs, and VST bundle validation |
| Governance | `.agentile/` | Sprint records, rules, templates, plansets, audits, and coverage tracking |
| Coverage tracking | `.agentile/coverage/` | Test baselines and quality gates |
| Active sprint | `.agentile/sprints/active/sprint-11-sdk-and-http/` | Canonical record of the SDK polish and HTTP transport milestone |
| Project docs | `README.md`, `SPIRIT.md` | Public project description and collaboration norms |

## Delivery Surfaces

| Surface | Interface | Primary Contract |
|---------|-----------|------------------|
| Core SDK | LIBRARY | State-space models and related music structures must remain deterministic, validated, and well-tested |
| Local CLI | CLI | Commands must generate real artifacts or return real deterministic summaries backed by the shared generation core |
| MCP integration | CONTRACT | The stdio MCP server must expose real tools over the same backend as the CLI and keep protocol traffic off `stderr` |
| Local governance | FILE_AND_CONTRACT | Dataset registration, approval tokens, and rollback surfaces must persist durable machine-readable records and fail closed on invalid or unauthorized mutations |
| Local provenance | FILE_AND_CONTRACT | Render and governance actions must persist durable manifests and append-only audit events that can be inspected without mutating runtime state |
| Local session and evaluation | FILE_AND_CONTRACT | Session mutations, preview renders, and review surfaces must persist durable records linked to real run manifests and must keep raw scores separate from computed reward summaries |
| Local deck control | FILE_AND_CONTRACT | Decks must persist loaded clips, queue state, active clip state, and transport snapshots linked back to session preview artifacts |
| Local harness | FILE_AND_CONTRACT | Harness plans and outcomes must persist deterministic signatures, bounded action proposals, execution results, and rollback payloads over the real backend services |
| Local scheduler | FILE_AND_CONTRACT | Stored jobs must keep immutable configs, approval-linked scheduling, local batch execution, and exported adapter bundles for external schedulers |
| Local realtime bridge | FILE_AND_CONTRACT | Realtime adapters must persist endpoint configs, dispatch logs, and emit live OSC packets from real preview and deck data |
| Agentic DJ planset | OTHER | Architecture, security, and deployment plans must stay aligned with the real codebase baseline |

## Sensitive Areas

| Area | Path / Topic | Why Sensitive |
|------|---------------|---------------|
| State evolution math | `src/state_space.rs` | Numerical correctness and deterministic behavior |
| Audio rendering | `src/audio_engine.rs` | User-facing artifact quality and future realtime stability |
| State transitions | `src/state_machine.rs` | Critical control-flow behavior for future agents and automation |
| Agent execution surfaces | `src/mcp.rs`, future DAW/agent harness code | Remote execution, tool safety, and authorization boundaries |
| Governance state | `src/governance/`, `.agentile/runtime/` | License provenance, approval consumption, and rollback integrity must remain trustworthy |
| Security and deployment planning | `.agentile/planset/agentic-dj/` | Approvals, publishing, training, and cloud deployment design |

## Naming Conventions

- Canonical names to use: `state-space-music-box`, `StateSpaceSystem`, `AudioEngine`, `Agentic DJ`, `sprint-10-orchestrated-realtime`
- Deprecated or confusing names to avoid: generic "starter" language, claims that MCP/VST/audio export are already production-ready
- Branch scopes commonly used in commits: `core`, `audio`, `cli`, `mcp`, `docs`, `governance`, `audit`, `sprint`, `security`

## Notes

- The repo now has a deterministic artifact-generation core, matching CLI and MCP surfaces, a local governance layer, durable provenance records, durable session/evaluation records, honest local live-control plus review surfaces, a DAW-agnostic deck adapter, a constrained harness contract, an unattended job layer, a local OSC bridge, and orchestrated realtime dispatch through the harness and scheduler with policy enforcement, but it is not yet a full remote orchestration runtime.
- The next implementation milestone is native MIDI port output, remote scheduler integration, and stronger cloud deployment hardening on top of the verified orchestrated realtime baseline.

## Changelog

### 2026-04-19

- Replaced starter placeholders with actual project facts, commands, surfaces, and risks.
- Recorded the deterministic-foundation phase and the current boundary between the shipped offline core and future Agentic DJ work.
- Updated the phase, stack, and delivery surfaces to reflect real MIDI/WAV generation and the shipped stdio MCP server.
- Recorded the governance-control phase after shipping the dataset registry, approval token flow, and preset snapshot/rollback surfaces.
- Recorded the provenance-and-audit phase after shipping run manifests, append-only audit events, and inspection surfaces for runtime records.
- Recorded the session-and-evaluation phase after shipping durable session records, run comparison, and evaluation submissions.
- Recorded the live-control-and-review phase after shipping session play/stop control, deterministic preview renders, evaluation inspection, and review bundle construction.
- Recorded the DAW-control-adapter phase after shipping deck creation, preview clip loading, queue/launch/stop flows, and transport inspection.
- Recorded the agent-harness phase after shipping persisted plans, persisted outcomes, bounded live patch execution, and matching CLI/MCP surfaces.
- Recorded the scheduler-adapter phase after shipping immutable job configs, local batch entrypoints, approval-gated scheduling and cancellation, and Hermes/OpenClaw-friendly exported bundles.
- Recorded the realtime-adapter phase after shipping persisted OSC adapter configs, live preview dispatch, live transport dispatch, and matching CLI/MCP surfaces.
