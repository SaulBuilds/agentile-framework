Feature: Growth Cycle
  # @TLA+ ../tla/SeedLifecycle.tla
  # @tags @core @federated
  
  Background:
    Given a player has planted a seed with ID "seed-123" in federation "Marine Biology 101"
    And the seed payload is "Explain coral bleaching"
    And the seed requires 5 checkpoints to mature
    And the seed is currently in state "PLANTED" with 0 checkpoints completed

  Scenario: Seed grows through checkpoint cycles
    When a BFT checkpoint occurs
    Then the GrowthEngine requests inference from 0x1001 for the seed's payload
    And validators return responses
    And Belnap classification assigns the seed a state
    And the seed's checkpoint increments by 1
    And the seed's state becomes "GROWING"
    And the p5.js canvas animates the plant growing taller
    And the plant's color reflects its Belnap state

  Scenario: Seed reaches maturity after required checkpoints
    Given the seed has completed 4 of 5 required checkpoints
    When a BFT checkpoint occurs
    Then the seed's checkpoint increments to 5
    And the seed's state becomes "READY"
    And the p5.js canvas shows the plant as fully grown
    And the player can now call harvest() on the seed
    And the seed is eligible for reward calculation

  Scenario: Seed growth is tracked per federation
    Given federation "Marine Biology 101" has 3 active seeds
    And federation "Physics 101" has 2 active seeds
    When 5 checkpoint cycles occur
    Then all 5 seeds have their checkpoint counters incremented
    And federation statistics are updated accordingly
    And no seeds from other federations are affected