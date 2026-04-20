---
created: 2026-04-20T12:00:00Z
branch: main
author: claude
sprint: sprint-8-scheduler-adapters
status: closed
---

# Sprint 8 Report: Scheduler Adapters

## Outcome

**CLOSED** -- All exit criteria met. Unattended job layer is shipped and verified.

## Summary

| Field | Value |
|-------|-------|
| Sprint ID | S-8 |
| Goal | Ship immutable unattended job configs, local batch entrypoints, and Hermes/OpenClaw-friendly scheduler bundles |
| Start Date | 2026-04-20 |
| Close Date | 2026-04-20 |
| Test Delta | 64 -> 67 (+3) |

## What Shipped

- Immutable job store with config hashing, approval linkage, and per-run history.
- Local batch execution through the shared harness backend with bounded retry rules.
- Approval-gated scheduling and cancellation with single-use token consumption.
- Exported scheduler bundles (JSON) for Hermes/OpenClaw-style external runners.
- CLI and MCP surfaces for job validate/schedule/list/inspect/run/cancel.

## Work Packages

| WP | Title | Status |
|----|-------|--------|
| WP-1 | Immutable Job Store | COMPLETE |
| WP-2 | Local Batch Execution | COMPLETE |
| WP-3 | Approval-Gated Mutations | COMPLETE |
| WP-4 | CLI And MCP Surface | COMPLETE |
| WP-5 | Verification And Truth | COMPLETE |

## Verification At Close

| Metric | Value |
|--------|-------|
| Passing tests | 67 |
| `cargo test` | Pass |
| `cargo clippy -D warnings` | Pass |
| `cargo fmt --check` | Pass |

## Carry-Forward

- Scheduler jobs do not yet dispatch to realtime adapters.
- No actual Hermes/OpenClaw integration test; bundles are exported but not consumed by a real external runner.
- No recursive job prevention enforcement beyond policy documentation.
