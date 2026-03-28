# Agentile Journaling & Introspection Rules

## Purpose

Sprint journals are the institutional memory of a project. They capture what the numbers cannot: why decisions were made, what surprised the team, what technical debt was created, and what the honest state of the work is.

Code tells you WHAT was built. Tests tell you IF it works.
Journals tell you WHY it matters and WHERE it is fragile.

## Rule 1: Every Sprint Gets a Journal

**Trigger:** The last WP of a sprint is committed.
**Action:** STOP. Write a journal BEFORE starting the next sprint.
**Gate:** The journal commit must exist between the last WP commit and the first commit of the next sprint.

## Rule 2: Ask the User for Angle

Before writing, ask:
- "Sprint X is complete. What angle do you want in the journal?"
- "Any hiccups or themes to cover?"
- "Anything that felt off while watching the build?"

The user's perspective is the journal's most valuable input.

## Rule 3: Required Sections

Every sprint journal MUST contain:

### What Was Delivered
- Table of WPs with lines added, tests, commit hashes
- Total metrics (lines, tests, files)

### What's Wrong (Honest Assessment)
- Technical debt created or carried forward
- Missing tests, mock data, unconnected wiring
- Architectural problems exposed
- Commit hygiene issues

### What I Learned
- Surprising outcomes (positive or negative)
- Patterns that worked well
- Patterns that should be avoided

### What's Next
- Preview of next sprint's WPs
- Proposed additions based on this retro
- Open questions for the user

### Timestamp
- UTC timestamp
- Branch name
- Key commit hashes

## Rule 4: Be Direct About Problems

Do NOT:
- Bury issues in positive language
- Say "minor issue" when it is a real gap
- Skip mentioning fragile code
- Pretend mock data is real data

DO:
- State the problem clearly
- Quantify it (how many files, how many missing tests)
- Propose the fix with effort estimate
- Let the user prioritize

## Rule 5: The Bug-to-Feature Question

For every problem identified, ask: "Is this a bug that could become a feature?" Technical debt often reveals architectural opportunities.

## Rule 6: Configurable by Project

Projects using Agentile can adjust these rules:

| Setting | Default | Options |
|---------|---------|---------|
| journal_frequency | every_sprint | every_sprint, every_week, every_milestone |
| required_sections | all 5 | any subset |
| ask_user_angle | true | true, false (auto-write) |
| honesty_level | direct | direct, diplomatic, minimal |
| timestamp_format | UTC ISO 8601 | any |
| journal_location | .agentile/docs/journals/ | configurable path |
| naming_convention | YYYY-MM-DD_TITLE.md | configurable |

## Why This Matters

Without journals, the commit history tells you a struct changed in 59 files. The journal tells you WHY that happened and that it should become a builder pattern. That is the difference between a git log and institutional knowledge.
