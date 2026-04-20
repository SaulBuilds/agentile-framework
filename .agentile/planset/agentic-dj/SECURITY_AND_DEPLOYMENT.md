---
created: 2026-04-19T15:32:26Z
branch: main
author: Codex
sprint: planning
status: active
---

# Security And Deployment

## Security Goals

Protect:

- the creative control surface
- the live music feed
- publishing and promotion actions
- adaptation and future training data
- remote control paths
- secrets used for cloud, webhooks, or external platforms

## Local-First Security Model

Even in local development, the architecture should already separate:

- operator identity
- agent identity
- action policy
- audit trail

Minimum local controls:

- explicit operator session
- policy checks before publish, promote, or schedule actions
- append-only audit log for tool calls and approvals
- isolated config and secret storage
- immutable run manifest for each generation and evaluation cycle

## Cloud Deployment Recommendation

Start with one hardened DigitalOcean Droplet.

Recommended baseline:

- Ubuntu LTS
- SSH keys only
- non-root sudo user
- DigitalOcean Cloud Firewall
- service processes bound to `127.0.0.1` by default
- reverse proxy only for the specific remote entrypoints we intentionally publish
- TLS on any remote UI or API
- DigitalOcean Monitoring agent and alerts
- backups enabled plus periodic snapshots

## Droplet Hardening Notes

DigitalOcean guidance supports the following baseline:

- Cloud Firewalls are stateful and block traffic that is not expressly permitted.
- Monitoring is opt-in through the `do-agent` metrics agent and can alert through email or Slack.
- Droplet backups can be enabled at creation time or later from the control panel.
- MCP Streamable HTTP requires origin validation, localhost binding where possible, and authentication.

## Network Policy Recommendation

Publicly exposed:

- `443/tcp` only if a remote UI or API is intentionally enabled
- `22/tcp` only from operator allowlists or through a bastion pattern

Not public by default:

- MCP HTTP
- internal control plane
- worker coordination
- metrics endpoints other than the DigitalOcean agent's outbound traffic

Internal-only or localhost-only:

- agent-to-engine control channel
- local DAW interface backend
- OSC bridge where possible
- admin endpoints

## Approval Policy

Always require approval for:

- publishing artifacts externally
- promoting presets to shared or production scope
- creating or editing scheduler jobs
- enabling remote write-capable interfaces
- exporting adaptation or training datasets

Require explicit logged intent for:

- any cloud credential change
- any webhook destination change
- any remote-control authorization change

## Data Retention Guidance

Persist by default:

- run manifests
- preset hashes and diffs
- approval events
- publish events
- evaluation metrics

Do not persist without explicit policy:

- raw chat history beyond what is needed for audit
- unnecessary operator secrets
- full training corpora exports

## Recommended First Cloud Shape

One box, four services:

1. reverse proxy
2. app server with local DAW backend and policy layer
3. render worker
4. scheduler worker

This keeps the first cloud deployment simple while still separating concerns enough to audit and lock down behavior.
