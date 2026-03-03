# BDD_RULES.md — Behavior-Driven Development Rules

## Gherkin Standards

All features MUST be written in Gherkin syntax and stored in `.agentile/features/` with the `.feature` extension.

### File Naming
```
features/
├── auth/
│   ├── user-login.feature
│   ├── user-registration.feature
│   └── password-reset.feature
├── dashboard/
│   ├── data-display.feature
│   └── user-preferences.feature
```
- Use kebab-case for filenames
- Group by domain/module in subdirectories
- One feature per file (a feature may have multiple scenarios)

### Gherkin Syntax

```gherkin
Feature: [Short description of the feature]
  As a [role]
  I want [capability]
  So that [business value]

  Background:
    Given [shared precondition across scenarios]

  Scenario: [Descriptive name of the scenario]
    Given [initial context]
    And [additional context]
    When [action is performed]
    And [additional action]
    Then [expected outcome]
    And [additional outcome]
    But [exception or negative case]

  Scenario Outline: [Parameterized scenario]
    Given <parameter> is provided
    When the action is performed
    Then the result is <expected>

    Examples:
      | parameter | expected |
      | value1    | result1  |
      | value2    | result2  |
```

### Writing Rules

1. **Given** — Describe the world BEFORE the action. Setup state.
2. **When** — Describe the ACTION. One primary action per scenario.
3. **Then** — Describe the EXPECTED OUTCOME. Observable, testable results.

### Quality Checks
- Every scenario MUST be independently runnable
- No scenario should depend on another scenario's side effects
- Use `Background` for shared setup, not copy-paste
- Scenario names must be unique and descriptive
- Avoid implementation details in Gherkin — describe BEHAVIOR not code
- Write from the USER's perspective, not the system's

### Bad vs Good

**Bad:**
```gherkin
Scenario: Test the API
  Given the database has a user row with id 1
  When I send a POST to /api/auth with JSON body
  Then the response status code is 200
```

**Good:**
```gherkin
Scenario: Registered user can log in with valid credentials
  Given a user has registered with email "dev@example.com"
  When the user logs in with correct credentials
  Then the user is granted access to their dashboard
  And a session is created
```

## Feature Lifecycle

1. **DRAFT**: Agent writes the feature based on planset/roadmap item
2. **REVIEW**: Human approves or requests changes (BLOCKER level)
3. **ACCEPTED**: Feature is locked. Tests are written against it.
4. **IMPLEMENTED**: All scenarios pass.
5. **ARCHIVED**: Feature moves to `completed/` with the sprint.

## Tagging Convention

Use tags to organize and filter:
```gherkin
@sprint-1 @priority-high @auth
Feature: User Login
```

Standard tags:
- `@sprint-N` — Sprint association
- `@priority-high | @priority-medium | @priority-low`
- `@wip` — Work in progress (not ready for testing)
- `@blocked` — Waiting on human input or external dependency
- `@smoke` — Core smoke test scenario
- `@edge-case` — Non-happy-path scenario
