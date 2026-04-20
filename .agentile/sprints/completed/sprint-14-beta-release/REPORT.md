---
created: 2026-04-20T19:00:00Z
branch: main
author: claude
sprint: sprint-14-beta-release
status: closed
---

# Sprint 14 Report: Beta Release

## Outcome

**CLOSED** -- v0.2.0-beta.1 tagged. All gates green. Beta is shippable.

## Summary

| Field | Value |
|-------|-------|
| Sprint ID | S-14 |
| Goal | End-to-end integration testing, CI parity script, and tag v0.2.0-beta |
| Start Date | 2026-04-20 |
| Close Date | 2026-04-20 |
| Test Delta | 79 -> 80 (+1 HTTP integration test) |

## What Shipped

- End-to-end HTTP integration test exercising the full creative workflow over real HTTP: health check, tool listing, auth, generate, session create, preview render, parameter sweep, audit.
- CI parity script (`scripts/ci-check.sh`) running Rust fmt + clippy + tests and Next.js build.
- Version bumped to 0.2.0-beta.1 in Cargo.toml and package.json.
- CHANGELOG tagged with [0.2.0-beta] release notes.

## Verification At Close

| Metric | Value |
|--------|-------|
| Passing tests | 80 |
| `cargo test` | Pass |
| `cargo clippy -D warnings` | Pass |
| `cargo fmt --check` | Pass |
| `next build` | Pass (11 pages) |
| HTTP integration test | Pass (12-step workflow) |

## Beta Readiness

| Gate | Status |
|------|--------|
| Build green | Pass |
| Deterministic generation | Pass (same seed = same output) |
| HTTP API accessible | Pass (32 tool endpoints + health + tools listing) |
| Auth enforced | Pass (bearer token required) |
| Creative workflow end-to-end | Pass (sweep -> patch -> render) |
| Agent docs available | Pass (AGENT_GUIDE.md, HERMES_TEMPLATE.md, OPENCLAW_TEMPLATE.md) |
| Web dashboard deployable | Pass (11 pages, Vercel config) |
| Governance audit trail | Pass (append-only, immutable manifests) |
