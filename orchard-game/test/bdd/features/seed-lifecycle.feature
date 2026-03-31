Feature: Seed Planting and Growth
  As a player,
  I want to plant seeds and watch them grow through checkpoints,
  So that I can earn rewards through the growing process

  @seed @growth
  Scenario: Player plants a seed with valid parameters
    Given the SeedNFT contract is deployed
    When I plant a seed with payload "oak_tree", stake 50, federation 1, maxCheckpoints 5
    Then the seed should be owned by me
    And the seed checkpoint should be 0
    And the seed stake should be 50

  @seed @checkpoint
  Scenario: Player advances seed through checkpoints
    Given I have planted a seed with maxCheckpoints 3
    When I advance the checkpoint 3 times
    Then the seed checkpoint should be 3
    And I should not be able to advance past max checkpoint

  @seed @harvest
  Scenario: Player harvests a mature seed
    Given I have planted a seed with maxCheckpoints 1
    And I have advanced the checkpoint to the maximum
    When I harvest the seed with growth score 85
    Then the seed should be marked as harvested
    And the growth score should be 85

  @seed @failure
  Scenario: Seed fails due to invalid activity
    Given I have planted a seed with maxCheckpoints 3
    When I fail the seed with reason "insufficient_validation"
    Then the seed should be marked as failed
