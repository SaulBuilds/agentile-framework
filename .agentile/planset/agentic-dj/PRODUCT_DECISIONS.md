---
created: 2026-04-19T15:32:26Z
branch: main
author: Codex
sprint: planning
status: active
---

# Product Decisions

This file records the current high-level product decisions from the human stakeholder.

## Confirmed Decisions

### DAW Direction

Decision:

- DAW-agnostic
- build our own simple DAW interface as the primary local control surface

Interpretation:

- do not optimize for one commercial DAW first
- prioritize import/export and realtime interoperability over plugin hosting
- the product owns a minimal transport, scene or clip, parameter, and render workflow

### First Demo Scope

Decision:

- include all major feature modes in the first demo

Implementation interpretation:

- the first demo is a vertical slice, not a deep feature-complete release
- every major subsystem must be real, but each may be intentionally minimal

The first demo should include:

- simple DAW-like local interface
- chat or prompt-driven agent interaction
- deterministic MIDI and WAV generation
- live control and mutation
- evaluation and scoring
- parameter adaptation
- gated publishing or promotion path

### Reward Model

Decision:

- optimize both human preference and automatic metrics

Implementation interpretation:

- store both score types in every run record
- do not allow the system to optimize only for automatic metrics without preserving human feedback

### Security Posture

Decision:

- use proper approvals and authorizations
- secure the music feed, publishing path, and any training or adaptation data flows

Implementation interpretation:

- deny by default for sensitive actions
- require approvals for publishing and remote-impacting mutations
- maintain auditability across tool calls, outputs, feedback, and promotions

### Deployment Path

Decision:

- develop and test locally first
- later deploy to a cloud droplet

Implementation interpretation:

- local-first architecture
- avoid local-only shortcuts that block remote deployment
- design the auth, secret, audit, and approval model so it survives the move to cloud

## Working Assumptions

Unless superseded, plan against these assumptions:

- first cloud target is a DigitalOcean Droplet
- first MCP transport is local `stdio`
- first remote exposure is minimal and authenticated
- first live bridge is MIDI plus a control protocol, not VST hosting
