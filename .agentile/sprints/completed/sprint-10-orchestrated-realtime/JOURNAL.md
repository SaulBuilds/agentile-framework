---
created: 2026-04-20T12:30:00Z
branch: main
author: claude
sprint: sprint-10-orchestrated-realtime
status: active
---

# Sprint 10 Journal

## Design Decisions

### Why Wire Through The Harness Instead Of Direct Dispatch

The realtime adapter already works for direct CLI/MCP dispatch. The temptation is to just let scheduled jobs call the realtime functions directly. But that would bypass the harness's mediated execution, which means no plan records, no outcome tracking, no rollback handles, and no policy enforcement. Wiring through the harness keeps the orchestration audit trail intact and means the same policy rules apply to interactive and unattended dispatch.

### Orchestration Policy As A Separate Concern

Policy enforcement (max actions, max dispatches, recursive job prevention) is being added as a separate module rather than sprinkled into existing functions. This keeps the policy logic testable in isolation and makes it possible to adjust limits without touching execution code.

### Adapter Discovery Strategy

When the prompt contains dispatch intent but no explicit `adapter_id` is provided, the planner falls back to using the first available adapter from `list_realtime_adapters()`. This is a reasonable default for single-adapter setups and avoids requiring the operator to specify the adapter every time. The explicit `adapter_id` path takes priority when provided.

### Backward-Compatible Struct Extensions

All struct extensions (`adapter_id`, `max_actions`, `max_dispatches`) use `Option<T>` with `#[serde(default)]` where needed so that existing serialized data, CLI invocations, and MCP tool calls continue to work without modification. No existing tests required changes to their struct construction beyond adding the new `None` fields.
