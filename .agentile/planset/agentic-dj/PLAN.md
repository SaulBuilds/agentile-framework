---
created: 2026-04-19T15:01:20Z
branch: main
author: Codex
sprint: planning
status: active
---

# Agentic DJ Plan

## Goal

Turn `state-space-music-box` into a truthful, production-minded SDK that:

- generates valid MIDI from state-space systems
- renders audible WAV output from those same systems
- exposes stable CLI and MCP control surfaces
- supports an "Agentic DJ" loop that can evaluate and adapt generation parameters
- can be orchestrated by external scheduler systems such as OpenClaw and Hermes

## Non-Goals For The First Release

- LLM weight fine-tuning
- full DAW plugin hosting
- advanced FM/additive/granular synth engines
- `no_std` release claims
- autonomous destructive actions without human approval

## External Research Basis

- OpenClaw cron supports persisted scheduled jobs, isolated sessions, and webhook or announce delivery.
  Source: https://docs.openclaw.ai/cron/
- Hermes cron uses a single `cronjob` tool, fresh per-run sessions, attached skills, and atomic job persistence.
  Source: https://hermes-agent.nousresearch.com/docs/user-guide/features/cron/
- MCP tools are exposed over JSON-RPC, usually via `stdio` or Streamable HTTP, and require lifecycle negotiation plus declared tool capabilities.
  Sources:
  https://modelcontextprotocol.io/docs/concepts/transports
  https://modelcontextprotocol.io/specification/2025-03-26/basic/lifecycle
  https://modelcontextprotocol.io/specification/2025-03-26/server/tools
- `midly` provides Standard MIDI File writing in Rust.
  Source: https://docs.rs/midly
- `hound` provides WAV writing in Rust.
  Source: https://docs.rs/hound/latest/hound/struct.WavWriter.html
- `cpal` and `rodio` provide realtime playback building blocks.
  Sources:
  https://docs.rs/cpal
  https://docs.rs/rodio

## Product Contract

The product should be delivered in layers.

### Layer 1: Deterministic Core

Input:

- state-space matrices
- simulation parameters
- mapping parameters
- optional random seed

Output:

- deterministic state trajectory
- deterministic note events
- deterministic audio samples

### Layer 2: Export And Playback

Output surfaces:

- `.mid` Standard MIDI File
- `.wav` audio file
- optional live playback

### Layer 3: Control Plane

Control surfaces:

- Rust library API
- CLI
- MCP server

### Layer 4: Agent Loop

Capabilities:

- prompt-to-parameter selection
- evaluation
- parameter adaptation
- preset memory

### Layer 5: External Orchestration

Adapters:

- OpenClaw scheduled jobs
- Hermes scheduled jobs
- CI or local batch regression jobs

## Architecture

### Core Modules

1. `state_space`
   Contract: deterministic simulation of discrete or discretized systems.
2. `music_mapping`
   Contract: translate trajectories into note, velocity, duration, and controller events.
3. `midi_export`
   Contract: write valid SMF files with tempo, track metadata, note-on, note-off, and end-of-track events.
4. `audio_render`
   Contract: render bounded PCM samples to WAV and optional live playback.
5. `presets`
   Contract: serialize generation recipes and adaptation state.
6. `mcp`
   Contract: expose safe, transport-clean tools around the real library functions.
7. `agent_harness`
   Contract: choose actions over the real tool surface and persist evaluation outcomes.

### Critical Design Rule

Agents do not generate music by improvising around fake backends.

Agents must only operate against:

- deterministic library functions
- tested CLI commands
- verified MCP tools

## Chronological Sprint Sequence

## Sprint A: Recovery And Truth Alignment

Purpose:

Return the repo to a green, honest baseline.

Work:

- remove duplicate definitions and broken edits
- restore `cargo test`, `cargo clippy`, and `cargo fmt --check`
- eliminate placeholder claims in sprint and README artifacts
- define real commands in `CONFIG.md`

Hard acceptance criteria:

- `cargo test` passes
- `cargo clippy --all-targets --all-features -- -D warnings` passes
- `cargo fmt --check` passes
- `rg -n "TODO|FIXME|HACK|STUB|placeholder|dummy|mock|fake" src` returns zero acceptable production hits
- README examples compile or are removed
- `CURRENT.md`, `SPRINT.md`, and `CONFIG.md` match reality exactly

## Sprint B: MIDI-First SDK

Purpose:

Make the library produce actual music structure, even if the output is still musically primitive.

Work:

- add deterministic simulation settings with explicit seed handling
- define `TrajectoryToMidiMapper`
- add tempo, ticks-per-beat, scale, quantization, and note-range controls
- export `.mid` via `midly`
- add a CLI command that writes a MIDI file from a built-in example system

Data sources and contracts:

- source of dynamics: `StateSpaceSystem`
- source of musical policy: mapping config and preset files
- output contract: Standard MIDI File written by `midly`

Hard acceptance criteria:

