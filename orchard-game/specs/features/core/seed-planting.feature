Feature: Seed Planting
  # @TLA+ ../tla/SeedLifecycle.tla
  # @tags @core @solo @federated
  
  Background:
    Given a player with 100 ORT in their wallet
    And a federation "Marine Biology 101" with minimum stake 10 ORT
    And the federation reward pool has 0 ORT

  Scenario: Player plants a PROMPT seed in an active federation
    When the player plants a PROMPT seed with payload "Explain coral bleaching"
    Then 10 ORT is transferred from player wallet to federation reward pool
    And a SeedNFT is minted to the player's address
    And the seed enters the federation's active pool
    And the p5.js canvas renders a new sprout at the player's garden coordinates
    And the seed state is "PLANTED"
    And the seed has 0 checkpoints completed
    And the seed requires 5 checkpoints to mature

  Scenario: Player plants a seed with insufficient ORT
    Given a player with 5 ORT in their wallet
    When the player attempts to plant a seed
    Then the transaction reverts with "Insufficient ORT for federation minimum"
    And no SeedNFT is minted
    And the p5.js canvas shows a "Not enough sunlight" animation
    And the federation reward pool remains unchanged

  Scenario: Player plants multiple seeds in different federations
    Given a player with 100 ORT in their wallet
    And a federation "Physics 101" with minimum stake 15 ORT
    When the player plants a PROMPT seed with payload "Explain coral bleaching" in "Marine Biology 101"
    And the player plants a PROMPT seed with payload "Explain quantum entanglement" in "Physics 101"
    Then 10 ORT is transferred to "Marine Biology 101" reward pool
    And 15 ORT is transferred to "Physics 101" reward pool
    And two SeedNFTs are minted to the player's address
    And each seed is tracked in its respective federation

  Scenario: Seed cannot be replanted with same ID
    Given a player has planted a seed with ID "seed-123"
    When the player attempts to plant another seed with ID "seed-123"
    Then the transaction reverts with "Seed ID already exists"
    And no additional SeedNFT is minted
    And the federation reward pool count remains unchanged