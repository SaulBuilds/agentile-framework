---
created: 2026-04-19T15:01:20Z
branch: main
author: Codex
sprint: planning
status: active
---

# ADR-001: Build A Deterministic Music Core Before The Agent Layer

## Status

Accepted for planning.

## Context

The project goal has expanded from state-space sonification into an "Agentic DJ" system.

The current codebase does not yet provide a trustworthy base runtime:

- the build is red
- the CLI contains placeholder behavior
- the MCP server is not implemented
- the audio path is unsafe
- the VST layer is stubbed

At the same time, the desired future system needs:

- agent control
- scheduled automation
- evaluation loops
- possible overnight experimentation via OpenClaw or Hermes

If the project adds orchestration before the core music contracts are real, the agent will optimize against placeholder behavior and the repo will accumulate false confidence faster than it accumulates working software.

## Decision

The project will enforce the following order:

1. deterministic simulation and mapping
2. valid MIDI and WAV outputs
3. stable CLI
4. stable MCP server
5. agent harness
6. external scheduler adapters

Additional boundary decisions:

- The first real synthesis path will be a built-in renderer, not VST hosting.
- "Live retraining" in MVP means parameter adaptation with audit logs, not foundation-model fine-tuning.
- OpenClaw and Hermes will be treated as external orchestration systems that call the product, not as places where core music logic lives.
- MCP will target `stdio` first because it is the simplest compliant transport for local agent integration.

## Rationale

This order keeps the system testable and honest.

It also aligns with the external tools' actual strengths:

- OpenClaw cron is strong for persisted scheduled jobs, isolated sessions, and delivery.
  Source: https://docs.openclaw.ai/cron/
- Hermes cron is strong for fresh-session scheduled tasks with skill attachment and lifecycle controls.
  Source: https://hermes-agent.nousresearch.com/docs/user-guide/features/cron/
- MCP is strong for exposing a tool contract over JSON-RPC after the underlying operations are real.
  Sources:
  https://modelcontextprotocol.io/docs/concepts/transports
  https://modelcontextprotocol.io/specification/2025-03-26/basic/lifecycle
  https://modelcontextprotocol.io/specification/2025-03-26/server/tools

## Consequences

Positive:

- better determinism
- cleaner tests
- reproducible evaluation
- simpler debugging
- safer agent autonomy

Negative:

- VST work is deferred
- the first release will be less flashy than the README currently implies
- the team must correct existing overstatements before shipping more features

## Implementation Notes

The first end-to-end demo should be:

`cargo run -- generate-demo --midi out/demo.mid --wav out/demo.wav`

That command should:

- work from a fresh checkout
- use a built-in preset
- emit both artifacts
- be deterministic with a fixed seed

Only after this exists should the team wire agent prompts, MCP tools, or cron jobs around it.
