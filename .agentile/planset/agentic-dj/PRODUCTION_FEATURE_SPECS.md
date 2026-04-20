---
created: 2026-04-19T22:30:00Z
branch: main
author: codex
sprint: planning
status: active
---

# Production Feature Specs

This document turns the Agentic DJ roadmap into hard product behavior.

Every feature below is written as an executable product contract:

- Gherkin feature and scenario statements
- measurable acceptance criteria
- explicit safety, rollback, and observability requirements

If code, prompts, tools, or docs disagree with this file, this file wins until superseded by a newer ADR or approved planset revision.

## Global Release Gates

The system is not production-ready unless all of the following are true:

1. Every public action path produces a run manifest with timestamps, hashes, inputs, outputs, approval events, and actor identity.
2. Every mutation path supports rollback to the last good snapshot.
3. Every write-capable MCP tool has a declared risk level and approval policy.
4. Deterministic paths remain reproducible under fixed preset, seed, and runtime version.
5. Dataset provenance and license class are recorded before any corpus enters training, adaptation, captioning, evaluation, or publishing workflows.
6. No remote write-capable interface is enabled without authentication, rate limiting, audit logs, and operator-visible approval prompts.

## Feature 1: Deterministic Artifact Generation

Feature: deterministic trajectory, MIDI, and WAV generation

Scenario: identical inputs reproduce identical artifacts
Given a preset hash, seed, engine version, and render config
When the system generates a trajectory, MIDI file, and WAV file twice
Then the trajectory summary must match exactly
And the MIDI bytes must match exactly
And the WAV sample stream must match exactly
And the run manifest must record the same preset hash, seed, and engine version in both runs

Scenario: changed seed creates controlled variation
Given a preset with declared pitch, velocity, and duration bounds
When the operator changes only the seed
Then the note sequence must change in at least one note event
And all note events must remain inside the preset's configured bounds
And the trajectory summary must still pass finite-value and duration invariants

Acceptance Criteria:

- Fixed-seed reproducibility is enforced in automated tests for trajectory, MIDI, and WAV outputs.
- Every artifact is accompanied by a manifest containing `preset_hash`, `seed`, `engine_version`, `tool_chain_version`, and `artifact_hash`.
- Artifact generation fails closed on invalid dimensions, invalid output paths, NaN/Inf outputs, or unclamped sample overflow.
- Production builds expose a single artifact service used by library, CLI, MCP, and future UI layers.

## Feature 2: Simple DAW Interface

Feature: DAW-agnostic local control surface

Scenario: operator loads a preset and performs a live session
Given the simple DAW UI is connected to a local engine session
And a preset is available
When the operator loads the preset
And presses play or render
Then the transport state must change visibly
And the active preset, scene, clip, tempo, and seed must be shown
And the resulting playback or render must be logged as a session event

Scenario: operator changes parameters during a live session
Given a running local session
When the operator changes tempo, seed, or a mapped state-space parameter
Then the parameter diff must be recorded with before and after values
And the next render or live phrase must reflect the updated values
And the operator must be able to restore the previous snapshot

Acceptance Criteria:

- The UI supports `load preset`, `render`, `play`, `stop`, `snapshot`, `rollback`, and parameter editing.
- UI state is driven from backend session state, not duplicated client-side business logic.
- Every live mutation creates a structured event with session id, actor id, parameter key, old value, new value, and timestamp.
- A soak test proves at least 30 minutes of local live use without panic, deadlock, or runaway memory growth.

## Feature 3: MCP Control Plane

Feature: transport-clean MCP server for Agentic DJ operations

Scenario: client discovers tools
Given an MCP client completes initialization
When it requests `tools/list`
Then the server returns only supported tools
And every tool definition includes a stable name, description, risk class, input schema, and output schema

Scenario: client calls a low-risk render tool
Given an authenticated local MCP session
When the client calls `generate_midi` or `generate_audio`
Then the server must validate input against schema and policy
And the server must return structured results or structured tool errors
And stdout must contain protocol traffic only

Scenario: client calls a high-risk tool
Given a tool call with risk level `approval_required`
When the client requests a publish, promote, scheduler, or remote-control action
Then the tool must enter `pending_approval`
And the action must not execute until approval is recorded
And the resulting approval decision must be written to the audit log

Acceptance Criteria:

- MCP tools implement versioned JSON schemas and stable output contracts.
- Tool names follow MCP naming guidance and remain unique within the server.
- Tool results use `isError: true` for business-logic failures and protocol errors for malformed requests.
- Sensitive tool calls present operator-visible confirmation with full input preview.
- Server-side path checks enforce an allowlisted workspace boundary regardless of client-provided roots.

## Feature 4: Agent Harness

Feature: constrained Agentic DJ harness over real tools

Scenario: agent proposes a safe action plan
Given a user prompt and the current session manifest
When the harness plans the next step
Then it must emit a bounded action plan referencing only allowed tools
And each action must include justification, expected effect, and rollback strategy
And the plan must not contain direct filesystem or process mutations outside the tool layer

Scenario: agent executes a reversible mutation
Given an approved mutable action
When the harness applies a preset mutation or live control change
Then it must create a pre-mutation snapshot
And execute the action through the tool interface
And record the exact diff, artifacts, and evaluation outputs
And surface a rollback handle

Scenario: agent attempts a disallowed action
Given a system prompt and policy that forbid external publishing without approval
When the model requests a publish action without an approval token
Then the harness must reject the action
And record the attempted action in the audit trail
And ask for approval rather than improvising around the policy

Acceptance Criteria:

