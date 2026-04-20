---
created: 2026-04-20T03:20:00Z
branch: main
author: codex
sprint: sprint-8-scheduler-adapters
status: closed
---

# Sprint 8: Scheduler Adapters

## Sprint Metadata

| Field | Value |
|-------|-------|
| Sprint ID | S-8 |
| Sprint Name | Scheduler Adapters |
| Goal | Ship immutable unattended job configs, local batch entrypoints, and Hermes/OpenClaw-friendly scheduler bundles on top of the constrained harness |
| Repo State | build-green deterministic core plus governance, provenance, session, review, deck, and harness layers |
| Start Date | 2026-04-20 |
| End Date | 2026-04-27 |
| Status | CLOSED |

## Verification Snapshot

| Metric | Value |
|--------|-------|
| Tests (start) | 64 |
| Tests (current) | 67 |
| Build | `cargo test` passes after shipping the scheduler adapter layer |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` passes after shipping the scheduler adapter layer |
| Format | `cargo fmt --check` passes after shipping the scheduler adapter layer |

## Work Package Summary

| WP | Title | Status | Outcome |
|----|-------|--------|---------|
| WP-1 | Immutable Job Store | COMPLETE | Shipped stored unattended jobs with immutable configs, config hashes, adapter bundles, and run history |
| WP-2 | Local Batch Execution | COMPLETE | Shipped local `job-run` execution through the shared harness backend with bounded retry rules |
| WP-3 | Approval-Gated Mutations | COMPLETE | Shipped approval-gated scheduling and approval-gated cancellation with single-use tokens |
| WP-4 | CLI And MCP Surface | COMPLETE | Exposed job validation, scheduling, inspection, execution, and cancellation through real CLI commands and MCP tools |
| WP-5 | Verification And Truth | COMPLETE | Added scheduler unit, CLI, and MCP coverage and updated sprint/docs truth sources |

## Work Packages

### WP-1: Immutable Job Store

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added a durable scheduler store for unattended jobs
- recorded immutable configs, config hashes, export bundle metadata, approval linkage, and per-run history

Acceptance Criteria:

- [x] Scheduled jobs persist a stable `job_id`, immutable config payload, `config_hash`, approval id, export path, adapter bundle, and status.
- [x] Stored job configs fail closed on missing prompt, zero retry limit, or missing referenced session/deck/run context.
- [x] Scheduler export bundles are machine-readable and include the local batch entrypoint needed by a Hermes/OpenClaw-style runner.
- [x] Automated tests cover validation, scheduling, and inspection over a real runtime directory.

### WP-2: Local Batch Execution

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added local unattended execution through the shared harness planner and executor
- recorded run history with plan ids, outcome ids, timestamps, and terminal status

Acceptance Criteria:

- [x] `job-run` executes a stored immutable config through the shared harness instead of bypassing the runtime services.
- [x] Job execution records `plan_id`, `outcome_ids`, start time, finish time, and terminal status.
- [x] Retry limits are enforced before execution begins.
- [x] Automated tests cover one successful unattended run over a real session context.

### WP-3: Approval-Gated Mutations

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | S |

Scope:

- consumed `jobs.schedule` approval tokens before creating jobs
- consumed `jobs.cancel` approval tokens before cancelling stored jobs

Acceptance Criteria:

- [x] Scheduling fails closed without a matching `jobs.schedule` token for the requested job name.
- [x] Cancellation fails closed without a matching `jobs.cancel` token for the target job id.
- [x] Completed jobs cannot be cancelled retroactively.
- [x] CLI and MCP tests prove both approved scheduling and approved cancellation over stored jobs.

### WP-4: CLI And MCP Surface

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- exposed scheduler validation, scheduling, listing, inspection, execution, and cancellation through CLI and MCP
- kept those surfaces on the same scheduler and harness backends

Acceptance Criteria:

- [x] CLI supports `job-validate`, `job-schedule`, `job-list`, `job-inspect`, `job-run`, and `job-cancel`.
- [x] MCP exposes `job_validate`, `job_schedule`, `job_list`, `job_inspect`, `job_run`, and `job_cancel`.
- [x] Scheduling, execution, and cancellation emit durable manifests and audit events instead of bypassing provenance.
- [x] Existing deterministic, governance, deck, and harness tests still pass after integration.

### WP-5: Verification And Truth

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | S |

Scope:

- added scheduler coverage in unit, CLI, and MCP tests
- updated sprint, README, config, coverage, and changelog truth sources after shipping

Acceptance Criteria:

- [x] Unit coverage includes one validate/schedule/run flow over the real scheduler store.
- [x] CLI and MCP coverage each include an end-to-end unattended job path.
- [x] `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` pass.
- [x] `CURRENT.md`, `DAILY.md`, and `JOURNAL.md` reflect the actual sprint state.

## Delivered In Sprint 8

- Added `src/governance/scheduler.rs` as a real unattended job backend with immutable configs, adapter bundles, run history, and approval-gated state changes.
- Exposed local batch entrypoints through CLI and MCP while keeping those flows on the shared scheduler, approvals, and harness stores.
- Added Hermes/OpenClaw-friendly export manifests that point external runners back to the local `job-run` entrypoint.
- Increased the total passing test count from 64 to 67 while keeping build, lint, and format gates green.
