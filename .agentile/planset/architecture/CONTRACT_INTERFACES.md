# Smart Contract Interface Specifications

## Overview

7 contracts forming the on-chain layer of Gradient Barter Protocol. All contracts follow CEI (Checks-Effects-Interactions) pattern, use OpenZeppelin access control, and implement emergency pause.

---

## Roles (AccessControl)

```solidity
bytes32 constant POOL_ADMIN = keccak256("POOL_ADMIN");
bytes32 constant WAREHOUSE_ROLE = keccak256("WAREHOUSE_ROLE");
bytes32 constant CLAIM_MINTER = keccak256("CLAIM_MINTER");
bytes32 constant ESCROW_OPERATOR = keccak256("ESCROW_OPERATOR");
bytes32 constant INVENTORY_OPERATOR = keccak256("INVENTORY_OPERATOR");
bytes32 constant DISPUTE_ADMIN = keccak256("DISPUTE_ADMIN");
bytes32 constant COURIER_ADMIN = keccak256("COURIER_ADMIN");
bytes32 constant EMERGENCY_ADMIN = keccak256("EMERGENCY_ADMIN");
```

---

## A. PoolRegistry

Manages pool definitions and rules.

```solidity
interface IPoolRegistry {
    // --- Events ---
    event PoolCreated(uint256 indexed poolId, uint8 hue, uint8 band, string region);
    event PoolRulesUpdated(uint256 indexed poolId, address updatedBy);
    event PoolPaused(uint256 indexed poolId, string reason);
    event PoolUnpaused(uint256 indexed poolId);

    // --- Structs ---
    struct PoolConfig {
        uint8 hue;          // enum: 0=Green, 1=Blue, 2=Amber, 3=Violet, 4=Teal, 5=Red
        uint8 band;         // 1-5
        string region;
        uint8 qualityTier;  // enum: 0=New, 1=Refurbished, 2=UsedGood, 3=Collectible
        bool active;
        bool paused;
    }

    struct PoolRules {
        uint8 transferPolicy;    // 0=blocked, 1=allowlist
        address[] allowlist;
        uint256 maxCapacity;
        uint256 holdingPeriodSeconds;
    }

    // --- Functions ---
    function createPool(uint8 hue, uint8 band, string calldata region, uint8 qualityTier, PoolRules calldata rules) external returns (uint256 poolId);
    // Access: POOL_ADMIN only
    // Checks: valid hue (0-5), valid band (1-5), valid tier (0-3)
    // Effects: stores pool config + rules, increments pool counter
    // Emits: PoolCreated

    function updatePoolRules(uint256 poolId, PoolRules calldata rules) external;
    // Access: POOL_ADMIN only
    // Checks: pool exists, not destroyed
    // Effects: updates rules mapping
    // Emits: PoolRulesUpdated

    function pausePool(uint256 poolId, string calldata reason) external;
    // Access: POOL_ADMIN or EMERGENCY_ADMIN
    // Effects: sets paused = true
    // Emits: PoolPaused

    function unpausePool(uint256 poolId) external;
    // Access: POOL_ADMIN
    // Effects: sets paused = false
    // Emits: PoolUnpaused

    function getPool(uint256 poolId) external view returns (PoolConfig memory);
    function getPoolRules(uint256 poolId) external view returns (PoolRules memory);
    function isPoolActive(uint256 poolId) external view returns (bool);
}
```

---

## B. ItemReceiptNFT (ERC-721, Non-Transferable)

Represents each physical item submission.

