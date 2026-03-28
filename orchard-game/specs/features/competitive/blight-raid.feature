Feature: Blight Raid
  # @TLA+ ../tla/SeedLifecycle.tla
  # @tags @competitive @federated @adversarial @visual
  
  Background:
    Given federation "Marine Biology 101" has been active for 50+ cycles
    And the federation has 10 active players with seeds
    And the Blight agent triggers occur probabilistically based on checkpoint RNG
    And the defense window is 120 seconds when raid is triggered

  Scenario: The Blight attacks a federation
    Given federation "Marine Biology 101" has been active for 50+ cycles
    When the Blight agent triggers a raid (probabilistic, based on checkpoint RNG)
    Then a 120-second defense window opens
    And the p5.js canvas renders spreading darkness across the garden
    And each active player must submit a CORRECTION or LABEL seed
    And if 60% of active players contribute defenses, the raid is defeated
    And all defenders receive a Mystery Box
    And if the defense fails, growth scores freeze for 3 cycles

  Scenario: Successful Blight defense
    Given federation has 10 active players
    When the Blight raid is triggered
    And 7 players submit CORRECTION or LABEL seeds within 120 seconds
    Then the raid is defeated (70% >= 60% threshold)
    And all 10 players receive a Mystery Box
    And growth continues normally after the raid
    And the p5.js canvas shows the Blight retreating

  Scenario: Failed Blight defense
    Given federation has 10 active players
    When the Blight raid is triggered
    And only 4 players submit CORRECTION or LABEL seeds within 120 seconds
    Then the raid succeeds (40% < 60% threshold)
    And no players receive Mystery Boxes
    And growth scores freeze for 3 checkpoints
    And the p5.js canvas shows the Blight overtaking the garden
    And normal growth resumes after 3 checkpoint cycles

  Scenario: Blight defense requires meaningful contribution
    Given federation has 5 active players
    When the Blight raid is triggered
    And 3 players submit low-quality CORRECTION seeds (gibberish)
    And 2 players submit high-quality LABEL seeds
    Then the raid outcome depends on validation quality, not just submission count
    And the Belnap classification determines if contributions are sufficient