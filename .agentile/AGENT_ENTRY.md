# Agent Entry Point

Read this file first. Every contributor, human or agent, starts here.

## First Routing Decision

| If you are entering a repo that is... | Go to... |
|---------------------------------------|----------|
| Missing a real `SPIRIT.md` or still using the starter README | [Bootstrap The Repo Contract](#bootstrap-the-repo-contract) |
| Healthy and you are starting cold | [Cold Start](#cold-start) |
| Already running an active sprint | [`sprints/CURRENT.md`](sprints/CURRENT.md) |
| In failing CI, review fallout, debugging, or pre-prod hardening | [`workflows/REVIEW.md`](workflows/REVIEW.md) then [`workflows/DEBUGGING.md`](workflows/DEBUGGING.md) |
| New contributor needing role assignment | [Onboarding](#onboarding) |

## Bootstrap The Repo Contract

Before feature work, make sure the repo actually speaks for itself.

1. Read [`../SPIRIT.md`](../SPIRIT.md) if it exists.
2. If `SPIRIT.md` is missing, stale, or still describes the starter, rewrite it using [`../SPIRIT_GUIDE.md`](../SPIRIT_GUIDE.md) and [`templates/SPIRIT.template.md`](templates/SPIRIT.template.md).
3. If [`../README.md`](../README.md) still describes the starter rather than the actual project, refactor it before feature work.
4. Confirm the root instruction files still route agents into this framework: [`../AGENTS.md`](../AGENTS.md), [`../CLAUDE.md`](../CLAUDE.md), and related tool files.
5. If the repo has no `CHANGELOG.md`, create one from [`templates/CHANGELOG.template.md`](templates/CHANGELOG.template.md).

GATE: Do not treat starter boilerplate as project truth. If the repo still describes the framework instead of the project, fix that first or create a sprint item that does.

## Cold Start

If you have no useful context about this repository:

1. Read [`../SPIRIT.md`](../SPIRIT.md) if it exists. If it does not, return to [Bootstrap The Repo Contract](#bootstrap-the-repo-contract).
2. Read [`CONFIG.md`](CONFIG.md) for canonical project constants.
3. Read [`rules/CORE_RULES.md`](rules/CORE_RULES.md) for the hard gates.
4. Read [`sprints/CURRENT.md`](sprints/CURRENT.md) for the active state.
5. Decide which workflow matches the repo state:
   - Healthy planned work: [`workflows/FEATURE.md`](workflows/FEATURE.md)
   - New project setup: [`workflows/INIT.md`](workflows/INIT.md)
   - Existing repo adoption: [`workflows/RETROFIT.md`](workflows/RETROFIT.md)
   - Review or hardening: [`workflows/REVIEW.md`](workflows/REVIEW.md)
   - Debugging or failing CI: [`workflows/DEBUGGING.md`](workflows/DEBUGGING.md)

GATE: Do not write code until you can explain the project, the current repo state, the active sprint state, and the rules that apply to your next move.

## Onboarding

New contributors take the adaptive quiz in [`onboarding/QUIZ_SPEC.md`](onboarding/QUIZ_SPEC.md) unless the skip protocol applies.

Use:
- [`onboarding/QUIZ_SPEC.md`](onboarding/QUIZ_SPEC.md)
- [`onboarding/SKIP_PROTOCOL.md`](onboarding/SKIP_PROTOCOL.md)

The quiz or skip protocol determines:
- starting tier
- starting zooid
- initial ELO score

## Golden Moves

Before any meaningful work:

1. Confirm the repo has a project-specific `SPIRIT.md`.
2. Confirm the root `README.md` describes the project rather than the starter.
3. Read `CONFIG.md`, `CORE_RULES.md`, and `CURRENT.md`.
4. Choose the workflow that matches the repo state.
5. Record work in sprint and journal artifacts so the next contributor inherits context instead of guesses.

## Project Overview

Use the root [`../README.md`](../README.md) for the project description.

Use [`CONFIG.md`](CONFIG.md) for:
- canonical names
- commands
- workspace map
- critical paths and sensitive areas

Use [`../SPIRIT.md`](../SPIRIT.md) for:
- collaboration norms
- non-negotiables specific to this project
- how the spirit itself may be changed

## Framework Structure

```text
.agentile/
├── AGENT_ENTRY.md
├── CONFIG.md
├── MANIFEST.md
├── rules/
├── workflows/
├── templates/
├── docs/
├── formal/
├── coverage/
├── sprints/
└── audits/
```

## Timestamp Requirement

Project-generated governance artifacts such as `SPIRIT.md`, sprint records, journals, essays, case studies, ADRs, and specs should use timestamp + branch frontmatter.

Starter bootstrap files such as `README.md`, `AGENTS.md`, `CLAUDE.md`, `.cursorrules`, and templates may stay frontmatter-free so tools discover and render them cleanly.

See [`rules/CORE_RULES.md`](rules/CORE_RULES.md) Rule 12 for the full standard.
