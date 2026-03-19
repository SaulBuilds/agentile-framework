# BDD Rules -- Behavior-Driven Development

> Gherkin feature specs are **required** for new user-facing features and **optional** for hotfixes and internal refactors.

---

## When to Write Gherkin Features

| Change Type | Gherkin Required? |
|-------------|-------------------|
| New user-facing feature | **YES** |
| New API endpoint | **YES** |
| New CLI command | **YES** |
| GUI workflow change | **YES** |
| Bug fix (hotfix) | No (but recommended for regression) |
| Internal refactor | No |
| Performance optimization | No |
| Documentation-only change | No |

**GATE:** New user-facing features must have a `.feature` file before implementation begins. DO NOT PROCEED to code without a feature spec for qualifying changes.

---

## Gherkin Syntax Standards

### Structure

```gherkin
Feature: <Short description of the capability>
  As a <role>
  I want <goal>
  So that <benefit>

  Background:
    Given <shared precondition>

  Scenario: <Specific behavior>
    Given <precondition>
    When <action>
    Then <expected outcome>

  Scenario Outline: <Parameterized behavior>
    Given <precondition with <param>>
    When <action with <param>>
    Then <expected outcome with <param>>

    Examples:
      | param | expected |
      | value1 | result1 |
      | value2 | result2 |
```

### Rules

- **One feature per file.** Each `.feature` file describes exactly one capability.
- **Scenarios describe behavior, not implementation.** Write "the balance decreases by 10" not "the state_db nonce field increments."
- **Use domain language.** Say "block", "transaction", "score" -- not internal struct names.
- **Keep scenarios independent.** Each scenario must be executable in isolation without depending on another scenario's side effects.
- **Use Background for shared setup.** If every scenario in a file needs the same precondition, put it in Background.
- **Limit scenarios per file.** A feature file with more than 10 scenarios should be split into sub-features.

### Naming Conventions

- File names: `snake_case.feature` (e.g., `wallet_transfer.feature`, `model_registration.feature`)
- Feature titles: sentence case, no trailing period
- Scenario titles: sentence case, describe the specific behavior being tested

---

## File Location Conventions

```
features/
├── <domain-1>/
│   ├── feature_a.feature
│   └── feature_b.feature
├── <domain-2>/
│   ├── feature_c.feature
│   └── feature_d.feature
└── <domain-3>/
    └── feature_e.feature
```

Feature files live in the repository root under `features/`, organized by domain. They do not live inside crate directories.

---

## From Feature to Test

The Gherkin feature spec informs -- but does not replace -- the TDD cycle.

```
Feature Spec (BDD)
    │
    ├── Maps to unit tests (TDD RED phase)
    │
    ├── Maps to integration tests
    │
    └── Maps to E2E tests (optional)
```

**Workflow:**
1. Write the `.feature` file describing the behavior.
2. For each scenario, identify the unit tests needed (RED phase of TDD).
3. Implement using the TDD cycle (RED -> GREEN -> REFACTOR).
4. After all scenarios are covered by passing tests, mark the feature as implemented.

**GATE:** A feature is not considered complete until every scenario in its `.feature` file has at least one corresponding passing test. The mapping from scenario to test should be documented in the feature file as comments or in the sprint `SPRINT.md`.

---

## Tags

Use tags to categorize and filter scenarios:

```gherkin
@critical
Scenario: Critical path behavior
  ...

@smoke
Scenario: Quick verification behavior
  ...

@slow @integration
Scenario: Full cycle with many inputs
  ...
```

| Tag | Meaning |
|-----|---------|
| `@critical` | Must pass for any release |
| `@smoke` | Included in smoke test suite |
| `@slow` | Takes > 5 seconds, excluded from fast feedback loop |
| `@integration` | Requires multiple components running |
| `@wip` | Work in progress, not yet implemented |

---

## Anti-Patterns

| Anti-Pattern | Why It Is Wrong | What to Do Instead |
|--------------|-----------------|---------------------|
| Scenarios that test implementation details | Brittle, break on refactor | Test observable behavior |
| One giant scenario per feature | Hard to pinpoint failures | Split into focused scenarios |
| Scenarios with no assertions | False confidence | Every scenario must have a `Then` step |
| Copying scenarios instead of using Outline | Duplication | Use `Scenario Outline` with `Examples` |
| Skipping feature specs for "simple" features | Scope creep disguised as simplicity | Write the spec; it takes 5 minutes |
