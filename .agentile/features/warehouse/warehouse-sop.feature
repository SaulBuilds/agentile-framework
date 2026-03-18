@sprint-1 @priority-high @warehouse
Feature: Warehouse Standard Operating Procedures v1
  As a warehouse operator
  I want standardized procedures for receiving, grading, storing, and managing items
  So that the chain of custody is maintained and claims unlock only after proper verification

  Scenario: Inbound item is received and scanned
    Given an item arrives at the warehouse
    When the operator scans the QR label
    Then the submission record loads with item details, photos, and estimated band
    And the intake timestamp is recorded
    And the operator's identity is logged as the receiving actor

  Scenario: Item passes grading and is accepted
    Given an operator has scanned an inbound item
    When the operator grades the item as meeting pool standards
    Then the operator records condition grade, photos, notes, and final pool band
    And the operator assigns a bin location
    And the intake status is set to accepted
    And the claim service is notified to activate the contributor's claim

  Scenario: Item fails grading and is rejected
    Given an operator has scanned an inbound item
    When the operator determines the item does not meet pool standards
    Then the operator records rejection reason with evidence
    And the intake status is set to rejected
    And no claim is activated
    And the contributor is notified with the rejection reason

  Scenario: Item is quarantined for safety concerns
    Given an operator suspects an item is unsafe or counterfeit
    When the operator quarantines the item
    Then a mandatory reason is recorded
    And the item is moved to the quarantine lane
    And no claim is activated
    And an admin alert is generated for severe categories

  Scenario: Chain of custody is maintained throughout
    Given an item is in the warehouse
    When any status change occurs
    Then the change records: actor, timestamp, previous status, new status, and evidence hash
    And CCTV coverage is available for the intake area
    And all evidence is retained per the data retention policy

  Scenario: High-value items require additional controls
    Given an item is classified in Band 4 or Band 5
    When the item enters the warehouse
    Then dual-control verification is required (two operators)
    And the item is stored in a controlled-access high-value cage
    And additional photographic evidence is captured

  Scenario: Employee role separation is enforced
    Given warehouse security requires separation of duties
    When operators are assigned roles
    Then the receiving operator and the grading operator should be different people when possible
    And access to high-value cages requires explicit authorization
    And all role assignments are audit-logged
