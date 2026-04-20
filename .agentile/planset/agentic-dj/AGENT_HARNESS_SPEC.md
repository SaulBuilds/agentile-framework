---
created: 2026-04-19T22:30:00Z
branch: main
author: codex
sprint: planning
status: active
---

# Agent Harness Spec

This document specifies the agent-facing contract for the Agentic DJ layer.

The harness is intentionally constrained:

- agents plan and act through tools
- agents do not access internal modules directly
- risky actions require approval
- every action is auditable and reversible where technically possible

## Harness Roles

### 1. Session DJ

Purpose:

- respond to operator chat
- inspect the current session
- propose and apply low-risk mutations
- request renders and live-control actions

Allowed default tools:

- `session.get_status`
- `preset.list`
- `preset.inspect`
- `trajectory.inspect`
- `render.generate_midi`
- `render.generate_audio`
- `eval.compare_candidates`
- `live.preview_patch`
- `live.apply_patch`
- `snapshot.create`
- `snapshot.rollback`

Blocked by default:

- publishing
- scheduling
- remote enablement
- credential changes

### 2. Evaluator

Purpose:

- compare candidate outputs
- score runs using objective metrics and human annotations
- recommend next experiments

Allowed default tools:

- `eval.list_runs`
- `eval.inspect_run`
- `eval.compare_candidates`
- `eval.submit_scores`
- `report.generate_summary`

### 3. Librarian

Purpose:

- curate presets, datasets, and attribution bundles
- manage provenance and registry metadata

Allowed default tools:

- `preset.create`
- `preset.diff`
- `preset.promote` with approval
- `dataset.list`
- `dataset.register` with approval
- `dataset.inspect`

### 4. Publisher

Purpose:

- package approved artifacts and push them to an external destination

Allowed default tools:

- `publish.prepare`
- `publish.validate_rights`
- `approval.request`
- `publish.execute` with approval token only

### 5. Scheduler

Purpose:

- create unattended batch runs through Hermes or OpenClaw after policy validation

Allowed default tools:

- `jobs.plan`
- `jobs.validate`
- `jobs.schedule` with approval
- `jobs.list`
- `jobs.cancel` with approval

## Tool Taxonomy

## Read-Only Tools

Low risk, no approval required:

- `session.get_status`
- `preset.list`
- `preset.inspect`
- `trajectory.inspect`
- `eval.list_runs`
- `eval.inspect_run`
- `dataset.list`
- `dataset.inspect`

## Reversible Mutation Tools

Medium risk, snapshot required:

- `live.apply_patch`
- `preset.create`
- `preset.update`
- `snapshot.rollback`
- `candidate.promote_local`

Rules:

- pre-action snapshot is mandatory
- parameter diff must be recorded
- tool must return rollback handle

## High-Risk Tools

Approval required:

- `publish.execute`
- `preset.promote_shared`
- `jobs.schedule`
- `jobs.cancel`
- `remote.enable_write_api`
- `dataset.register_production`
- `secrets.rotate`

Rules:

- operator-visible input preview
- single-use approval token
- no fallback execution path

## Required MCP Tool Set

The production MCP surface should expose at least the following tools:

| Tool | Risk | Purpose | Required Output |
|------|------|---------|-----------------|
| `preset.list` | low | enumerate presets | names, ids, hashes, sources |
| `preset.inspect` | low | inspect one preset | config, bounds, provenance |
| `preset.create` | medium | create preset from system or diff | preset id, hash, path |
| `preset.update` | medium | update preset fields | new hash, diff summary, rollback handle |
| `trajectory.inspect` | low | inspect deterministic trajectory | summary stats, preview |
| `render.generate_midi` | low | create MIDI artifact | path, hash, note count, duration |
| `render.generate_audio` | low | create WAV artifact | path, hash, sample count, peak |
| `live.preview_patch` | low | dry-run live mutation | predicted diff, impacted params |
| `live.apply_patch` | medium | mutate running session | snapshot id, diff, new session state |
| `snapshot.create` | low | snapshot current state | snapshot id, manifest pointer |
| `snapshot.rollback` | medium | restore previous state | restored snapshot id, session state |
| `eval.compare_candidates` | low | compare outputs | metrics, side-by-side summary |
| `eval.submit_scores` | low | persist human/objective ratings | evaluation id, reward summary |
| `approval.request` | low | create pending approval record | approval id, requested action |
| `publish.prepare` | medium | build rights and attribution bundle | package id, rights summary |
| `publish.execute` | high | push approved artifact externally | publish id, destination, audit id |
| `jobs.validate` | medium | validate scheduled job config | policy result, warnings |
| `jobs.schedule` | high | create unattended job | job id, scheduler backend, config hash |

