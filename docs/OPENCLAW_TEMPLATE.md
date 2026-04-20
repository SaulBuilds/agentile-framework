# OpenClaw Integration Template

Use this template to configure OpenClaw cron jobs for automated music generation, evaluation, and parameter exploration.

## Prerequisites

1. The HTTP server is running and accessible from the OpenClaw runner.
2. The API key is stored as an OpenClaw secret: `MUSIC_BOX_API_KEY`.

## OpenClaw Cron Job: Nightly Evaluation Run

```json
{
  "name": "music-box-nightly-eval",
  "schedule": "0 2 * * *",
  "isolated": true,
  "prompt": "Run a parameter sweep against the music box API. Call POST http://<host>:3001/api/tools/parameter_sweep with Authorization: Bearer ${MUSIC_BOX_API_KEY} and body {\"preset\": \"demo\", \"seeds\": [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20]}. Report the top 5 ranked seeds with their trajectory summaries.",
  "announce": {
    "type": "webhook",
    "url": "https://your-webhook-endpoint.com/music-box-results"
  }
}
```

## OpenClaw Cron Job: Parameter Exploration

```json
{
  "name": "music-box-explore",
  "schedule": "0 4 * * *",
  "isolated": true,
  "prompt": "Explore the music box parameter space. First snapshot the demo preset (POST /api/tools/snapshot_create with body {\"preset\":\"demo\",\"reason\":\"openclaw exploration\"}). Then patch it with a creative variation (POST /api/tools/preset_patch with body {\"preset\":\"demo\",\"tempo_bpm\":145,\"low_note\":48,\"high_note\":84,\"scale\":[0,2,4,7,9],\"reason\":\"openclaw exploration\"}). Then sweep 10 seeds (POST /api/tools/parameter_sweep). Report the best results. All calls need Authorization: Bearer ${MUSIC_BOX_API_KEY}."
}
```

## OpenClaw Cron Job: Session Render Pipeline

```json
{
  "name": "music-box-render-pipeline",
  "schedule": "0 6 * * 1",
  "isolated": true,
  "prompt": "Weekly music generation pipeline. 1) Create a session: POST /api/tools/session_create body {\"display_name\":\"Weekly Render\",\"preset\":\"demo\",\"seed\":42}. 2) Render preview: POST /api/tools/session_render_preview body {\"session_id\":\"<from step 1>\"}. 3) Report session ID, preview details, and note count. Authorization: Bearer ${MUSIC_BOX_API_KEY}."
}
```

## Key Constraints for OpenClaw Jobs

- OpenClaw runs persisted jobs with **isolated sessions** -- each run is independent.
- Job configs are stored and can be inspected via the OpenClaw dashboard.
- Results can be delivered via **webhook** or **announce** endpoints.
- Jobs should NOT attempt to:
  - Schedule new jobs (recursive scheduling is blocked by policy)
  - Approve pending requests (requires human operator)
  - Publish or promote artifacts (requires approval token)
- The HTTP API enforces all governance rules server-side -- the agent cannot bypass them.

## API Endpoint Summary

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | Health check |
| `/api/tools` | GET | List all available tools |
| `/api/tools/{name}` | POST | Call any tool (requires auth) |

## Response Format

```json
// Success
{"success": true, "data": { ... tool-specific result ... }}

// Error
{"success": false, "error": "human-readable error message"}
```
