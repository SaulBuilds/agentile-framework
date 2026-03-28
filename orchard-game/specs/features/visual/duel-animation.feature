Feature: Duel Animation
  # @TLA+ ../tla/SeedLifecycle.tla
  # @tags @visual @competitive @federated
  
  Background:
    Given federation "Marine Biology 101" is active
    And Player A has an active seed "seed-A" at coordinates (100, 200)
    And Player B has an active seed "seed-B" at coordinates (300, 200)
    And both seeds are in state "GROWING"
    And a Pollination Duel has been initiated between Player A and Player B

  Scenario: Duel initializes with split-screen visualization
    When the duel animation system starts
    Then the p5.js canvas splits vertically at x=200
    And the left side shows Player A's seed environment
    And the right side shows Player B's seed environment
    And both seeds are visible at their respective coordinates
    And a "VS" indicator appears in the center

  Scenario: Duel shows real-time plant growth comparison
    Given both players have submitted their prompts
    And the inference results are being processed
    When the duel animation updates during processing
    Then both plants show growth animations
    And the growth rate reflects the processing speed
    And the canvas shows "Evaluating..." in the center

  Scenario: Duel shows winner with growth bonus visualization
    Given Player A's prompt received higher validator agreement
    When the duel concludes
    Then Player A's seed shows accelerated growth animation
    And Player B's seed continues normal growth animation
    And a trophy appears above Player A's seed
    And growth bonus indicators (e.g., glowing edges) show on Player A's seed
    And the split remains visible but both sides show their respective growth

  Scenario: Duel shows draw outcome visualization
    Given both players' prompts received equal validator agreement
    When the duel concludes
    Then neither seed shows accelerated growth
    And both seeds continue normal growth animation
    And a "=" symbol appears in the center
    And both seeds show "Even Match" indicators

  Scenario: Duel timeout visualization
    Given Player B did not submit a prompt within the time limit
    When the duel times out
    Then Player A's seed shows no growth bonus
    And Player B's seed continues normal growth
    And a "Timeout" message appears above Player B's seed
    And the split-screen visualization fades back to normal garden view

  Scenario: Duel animation returns to normal garden view
    Given a duel has concluded (win, loss, draw, or timeout)
    When the duel animation system completes
    Then the p5.js canvas returns to unified garden view
    And the split-screen divider disappears
    And all seeds are visible in the standard garden layout
    And any duel-specific indicators are removed