- `cargo run -- generate-midi --preset demo --output out/demo.mid` succeeds on a fresh checkout
- `midly::Smf::parse()` can parse the generated file
- the file contains at least one track name event, one tempo event, one note-on event, one note-off event, and one end-of-track event
- repeated runs with the same seed produce byte-identical MIDI files
- different seeds produce different note sequences while staying within configured pitch and velocity bounds
- no note has negative duration, zero duration, or pitch outside `0..=127`

## Sprint C: WAV Rendering And Audibility

Purpose:

Make the project audibly useful out of the box without waiting for VST integration.

Work:

- implement a minimal built-in synth path
- map note events or state trajectories to bounded PCM samples
- export `.wav` via `hound`
- add optional playback through `cpal` or `rodio`

Data sources and contracts:

- source of synthesis parameters: preset file or CLI flags
- output contract: valid WAV file readable by `hound`

Hard acceptance criteria:

- `cargo run -- generate-audio --preset demo --output out/demo.wav` succeeds on a fresh checkout
- `hound::WavReader::open("out/demo.wav")` succeeds
- file duration is within 1 percent of requested duration
- channel count, sample rate, and sample format match CLI arguments
- peak amplitude stays within configured safety bound
- generated file is not silent for the whole duration
- repeated runs with the same seed produce identical PCM samples

## Sprint D: Real CLI And MCP Control Plane

Purpose:

Expose the working SDK through stable interfaces.

Work:

- replace placeholder CLI flows with real commands
- implement a transport-clean MCP server over `stdio` first
- add tools for create-system, create-preset, generate-midi, generate-audio, inspect-trajectory, and list-presets
- return structured tool errors instead of log-only failures

Data sources and contracts:

- transport contract: MCP JSON-RPC lifecycle and tool calling
- artifact contract: generated files and structured in-memory results

Hard acceptance criteria:

- CLI help text matches real commands
- the MCP server completes `initialize`, `notifications/initialized`, `tools/list`, and `tools/call` correctly
- the server writes only MCP messages to `stdout`
- one MCP tool call can generate a MIDI file and return its path plus summary metadata
- one MCP tool call can generate a WAV file and return its path plus summary metadata
- integration tests cover CLI and MCP happy path plus one error path per tool

## Sprint E: Agentic DJ Harness

Purpose:

Add an agent layer that manipulates real music primitives instead of imaginary ones.

Work:

- define an action space over presets, mappings, seeds, and render requests
- persist run logs, evaluation metrics, and user feedback
- implement parameter adaptation, not LLM weight retraining
- add human-in-the-loop approvals for risky or external actions

Data sources and contracts:

- source of reward signal: objective metrics plus optional human ratings
- source of memory: persisted run records and preset history

Hard acceptance criteria:

- given the same prompt, preset, and seed, the harness produces the same action plan
- every generation run is logged with prompt, seed, preset hash, output artifacts, and evaluation metrics
- a failed generation never mutates the active preset without an explicit rollback record
- the harness can rank at least three candidate outputs for one prompt using deterministic metrics
- user feedback can alter future parameter selection without changing LLM weights

## Sprint F: Scheduler Adapters For OpenClaw And Hermes

Purpose:

Make unattended evaluation and overnight experimentation operational.

Work:

- add CLI workflows that scheduler jobs can invoke safely
- provide example job specs for OpenClaw and Hermes
- isolate scheduled runs from live interactive sessions

Data sources and contracts:

- OpenClaw scheduler contract: persisted jobs with main or isolated sessions
- Hermes scheduler contract: `cronjob` tool or `hermes cron` CLI with fresh sessions

Hard acceptance criteria:

- one documented OpenClaw job can run nightly evaluation against the CLI without manual intervention
- one documented Hermes job can run nightly evaluation against the CLI without manual intervention
- scheduler jobs write results to timestamped local artifacts
- scheduled runs cannot recursively schedule more jobs from inside the run
- scheduled runs never require chat-history memory to succeed

## Sprint G: Hardening And Release

Purpose:

Turn the working prototype into a production-minded release candidate.

Work:

- property-based tests for mapping and invariants
- audio safety limits and clipping tests
- preset schema versioning
- reproducibility report
- release packaging and examples

Hard acceptance criteria:

- full test suite passes
- property-based tests cover mapping invariants and serialization boundaries
- release build succeeds
- generated demo artifacts are included or reproducibly regenerable
- README setup instructions work from a fresh clone
- no public claim in README exceeds verified functionality

## Hard Global Acceptance Criteria

The project is not release-ready until all of these are true:

1. Fresh checkout, one command, actual MIDI and WAV artifacts.
2. Same seed, same output.
3. Valid files parse with independent libraries.
4. No placeholder, dummy, or fake behavior on the production path.
5. CLI and MCP are both backed by the same real library functions.
6. Agent actions are logged, reviewable, and reversible.
7. External schedulers invoke the product; they do not contain the product logic.

## Clarification On "Live Retraining"

For this program, use the term carefully.

Allowed in MVP:

- parameter search
- preset adaptation
- score-based selection
- reward logging

Not allowed in MVP:

- silent model fine-tuning
- hidden online weight updates
- irreversible learning without audit logs
