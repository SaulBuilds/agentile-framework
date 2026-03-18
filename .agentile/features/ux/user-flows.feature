@sprint-1 @priority-medium @ux
Feature: Core User Flow Maps
  As a product designer
  I want documented user flows for all core interactions
  So that development teams build consistent, intuitive experiences

  Scenario: Onboarding and wallet link flow
    Given a new user arrives at the platform
    When they complete onboarding
    Then the flow is: landing → register (email/phone) → verify → connect wallet → sign nonce → dashboard
    And optional KYC prompt appears for higher-value pool access

  Scenario: Submit item flow
    Given a registered user has an idle item
    When they submit an item
    Then the flow is: select category → upload photos (min 2) → describe condition → estimate band → review summary → submit
    And the submission is screened against restricted items before finalizing

  Scenario: Print label and QR flow
    Given a user has a confirmed submission
    When they need to ship the item
    Then the flow is: view submission → generate label → print PDF or save QR → view packing instructions
    And the label includes warehouse address and handling notes

  Scenario: Ship or schedule pickup flow
    Given a user has a labeled item ready to ship
    When they arrange shipping
    Then the flow is: choose shipping method → view carrier options → confirm → receive tracking number
    And for local pickup: schedule window → courier assigned → pickup confirmed

  Scenario: Track intake flow
    Given a user has shipped an item
    When they check status
    Then the flow is: view shipment timeline → see carrier tracking → see warehouse received → see grading status → see claim activated or rejection
    And notifications are sent at each major status change

  Scenario: Claim unlock flow
    Given a user's item has been verified at the warehouse
    When their claim activates
    Then the flow is: notification received → view claims dashboard → see active claim with pool details → browse claimable items
    And the claim shows the exact pool and withdrawal rights

  Scenario: Browse and reserve item flow
    Given a user has an active claim
    When they browse their pool
    Then the flow is: open pool inventory → filter/sort items → view item detail (photos, condition, grade) → reserve item → confirm reservation
    And reservation timeout is clearly displayed

  Scenario: Receive item flow
    Given a user has finalized a reservation
    When they complete checkout
    Then the flow is: select delivery method → view fees and ETA → confirm address → track delivery → receive item → confirm receipt
    And proof-of-delivery is recorded

  Scenario: Confirm completion and rate flow
    Given a user has received their item
    When they confirm the trade
    Then the flow is: confirm item received → rate experience (optional) → view trade summary → return to dashboard
    And the trade is recorded in their history
