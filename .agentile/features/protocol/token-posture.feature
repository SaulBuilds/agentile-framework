@sprint-1 @priority-high @protocol
Feature: Token & Legal Posture
  As the protocol operator
  I want a clearly defined token and legal posture
  So that pool claims are properly characterized as barter entitlements and regulatory risk is minimized

  Scenario: Pool claims are barter entitlements, not securities
    Given the protocol issues pool claim tokens
    When a claim is minted for a user
    Then it represents a specific contractual right to withdraw from a defined item pool
    And it does not represent a share of enterprise profits
    And no promise of appreciation is made

  Scenario: Claims are non-cash redeemable in v1
    Given the v1 protocol restricts claim usage
    When a user holds an active claim
    Then the claim can only be used to reserve a same-pool item
    And the claim cannot be redeemed for cash
    And the claim cannot be sold on an open marketplace

  Scenario: Claims are non-transferable in v1
    Given the v1 protocol restricts claim transfers
    When a user attempts to transfer a claim
    Then the transfer is blocked by the ERC-1155 transfer policy
    And the user is informed that claims are non-transferable in v1

  Scenario: Protocol avoids loan and collateral language
    Given the protocol is a barter network, not a lending platform
    When any user-facing content references claims
    Then the language uses: contribute, claim, pool, withdraw, entitlement
    And the language never uses: loan, collateral, interest, redemption, investment, returns

  Scenario: Onboarding includes compliance disclosures
    Given barter exchanges may have tax and reporting obligations
    When a new user completes onboarding
    Then they see disclosures about potential tax obligations
    And they acknowledge that barter may be reportable
    And the protocol does not market itself as tax-free or profit-generating
