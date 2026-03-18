@sprint-1 @priority-high @protocol
Feature: Item Policy & Restricted Goods Matrix
  As the protocol operator
  I want a clear item policy defining allowed, excluded, and restricted categories
  So that the network remains safe and legally compliant

  Scenario: Allowed categories are accepted for submission
    Given the protocol has defined allowed categories for v1
    When a user submits an item in an allowed category
    Then the submission proceeds to the intake flow
    And the allowed categories include power tools, home goods, small appliances, non-luxury sneakers and apparel, books and media bundles, gaming consoles and accessories, and bounded-risk collectibles

  Scenario: Excluded categories are blocked at submission
    Given the protocol has defined excluded categories
    When a user attempts to submit an excluded item
    Then the submission is rejected before completion
    And the user sees a clear rejection reason
    And excluded categories include firearms, controlled substances, perishables, recalled products, mattresses, damaged lithium battery devices, stolen or serial-blacklisted items, medical devices, and high-value luxury items

  Scenario: Restricted items require manual review
    Given some items fall into a gray area requiring operator judgment
    When a user submits a restricted-category item
    Then the submission is flagged for manual review
    And the item is assigned to the Red hue (internal review)
    And an operator must approve or reject before intake proceeds

  Scenario: Recalled product is caught during screening
    Given the restricted goods engine checks against recall databases
    When a user submits an item matching a known recall
    Then the submission is blocked
    And the user is informed the item is under recall
    And the recall source is logged

  Scenario: Restricted rules matrix is updated by admin
    Given an admin needs to add a new restriction
    When the admin creates a new restricted item rule
    Then the rule specifies a category pattern, rule type, and description
    And the rule applies to all future submissions
    And the change is audit-logged
