---
created: 2026-04-19T15:32:26Z
branch: main
author: Codex
sprint: planning
status: active
---

# Implementation Plan V2

## Product Vision

Build a deterministic music engine that turns state-space systems into musical artifacts and live control events, then layer a constrained LLM agent on top so the agent can act like an "Agentic DJ" through real tools instead of invented behavior.

The finished system should support:

- deterministic state simulation
- deterministic MIDI generation
- bounded WAV rendering
- realtime parameter mutation through a control plane
- MCP tool access for agents and IDEs
- scheduled evaluation and unattended runs through Hermes and OpenClaw
- human-in-the-loop approvals for risky actions

## Product Decisions Locked In

These decisions are now treated as active planning constraints:

- DAW strategy: DAW-agnostic, with an in-house simple DAW interface rather than a first-party dependency on one external DAW
- First demo scope: include a thin vertical slice of all major modes, not a deep implementation of only one mode
- Reward model: hybrid, combining human judgment and automatic system/music metrics
- Security posture: explicit approvals, authorization boundaries, audit logging, and secured publishing/training data flows
- Deployment path: local-first for development and validation, then a hardened DigitalOcean Droplet deployment

## Companion Specs

The following documents are now part of the active planset and should be treated as implementation inputs, not optional notes:

- `PRODUCTION_FEATURE_SPECS.md`
- `AGENT_HARNESS_SPEC.md`
- `EVALUATION_AND_DATASETS.md`

Use them as follows:

- `PRODUCTION_FEATURE_SPECS.md`: feature-level Gherkins and hard acceptance criteria
- `AGENT_HARNESS_SPEC.md`: role, prompt, tool, audit, and failure contracts
- `EVALUATION_AND_DATASETS.md`: evaluation workbench requirements and dataset procurement policy

## Explicit Non-Goals For The First Production Release

- foundation-model fine-tuning
- self-modifying prompts without audit logs
- VST hosting as a dependency for core value
- unsupported README claims about synthesis families or `no_std`
- "magic" agent behavior that bypasses the SDK

## Runtime Model

### Layer 1: Deterministic Core

Responsibilities:

- discretize or simulate the state-space system
- emit trajectories
- convert trajectories into note and control events
- render note events into bounded PCM

Source of truth:

- library functions only

### Layer 2: Artifact And Live I/O

Responsibilities:

- write `.mid`
- write `.wav`
- emit realtime MIDI
- expose a live control surface for parameter updates

Source of truth:

- services that call the deterministic core

### Layer 3: Control Plane

Responsibilities:

- stable Rust API
- stable CLI
- stable MCP server

Source of truth:

- thin wrappers over the same application services

### Layer 4: Agent Harness

Responsibilities:

- translate prompt and context into tool calls
- evaluate outputs
- adapt parameters
- record provenance and rollback history

Source of truth:

- tool calls over the real control plane

### Layer 5: Orchestration

Responsibilities:

- batch evaluation
- overnight sweeps
- scheduled rendering
- promotion workflows

Source of truth:

- stable CLI commands first, MCP tools second

## First Demo Definition

The first demo should prove the whole concept in one coherent local experience.

It should include all of the following in thin but real form:

1. a simple local DAW-like interface with transport controls, pattern slots or clips, parameter panels, and a timeline or scene view
2. an agent chat or instruction surface that can mutate generation parameters and request renders
3. deterministic MIDI and WAV generation from the same backend
4. realtime note or control output through the live control surface
5. an evaluation loop that combines automatic metrics and human rating input
6. a parameter adaptation step that proposes or applies the next state
7. an approval gate for sensitive actions such as publishing, scheduler creation, or preset promotion

Important constraint:

The first demo must be broad but shallow. It is a vertical slice proving the end-to-end product shape, not the full production depth of every subsystem.

## Chronological Delivery Plan

## Sprint 0: Recovery

Goal:

Return the repository to a truthful and buildable state.

