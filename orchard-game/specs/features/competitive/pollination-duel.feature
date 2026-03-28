Feature: Pollination Duel
  # @TLA+ ../tla/SeedLifecycle.tla
  # @tags @competitive @federated @adversarial
  
  Background:
    Given federation "Marine Biology 101" is active
    And Player A has an active seed "seed-A" with payload "Explain photosynthesis"
    And Player B has an active seed "seed-B" with payload "What is photosynthesis?"
    And both seeds are in the same federation
    And both seeds have completed at least 2 checkpoint cycles

  Scenario: Two players duel in the same federation
    When Player A challenges Player B to a Pollination Duel
    And both players submit a prompt within the 60-second window
    Then both prompts are evaluated by on-chain inference at 0x1001
    And the p5.js canvas renders a split-screen plant race animation
    And the response with higher validator agreement wins
    And the winner's seed receives a 1.2x growth multiplier for the next 5 cycles
    And the loser's seed continues growing at normal rate (no penalty)
    And the duel is recorded on-chain for transparency

  Scenario: Duel timeout if player doesn't respond
    Given Player A has challenged Player B to a Pollination Duel
    When Player B does not submit a prompt within 60 seconds
    Then the duel expires
    And Player A's seed receives no growth bonus
    And Player B's seed continues growing normally
    And a "Duel timeout" event is emitted

  Scenario: Duel with equal validator agreement
    Given both players submit prompts
    And validators return equal agreement scores for both responses
    Then neither player receives a growth bonus
    And both seeds continue growing at normal rate
    And a "Draw" outcome is recorded
    And both players may retry after cooldown period

  Scenario: Duel cooldown period
    Given Player A and Player B just completed a Pollination Duel
    When either player attempts to initiate another duel
    Then the transaction reverts with "Duel cooldown active"
    And the cooldown period lasts for 3 checkpoint cycles
    After which duels can be initiated again