# Hermes Integration Template

Use this template to configure Hermes cron jobs that run nightly evaluations and parameter sweeps against the state-space-music-box HTTP API.

## Prerequisites

1. The HTTP server is running: `cargo run -- http --port 3001 --api-key $MUSIC_BOX_API_KEY`
2. At least one session exists with a rendered preview.
3. The `MUSIC_BOX_API_KEY` environment variable is set.

## Hermes Cron Job: Nightly Parameter Sweep

This job runs every night at 2 AM, sweeps 20 seeds, and reports the top 5.

```yaml
name: nightly-parameter-sweep
schedule: "0 2 * * *"
skills: []
prompt: |
  You are an automated music evaluation agent. Run a parameter sweep
  against the state-space-music-box API and report the best seeds.

  Steps:
  1. Call POST http://localhost:3001/api/tools/parameter_sweep
     with header "Authorization: Bearer ${MUSIC_BOX_API_KEY}"
     and body: {"preset": "demo", "seeds": [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]}

  2. From the response, extract ranked_seeds (top 5).

  3. For the top seed, call POST http://localhost:3001/api/tools/session_create
     with body: {"display_name": "Nightly Best ${DATE}", "preset": "demo", "seed": <top_seed>}

  4. Render a preview: POST http://localhost:3001/api/tools/session_render_preview
     with body: {"session_id": "<session_id>"}

  5. Report: the top 5 seeds, the session ID, and the preview details.

  Do NOT schedule new jobs or publish anything. This is a read-heavy evaluation job.
```

## Hermes Cron Job: Nightly Preset Exploration

This job patches the preset with a random variation and evaluates the result.

```yaml
name: nightly-preset-exploration
schedule: "0 3 * * *"
skills: []
prompt: |
  You are an automated music exploration agent. Patch the demo preset
  with a new configuration and evaluate the result.

  Steps:
  1. Snapshot the current preset:
     POST http://localhost:3001/api/tools/snapshot_create
     body: {"preset": "demo", "reason": "nightly exploration backup"}

  2. Patch the preset with a creative variation:
     POST http://localhost:3001/api/tools/preset_patch
     body: {"preset": "demo", "tempo_bpm": <random 80-180>, "low_note": <random 36-60>, "high_note": <random 72-96>, "reason": "nightly exploration"}

  3. Sweep 10 seeds with the patched preset:
     POST http://localhost:3001/api/tools/parameter_sweep
     body: {"preset": "demo", "seeds": [1,2,3,4,5,6,7,8,9,10]}

  4. Report the ranked results and the patch that was applied.

  Authorization header: "Bearer ${MUSIC_BOX_API_KEY}"
```

## Key Constraints for Hermes Jobs

- Hermes runs jobs in **fresh sessions** -- no conversation memory carries over.
- Every curl call must include the full Authorization header.
- Jobs should be **idempotent** -- running the same job twice should not corrupt state.
- Jobs must NOT call `approval_resolve` or `job_schedule` -- these require human approval.
- All results are persisted in the runtime directory and can be inspected later via `run_list` and `audit_list`.
