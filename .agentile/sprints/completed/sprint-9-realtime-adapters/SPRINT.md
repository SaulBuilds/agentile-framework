---
created: 2026-04-20T05:10:00Z
branch: main
author: codex
sprint: sprint-9-realtime-adapters
status: closed
---

# Sprint 9: Realtime Adapters

## Sprint Metadata

| Field | Value |
|-------|-------|
| Sprint ID | S-9 |
| Sprint Name | Realtime Adapters |
| Goal | Ship a real OSC bridge for live preview and transport dispatch on top of the deck, session, harness, and scheduler layers |
| Repo State | build-green deterministic core plus governance, provenance, session, review, deck, harness, and scheduler layers |
| Start Date | 2026-04-20 |
| End Date | 2026-04-27 |
| Status | CLOSED |

## Verification Snapshot

| Metric | Value |
|--------|-------|
| Tests (start) | 67 |
| Tests (current) | 70 |
| Build | `cargo test` passes after shipping the realtime adapter layer |
| Lint | `cargo clippy --all-targets --all-features -- -D warnings` passes after shipping the realtime adapter layer |
| Format | `cargo fmt --check` passes after shipping the realtime adapter layer |

## Work Package Summary

| WP | Title | Status | Outcome |
|----|-------|--------|---------|
| WP-1 | Realtime Adapter Store | COMPLETE | Shipped persisted OSC adapter configs and dispatch history |
| WP-2 | Live Preview Dispatch | COMPLETE | Shipped OSC preview streaming from real rendered MIDI previews |
| WP-3 | Live Transport Dispatch | COMPLETE | Shipped OSC transport and clip dispatch from real deck state |
| WP-4 | CLI And MCP Surface | COMPLETE | Exposed realtime adapter creation, inspection, and dispatch through real CLI and MCP tools |
| WP-5 | Verification And Truth | COMPLETE | Added realtime unit, CLI, and MCP coverage and updated sprint/docs truth sources |

## Work Packages

### WP-1: Realtime Adapter Store

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- added a durable realtime adapter store for OSC endpoints
- recorded endpoint config, base path, protocol, and dispatch history

Acceptance Criteria:

- [x] Realtime adapters persist a stable `adapter_id`, protocol, host, port, base path, timestamps, and dispatch log.
- [x] Adapter creation fails closed on invalid display name, invalid port, or invalid base path.
- [x] Realtime config is persisted in the shared runtime store rather than hidden in CLI flags or chat state.
- [x] Automated tests cover adapter creation and inspection.

### WP-2: Live Preview Dispatch

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- parsed real preview MIDI artifacts and converted them into OSC note messages
- supported immediate and timed dispatch modes with an explicit `time_scale`

Data sources:

- preview MIDI files from `SessionPreviewRecord`
- session identity from `SessionRecord`

Acceptance Criteria:

- [x] Preview dispatch reads real stored preview MIDI data instead of inventing note events.
- [x] OSC note messages include stable session and preview identifiers plus channel and note data.
- [x] Dispatches are logged with actor id, mode, message count, and timestamp.
- [x] Automated tests prove that a local UDP listener receives packets from preview dispatch.

### WP-3: Live Transport Dispatch

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | S |

Scope:

- emitted OSC transport messages from real deck transport snapshots
- sent active clip metadata when a deck is in playing state

Data sources:

- deck state from `DeckTransportSnapshot`
- active clip metadata from `DeckClipRecord`

Acceptance Criteria:

- [x] Transport dispatch uses real deck state from the shared DAW control store.
- [x] Stopped decks emit stop messages and active decks emit play plus clip metadata messages.
- [x] Every dispatch records source type, related session or deck ids, and message count.
- [x] Automated tests prove that a local UDP listener receives packets from transport dispatch.

### WP-4: CLI And MCP Surface

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | M |

Scope:

- exposed realtime adapter creation, listing, inspection, preview dispatch, and transport dispatch through CLI and MCP
- kept those surfaces on the same realtime backend store

Acceptance Criteria:

- [x] CLI supports `realtime-create`, `realtime-list`, `realtime-inspect`, `realtime-send-preview`, and `realtime-send-transport`.
- [x] MCP exposes `realtime_create`, `realtime_list`, `realtime_inspect`, `realtime_send_preview`, and `realtime_send_transport`.
- [x] Realtime dispatch surfaces emit durable manifests and audit events instead of bypassing provenance.
- [x] Existing deterministic, governance, deck, harness, and scheduler tests still pass after integration.

### WP-5: Verification And Truth

| Field | Value |
|-------|-------|
| Status | [x] COMPLETE |
| Assignee | codex |
| Effort | S |

Scope:

- added realtime coverage in unit, CLI, and MCP tests
- updated sprint, README, config, coverage, and changelog truth sources after shipping

Acceptance Criteria:

- [x] Unit coverage includes one end-to-end OSC adapter flow over real preview and deck data.
- [x] CLI and MCP coverage each include one end-to-end realtime dispatch flow.
- [x] `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo fmt --check` pass.
- [x] `CURRENT.md`, `DAILY.md`, and `JOURNAL.md` reflect the actual sprint state.

## Delivered In Sprint 9

- Added `src/governance/realtime.rs` as a real local OSC bridge with persisted adapters and dispatch logs.
- Exposed realtime adapter creation and live dispatch through CLI and MCP while keeping those flows on the shared runtime and audit layers.
- Increased the total passing test count from 67 to 70 while keeping build, lint, and format gates green.