Scope:

- recover the cleanest known baseline from `39c2ad7`
- repair `main`
- remove malformed or duplicate edits
- make test, clippy, and fmt all pass together
- correct false sprint and config claims

Hard acceptance criteria:

- `cargo test` passes on `main`
- `cargo clippy --all-targets --all-features -- -D warnings` passes
- `cargo fmt --check` passes
- `README.md`, `.agentile/CONFIG.md`, `.agentile/sprints/CURRENT.md`, and sprint status all match reality
- no public command claims to save audio, start an MCP server, or load VSTs unless it actually does

## Sprint 1: Deterministic Music Core

Goal:

Make state evolution and mapping deterministic and testable.

Scope:

- define explicit simulation config
- define explicit seed handling
- separate continuous-time integration from discrete stepping
- add a `Trajectory` model
- add a `TrajectoryToMidiMapper`

Hard acceptance criteria:

- the same system plus the same config plus the same seed produces bit-for-bit identical trajectory snapshots
- there is at least one golden trajectory fixture committed to the repo
- property tests cover dimension mismatches and bounded output invariants
- the audio path no longer uses ambient randomness
- mono and stereo buffer writes are memory-safe and unit-tested

## Sprint 2: Artifact Generation

Goal:

Produce importable music artifacts from a fresh checkout.

Scope:

- add `.mid` export through `midly`
- add `.wav` export through `hound`
- implement one built-in synth path
- add one demo preset and one demo command

Hard acceptance criteria:

- `cargo run -- generate-demo --midi out/demo.mid --wav out/demo.wav --seed 1` succeeds
- `midly::Smf::parse()` accepts the generated MIDI
- `hound::WavReader::open()` accepts the generated WAV
- the same seed yields byte-identical MIDI and sample-identical WAV
- a different seed changes the result while staying inside configured musical bounds
- the demo artifacts can be imported into at least one target DAW during manual verification

## Sprint 3: Realtime Control Surface

Goal:

Give the engine a credible live path and a minimal in-house DAW interface without depending on VST hosting.

Scope:

- add realtime MIDI output support
- add a live control protocol for parameter mutation
- prefer virtual MIDI plus OSC over plugin hosting
- add state snapshot and restore
- build the first simple DAW-like local interface over the same backend

Hard acceptance criteria:

- a local live session can send notes or control changes without restarting the process
- live parameter mutations are reflected in subsequent renders
- every mutation is timestamped and logged with before/after values
- a snapshot can restore the engine to a prior known state
- the live path has one soak test for sustained operation without panic or deadlock
- the local DAW interface can load a demo preset, trigger playback or render, and display the active clip, scene, or transport state

## Sprint 4: CLI And MCP

Goal:

Expose the real backend to humans, IDEs, and agents.

Scope:

- stabilize CLI verbs around generate, inspect, mutate, save, and evaluate
- implement MCP over `stdio` first
- use `rmcp` unless a stronger reason emerges not to
- add structured errors and schema-stable responses

Hard acceptance criteria:

- every CLI command maps to one backend service function
- the MCP server completes `initialize`, `tools/list`, and `tools/call` against the real backend
- `stdout` is reserved for MCP messages only
- there is an integration test for each public MCP tool
- there is at least one MCP inspector or protocol-level verification step in CI or the sprint report

## Sprint 5: Agentic DJ Harness

Goal:

Make the agent operate as a constrained musical controller and evaluator.

Scope:

- define the action space
- define allowed tools
- define evaluation metrics
- define parameter adaptation logic
- define human approval checkpoints

Hard acceptance criteria:

- the harness cannot mutate active presets without creating an audit record
- every run persists prompt, tool calls, seed, preset hash, outputs, metrics, and final disposition
- the harness can re-run a prior session from its saved provenance record
- risky actions require explicit approval according to policy
- there is a negative-path test showing the harness cannot bypass the core API
- the evaluator records both machine metrics and human feedback in one run record
- the adaptation step can explain which metrics or feedback caused a proposed change

