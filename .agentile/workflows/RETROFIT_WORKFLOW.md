# RETROFIT_WORKFLOW.md — Adopting Agentile in an Existing Repo

> Use this workflow when `.agentile/` is dropped into a project that already has code.
> **This is a HARD GATE workflow. Every step must complete before the next begins. Do not skip steps.**

---

## Entry Gate

**Before starting retrofit, verify:**
- [ ] `.agentile/` folder exists with all framework files
- [ ] The agent has read `AGENT_ENTRY.md`, `MANIFEST.md`, and all `rules/`
- [ ] The project has existing code (if it doesn't, use INIT_WORKFLOW.md instead)

**GATE: If there's no existing code, you're in the wrong workflow. Use INIT_WORKFLOW.md.**

---

## Step 1: Audit the Existing Project

### 1.1 Inventory
Scan the codebase and document:
- Language(s) and frameworks in use
- Existing test files and coverage (if any)
- Folder structure and conventions
- Dependencies and their versions
- CI/CD configuration (if any)
- Existing documentation

### 1.2 Fill CONFIG.md
Populate based on what's discovered. Present to human for confirmation.

### 1.3 Create Audit Report
Save to `reports/RETROFIT-AUDIT.md`:
```markdown
# Retrofit Audit Report

## Current State
- Language: [x]
- Framework: [x]
- Test coverage: [x]% (or "none")
- Documentation: [state]
- CI/CD: [state]

## Identified Gaps
- [list of missing tests, docs, etc.]

## Recommendations
- [prioritized list of improvements]
```

**GATE: CONFIG.md must be populated. `reports/RETROFIT-AUDIT.md` must exist with real findings (not placeholders). The agent must be able to name the language, framework, and current test state. If any are unknown, investigate further before proceeding.**

---

## Step 2: Create the Planset (Retrospectively)

### 2.1 Executive Summary
Write `planset/executive-summary/VISION.md` based on:
- Reading the existing README
- Analyzing the codebase purpose
- Conversation with the human

If the project's purpose is unclear, ASK the human. This is a BLOCKER.

### 2.2 Architecture
Document the EXISTING architecture in `planset/architecture/`:
- `SYSTEM_DESIGN.md` — What's actually built (not aspirational)
- `TECH_STACK.md` — What's actually in use
- `DATA_MODEL.md` — Existing schema/data structures

**GATE: VISION.md, SYSTEM_DESIGN.md, and TECH_STACK.md must all exist with substantive content describing the current state. These documents must reflect reality, not aspirations. Verify by cross-referencing against the actual codebase.**

---

## Step 3: Wrap Existing Code in Features

### 3.1 Identify Core Behaviors
For each major module/component, identify:
- What does it do?
- What are the happy paths?
- What are the error cases?

### 3.2 Write Retroactive Features
Create `.feature` files for existing functionality:
```gherkin
@retrofit @module-name
Feature: [Existing Behavior]
  [Document what the code already does]
```

### 3.3 Write Tests for Existing Behavior
Following TDD_RULES but in reverse:
1. Write a test for existing behavior
2. Run it — it SHOULD pass (the code already exists)
3. If it fails, you've found a bug. Log it in backlog.

This creates a safety net before making any changes.

**GATE: Every major module must have at least one `.feature` file documenting its current behavior. Tests for existing behavior must be written and run. Any test failures (bugs) must be logged as backlog items. Do not proceed until the safety net is in place.**

---

## Step 4: Build the Backlog

Based on the audit, create backlog items for:
- Missing test coverage (highest priority)
- Missing documentation
- Identified bugs (from Step 3.3)
- Architectural improvements
- New features the human wants

**GATE: Backlog must have at least 3 items. Items must be prioritized (High/Medium/Low) and sized (S/M/L/XL). Each item must reference either the audit report or a specific gap found during feature wrapping.**

---

## Step 5: Plan Sprint 1

Typically, Sprint 1 of a retrofit focuses on:
1. Getting test coverage to a baseline (at least 50%)
2. Documenting the existing architecture
3. Setting up CI/CD if missing
4. ONE small new feature to demonstrate the workflow

Create `sprints/active/SPRINT-1.md` using the sprint template. Present to human for approval (BLOCKER).

**GATE: Sprint 1 must prioritize test coverage and documentation over new features. The sprint file must exist with at least 2 items. Human must approve.**

---

## Step 6: Resume Normal Workflow

From here, follow `SPRINT_WORKFLOW.md` and `FEATURE_WORKFLOW.md` as normal.

---

## Important Notes

- **DO NOT refactor existing code until it has test coverage**
- **DO NOT delete anything without tests proving it's unused**
- **Respect the existing team's conventions** unless explicitly told to change them
- **The first sprint is about safety nets, not new features**

---

## Retrofit Completion Checklist

**All must pass before retrofit is considered complete and normal workflow begins:**

- [ ] CONFIG.md is populated with accurate project info
- [ ] Audit report exists at `reports/RETROFIT-AUDIT.md`
- [ ] `planset/executive-summary/VISION.md` exists
- [ ] `planset/architecture/SYSTEM_DESIGN.md` describes the real architecture
- [ ] `planset/architecture/TECH_STACK.md` describes the real tech stack
- [ ] Core modules have `.feature` files documenting current behavior
- [ ] Tests exist for existing behavior (safety net)
- [ ] Backlog has prioritized items
- [ ] Sprint 1 is planned and approved
- [ ] Any bugs found during wrapping are logged in backlog
