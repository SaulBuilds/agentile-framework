Feature: Reward Distribution
  # @TLA+ ../tla/FederationEcon.tla
  # @tags @economic @federated
  
  Background:
    Given federation "Marine Biology 101" has a reward pool of 1000 ORT
    And federation "Marine Biology 101" has total growth score of 200 from all seeds
    And Player A has seed with growthScore 100
    And Player B has seed with growthScore 60
    And Player C has seed with growthScore 40

  Scenario: Multiple players harvest from the same reward pool
    When all players harvest in the same block
    Then Player A receives (100/200)*1000 = 500 ORT
    And Player B receives (60/200)*1000 = 300 ORT
    And Player C receives (40/200)*1000 = 200 ORT
    And total distributed = 1000 ORT (no rounding leakage exceeding 1 ORT)
    And the reward pool balance = 0 after distribution

  Scenario: Harvest with zero total federation score
    Given federation "Marine Biology 101" has a reward pool of 500 ORT
    And federation "Marine Biology 101" has total growth score of 0
    And Player A has seed with growthScore 0
    When Player A calls harvest(seed-A)
    Then the transaction reverts with "Cannot harvest from zero total score"
    And no ORT is transferred
    And the reward pool remains unchanged

  Scenario: Harvest exceeds reward pool (due to concurrent updates)
    Given federation "Marine Biology 101" has a reward pool of 100 ORT
    And federation "Marine Biology 101" has total growth score of 50
    And Player A has seed with growthScore 100 (more than total score - invalid, but testing edge case)
    When Player A calls harvest(seed-A)
    Then the transaction reverts with "Invalid growth score"
    And no ORT is transferred

  Scenario: Reward distribution with fractional ORT (handling of remainders)
    Given federation "Marine Biology 101" has a reward pool of 999 ORT
    And federation "Marine Biology 101" has total growth score of 3
    And Player A has seed with growthScore 2
    And Player B has seed with growthScore 1
    When both players harvest
    Then Player A receives floor((2/3)*999) = 666 ORT
    And Player B receives floor((1/3)*999) = 333 ORT
    And total distributed = 999 ORT (no leakage)
    And the reward pool balance = 0