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

### Non-Functional Requirements

- **Performance:** <constraints>
- **Security:** <access control, validation>
- **Compatibility:** <backward compatibility>

---

## Gherkin Feature Spec (Optional)

> Required for new user-facing features. Optional for internal work.

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
```

---

## Design

### Affected Modules

| Module | Changes |
|--------|---------|
| `<module>` | <brief description> |

### Alternatives Considered

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| <Option A> | <pros> | <cons> | Chosen / Rejected |

---

## Test Plan

### Unit Tests

| Test Name | What It Verifies |
|-----------|-----------------|
| `test_<behavior>` | <description> |

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
