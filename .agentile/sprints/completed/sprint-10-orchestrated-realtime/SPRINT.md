---
created: 2026-04-20T12:30:00Z
branch: main
author: claude
sprint: sprint-10-orchestrated-realtime
status: closed
---

# Sprint 10: Orchestrated Realtime

## Sprint Metadata

| Field | Value |
|-------|-------|
| Sprint ID | S-10 |
| Sprint Name | Orchestrated Realtime |
| Goal | Wire the harness and scheduler to the realtime adapter so that planned and scheduled actions can include live OSC dispatch, and add orchestration policy enforcement for unattended runs |
| Repo State | build-green v0.1.0 baseline with 70 passing tests across deterministic core, governance, sessions, decks, harness, scheduler, and realtime layers |
| Start Date | 2026-04-20 |
| End Date | 2026-04-27 |
| Status | CLOSED |

## Verification Snapshot

| Metric | Value |
|--------|-------|
| Tests (start) | 70 |
| Tests (current) | 76 |
| Build | `cargo test` passes after shipping orchestrated realtime layer |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` passes after shipping orchestrated realtime layer |
| Format | `cargo fmt --check` passes after shipping orchestrated realtime layer |

## Motivation

Sprints 7, 8, and 9 each shipped their layer in isolation. The harness can plan and execute bounded actions but cannot dispatch to realtime adapters. The scheduler can run unattended jobs through the harness but those jobs cannot include realtime dispatch. The realtime adapter can send OSC packets but is only reachable through direct CLI/MCP calls, not through harness-mediated or scheduler-driven flows.

This sprint closes those gaps so the Agentic DJ stack has a real end-to-end path from prompt -> plan -> execute -> live dispatch, both interactively and unattended.

## Work Package Summary

| WP | Title | Status | Outcome |
|----|-------|--------|---------|
| WP-1 | Harness Realtime Actions | COMPLETE | Shipped `realtime.send_preview` and `realtime.send_transport` as mediated harness actions with real OSC dispatch |
| WP-2 | Scheduler Realtime Dispatch | COMPLETE | Scheduler batch execution now routes through policy-aware harness with realtime dispatch support |
| WP-3 | Orchestration Policy | COMPLETE | Shipped configurable policy module with max actions, max dispatches, and recursive job prevention |
| WP-4 | CLI And MCP Surface Updates | COMPLETE | Added `--adapter-id` and `--max-actions` to harness-plan CLI/MCP; scheduler surfaces accept adapter_id and max_dispatches |
| WP-5 | Verification And Truth | COMPLETE | Added 6 new tests and updated all governance artifacts |

## Work Packages

### WP-1: Harness Realtime Actions

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | claude |
| Effort | M |

Scope:

- extend `derive_actions()` in `harness.rs` to recognize realtime dispatch intents in prompts
- add `realtime.send_preview` and `realtime.send_transport` as mediated harness actions
- the executor should call the real `send_preview_to_realtime_adapter()` and `send_transport_to_realtime_adapter()` functions from `realtime.rs`
- persist outcome records with dispatch metadata (adapter id, message count, mode)

Data sources:

- adapter configs from `RealtimeAdapterRecord` in `realtime.rs`
- preview artifacts from `SessionPreviewRecord` in `sessions.rs`
- deck state from `DeckRecord` in `daw.rs`

Acceptance Criteria:

- [x] The harness planner can derive `realtime.send_preview` and `realtime.send_transport` actions from prompts containing dispatch intent for the SessionDj role.
- [x] The harness executor dispatches real OSC packets through the existing realtime adapter backend.
- [x] Execution outcomes include adapter id, protocol, message count, and dispatch mode.
- [x] Automated tests cover one harness-mediated preview dispatch over a real UDP listener.

### WP-2: Scheduler Realtime Dispatch

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | claude |
| Effort | M |

Scope:

- extend `run_scheduled_job()` in `scheduler.rs` so that harness plans generated during batch execution can include realtime dispatch steps
- the scheduler itself does not bypass the harness; it runs through the same mediated execution path that now supports realtime actions
- record dispatch metadata in the job run history

Data sources:

- job configs from `ScheduledJobRecord` in `scheduler.rs`
- harness plans from `HarnessPlanRecord` in `harness.rs`
- adapter configs from `RealtimeAdapterRecord` in `realtime.rs`

Acceptance Criteria:

- [x] A scheduled job whose prompt includes dispatch intent generates a harness plan with realtime actions.
- [x] Local batch execution of that job dispatches real OSC packets through the existing adapter backend.
- [x] Job run records include plan ids that reference realtime dispatch outcomes.
- [x] Automated tests cover one scheduled job producing a realtime dispatch through the harness.

### WP-3: Orchestration Policy

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | claude |
| Effort | M |

Scope:

- add an orchestration policy module that governs unattended execution boundaries
- enforce a maximum action chain length per harness plan (prevent unbounded execution)
- enforce a maximum dispatch count per scheduled job run (prevent runaway OSC floods)
- enforce recursive job prevention: a job run cannot schedule new jobs from inside its execution
- the policy should be configurable via a policy struct with sensible defaults

Acceptance Criteria:

- [x] A harness plan that exceeds the maximum action count is rejected at planning time.
- [x] A scheduled job run that attempts more dispatches than the per-run limit is stopped at the limit.
- [x] A harness executor running inside a scheduled job context cannot call `schedule_job()`.
- [x] Policy limits are configurable and have documented defaults.
- [x] Automated tests cover one plan rejection and one dispatch limit enforcement.

### WP-4: CLI And MCP Surface Updates

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | claude |
| Effort | S |

Scope:

- update `harness-plan` and `harness-execute` CLI/MCP responses to include realtime dispatch outcomes when present
- add a `--max-actions` flag to `harness-plan` for policy override
- add a `--max-dispatches` flag to `job-run` for per-run dispatch limits
- update MCP tool schemas to reflect the new capabilities

Acceptance Criteria:

- [x] CLI `harness-plan` output includes realtime actions when the prompt triggers them.
- [x] CLI `harness-execute` output includes dispatch metadata in the outcome.
- [x] CLI `job-run` respects `--max-dispatches` and defaults to the policy limit.
- [x] MCP tools return the same enriched responses.
- [x] Existing CLI and MCP tests still pass after the surface updates.

### WP-5: Verification And Truth

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | claude |
| Effort | S |

Scope:

- add unit tests for orchestration policy enforcement
- extend CLI integration and MCP tests for the new harness-realtime and scheduler-realtime flows
- update README, CONFIG, CURRENT, DAILY, JOURNAL, coverage baseline, and CHANGELOG

Acceptance Criteria:

- [x] Total passing test count increases from the sprint-open baseline of 70.
- [x] `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` pass.
- [x] `CURRENT.md`, `DAILY.md`, and `JOURNAL.md` reflect the actual sprint state.
- [x] README and CONFIG reflect the new orchestration capabilities honestly.

## Exit Criteria

This sprint is ready for closeout when:

- [x] The harness can plan and execute realtime dispatch actions through the real adapter backend.
- [x] The scheduler can run jobs that include realtime dispatch through the harness.
- [x] Orchestration policy prevents unbounded plans, runaway dispatches, and recursive scheduling.
- [x] All governance artifacts are updated and the build is green.
