# Project Configuration

This file is the canonical source for repo-specific facts. When commands, names, paths, or lifecycle assumptions disagree, this file wins.

## Identity

| Key | Value |
|-----|-------|
| Project Name | state-space-music-box |
| Repository Purpose | A library for generating procedural music based on state space representations |
| Primary Domain | SDK |
| Current Phase | GREENFIELD |
| Primary Users | Both developers and end users |

## Collaboration Context

| Key | Value |
|-----|-------|
| Human Counterparts | `<MAINTAINERS_OR_TEAMS>` |
| Preferred Work Mode | `<PLANNED_SPRINTS | INCIDENT_RESPONSE | MIXED>` |
| Definition Of Done | `<WHAT_MUST_BE_TRUE_BEFORE_WORK_COUNTS_AS_DONE>` |
| Review Standard | `<WHO_MUST_REVIEW_WHICH_KINDS_OF_CHANGES>` |

## Technology Stack

| Layer | Technology |
|-------|------------|
| Primary Languages | Rust |
| Frameworks / Runtimes | Tokio (async), rodio (audio output) |
| Package / Build Tooling | Cargo |
| Storage / State | In-memory state representation |
| Test Tooling | cargo test, proptest (property-based testing) |
| Lint / Static Analysis | clippy, rustfmt |
| Formal Verification | MIRAI (optional) |

## Core Commands

```bash
# Bootstrap / install
<bootstrap command>

# Build
<build command>

# Test
<test command>

# Lint / typecheck
<lint command>

# Format
<format command>

# Run locally
<run command>

# CI parity command
<ci command>
```

## Workspace Map

| Area | Path | Purpose |
|------|------|---------|
| Application / Service 1 | `<path>` | `<purpose>` |
| Application / Service 2 | `<path>` | `<purpose>` |
| Shared Library / Core | `<path>` | `<purpose>` |
| Tests / Fixtures | `<path>` | `<purpose>` |
| Infra / Deployment | `<path>` | `<purpose>` |
| Docs / Specs | `<path>` | `<purpose>` |

## Delivery Surfaces

List the user-facing or system-facing surfaces this repo owns.

| Surface | Interface | Primary Contract |
|---------|-----------|------------------|
| `<surface>` | `<CLI | API | UI | JOB | LIBRARY | CONTRACT | OTHER>` | `<what must stay true>` |

## Sensitive Areas

Changes in these paths or topics require elevated care and usually review:

| Area | Path / Topic | Why Sensitive |
|------|---------------|---------------|
| `<area>` | `<path or topic>` | `<security | money | data loss | availability | compliance | other>` |

## Naming Conventions

- Canonical names to use: `<official product, module, domain, and environment names>`
- Deprecated or confusing names to avoid: `<legacy names>`
- Branch scopes commonly used in commits: `<api, web, infra, core, docs, ...>`

## Notes

Use this section for domain-specific facts that do not fit the tables above but must stay canonical.
