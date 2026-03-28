Feature: Harvest
  # @TLA+ ../tla/SeedLifecycle.tla, ../tla/FederationEcon.tla
  # @tags @core @federated @economic
  
  Background:
    Given a player has planted a seed with ID "seed-123" in federation "Marine Biology 101"
    And the seed payload is "Explain coral bleaching"
    And the seed has completed all 5 required checkpoints
    And the seed is in state "READY"
    And federation "Marine Biology 101" has a reward pool of 1000 ORT
    And federation "Marine Biology 101" has a total growth score of 200 from all seeds

  Scenario: Player harvests a completed seed
    When the player calls harvest(seed-123)
    Then SALT reward is calculated as (growthScore / totalFederationScore) * rewardPool
    And assuming a growthScore of 85 for this seed
    Then reward = (85 / 200) * 1000 = 425 ORT
    And 425 ORT is transferred to the player's wallet
    And the SeedNFT is burned
    And optionally a KnowledgeNFT is minted if growthScore > threshold (e.g., 80)
    And the p5.js canvas shows a harvest animation with particle effects
    And the seed's state becomes "HARVESTED"
    And federation "Marine Biology 101" reward pool decreases by 425 ORT
    And federation "Marine Biology 101" total growth score increases by 85

  Scenario: Harvest fails for immature seed
    Given a seed has only completed 3 of 5 required checkpoints
    And the seed is in state "GROWING"
    When the player calls harvest(seed-123)
    Then the transaction reverts with "Seed not ready for harvest"
    And no ORT is transferred
    And the SeedNFT is not burned
    And the p5.js canvas shows a "Keep growing!" animation

  Scenario: Multiple players harvest from same reward pool
    Given federation "Marine Biology 101" has a reward pool of 1000 ORT
    And federation "Marine Biology 101" has total growth score of 200
    And Player A has seed with growthScore 100
    And Player B has seed with growthScore 60
    And Player C has seed with growthScore 40
    When all players harvest in the same block
    Then Player A receives (100/200)*1000 = 500 ORT
    And Player B receives (60/200)*1000 = 300 ORT
    And Player C receives (40/200)*1000 = 200 ORT
    And total distributed = 1000 ORT (no rounding leakage exceeding 1 ORT)
    And the reward pool balance = 0 after distribution