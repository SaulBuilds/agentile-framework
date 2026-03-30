# Project Configuration

This file is the canonical source for repo-specific facts. When commands, names, paths, or lifecycle assumptions disagree, this file wins.

## Identity

| Key | Value |
|-----|-------|
| Project Name | `<PROJECT_NAME>` |
| Repository Purpose | `<WHAT_THIS_REPO_BUILDS_OR_OPERATES>` |
| Primary Domain | `<WEB_APP | API | INFRA | SDK | DATA | ML | PROTOCOL | OTHER>` |
| Current Phase | `<GREENFIELD | RETROFIT | DEBUGGING | CI_STABILIZATION | PRE_PROD | PRODUCTION>` |
| Primary Users | `<WHO_THIS_PROJECT_SERVES>` |

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
| Primary Languages | `<LANGUAGES>` |
| Frameworks / Runtimes | `<FRAMEWORKS_OR_RUNTIMES>` |
| Package / Build Tooling | `<PACKAGE_MANAGERS_AND_BUILD_SYSTEMS>` |
| Storage / State | `<DATABASES_QUEUES_FILESYSTEM_ETC>` |
| Test Tooling | `<TEST_COMMANDS_AND_FRAMEWORKS>` |
| Lint / Static Analysis | `<LINTERS_TYPECHECKERS_SECURITY_SCANNERS>` |
| Formal Verification | `<OPTIONAL_TOOLING_OR_NA>` |

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
