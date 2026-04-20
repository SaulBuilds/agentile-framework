---
created: 2026-04-19T15:01:20Z
branch: main
author: Codex
sprint: sprint-1-foundation
status: active
---

# Recommendations

## Immediate Order Of Operations

1. Stop feature expansion until the repo is green again.
2. Recover truthful documentation and sprint status before claiming more completed work.
3. Re-scope the near-term MVP to deterministic offline MIDI plus WAV generation.
4. Defer VST hosting, DAW-specific integrations, and foundation-model retraining until the SDK contracts are real.
5. Build the MCP surface only after the CLI and library API are stable and tested.
6. Use OpenClaw and Hermes only as external orchestration layers for evaluation jobs, not as the primary product runtime.

## Contract Reset

The first product contract should become:

"A fresh checkout can produce one valid `.mid` file and one valid `.wav` file with a single command, using deterministic state-space-driven generation."

Only after that contract is met should the next one become:

"An MCP client can create a system, request generation, inspect results, and mutate parameters safely over a real transport."

Only after that should the project promise:

"An agent loop can evaluate outputs, adapt parameters, and schedule unattended runs."

## Technical Re-scope

Treat "live retraining" in the MVP as online parameter adaptation, not LLM weight training.

Recommended interpretation:

- log every generation run
- compute objective metrics
- collect optional human scores
- update mapping or preset parameters
- keep full rollback history

This is achievable and testable. Full model fine-tuning is not.

## Delivery Priorities

### Priority 0

- fix compile break
- fix buffer safety
- remove placeholder completion claims

### Priority 1

- deterministic state trajectory simulation
- state-to-note mapping
- MIDI file export
- WAV rendering

### Priority 2

- working CLI commands
- examples that produce actual artifacts
- stable MCP tool surface

### Priority 3

- agent harness
- evaluator
- scheduler adapters for OpenClaw and Hermes

### Priority 4

- DAW integration
- VST hosting
- advanced synthesis engines

## Release Gate Recommendation

Do not describe the project as:

- realtime
- VST capable
- MCP complete
- `no_std`
- multi-synthesis

until those claims are backed by passing verification and working examples in the tree.
