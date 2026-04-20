---
created: 2026-04-19T15:01:20Z
branch: main
author: Codex
sprint: sprint-1-foundation
status: active
---

# Findings

## F-001: BLOCKER - Mainline build is red

Evidence:

- `cargo test` fails with duplicate `new` and `start` definitions in [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:24) and [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:45).
- `cargo test` fails with duplicate `new` definitions in [src/mcp.rs](/home/saul/Projects/hermes/agentile-framework/src/mcp.rs:37) and [src/mcp.rs](/home/saul/Projects/hermes/agentile-framework/src/mcp.rs:51).
- The same failure blocks `cargo clippy --all-targets --all-features -- -D warnings`.

Impact:

- No production claim is trustworthy until the repo is back to green.
- Current sprint completion claims are invalidated by the failing verification gate.

## F-002: BLOCKER - The user-facing CLI and MCP surfaces are still placeholders

Evidence:

- The `mcp` command logs `"MCP server started (placeholder)"` instead of starting a real server in [src/cli.rs](/home/saul/Projects/hermes/agentile-framework/src/cli.rs:68).
- The `generate` command does not save or play audio; it only logs that it "would" do so in [src/cli.rs](/home/saul/Projects/hermes/agentile-framework/src/cli.rs:91).
- The example commands only print placeholder text in [src/cli.rs](/home/saul/Projects/hermes/agentile-framework/src/cli.rs:126).
- `start_mcp_server()` only initializes tracing and logs startup in [src/mcp.rs](/home/saul/Projects/hermes/agentile-framework/src/mcp.rs:63).

Impact:

- The project does not currently produce music out of the box.
- The project does not currently provide a working MCP server for agent control.

## F-003: BLOCKER - The audio generation path is unsafe and mathematically unsound for the intended contract

Evidence:

- The audio buffer is allocated as `num_samples` but written as if it were interleaved stereo in [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:81) and [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:114).
- For mono output, the code still writes `audio_buffer[n * 2]` and `audio_buffer[n * 2 + 1]`, which will run out of bounds before loop completion in [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:116).
- The function injects fresh random input on every sample in [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:97), which makes output non-reproducible.
- The function directly applies `A*x + B*u` as a sample-to-sample state update even when the model semantics are continuous-time, bypassing proper integration and the existing `predict()` contract in [src/audio_engine.rs](/home/saul/Projects/hermes/agentile-framework/src/audio_engine.rs:103).

Impact:

- Even after the compile break is fixed, this path is not a safe foundation for realtime or offline rendering.
- It is not suitable for evaluation, regression testing, or agent-loop optimization.

## F-004: HIGH - Production code still contains stubbed backend behavior

Evidence:

- The VST layer explicitly says "In a real implementation" and stores only metadata in [src/vst_synthesizer.rs](/home/saul/Projects/hermes/agentile-framework/src/vst_synthesizer.rs:21).
- `load()` unconditionally marks the plugin as loaded in [src/vst_synthesizer.rs](/home/saul/Projects/hermes/agentile-framework/src/vst_synthesizer.rs:31).
- `get_parameter()` returns a dummy `0.5` in [src/vst_synthesizer.rs](/home/saul/Projects/hermes/agentile-framework/src/vst_synthesizer.rs:81).
- The sprint record explicitly treats dummy implementations as part of completion in [.agentile/sprints/active/sprint-1-foundation/SPRINT.md](/home/saul/Projects/hermes/agentile-framework/.agentile/sprints/active/sprint-1-foundation/SPRINT.md:157).

Impact:

- This violates the repo's own no-stubs rule for production code.
- VST hosting is currently a documentation claim, not a delivered capability.

## F-005: HIGH - Governance and product docs drift away from reality

Evidence:

- The README claims FM, additive, and granular synthesis, visualization tools, and `no_std` support in [README.md](/home/saul/Projects/hermes/agentile-framework/README.md:16), none of which are present in the audited code.
- The README usage example references `Synthesizer` and `AudioOutput`, which do not exist in the library surface shown by [src/lib.rs](/home/saul/Projects/hermes/agentile-framework/src/lib.rs:1).
- `.agentile/CONFIG.md` still contains starter placeholders for counterparts, commands, workspace map, delivery surfaces, and sensitive areas in [.agentile/CONFIG.md](/home/saul/Projects/hermes/agentile-framework/.agentile/CONFIG.md:19).
- Sprint WP-7 is marked complete despite placeholder CLI/MCP behavior in [.agentile/sprints/active/sprint-1-foundation/SPRINT.md](/home/saul/Projects/hermes/agentile-framework/.agentile/sprints/active/sprint-1-foundation/SPRINT.md:260).
- `CURRENT.md` reports 23 tests while source inspection currently finds 18 test functions in [.agentile/sprints/CURRENT.md](/home/saul/Projects/hermes/agentile-framework/.agentile/sprints/CURRENT.md:16).

Impact:

- Future contributors are being routed by stale or incorrect project truth.
- The sprint file is no longer a reliable source of truth until corrected.

## F-006: HIGH - The current plan jumps too early to agentic orchestration

Evidence:

- The user's desired future system depends on working MIDI export, stable MCP contracts, and deterministic rendering, none of which are complete.
- The current codebase has no real MIDI file export path and no transport-safe MCP tool implementation.
- OpenClaw and Hermes scheduler features are designed to invoke existing skills, prompts, or tools, not to replace core product implementation.

Impact:

- If the team jumps to LLM harness work now, it will optimize prompts around a fake or unstable backend.
- This would amplify the repo's current truthfulness gap rather than closing it.