- Harness behavior is tool-mediated only; no hidden side channels are permitted.
- Planning output is deterministic under fixed prompt template, context snapshot, and model settings where deterministic decoding is supported.
- Every executed action links to a prior plan id and a later outcome id.
- Failed runs cannot mutate the active preset unless an explicit recovery entry records the failure and post-failure state.

## Feature 5: Evaluation Workbench

Feature: hybrid evaluation of creative and technical quality

Scenario: operator reviews a candidate run
Given a completed run with MIDI, WAV, preset diff, and metrics
When the operator opens the evaluation view
Then the operator must see transport controls, artifact preview, parameter diff, objective metrics, and provenance
And the operator must be able to rate musicality, novelty, controllability, and usefulness on explicit scales

Scenario: automatic metrics are combined with human feedback
Given a run has objective metrics and optional human ratings
When the system computes a reward summary
Then it must preserve the raw component scores
And compute a weighted aggregate score from named weights
And record which weights were used
And never overwrite human scores with inferred values

Acceptance Criteria:

- Evaluation records store objective metrics, subjective ratings, free-text notes, and final reward weights separately.
- The UI exposes a side-by-side comparison mode for at least two candidate runs.
- The operator can mark a run as `reject`, `keep for reference`, `promote`, or `queue for further search`.
- Evaluation exports are machine-readable and can be reused in scheduled experiments.

## Feature 6: Dataset Ingestion And Governance

Feature: dataset intake with license and provenance enforcement

Scenario: approved dataset enters the system
Given a new dataset candidate
When an operator registers it
Then the registry must store dataset id, source URL, license, commercial-use status, redistribution status, provenance notes, checksum, and approved use class
And the dataset must remain unavailable to harness or training workflows until approved

Scenario: research-only dataset is selected for a production path
Given a dataset marked `research_only`
When an operator or agent tries to use it in a production or commercial workflow
Then the policy layer must block the request
And the system must explain the policy reason
And the blocked request must be logged

Acceptance Criteria:

- Every dataset has a machine-readable registry record before use.
- Registry records distinguish `production_allowed`, `research_only`, and `license_review_required`.
- Data pipelines refuse unregistered or checksum-mismatched corpora.
- Derived datasets record parent datasets and transformation scripts.

## Feature 7: Approval And Publishing

Feature: human-in-the-loop approvals for external impact

Scenario: operator approves a publish action
Given a publish request with artifact hashes, target destination, and attribution bundle
When an authorized operator approves the request
Then the approval record must include operator id, timestamp, target, artifact hashes, and reason
And only the approved artifact hashes may be published

Scenario: artifact changes after approval
Given an approved publish request
When the artifact hash changes before the publish step
Then the publish action must be invalidated
And a new approval must be required

Acceptance Criteria:

- Publish, promote, schedule, and remote-enable actions are all gated by approval tokens.
- Approval tokens are single-use, scoped, and expire.
- Attribution and license notices are generated before publish eligibility is granted.
- A publish event always references the exact approval event that authorized it.

## Feature 8: Scheduler And Unattended Runs

Feature: reproducible unattended jobs through Hermes and OpenClaw

Scenario: overnight evaluation job executes in a fresh session
Given a scheduled evaluation job
When Hermes or OpenClaw starts the run
Then the job must load a pinned prompt pack, preset set, and tool policy
And the run must not rely on hidden conversational memory
And the job must emit manifests, metrics, and failure logs

Scenario: unattended job requests a sensitive action
Given a scheduled run is executing without an active operator
When the agent reaches an approval-required action
Then the job must stop at `awaiting_approval`
And must not bypass approval via alternative tools or fallback paths

Acceptance Criteria:

- Every scheduled job points to immutable configuration inputs by hash or version.
- Jobs are idempotent or explicitly marked non-idempotent.
- Retry policy is declared per job and logged per attempt.
- Unattended jobs cannot publish, enable remote control, or alter policy without separate approval.

## Feature 9: Cloud Deployment

Feature: hardened first cloud deployment

Scenario: remote deployment is enabled
Given the system is deployed on a droplet
When remote access is turned on
Then authenticated entrypoints must terminate TLS
And write-capable services must be rate limited
And secrets must not be stored in the repo or plaintext audit logs
And alerting must exist for service failure, disk pressure, and repeated authorization failures

Acceptance Criteria:

- The first remote deployment binds internal services to localhost and exposes only intended entrypoints.
- MCP over HTTP is disabled by default and requires explicit auth and origin validation.
- Backups, monitoring, and firewall policy are documented and provisioned.
- Remote approval actions require strong operator identity and audit logging.

## Definition Of "100% Complete" For This Planset

A feature area is only complete when:

1. The Gherkin scenarios are covered by automated tests where feasible and by named manual checks where not.
2. The operator workflow is documented in the UI or CLI help and the repo docs.
3. Failure behavior is tested, not guessed.
4. Rollback or recovery behavior is demonstrated.
5. Audit records are produced and inspectable.
6. The sprint record names the exact command or verification step that proves the feature works.

## Research Basis

- MCP lifecycle, tools, transports, authorization, elicitation, and roots:
  - https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle
  - https://modelcontextprotocol.io/specification/2025-11-25/server/tools
  - https://modelcontextprotocol.io/specification/2025-11-25/basic/transports
  - https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization
  - https://modelcontextprotocol.io/specification/2025-06-18/client/roots
  - https://modelcontextprotocol.io/specification/2025-06-18/client/elicitation
- Hermes hooks:
  - https://hermes-agent.nousresearch.com/docs/user-guide/features/hooks/
- OpenClaw cron and hooks:
  - https://docs.openclaw.ai/automation/cron-jobs
  - https://docs.openclaw.ai/automation/hooks
