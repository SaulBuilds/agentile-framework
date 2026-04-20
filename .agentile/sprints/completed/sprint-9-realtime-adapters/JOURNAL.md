---
created: 2026-04-20T05:10:00Z
branch: main
author: codex
sprint: sprint-9-realtime-adapters
status: active
---

# Sprint Journal: sprint-9-realtime-adapters -- Realtime Adapters

## What Happened

- Sprint created to turn the planned live-control path into a real local transport contract.
- The sprint shipped persisted OSC adapter configs and concrete UDP dispatch from real preview MIDI data and real deck transport state.
- The sprint deliberately chose OSC first because it is portable, testable, and already aligned with the repo’s local-first bridge strategy.
- CLI and MCP now expose the same realtime flows, and those flows stay on the same audit and manifest rails as the rest of the system.
- The shipped path is honest: local OSC is real today, while native virtual MIDI ports remain future work.

## What I Think Is True At Sprint Closeout

- The repo now has a deterministic artifact core, governance and provenance systems, session/review/deck control layers, a constrained harness, an unattended scheduler layer, and a first local realtime bridge over OSC.
- Operators and future automation can now create a realtime endpoint, stream a stored preview, and push deck transport updates without restarting the process.
- The live path is still local-first and protocol-limited; it does not yet provide native virtual MIDI output or remote orchestration.
- The next safe move is to let the harness and scheduler target these live adapters under stronger orchestration and approval policy.
