@sprint-1 @priority-medium @protocol
Feature: Success Metrics & KPIs
  As the protocol operator
  I want defined KPIs with pilot targets
  So that we can measure whether the system is working and identify problems early

  Scenario: Intake acceptance rate is tracked
    Given items are submitted and graded
    When the metric is calculated
    Then intake acceptance rate = accepted items / total graded items
    And the pilot target is >= 80%

  Scenario: Time to claim unlock is tracked
    Given items move from submission to claim activation
    When the metric is calculated
    Then average time to claim unlock = time from item received to claim active
    And the pilot target is <= 48 hours

  Scenario: Pool fill rate is tracked
    Given items enter pools
    When the metric is calculated
    Then pool fill rate = items added per pool per week
    And the pilot target is at least 5 items per active pool per week

  Scenario: Average dwell time is tracked
    Given items sit in inventory awaiting claims
    When the metric is calculated
    Then average dwell time = time from available to reserved
    And the pilot target is <= 14 days

  Scenario: Reservation to delivery success rate is tracked
    Given reservations are finalized and deliveries attempted
    When the metric is calculated
    Then success rate = successfully delivered / total finalized reservations
    And the pilot target is >= 95%

  Scenario: Dispute rate is tracked
    Given trades complete and some are disputed
    When the metric is calculated
    Then dispute rate = disputes opened / total completed trades
    And the pilot target is <= 5%

  Scenario: Courier completion rate is tracked
    Given courier tasks are created and assigned
    When the metric is calculated
    Then courier completion rate = successfully completed tasks / total accepted tasks
    And the pilot target is >= 95%

  Scenario: Shrinkage rate is tracked
    Given items are stored in warehouses
    When the metric is calculated
    Then shrinkage rate = (items unaccounted for or damaged in warehouse) / total items received
    And the pilot target is <= 1%

  Scenario: Support tickets per 100 trades is tracked
    Given trades complete and support requests are filed
    When the metric is calculated
    Then support rate = support tickets / (completed trades * 100)
    And the pilot target is <= 10 tickets per 100 trades
