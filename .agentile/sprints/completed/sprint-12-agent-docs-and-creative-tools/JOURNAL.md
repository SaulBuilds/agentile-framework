---
created: 2026-04-20T16:00:00Z
branch: main
author: claude
sprint: sprint-12-agent-docs-and-creative-tools
status: active
---

# Sprint 12 Journal

## Design Decisions

### Preset Patch vs Session Patch

The session already has `update_session` for tempo/seed changes. The new preset_patch tool goes further -- it allows agents to mutate the generation parameters themselves (mapping config, simulation config) with a snapshot-before for rollback. This is how agents will creatively explore the parameter space.

### Parameter Sweep As A First-Class Tool

Rather than making agents loop manually, a dedicated sweep tool runs N compositions in one call, collects summaries, and ranks them. This is the core creative workflow: try many seeds, compare, pick the best, adapt.
