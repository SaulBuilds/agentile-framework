---
created: 2026-04-19T23:05:00Z
branch: main
author: codex
sprint: sprint-2-governance-control
status: closed
---

# Sprint 2: Governance Control

## Sprint Metadata

| Field | Value |
|-------|-------|
| Sprint ID | S-2 |
| Sprint Name | Governance Control |
| Goal | Ship the first real dataset registry, approval token flow, and preset snapshot/rollback layer for the future Agentic DJ harness |
| Repo State | build-green deterministic foundation with real CLI and stdio MCP surfaces |
| Start Date | 2026-04-19 |
| End Date | 2026-04-26 |
| Status | CLOSED |

## Verification Snapshot

| Metric | Value |
|--------|-------|
| Tests (start) | 35 |
| Tests (current) | 49 |
| Build | `cargo test` passes after shipping governance-control work |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` passes after shipping governance-control work |
| Format | `cargo fmt --check` passes after shipping governance-control work |

## Work Package Summary

| WP | Title | Status | Outcome |
|----|-------|--------|---------|
| WP-1 | Dataset Registry | COMPLETE | Shipped persistent dataset records with provenance, use-class metadata, inspection, and policy enforcement helpers |
| WP-2 | Approval Tokens And Decisions | COMPLETE | Shipped approval request, operator resolution, single-use tokens, and failure-closed consumption rules |
| WP-3 | Preset Snapshots And Rollback | COMPLETE | Shipped persistent preset snapshots with exact rollback for file-backed presets |
| WP-4 | CLI And MCP Governance Surface | COMPLETE | Exposed the new governance services through real CLI commands and MCP tools backed by the same core |
| WP-5 | Documentation And Sprint Truth | COMPLETE | Updated the README, sprint files, coverage baseline, and changelog to match the shipped governance layer |

## Work Packages

### WP-1: Dataset Registry

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | L |

Scope:

- added persistent dataset records with license, provenance, use-class, and checksum fields
- validated required metadata before writes
- rejected duplicate or malformed records on create paths

Acceptance Criteria:

- [x] Dataset records are persisted in a machine-readable format.
- [x] Records include source URL, version, checksum manifest, license, commercial-use status, redistribution status, and approved use class.
- [x] Duplicate ids are rejected unless an explicit update path is used.
- [x] Automated tests cover create, list, inspect, and invalid-record rejection.

### WP-2: Approval Tokens And Decisions

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added approval requests, operator decisions, and single-use approval tokens
- modeled high-risk action scopes and expiration

Acceptance Criteria:

- [x] Approval requests can be created with action scope, target, and reason.
- [x] Approvals can be approved or denied by a named operator.
- [x] Approval tokens are single-use and scope-checked.
- [x] Automated tests cover approve, deny, expired token, and wrong-scope rejection.

### WP-3: Preset Snapshots And Rollback

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- created persistent snapshots for mutable preset workflows
- supported restoring a preset from a prior snapshot

Acceptance Criteria:

- [x] A snapshot captures preset content, preset hash, reason, and timestamp.
- [x] Rollback restores the exact serialized preset content from the chosen snapshot.
- [x] Rollback fails closed if the snapshot content is malformed or missing.
- [x] Automated tests cover snapshot creation, rollback success, and invalid snapshot failure.

### WP-4: CLI And MCP Governance Surface

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | L |

Scope:

- exposed dataset registry, approvals, and snapshots through real commands and tools
- kept the CLI and MCP surfaces on the same backend services

Acceptance Criteria:

- [x] CLI commands exist for dataset list/register, approval request/resolve, and snapshot create/rollback.
- [x] MCP exposes at least one real tool per new governance area.
- [x] Sensitive mutation tools require approval tokens where applicable.
- [x] Automated tests cover one happy path and one rejection path for each new surface.

### WP-5: Documentation And Sprint Truth

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | S |

Scope:

- kept sprint, README, changelog, and planset references aligned

Acceptance Criteria:

- [x] `CURRENT.md` points to this sprint while it is active.
- [x] Sprint `DAILY.md` and `JOURNAL.md` reflect actual work.
- [x] User-facing docs are updated if CLI or MCP behavior changes.
- [x] CHANGELOG includes any user-facing governance surface changes.

## Delivered In Sprint 2

- Added `src/governance/datasets.rs` with durable dataset registry records, inspection helpers, use-class enforcement, and record validation.
- Added `src/governance/approvals.rs` with request, approve/deny, expiring single-use tokens, and strict scope/target consumption rules.
- Added `src/governance/snapshots.rs` with preset snapshot creation, preset hashing, and exact rollback for file-backed presets.
- Wired the governance services into the library, CLI, and MCP server so the local deterministic stack now has a real control layer.
- Added unit and integration coverage for happy paths, rejection paths, and authorization failures.
