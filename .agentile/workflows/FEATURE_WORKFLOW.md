# FEATURE_WORKFLOW.md — Feature Development Lifecycle

> This is the core loop. Every feature follows this exact path.
> **Every phase has an enforceable gate. Do not advance to the next phase until the current gate passes.**

---

## Entry Gate

**Before starting ANY feature, all conditions must be true. If any fail, stop and resolve.**

- [ ] Project is initialized (planset populated, CONFIG.md filled, sprint active)
- [ ] Feature is listed in the active sprint (`sprints/active/SPRINT-N.md`)
- [ ] Feature traces back to a planset item (can name which SCOPE.md or VISION.md item it serves)
- [ ] Architecture supports this feature (checked `planset/architecture/`)

**GATE: If any checkbox fails, do not begin. If the feature isn't in the sprint, add it to backlog instead. If the planset trace is missing, create it first.**

---

## Phase 1: SPECIFY (Gherkin)

### 1.1 Understand the Requirement
- Read the sprint item description
- Read the relevant planset documents
- Read the architecture docs for the affected components

### 1.2 Write the Feature File
Create a `.feature` file in `.agentile/features/[module]/[feature-name].feature`

Follow `BDD_RULES.md` strictly:
```gherkin
@sprint-N @priority-high @module-name
Feature: [Feature Name]
  As a [role]
  I want [capability]
  So that [business value]

  Scenario: [Happy path]
    Given [context]
    When [action]
    Then [expected result]

  Scenario: [Error handling]
    Given [context]
    When [invalid action]
    Then [error response]
```

### 1.3 Review Feature (OPINION)
Present the feature to the human:
- "I've written the following feature specification. Does this match your expectations?"
- If no response, proceed with the feature as written
- If corrections needed, update and re-present

Commit: `plan: add feature spec for [feature-name]`

**GATE: A `.feature` file must exist with at least one happy-path scenario and one error scenario. The feature must have a planset trace (the As a / I want / So that must connect to a real scope item). Verify the file exists and is well-formed before proceeding.**

---

## Phase 2: RED (Failing Tests)

### 2.1 Write Step Definitions
Map each Gherkin step to test code:
```
Given → setup/arrange
When → action/act
Then → assertion/assert
```

### 2.2 Write Unit Tests
For each component involved in the feature:
- Write tests that describe the expected behavior
- One test per scenario or sub-behavior

### 2.3 Run Tests — Confirm FAILURE
```bash
# Run the specific feature tests
[test command] --filter [feature-name]
```
- ALL new tests MUST fail (they have no implementation yet)
- If any new test passes, it's testing the wrong thing or the implementation already exists. Investigate.

### 2.4 Commit
`test(red): [feature-name] - failing tests for [N] scenarios`

**GATE: All new tests must be shown to fail. Paste or reference the failing test output. If tests pass unexpectedly, investigate and fix before proceeding. Do not move to GREEN until RED is proven.**

---

## Phase 3: GREEN (Implementation)

### 3.1 Implement Minimum Code
- Write the smallest amount of code that makes the tests pass
- Do NOT optimize. Do NOT add unrequested features.
- Follow the architecture docs for patterns and conventions

### 3.2 Run Tests — Confirm PASS
```bash
# Run feature tests
[test command] --filter [feature-name]

# Run FULL suite
[test command]
```
- Feature tests MUST pass
- Full suite MUST pass (no regressions)

### 3.3 Commit
`feat(green): [feature-name] - implementation passing [N] scenarios`

**GATE: ALL tests must pass — both the new feature tests and the entire existing suite. If any test fails, fix it before proceeding. Paste or reference the passing test output as proof. Zero regressions allowed.**

---

## Phase 4: REFACTOR

### 4.1 Review Implementation
- Is the code readable?
- Are there duplications to extract?
- Are functions/methods small and focused?
- Are names clear and descriptive?

### 4.2 Review Tests
- Are tests clear and maintainable?
- Can any test utilities be extracted?
- Are there redundant test setups?

### 4.3 Refactor
Make improvements. After EVERY change:
```bash
[test command]  # Full suite
```
Tests MUST stay green through every refactor step.

### 4.4 Commit
`refactor: [what was improved in feature-name]`

**GATE: Full test suite must pass after refactoring. If any test broke during refactor, you are not done — fix it. Compare test count before and after refactor: no tests should have been removed unless intentionally consolidated (document why).**

---

## Phase 5: VERIFY

### 5.1 Coverage Check
```bash
[coverage command]
```
- Must meet or exceed the target in CONFIG.md
- If below target, write additional tests before proceeding

### 5.2 Quality Gate
Activate QA_ENGINEER role mentally:
- Are edge cases covered?
- Is error handling tested?
- Are boundaries tested?

### 5.3 Integration Check
If the feature touches multiple components:
- Write or update integration tests
- Verify components work together

**GATE: Coverage must meet the CONFIG.md target. If it doesn't, write more tests until it does. All edge cases identified in the Gherkin scenarios must have corresponding test coverage. Do not proceed to DOCUMENT until coverage gate passes.**

---

## Phase 6: DOCUMENT

### 6.1 Update Sprint Tracker
Mark the feature as complete in `sprints/active/SPRINT-N.md`

### 6.2 Update AGENT_NOTES.md
Record:
- Any design decisions made during implementation
- Surprises or issues encountered
- Refactoring opportunities discovered

### 6.3 Update Project Docs
If the feature changes:
- Public API → Update API docs
- Setup requirements → Update SETUP.md
- User-facing behavior → Update CHANGELOG.md

**GATE: Sprint tracker must show the feature as DONE with coverage percentage. AGENT_NOTES.md must have an entry for this feature. Any changed public interfaces must have updated docs. Verify all three before proceeding.**

---

## Phase 7: REPORT

Summarize to the human:
```
Feature: [name]
   Scenarios: [N] passing
   Coverage: [X]%
   Tests: [N] new, [N] total passing

   Notes: [any decisions or observations]

   Next: [next feature in sprint]

   Should I proceed?
```

Wait for "go ahead" (or OPINION-level: proceed if the sprint plan is clear).

**GATE: Report must be presented. If the user gives feedback or corrections, loop back to the appropriate phase and re-execute. A feature is only truly done when the user acknowledges the report (explicitly or by not objecting).**

---

## Exit Gate

Before moving to the next feature, verify the complete chain:

```
[ ] Planset item exists and is referenced
[ ] .feature file exists with scenarios
[ ] Tests exist and all pass
[ ] Implementation is minimal and correct
[ ] Coverage meets target
[ ] Sprint tracker is updated
[ ] AGENT_NOTES.md is updated
[ ] Report was presented to user
```

**If any checkbox fails, the feature is not done. Go back and fix it.**

---

## Repeat
Return to Phase 1 for the next feature in the sprint.