```solidity
interface IItemReceiptNFT {
    // --- Events ---
    event ReceiptMinted(uint256 indexed tokenId, address indexed contributor, uint256 indexed poolId, bytes32 submissionHash);
    event IntakeStatusUpdated(uint256 indexed tokenId, uint8 newStatus);
    event EvidenceAttached(uint256 indexed tokenId, bytes32 cidHash);
    event ItemQuarantined(uint256 indexed tokenId, string reason);

    // --- Enums ---
    // IntakeStatus: 0=Pending, 1=Accepted, 2=Rejected, 3=Quarantined

    // --- Structs ---
    struct ReceiptData {
        bytes32 submissionHash;
        uint256 poolId;
        uint8 intakeStatus;
        uint8 conditionGrade;
        bytes32 evidenceHash;
        uint8 restrictionFlags;
        uint256 mintedAt;
    }

    // --- Functions ---
    function mintReceipt(address contributor, bytes32 submissionHash, uint256 poolId) external returns (uint256 tokenId);
    // Access: WAREHOUSE_ROLE only
    // Checks: pool exists, pool not paused, contributor != zero
    // Effects: mints ERC-721, stores receipt data
    // Emits: ReceiptMinted

    function updateIntakeStatus(uint256 tokenId, uint8 status) external;
    // Access: WAREHOUSE_ROLE only
    // Checks: token exists, valid status transition
    // Effects: updates status
    // Emits: IntakeStatusUpdated

    function attachEvidence(uint256 tokenId, bytes32 cidHash) external;
    // Access: WAREHOUSE_ROLE only
    // Checks: token exists
    // Effects: stores evidence hash
    // Emits: EvidenceAttached

    function markQuarantined(uint256 tokenId, string calldata reason) external;
    // Access: WAREHOUSE_ROLE only
    // Checks: token exists, status != Rejected
    // Effects: sets status to Quarantined
    // Emits: ItemQuarantined

    function getReceiptData(uint256 tokenId) external view returns (ReceiptData memory);

    // --- Transfer Override ---
    // All transfers revert — receipts are soul-bound
}
```

---

## C. PoolClaimToken (ERC-1155, Non-Transferable v1)

Represents claim rights by pool.

```solidity
interface IPoolClaimToken {
    // --- Events ---
    event ClaimMinted(address indexed to, uint256 indexed poolId, uint256 amount);
    event ClaimBurned(address indexed from, uint256 indexed poolId, uint256 amount);
    event TransferPolicyUpdated(uint256 indexed poolId, uint8 policy);

    // --- Functions ---
    function mintClaim(address to, uint256 poolId, uint256 qty) external;
    // Access: CLAIM_MINTER only
    // Checks: pool exists, pool active, qty > 0
    // Effects: mints ERC-1155 tokens (poolId as token ID)
    // Emits: ClaimMinted

    function burnClaim(address from, uint256 poolId, uint256 qty) external;
    // Access: ESCROW_OPERATOR only
    // Checks: from has sufficient balance
    // Effects: burns tokens
    // Emits: ClaimBurned

    function setTransferPolicy(uint256 poolId, uint8 policy) external;
    // Access: POOL_ADMIN only
    // Effects: updates transfer policy (0=blocked, 1=allowlist)
    // Emits: TransferPolicyUpdated

    function claimBalance(address account, uint256 poolId) external view returns (uint256);

    // --- Transfer Override ---
    // v1: All transfers revert unless sender/receiver is on pool allowlist
    // _beforeTokenTransfer checks transfer policy
}
```

---

## D. EscrowSettlement

Handles entitlement locking/unlocking during reservation.

