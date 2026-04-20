---
created: 2026-04-19T15:32:26Z
branch: main
author: Codex
sprint: planning
status: active
---

# Senior Baseline

## Purpose

This document is the senior-engineer baseline for the current repository and the planned "Agentic DJ" direction.

It does not replace the earlier plan drafts in this directory.
It clarifies what is actually true today, what is only partially true, and what is still aspirational.

## Current State Snapshot

### Baseline A: Current Working Tree

Status: red

Observed facts:

- `cargo test` fails in the current workspace because `src/audio_engine.rs` and `src/mcp.rs` contain duplicate `new()` definitions.
- `cargo clippy --all-targets --all-features -- -D warnings` fails for the same reason.
- `cargo fmt --check` fails because the current working tree is not syntactically valid.

Implication:

- The workspace the next contributor opens is not currently safe to extend.

### Baseline B: Committed `main` HEAD (`441c682`)

Status: red

Observed facts:

- `main` is also broken, even without the extra local duplicate blocks.
- `src/audio_engine.rs` contains malformed braces and references to non-existent fields.
- A fresh archive of `HEAD` fails `cargo test`, `cargo clippy`, and `cargo fmt --check`.

Implication:

- The committed baseline is not releasable and not even parse-clean.

### Baseline C: Previous Commit (`39c2ad7`)

Status: mixed

Observed facts:

- `cargo test` passes with 18 tests.
- `cargo clippy --all-targets --all-features -- -D warnings` fails.
- The runtime surface is still mostly scaffolding and placeholder behavior.

Implication:

- The repo briefly reached "tests green but not production-ready."
- That earlier state is the most useful short-term recovery target.

## What The Code Actually Delivers

Delivered in some form:

- `StateSpaceSystem` with dimension checks, prediction, output, controllability, and observability helpers.
- `MidiModel`, `InstrumentModel`, `EffectModel`, and `StateMachine` as in-memory data structures.
- A CLI shell with subcommands and a validation path.
- An MCP state container type.

Not truly delivered yet:

- MIDI file export.
- WAV export.
- Live playback.
- MCP transport, lifecycle, and tool calling.
- Real VST loading or parameter control.
- Deterministic offline rendering.
- Realtime DAW integration.
- Any production-safe "agent harness."

## Truthfulness Review Of Prior Work

The earlier audit and planset in this directory are directionally good.

What they got right:

- The project should not jump straight to LLM orchestration.
- MIDI and WAV need to exist before MCP and scheduler automation matter.
- "Live retraining" should be interpreted as parameter adaptation, not model weight training.
- OpenClaw and Hermes are orchestration layers, not replacements for the product runtime.

What needs correction or sharpening:

- The baseline must distinguish current local workspace, committed `main`, and the last buildable commit.
- The repo is not just "missing features"; it currently fails basic integrity gates.
- The existing plan does not yet pin down the realtime integration path for DAWs.
- Production acceptance needs stronger gates around determinism, provenance, rollback, observability, and human approval boundaries.

## Architectural Direction That Best Fits The Codebase

The intended product should be reframed as:

"A deterministic state-space music SDK with artifact generation, realtime control surfaces, MCP tooling, and an agent layer that selects and adapts parameters over a real backend."

That implies the following order:

1. Recover a truthful, green repository.
2. Build deterministic trajectory simulation and mapping.
3. Export valid MIDI and WAV artifacts.
4. Add a real CLI and MCP control plane over the same backend.
5. Add a constrained agent harness with logging, evaluation, rollback, and human approvals.
6. Add scheduler adapters for Hermes and OpenClaw.
7. Add optional realtime DAW bridges and only later consider plugin hosting.

## Recommended DAW Strategy

Do not make VST hosting the critical path.

Use three phases instead:

1. Offline DAW interoperability:
   MIDI and WAV export that any DAW can import.
2. Realtime DAW control:
   virtual MIDI ports plus OSC or another control protocol for live parameter updates.
3. Plugin packaging:
   only after the core engine and control plane are stable.

This is lower risk, easier to test, and much closer to how an "Agentic DJ" can become useful quickly.

## Research-Constrained Conclusions

The current external ecosystem supports the direction above:

- MCP expects a real JSON-RPC tool server with proper lifecycle negotiation and declared tool capabilities.
- Hermes cron is useful for scheduled evaluation or generation jobs because runs execute in fresh sessions.
- OpenClaw cron is useful for persisted scheduled tasks, isolated runs, and result delivery.
- `rmcp` now provides an official Rust SDK path for MCP server implementation.
- `midly` and `hound` are appropriate building blocks for deterministic MIDI and WAV generation.

## Hard Product Boundary

Until this repo can produce one valid `.mid` file and one valid `.wav` file from a fresh checkout with a fixed seed, it is not ready for claims about:

- agentic DJs
- live retraining
- DAW automation
- MCP-powered composition
- production readiness

## Immediate Questions For The Human Team

These are the highest-value product questions still open:

1. Which DAW is the first-class target, if any: Ableton, REAPER, Bitwig, Logic, or "DAW-agnostic"?
2. Is the first live loop expected to be MIDI-only, audio-only, or both?
3. Should the first agent harness optimize for user taste, system-theoretic objectives, or both?
4. What actions must always require human approval: file writes, preset promotion, scheduler creation, external process launch, or DAW control?
5. Is the first release local-only, or do you want remote MCP/HTTP from day one?
