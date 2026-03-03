# Feature Template

Copy this file to `.agentile/features/[module]/[feature-name].feature` and fill in.

```gherkin
@sprint-N @priority-[high|medium|low] @[module]
Feature: [Feature Name]
  As a [role/persona]
  I want [capability/action]
  So that [business value/outcome]

  Background:
    Given [shared precondition if any]

  Scenario: [Happy path - descriptive name]
    Given [initial state/context]
    And [additional context if needed]
    When [the primary action is taken]
    Then [the expected outcome]
    And [additional expected outcome]

  Scenario: [Error/edge case - descriptive name]
    Given [initial state/context]
    When [an invalid or edge-case action is taken]
    Then [the expected error handling/response]

  Scenario Outline: [Parameterized - descriptive name]
    Given <precondition>
    When <action> is performed
    Then the result is <expected_outcome>

    Examples:
      | precondition | action | expected_outcome |
      | value1       | act1   | result1          |
      | value2       | act2   | result2          |
```

## Checklist Before Accepting
- [ ] Feature traces to a planset/roadmap item
- [ ] Scenarios cover happy path, error cases, and edge cases
- [ ] Written from user perspective, not system perspective
- [ ] No implementation details in the Gherkin
- [ ] Each scenario is independently runnable