```solidity
interface IEscrowSettlement {
    // --- Events ---
    event SubmissionLocked(uint256 indexed receiptId, address indexed contributor);
    event ClaimUnlocked(uint256 indexed receiptId, address indexed contributor, uint256 poolId);
    event ReservationCreated(uint256 indexed reservationId, address indexed user, uint256 poolId, uint256 inventoryId);
    event ReservationCancelled(uint256 indexed reservationId);
    event ReservationFinalized(uint256 indexed reservationId);
    event ReservationExpired(uint256 indexed reservationId);

    // --- Structs ---
    struct Reservation {
        address user;
        uint256 poolId;
        uint256 inventoryId;
        uint256 claimAmount;
        uint256 createdAt;
        uint256 expiresAt;
        uint8 status; // 0=Active, 1=Finalized, 2=Cancelled, 3=Expired
    }

    // --- Functions ---
    function lockOnSubmission(uint256 receiptId) external;
    // Access: ESCROW_OPERATOR
    // Effects: marks receipt as locked (pending intake)
    // Emits: SubmissionLocked

    function unlockClaimOnVerifiedIntake(uint256 receiptId) external;
    // Access: ESCROW_OPERATOR
    // Checks: receipt exists, intake status == Accepted
    // Effects: triggers PoolClaimToken.mintClaim for contributor
    // Emits: ClaimUnlocked

    function consumeClaimForReservation(address user, uint256 poolId, uint256 inventoryId) external returns (uint256 reservationId);
    // Access: ESCROW_OPERATOR
    // Checks: user has claim balance >= 1 for poolId, inventory is available
    // Effects: decrements claim balance (escrow hold, not burn yet), creates reservation with timeout
    // Interactions: calls InventoryCommitment.reserveInventory
    // Emits: ReservationCreated

    function cancelReservation(uint256 reservationId) external;
    // Access: ESCROW_OPERATOR or reservation owner
    // Checks: reservation is Active
    // Effects: returns claim to user, releases inventory
    // Emits: ReservationCancelled

    function finalizeWithdrawal(uint256 reservationId) external;
    // Access: ESCROW_OPERATOR
    // Checks: reservation is Active, delivery confirmed
    // Effects: burns claim permanently, marks reservation Finalized
    // Interactions: calls PoolClaimToken.burnClaim
    // Emits: ReservationFinalized

    function expireReservation(uint256 reservationId) external;
    // Access: anyone (permissionless cleanup)
    // Checks: block.timestamp > reservation.expiresAt, status == Active
    // Effects: returns claim, releases inventory
    // Emits: ReservationExpired

    function getReservation(uint256 reservationId) external view returns (Reservation memory);
}
```

---

## E. InventoryCommitment

Binds warehouse inventory to on-chain availability.

```solidity
interface IInventoryCommitment {
    // --- Events ---
    event InventoryRegistered(uint256 indexed inventoryId, uint256 indexed receiptId, uint256 indexed poolId);
    event InventoryReserved(uint256 indexed inventoryId, uint256 indexed reservationId);
    event InventoryReleased(uint256 indexed inventoryId);
    event InventoryOutbound(uint256 indexed inventoryId, bytes32 trackingRef);
    event InventoryDelivered(uint256 indexed inventoryId, bytes32 proofHash);

    // --- Enums ---
    // InventoryState: 0=Available, 1=Reserved, 2=Outbound, 3=Delivered, 4=Disputed, 5=Quarantined

    // --- Functions ---
    function registerInventory(uint256 receiptId, uint256 poolId) external returns (uint256 inventoryId);
    // Access: INVENTORY_OPERATOR
    // Checks: receipt exists, receipt accepted
    // Effects: creates inventory record as Available
    // Emits: InventoryRegistered

    function reserveInventory(uint256 inventoryId, uint256 reservationId) external;
    // Access: ESCROW_OPERATOR (called by EscrowSettlement)
    // Checks: inventory is Available
    // Effects: sets state to Reserved
    // Emits: InventoryReserved

    function releaseReservation(uint256 inventoryId) external;
    // Access: ESCROW_OPERATOR
    // Checks: inventory is Reserved
    // Effects: sets state back to Available
    // Emits: InventoryReleased

    function markOutbound(uint256 inventoryId, bytes32 trackingRef) external;
    // Access: INVENTORY_OPERATOR
    // Checks: inventory is Reserved
    // Effects: sets state to Outbound, stores tracking ref
    // Emits: InventoryOutbound

    function confirmDelivered(uint256 inventoryId, bytes32 proofHash) external;
    // Access: INVENTORY_OPERATOR
    // Checks: inventory is Outbound, proofHash != 0
    // Effects: sets state to Delivered, stores proof
    // Emits: InventoryDelivered

    function getInventoryState(uint256 inventoryId) external view returns (uint8);
    function getAvailableCount(uint256 poolId) external view returns (uint256);
}
```