## Sprint 6: Scheduler Adapters

Goal:

Run unattended evaluations and render sweeps against the stable product.

Scope:

- add documented CLI entrypoints for batch jobs
- add Hermes cron examples
- add OpenClaw cron examples
- add artifact retention and run manifests

Hard acceptance criteria:

- one Hermes scheduled job can execute a nightly evaluation against the CLI in a fresh session
- one OpenClaw scheduled job can execute a nightly render or evaluation run and persist results
- scheduled jobs are idempotent for the same manifest input
- every job writes timestamped artifacts plus a machine-readable run summary
- scheduler integrations do not depend on hidden chat context

## Production Gates

The project is not production-ready until all of the following are true:

- build gate: test, clippy, fmt, and release build all pass
- truth gate: README, CONFIG, sprint status, and examples reflect the actual product
- determinism gate: fixed-seed runs reproduce artifacts and evaluations
- safety gate: audio is amplitude-bounded and panic-free under expected inputs
- provenance gate: prompts, seeds, presets, tool calls, and outputs are logged
- rollback gate: parameter adaptation is reversible
- interface gate: CLI and MCP both call the same backend services
- scheduler gate: unattended jobs are explicit, isolated, and auditable
- approval gate: risky external actions require a human checkpoint

## Security And Authorization Model

The system should use a deny-by-default posture for high-impact actions.

Control classes:

- read-only actions: inspect trajectories, list presets, preview metrics
- reversible creative actions: render, regenerate, audition, propose parameter updates
- gated state-changing actions: promote preset, publish output, create scheduler job, enable remote access, change authorization policy

Required controls:

- local authn/authz model even in single-user mode so cloud deployment does not require an architectural rewrite
- per-action policy checks before publishing, training-data persistence, remote control, or scheduler mutation
- append-only audit logs for tool calls, approvals, preset mutations, and publish events
- secret isolation for tokens, webhooks, and publishing credentials
- signed or hashed artifact manifests for outputs promoted beyond local scratch space
- explicit provenance for any data retained for training or adaptation

Human approval is mandatory for:

- publishing
- scheduler creation or modification
- remote live control
- preset promotion to shared or production namespaces
- any future training dataset export

## Deployment Path

### Phase 1: Local-First

- single-user local process
- local CLI
- local MCP over `stdio`
- local simple DAW interface
- local artifact storage and audit logs

### Phase 2: Hardened Droplet

- single DigitalOcean Droplet first
- SSH keys only, no password auth
- non-root operator user
- DigitalOcean Cloud Firewall with least-privilege inbound rules
- localhost bind for internal-only services unless explicitly published
- HTTPS and authenticated remote entrypoints only where needed
- backups, snapshots, monitoring agent, and alerts enabled

### Phase 3: Split Roles If Needed

- separate control plane and render worker
- isolated scheduler worker
- object storage or volume strategy for retained artifacts

## Recommended Technical Choices

Use now:

- `midly` for Standard MIDI File writing
- `hound` for WAV writing
- `rmcp` for Rust MCP server work
- virtual MIDI plus OSC for early DAW interoperability
- a small local GUI shell for the in-house DAW interface

Use later:

- richer synthesis engines
- VST plugin packaging or hosting
- advanced ranking or bandit-style adaptation
- remote transports beyond local `stdio`

## Why This Plan Matches The Research

- MCP now has stable guidance for transports, lifecycle, and tool contracts, so the control plane should be implemented as a real protocol surface, not an ad hoc logger.
- Hermes cron runs jobs in fresh sessions, which is a strong fit for reproducible nightly evaluations.
- OpenClaw cron is well-suited to persisted job definitions and isolated execution for sweeps or scheduled generations.
- The simplest reliable DAW bridge is still import/export plus live MIDI and OSC, not immediate plugin hosting.