## MCP Contract Notes

- `stdio` remains the default local transport.
- If HTTP transport is introduced later, auth and origin validation are mandatory.
- Client-provided roots help scope context, but the server must still enforce its own path policy because roots are informational guidance, not access control.
- Sensitive data collection should use explicit operator approvals and, if later needed, URL-mode elicitation rather than plain prompt text.

## System Prompt Pack

The harness should use role-specific prompts instead of one giant mutable prompt.

## Session DJ Prompt

```text
You are Session DJ, a constrained music-control agent.

Your job is to help the operator shape deterministic music sessions using the approved tool set.

Rules:
1. Use tools instead of guessing.
2. Keep actions reversible when possible.
3. Before any mutation, summarize the intended change and expected musical effect.
4. Never publish, schedule, enable remote access, or alter secrets without an approval token.
5. If a requested action is blocked by policy, say so plainly and request approval rather than improvising around the restriction.
6. Preserve provenance: always carry forward preset hash, seed, session id, and snapshot id when available.
7. Optimize for controllability, musical usefulness, and operator trust over novelty alone.
```

## Evaluator Prompt

```text
You are Evaluator, a scoring and analysis agent.

Your job is to compare candidate runs using both objective metrics and human judgments.

Rules:
1. Do not invent metrics; only use the metrics returned by tools.
2. Keep raw human scores separate from weighted aggregates.
3. Explain why a candidate is stronger or weaker in musical terms and control terms.
4. Recommend the next experiment as a bounded parameter change, not an open-ended rewrite.
5. Never mutate presets or publish artifacts directly.
```

## Librarian Prompt

```text
You are Librarian, the provenance and preset curation agent.

Your job is to keep presets, datasets, and rights metadata clean and auditable.

Rules:
1. Reject unregistered or license-ambiguous datasets for production use.
2. Track hashes, versions, parents, and diffs for every managed object.
3. Prefer the smallest truthful change that preserves reproducibility.
4. Request approval before moving any object into shared or production scope.
```

## Publisher Prompt

```text
You are Publisher, a high-risk gated agent.

Your job is to package and publish artifacts only after rights validation and human approval.

Rules:
1. Never publish without a valid approval token.
2. Never substitute different artifacts than the ones approved.
3. Abort if artifact hashes, attribution data, or destination policy do not match the approval record.
4. Record the final publish event with exact hashes, destination, time, and approval id.
```

## Scheduler Prompt

```text
You are Scheduler, the unattended-run planning agent.

Your job is to define reproducible batch jobs that run in fresh sessions.

Rules:
1. Use pinned prompts, presets, datasets, and policies by version or hash.
2. Assume no hidden conversation memory.
3. Stop and request approval for publishing, remote changes, or production-scope mutations.
4. Prefer idempotent jobs and explicit retry policies.
```

## Run Manifest Schema

Each harness run must produce at least:

- `run_id`
- `session_id`
- `agent_role`
- `operator_id` if present
- `model_id`
- `prompt_pack_version`
- `tool_policy_version`
- `dataset_registry_versions`
- `preset_hash`
- `seed`
- `snapshot_before`
- `action_plan`
- `tool_calls`
- `artifacts`
- `objective_metrics`
- `human_scores`
- `approval_events`
- `rollback_handle`
- `final_status`

## Failure Policy

The harness must fail closed when:

- a requested tool is missing
- a tool schema mismatches the expected contract
- an approval token is absent or expired
- a dataset is not approved for the requested use class
- a path falls outside the server allowlist
- a tool returns an error on a mutation path after a snapshot has been created

Required behavior on failure:

1. stop the action chain
2. preserve the manifest
3. mark the run failed with reason
4. expose the rollback handle if a mutation occurred
5. never silently continue with substitute behavior

## Research Basis

- MCP tools, lifecycle, authorization, roots, and elicitation:
  - https://modelcontextprotocol.io/specification/2025-11-25/server/tools
  - https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle
  - https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization
  - https://modelcontextprotocol.io/specification/2025-06-18/client/roots
  - https://modelcontextprotocol.io/specification/2025-06-18/client/elicitation
- Hermes hooks:
  - https://hermes-agent.nousresearch.com/docs/user-guide/features/hooks/
- OpenClaw cron and hooks:
  - https://docs.openclaw.ai/automation/cron-jobs
  - https://docs.openclaw.ai/automation/hooks
