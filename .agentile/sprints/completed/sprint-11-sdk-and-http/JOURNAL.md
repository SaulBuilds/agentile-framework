---
created: 2026-04-20T14:00:00Z
branch: main
author: claude
sprint: sprint-11-sdk-and-http
status: active
---

# Sprint 11 Journal

## Design Decisions

### HTTP Server Architecture

Using axum directly rather than trying to run MCP-over-HTTP. The MCP stdio server is great for agent IDEs but agents like Hermes and OpenClaw need a simple REST-like HTTP endpoint they can curl. The HTTP server will wrap the same governance functions the CLI and MCP use -- thin HTTP handlers calling the real backend.

### Auth Model

Simple bearer token auth for beta. API key maps to an actor_id for audit trails. No complex RBAC yet -- that comes with cloud deployment hardening.
