---
created: 2026-04-19T15:01:20Z
branch: main
author: Codex
sprint: planning
status: active
---

# Agent Orders

This is the handoff to the implementation agent.

## Immediate Directives

1. Stop marking work packages complete until the exact verification commands pass in the current tree.
2. Do not start LLM harness work until the repository can generate real MIDI and WAV artifacts from a fresh checkout.
3. Do not use placeholder, dummy, fake, or log-only behavior on production paths.
4. Do not treat VST hosting as the critical path. It is not.
5. Interpret "live retraining" as parameter adaptation with audit logs unless a separate approved spec defines true model training.

## First Milestone

Ship one truthful end-to-end command:

`cargo run -- generate-demo --midi out/demo.mid --wav out/demo.wav`

Done means:

- the command succeeds
- the MIDI file parses with `midly`
- the WAV file parses with `hound`
- the output is deterministic under a fixed seed
- the repo is green under test, clippy, and fmt

## Required Execution Order

1. Fix the compile break in `src/audio_engine.rs` and `src/mcp.rs`.
2. Correct the audio buffer contract so stereo and mono writes are memory-safe.
3. Replace placeholder CLI behavior with real artifact generation.
4. Implement MIDI export before any DAW or scheduler integration.
5. Implement WAV export before any agent autonomy work.
6. Make CLI and library surfaces share the same backend functions.
7. Implement MCP only after the CLI path is real.
8. Add the agent harness only after CLI and MCP are real.
9. Add OpenClaw and Hermes job wrappers only after the harness can run unattended against stable commands.

## Acceptance Discipline

For every work package, record:

- the exact command that proves it works
- the exact file or transport contract it depends on
- at least one automated test
- one negative-path test

If those do not exist, the work package is not complete.

## Product Truth Rules

- README may only describe features that exist now.
- `CONFIG.md` must contain real commands and paths, not placeholders.
- sprint status must be corrected when reality changes.
- if a public example does not run, remove it or fix it in the same change.

## Architecture Rules

- The music core must be deterministic.
- Seeds must be explicit and logged.
- Agents may choose parameters and actions, but they must not bypass the core API.
- Scheduler integrations must call stable CLI or MCP interfaces, not reach into internal unstable modules.
- Fresh-session scheduled jobs must be self-contained and must not depend on hidden conversational memory.

## Hard Stops

Pause and escalate instead of improvising if:

- the next step requires introducing another stub
- the next step requires lying in docs or sprint state
- the next step depends on VST hosting to prove basic music generation
- the next step requires model training infrastructure before parameter adaptation has been delivered
