@sprint-2 @priority-high @contracts
Feature: Smart Contract Interfaces & Invariants
  As a protocol developer
  I want detailed interface specifications for all smart contracts
  So that implementation follows a security-reviewed spec with tested invariants

  Scenario: PoolRegistry manages pool definitions
    Given the protocol needs configurable pools
    When the PoolRegistry contract is deployed
    Then it supports createPool with hue, band, region, and rules
    And it supports updatePoolRules for allowlist, blacklist, and courier policy
    And it supports pausePool and unpausePool per pool
    And only the POOL_ADMIN role can create or modify pools
    And pool creation emits a PoolCreated event

  Scenario: ItemReceiptNFT mints proof of contribution
    Given an item has been verified at the warehouse
    When the ItemReceiptNFT contract mints a receipt
    Then it creates an ERC-721 token bound to the contributor
    And the token metadata includes submission hash, pool ID, and condition grade
    And the token is non-transferable (soul-bound)
    And only the WAREHOUSE_ROLE can mint receipts
    And minting emits a ReceiptMinted event

  Scenario: PoolClaimToken represents withdrawal rights
    Given a contributor's item has been accepted
    When a claim is minted
    Then it creates an ERC-1155 token for the contributor's pool
    And the token transfer policy is set to non-transferable in v1
    And only the CLAIM_MINTER role can mint claims
    And minting emits a ClaimMinted event

  Scenario: EscrowSettlement locks claims during reservation
    Given a user wants to reserve an item
    When the escrow locks the claim
    Then the claim balance is decremented
    And the escrow records the reservation with timeout
    And if the reservation expires the claim is returned
    And only finalization or cancellation releases the escrow
    And finalization burns the claim permanently

  Scenario: InventoryCommitment tracks warehouse item state
    Given warehouse items need on-chain state
    When inventory is registered
    Then it records the item receipt ID and pool ID
    And it supports reserve, release, markOutbound, and confirmDelivered transitions
    And only authorized roles can transition states
    And each transition emits a state change event
    And confirmDelivered requires a proof hash

  Scenario: One-claim-per-verified-item invariant holds
    Given the protocol's core safety property
    When any sequence of transactions is executed
    Then the total active claims never exceeds total verified inventory
    And a claim cannot be minted without a corresponding accepted receipt
    And a claim cannot be consumed without a matching available inventory item
    And this invariant is tested with fuzz and invariant tests

  Scenario: Emergency pause stops all state changes
    Given an emergency is detected
    When the EMERGENCY_ADMIN triggers a global pause
    Then all minting, burning, transferring, and reservation operations revert
    And pause state is recorded with reason and timestamp
    And only EMERGENCY_ADMIN can unpause
