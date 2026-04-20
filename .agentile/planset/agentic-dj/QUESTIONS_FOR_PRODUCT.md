---
created: 2026-04-19T15:32:26Z
branch: main
author: Codex
sprint: planning
status: active
---

# Questions For Product

These questions should be answered before implementation moves past recovery and into realtime agent work.

## DAW Target

Pick the first-class integration target:

1. DAW-agnostic through MIDI and WAV only
2. REAPER-first
3. Ableton-first
4. Bitwig-first
5. Another DAW with a concrete scripting or control API

Why this matters:

- it changes whether we prioritize MIDI import/export, OSC, scripting hooks, or plugin packaging

## Live Loop Shape

Define the first useful live demo:

1. agent changes parameters while a human triggers regeneration
2. agent streams MIDI into a DAW or synth in realtime
3. agent renders short loops, scores them, and proposes next actions
4. agent and human co-edit presets through chat and tool calls

Why this matters:

- it determines the first realtime protocol and approval model

## Reward Model

Choose the first evaluation objective:

1. human taste only
2. system-theoretic objectives only
3. hybrid objective with both human and automatic metrics

Why this matters:

- it changes how the adaptation loop is designed and what data we must persist

## Approval Policy

Decide which actions must require human approval:

- preset promotion
- scheduler creation or modification
- external process launch
- live DAW control
- artifact publishing

Why this matters:

- it defines the harness safety model

## Deployment Boundary

Choose the initial deployment mode:

1. local-only CLI and local MCP
2. local CLI plus local and remote MCP
3. service deployment from day one

Why this matters:

- it changes authentication, transport, and observability requirements
