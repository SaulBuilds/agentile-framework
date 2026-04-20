---
created: 2026-04-20T03:20:00Z
branch: main
author: codex
sprint: sprint-8-scheduler-adapters
status: active
---

# Sprint Journal: sprint-8-scheduler-adapters -- Scheduler Adapters

## What Happened

- Sprint created to turn unattended execution from a planset item into a real stored backend contract.
- The sprint shipped immutable scheduled jobs with config hashes, approval linkage, run history, and export bundles that external runners can consume without inventing hidden state.
- The sprint also shipped local batch entrypoints that execute those jobs through the shared harness planner and executor instead of bypassing the runtime stores.
- Scheduling and cancellation were kept approval-gated, and the tests now prove that completed jobs cannot be cancelled retroactively.
- CLI and MCP expose the same scheduler surfaces, and the resulting job mutations stay on the same provenance rails as the rest of the system.

## What I Think Is True At Sprint Closeout

- The repo now has a real deterministic artifact core, governance and provenance systems, session/review/deck control layers, a constrained harness, and a first unattended scheduler layer over those services.
- Operators and future automation can now validate, schedule, inspect, execute, and cancel stored unattended jobs without bypassing approvals or local runtime state.
- Hermes/OpenClaw integration is now manifest-friendly but still local-first; the repo exports adapter bundles rather than creating remote jobs directly.
- The next safe move is to build realtime adapters and richer orchestration policy on top of the now-real scheduler contract.
