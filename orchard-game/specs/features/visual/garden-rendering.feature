Feature: Garden Rendering
  # @TLA+ ../tla/SeedLifecycle.tla
  # @tags @visual @core @solo
  
  Background:
    Given the game is running in solo mode (no network required)
    And the p5.js canvas is initialized
    And there is a planted seed with ID "seed-123" at coordinates (100, 150)
    And the seed is in state "PLANTED" with 0 of 5 checkpoints completed

  Scenario: Seed renders as sprout when newly planted
    When the garden rendering system updates
    Then the p5.js canvas shows a small sprout at (100, 150)
    And the sprout is colored gray (representing N/Belnap state)
    And the sprout size is proportional to checkpoint progress (0% growth)

  Scenario: Seed renders as growing plant as checkpoints advance
    Given the seed has advanced to 2 of 5 checkpoints completed
    And the seed state is "GROWING"
    When the garden rendering system updates
    Then the p5.js canvas shows a medium-sized plant at (100, 150)
    And the plant is colored light blue (representing G/Belnap state)
    And the plant size is 40% of mature size

  Scenario: Seed renders as mature plant when ready for harvest
    Given the seed has completed all 5 checkpoints
    And the seed state is "READY"
    When the garden rendering system updates
    Then the p5.js canvas shows a full-sized plant at (100, 150)
    And the plant is colored green (representing T/Belnap state)
    And the plant size is 100% of mature size

  Scenario: Seed renders as wilted when failed validation
    Given the seed has completed 3 of 5 checkpoints
    And the seed state is "F" (false)
    When the garden rendering system updates
    Then the p5.js canvas shows a wilted plant at (100, 150)
    And the plant is colored red (representing F/Belnap state)
    And the plant size is 60% of mature size (retains some growth)

  Scenario: Multiple seeds render correctly in garden
    Given seed "seed-123" is at (100, 150) in state "GROWING" (blue, medium)
    And seed "seed-456" is at (200, 300) in state "T" (green, full)
    And seed "seed-789" is at (150, 100) in state "N" (gray, sprout)
    When the garden rendering system updates
    Then all three seeds appear at their correct coordinates
    And each seed displays the correct color and size for its state
    And the garden background shows the appropriate ecosystem theme

  Scenario: Garden renders federation-specific themes
    Given the federation is "Marine Biology 101"
    When the garden rendering system initializes
    Then the background shows an oceanic theme with blue-green colors
    And the coordinate grid shows depth markers
    And seed labels show marine biology icons

    Given the federation is "Physics 101"
    When the garden rendering system initializes
    Then the background shows a laboratory theme with grayscale colors
    And the coordinate grid shows force vectors
    And seed labels show physics symbols