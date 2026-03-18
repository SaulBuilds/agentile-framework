# Epic 5: Claims & Entitlement

**Priority**: High
**Size**: XL
**Scope Reference**: SCOPE.md — Features 6-7 (Claim Activation, Same-Pool Reservation)
**Phase**: 2 (Sprint 8)

## Stories

### Story 5.1: Claim Activation
As a contributor, I want my claim to unlock after my item is verified so I can withdraw from the same pool.

**Acceptance Criteria**
- Claim is not active before verified intake
- Upon successful intake, claim status changes to active
- On-chain PoolClaimToken (ERC-1155) minted for user
- User sees the exact pool ID and withdrawal rights in dashboard
- Ineligible users cannot claim items

### Story 5.2: Browse Claimable Inventory
As a claim holder, I want to browse available items in my pool so I can choose what to claim.

**Acceptance Criteria**
- Only items in the user's entitled pool are shown
- Item details include condition, photos, grading notes
- Items in reserved/outbound status are excluded
- Pagination and sorting supported

### Story 5.3: Reserve Item
As a claim holder, I want to reserve an item from my pool so no one else takes it during checkout.

**Acceptance Criteria**
- Reservation checks claim balance and item availability atomically
- Reservation places item in reserved state
- On-chain EscrowSettlement locks claim
- On-chain InventoryCommitment reserves item
- Reservation expires after configurable timeout (default 24h)
- Claim consumed only on confirmed reservation finalization
- User can cancel reservation (releases both claim and item)

### Story 5.4: Finalize Reservation
As a claim holder, I want to finalize my reservation and trigger delivery.

**Acceptance Criteria**
- User selects delivery method and provides shipping address
- Claim is consumed (burned)
- Outbound shipment created
- On-chain state updated (claim consumed, inventory outbound)