---

## F. DisputeResolver

```solidity
interface IDisputeResolver {
    // --- Events ---
    event DisputeOpened(uint256 indexed disputeId, address indexed actor, uint8 objectType, uint256 objectId);
    event EvidenceSubmitted(uint256 indexed disputeId, bytes32 cidHash);
    event DisputeResolved(uint256 indexed disputeId, uint8 resolution);

    // --- Enums ---
    // ObjectType: 0=Receipt, 1=Inventory, 2=Claim, 3=Shipment, 4=CourierTask
    // Resolution: 0=RefundClaim, 1=Replacement, 2=Denied, 3=GoodwillCredit

    // --- Functions ---
    function openDispute(address actor, uint8 objectType, uint256 objectId, string calldata reason) external returns (uint256 disputeId);
    // Access: any authenticated user
    // Checks: object exists, dispute not already open for this object
    // Effects: creates dispute record
    // Emits: DisputeOpened

    function submitEvidence(uint256 disputeId, bytes32 cidHash) external;
    // Access: dispute parties
    // Checks: dispute is open
    // Effects: attaches evidence hash
    // Emits: EvidenceSubmitted

    function resolveDispute(uint256 disputeId, uint8 resolution) external;
    // Access: DISPUTE_ADMIN only
    // Checks: dispute is open
    // Effects: records resolution, triggers side effects (re-mint claim, release inventory, etc.)
    // Emits: DisputeResolved
}
```

---

## G. CourierRewards (Optional, Phase 3)

```solidity
interface ICourierRewards {
    // --- Events ---
    event TaskPosted(uint256 indexed taskId, uint256 fee);
    event TaskAccepted(uint256 indexed taskId, address indexed courier);
    event MilestoneProved(uint256 indexed taskId, bytes32 proofHash);
    event PayoutReleased(uint256 indexed taskId, address indexed courier, uint256 amount);

    // --- Functions ---
    function postDeliveryTask(uint256 taskId, bytes32 pickup, bytes32 dropoff, uint256 fee, uint256 tip) external;
    // Access: COURIER_ADMIN
    // Effects: creates task with fee escrow

    function acceptTask(uint256 taskId) external;
    // Access: approved couriers only
    // Effects: assigns courier to task

    function proveMilestone(uint256 taskId, bytes32 proofHash) external;
    // Access: assigned courier
    // Effects: records proof

    function releasePayout(uint256 taskId) external;
    // Access: COURIER_ADMIN
    // Checks: all milestones proved
    // Effects: transfers fee + tip to courier (pull pattern)
}
```

---

## Critical Invariants (Must Be Fuzz/Invariant Tested)

```
INVARIANT 1: totalActiveClaims(poolId) <= totalVerifiedInventory(poolId)
  - At no point can claims exceed verified available items

INVARIANT 2: For every mintClaim call, there exists a corresponding accepted receipt
  - Claims cannot appear from nothing

INVARIANT 3: reservedInventory + availableInventory = totalRegisteredInventory
  - No inventory can be in an undefined state

INVARIANT 4: A consumed claim always maps to exactly one delivered inventory item
  - No double-spending of claims

INVARIANT 5: Expired reservations return both the claim and the inventory to available state
  - No resources permanently locked by timeout
```

---

## Security Patterns (All Contracts)

- ReentrancyGuard on all state-changing functions
- AccessControl for role-based permissions
- Pausable with EMERGENCY_ADMIN role
- Pull payment pattern for CourierRewards payouts
- Explicit state machines (no implicit transitions)
- No unbounded loops over dynamic arrays
- All user-facing signatures use EIP-712 typed data
- No delegatecall
- No selfdestruct
