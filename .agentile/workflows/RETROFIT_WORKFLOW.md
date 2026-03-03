# RETROFIT_WORKFLOW.md — Adopting Agentile in an Existing Repo

> Use this workflow when `.agentile/` is dropped into a project that already has code.

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

## Step 2: Create the Planset (Retrospectively)

### 2.1 Executive Summary
Write `planset/executive-summary/VISION.md` based on:
- Reading the existing README
- Analyzing the codebase purpose
- Conversation with the human

### 2.2 Architecture
Document the EXISTING architecture in `planset/architecture/`:
- `SYSTEM_DESIGN.md` — What's actually built (not aspirational)
- `TECH_STACK.md` — What's actually in use
- `DATA_MODEL.md` — Existing schema/data structures

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
3. If it fails, you've found a bug. Log it.

This creates a safety net before making any changes.

## Step 4: Build the Backlog

Based on the audit, create backlog items for:
- Missing test coverage (highest priority)
- Missing documentation
- Identified bugs
- Architectural improvements
- New features the human wants

## Step 5: Plan Sprint 1

Typically, Sprint 1 of a retrofit focuses on:
1. Getting test coverage to a baseline (at least 50%)
2. Documenting the existing architecture
3. Setting up CI/CD if missing
4. ONE small new feature to demonstrate the workflow

## Step 6: Resume Normal Workflow

From here, follow `SPRINT_WORKFLOW.md` and `FEATURE_WORKFLOW.md` as normal.

## Important Notes

- **DO NOT refactor existing code until it has test coverage**
- **DO NOT delete anything without tests proving it's unused**
- **Respect the existing team's conventions** unless explicitly told to change them
- **The first sprint is about safety nets, not new features**
