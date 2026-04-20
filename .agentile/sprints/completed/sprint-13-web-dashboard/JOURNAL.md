---
created: 2026-04-20T17:00:00Z
branch: main
author: claude
sprint: sprint-13-web-dashboard
status: active
---

# Sprint 13 Journal

## Design Decisions

### Manual Scaffold Over create-next-app

Writing the scaffold by hand keeps it minimal and avoids pulling in unnecessary boilerplate.
App Router, TypeScript, Tailwind -- nothing else.

### API Client Pattern

A single `lib/api.ts` module that wraps fetch with auth headers and the consistent response envelope. Every page component calls this module -- no raw fetch in components.

### Vercel Deployment

The webapp is a standalone Next.js app under `web/`. Vercel will deploy from `web/` with `rootDirectory: web` in the project settings. The Rust API server runs separately.
