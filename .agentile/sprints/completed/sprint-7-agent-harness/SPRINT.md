---
created: 2026-04-20T01:10:00Z
branch: main
author: codex
sprint: sprint-7-agent-harness
status: closed
---

# Sprint 7: Agent Harness

## Sprint Metadata

| Field | Value |
|-------|-------|
| Sprint ID | S-7 |
| Sprint Name | Agent Harness |
| Goal | Ship a deterministic constrained harness that plans and executes bounded actions through the real session, review, and deck backends |
| Repo State | build-green deterministic core plus governance, provenance, session, review, deck, CLI, and MCP layers |
| Start Date | 2026-04-20 |
| End Date | 2026-04-27 |
| Status | CLOSED |

## Verification Snapshot

| Metric | Value |
|--------|-------|
| Tests (start) | 61 |
| Tests (current) | 64 |
| Build | `cargo test` passes after shipping the agent harness layer |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` passes after shipping the agent harness layer |
| Format | `cargo fmt --check` passes after shipping the agent harness layer |

## Work Package Summary

| WP | Title | Status | Outcome |
|----|-------|--------|---------|
| WP-1 | Harness Store | COMPLETE | Shipped durable harness plans and execution outcomes with deterministic signatures |
| WP-2 | Deterministic Planner | COMPLETE | Shipped a bounded rule-based planner over the real session, review, and deck backends |
| WP-3 | Mediated Executor | COMPLETE | Shipped a tool-mediated harness executor with reversible session patch application and persisted outcomes |
| WP-4 | CLI And MCP Surface | COMPLETE | Exposed harness planning and execution through real CLI commands and MCP tools |
| WP-5 | Verification And Truth | COMPLETE | Added end-to-end harness coverage and updated sprint/docs truth sources to match the shipped layer |

## Work Packages

### WP-1: Harness Store

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added a durable harness store for plans and outcomes
- recorded deterministic signatures, prompts, roles, context refs, and action proposals

Acceptance Criteria:

- [x] Harness plans persist a stable `plan_id`, role, prompt, context refs, system prompt, deterministic signature, and action list.
- [x] Harness outcomes persist a stable `outcome_id`, plan id, action id, tool name, status, result, rollback handle, and timestamp.
- [x] Harness plan and outcome helpers fail closed on missing ids.
- [x] Automated tests cover persisted plan creation and outcome listing.

### WP-2: Deterministic Planner

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added a bounded rule-based planner for Session DJ and Evaluator paths
- derived actions only from the real current runtime context and supported tool set

Acceptance Criteria:

- [x] Planning output is deterministic for the same role, prompt, and context.
- [x] Proposed actions always include tool name, risk level, justification, expected effect, rollback strategy, and concrete arguments.
- [x] The planner only references tools allowed for the requested role.
- [x] Automated tests cover one session-dj planning flow.

### WP-3: Mediated Executor

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added a mediated executor that runs bounded actions through the real session, review, and deck helpers
- added reversible session patch application with a captured rollback session payload

Acceptance Criteria:

- [x] Execution paths are tool-mediated and do not mutate state through hidden side channels.
- [x] `live.apply_patch` captures a rollback session payload before writing the new state.
- [x] Failed or blocked executions persist an outcome record instead of mutating state silently.
- [x] Automated tests cover one successful execution and one persisted rollback handle.

### WP-4: CLI And MCP Surface

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- exposed harness plan, plan inspection, execution, and outcome listing through CLI and MCP
- kept those surfaces on the same harness backend store

Acceptance Criteria:

- [x] CLI supports `harness-plan`, `harness-plan-inspect`, `harness-execute`, and `harness-outcome-list`.
- [x] MCP exposes `harness_plan`, `harness_plan_inspect`, `harness_execute`, and `harness_outcome_list`.
- [x] Harness flows execute only through the shared runtime services they wrap.
- [x] Existing deterministic, governance, provenance, session, review, and deck tests still pass after integration.

### WP-5: Verification And Truth

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | S |

Scope:

- added harness coverage in unit, CLI, and MCP tests
- updated sprint, README, config, coverage, and changelog truth sources after shipping

Acceptance Criteria:

- [x] Unit coverage includes one harness planning and execution flow.
- [x] CLI or MCP coverage includes one end-to-end harness execution over the real backend.
- [x] `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` pass.
- [x] `CURRENT.md`, `DAILY.md`, and `JOURNAL.md` reflect the actual sprint state.

## Delivered In Sprint 7

- Added `src/governance/harness.rs` with persisted harness plans, persisted outcomes, deterministic signatures, bounded action proposals, and mediated execution.
- Added session patch preview and session patch apply helpers so the harness can preview and perform reversible live mutations over the real session store.
- Exposed harness planning and execution through CLI and MCP while keeping those flows on the same shared backend.
- Increased the total passing test count from 61 to 64 while keeping build, lint, and format gates green.
