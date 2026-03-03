# FEATURE_WORKFLOW.md — Feature Development Lifecycle

> This is the core loop. Every feature follows this exact path.

## Prerequisite Checklist
Before starting, confirm:
- [ ] Feature is in the active sprint (`sprints/active/`)
- [ ] Feature traces back to a planset item
- [ ] Architecture supports this feature (check `planset/architecture/`)

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
- ALL tests MUST fail (they have no implementation yet)
- If any test passes, it's testing the wrong thing. Fix it.

### 2.4 Commit
`test(red): [feature-name] - failing tests for [N] scenarios`

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

## Phase 7: REPORT

Summarize to the human:
```
✅ Feature: [name]
   Scenarios: [N] passing
   Coverage: [X]%
   Tests: [N] new, [N] total passing
   
   Notes: [any decisions or observations]
   
   Next: [next feature in sprint]
   
   Should I proceed?
```

Wait for "go ahead" (or OPINION-level: proceed if the sprint plan is clear).

## Repeat
Return to Phase 1 for the next feature in the sprint.
