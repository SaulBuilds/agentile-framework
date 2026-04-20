---
created: 2026-04-19T16:17:14Z
branch: main
author: codex
sprint: sprint-10-orchestrated-realtime
status: active
---

# Coverage Baseline

> Record the sprint floor here. The total number of passing tests must never decrease.

**Baseline date:** 2026-04-19
**Baseline sprint:** sprint-1-foundation
**Total passing tests at baseline:** 35

## Current Observed Count

- 2026-04-20: 80 passing tests

## Breakdown

| Module | Tests | Passing | Failing | Ignored |
|--------|-------|---------|---------|---------|
| `audio_engine` | 4 | 4 | 0 | 0 |
| `cli` | 3 | 3 | 0 | 0 |
| `effect_model` | 3 | 3 | 0 | 0 |
| `generation` | 3 | 3 | 0 | 0 |
| `governance::approvals` | 4 | 4 | 0 | 0 |
| `governance::audit` | 2 | 2 | 0 | 0 |
| `governance::datasets` | 3 | 3 | 0 | 0 |
| `governance::daw` | 1 | 1 | 0 | 0 |
| `governance::evaluations` | 3 | 3 | 0 | 0 |
| `governance::harness` | 3 | 3 | 0 | 0 |
| `governance::policy` | 4 | 4 | 0 | 0 |
| `governance::realtime` | 1 | 1 | 0 | 0 |
| `governance::scheduler` | 1 | 1 | 0 | 0 |
| `governance::sessions` | 3 | 3 | 0 | 0 |
| `governance::snapshots` | 2 | 2 | 0 | 0 |
| `instrument_model` | 3 | 3 | 0 | 0 |
| `mcp` | 8 | 8 | 0 | 0 |
| `midi_model` | 3 | 3 | 0 | 0 |
| `state_machine` | 6 | 6 | 0 | 0 |
| `state_space` | 3 | 3 | 0 | 0 |
| `vst_synthesizer` | 3 | 3 | 0 | 0 |
| `cli_integration` | 9 | 9 | 0 | 0 |
| **Total** | **76** | **76** | **0** | **0** |

## History

| Date | Sprint | Total Tests | Delta |
|------|--------|-------------|-------|
| 2026-04-19 | sprint-1-foundation | 35 | baseline |
| 2026-04-19 | sprint-2-governance-control | 49 | +14 |
| 2026-04-19 | sprint-3-audit-and-manifests | 51 | +2 |
| 2026-04-19 | sprint-4-session-and-evaluation | 57 | +6 |
| 2026-04-19 | sprint-5-live-control-and-review | 60 | +3 |
| 2026-04-20 | sprint-6-daw-control-adapter | 61 | +1 |
| 2026-04-20 | sprint-7-agent-harness | 64 | +3 |
| 2026-04-20 | sprint-8-scheduler-adapters | 67 | +3 |
| 2026-04-20 | sprint-9-realtime-adapters | 70 | +3 |
| 2026-04-20 | sprint-10-orchestrated-realtime | 76 | +6 |
| 2026-04-20 | sprint-11-sdk-and-http | 77 | +1 |
| 2026-04-20 | sprint-12-agent-docs-and-creative-tools | 79 | +2 |
| 2026-04-20 | sprint-14-beta-release | 80 | +1 |

## Changelog

### 2026-04-19

- Replaced the starter placeholder coverage record with the actual sprint baseline and current observed test count.
- Updated the per-module breakdown after adding deterministic generation, MCP, VST validation, and CLI integration tests.
- Recorded the governance-control increase after adding dataset registry, approval-token, snapshot, and expanded CLI/MCP coverage.
- Recorded the audit-and-manifests increase after adding provenance storage and stronger runtime-record assertions.
- Recorded the session-and-evaluation increase after adding durable session records, evaluation records, and new end-to-end coverage.
- Recorded the live-control-and-review increase after adding session transport, preview rendering, review bundles, and stronger end-to-end coverage.
- Recorded the DAW-control-adapter increase after adding durable deck control and transport inspection coverage.
- Recorded the agent-harness increase after adding persisted plans, persisted outcomes, and harness end-to-end coverage.
- Recorded the scheduler-adapter increase after adding immutable job config coverage plus CLI and MCP unattended-job flows.
- Recorded the realtime-adapter increase after adding OSC adapter coverage plus CLI and MCP live-dispatch flows.
