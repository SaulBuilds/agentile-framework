# Feature: <Feature Name>

> Copy this template for each significant feature. Link it from the sprint WP.

---

## Metadata

| Field | Value |
|-------|-------|
| **Sprint** | S-<ID> |
| **Work Package** | WP-<N> |
| **Author** | <name> |
| **Date** | YYYY-MM-DD |
| **Status** | DRAFT / IN PROGRESS / COMPLETE |

---

## Summary

<One paragraph describing what this feature does, who it serves, and why it matters.>

---

## Specification

### Requirements

1. <Functional requirement 1>
2. <Functional requirement 2>
3. <Functional requirement 3>

### Non-Functional Requirements

- **Performance:** <latency, throughput, resource constraints>
- **Security:** <access control, validation, threat model>
- **Compatibility:** <backward compatibility, migration>

---

## Gherkin Feature Spec (Optional)

> Required for new user-facing features. Optional for internal work.
> If writing a full `.feature` file, place it in `features/<domain>/` and reference the path here.

```gherkin
Feature: <Feature name>
  As a <role>
  I want <goal>
  So that <benefit>

  Scenario: <Happy path>
    Given <precondition>
    When <action>
    Then <expected outcome>

  Scenario: <Error case>
    Given <precondition>
    When <invalid action>
    Then <error behavior>

  Scenario: <Edge case>
    Given <boundary condition>
    When <action>
    Then <expected outcome>
```

---

## Design

### Affected Modules

| Module | Changes |
|--------|---------|
| `<module-name>` | <brief description of changes> |

### Public API Changes

```
// New types/functions/traits being added or modified
```

### Data Flow

```
<input> --> <component A> --> <component B> --> <output>
```

### Alternatives Considered

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| <Option A> | <pros> | <cons> | Chosen / Rejected |
| <Option B> | <pros> | <cons> | Chosen / Rejected |

---

## Test Plan

### Unit Tests

| Test Name | What It Verifies |
|-----------|-----------------|
| `test_<behavior_1>` | <description> |
| `test_<behavior_2>` | <description> |
| `test_<error_case>` | <description> |

### Integration Tests (if applicable)

| Test Name | What It Verifies |
|-----------|-----------------|
| `test_<integration_scenario>` | <description> |

### Property Tests (if applicable)

| Test Name | Property |
|-----------|----------|
| `prop_<invariant>` | <invariant description> |

---

## Implementation Notes

<Any implementation details, gotchas, or decisions made during development that would help a future reader understand the code.>

---

## Checklist

- [ ] Specification complete
- [ ] Gherkin feature spec written (if user-facing)
- [ ] Failing tests written (RED)
- [ ] Implementation complete (GREEN)
- [ ] Code refactored (REFACTOR)
- [ ] Full verification passes (VERIFY)
- [ ] Documentation updated (DOCUMENT)
- [ ] Sprint file updated (REPORT)
