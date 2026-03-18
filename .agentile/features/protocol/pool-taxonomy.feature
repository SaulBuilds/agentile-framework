@sprint-1 @priority-high @protocol
Feature: Pool Taxonomy & Category Matrix
  As the protocol operator
  I want a clearly defined pool taxonomy with hues, value bands, quality tiers, and regions
  So that items are consistently classified and users understand where their goods belong

  Scenario: Pool is defined by hue, band, quality tier, and region
    Given the protocol defines pool families
    When a pool is created
    Then it must have a hue (category domain)
    And it must have a gradient band (value range)
    And it must have a quality tier (condition classification)
    And it may have a regional scope

  Scenario: Hue categories cover initial verticals
    Given the protocol launches with bounded categories
    When a user browses pools
    Then they see Green (tools and home utility)
    And they see Blue (electronics)
    And they see Amber (apparel and footwear)
    And they see Violet (collectibles)
    And they see Teal (household and small appliances)
    And Red (restricted / manual-review) is not publicly claimable

  Scenario: Value bands provide bounded ranges instead of exact pricing
    Given the protocol uses bounded pool bands
    When an item is classified
    Then Band 1 covers $10-$25 equivalent
    And Band 2 covers $25-$75
    And Band 3 covers $75-$200
    And Band 4 covers $200-$500
    And Band 5 covers $500+

  Scenario: Quality tiers classify item condition
    Given items vary in condition
    When an item is graded
    Then it is assigned one of: new, refurbished, used-good, or collectible-graded

  Scenario: User submits item to wrong pool band
    Given a user estimates their item in Band 3
    When the warehouse grades the item as Band 2
    Then the item is assigned to Band 2
    And the user's claim reflects the actual band
    And the user is notified of the reclassification

  Scenario: Pool reaches saturation
    Given a pool has reached its maximum capacity
    When a user tries to submit an item to that pool
    Then the pool is marked as saturated
    And the user is informed that the pool is temporarily full
