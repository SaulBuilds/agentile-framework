// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./GradientRoles.sol";
import "./PoolClaimToken.sol";
import "./InventoryCommitment.sol";

/// @title EscrowSettlement — Handles claim locking, reservation, and finalization
/// @notice Orchestrates the lifecycle: submission lock → claim unlock → reservation → finalization
///         Integrates with PoolClaimToken for claim minting/burning and InventoryCommitment for item state
contract EscrowSettlement is AccessControl, Pausable, ReentrancyGuard {
    // --- Enums ---
    enum ReservationStatus { Active, Finalized, Cancelled, Expired }

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
        ReservationStatus status;
    }

    // --- State ---
    PoolClaimToken public immutable claimToken;
    InventoryCommitment public immutable inventory;

    uint256 public reservationTimeout = 86400; // 24 hours default

    uint256 private _nextReservationId = 1;
    mapping(uint256 => Reservation) private _reservations;

    // Track locked submissions to prevent double-unlock
    mapping(uint256 => bool) private _lockedReceipts;
    mapping(uint256 => bool) private _unlockedReceipts;

    constructor(
        address admin,
        address _claimToken,
        address _inventory
    ) {
        require(_claimToken != address(0), "Invalid claim token");
        require(_inventory != address(0), "Invalid inventory");

        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(GradientRoles.ESCROW_OPERATOR, admin);
        _grantRole(GradientRoles.EMERGENCY_ADMIN, admin);

        claimToken = PoolClaimToken(_claimToken);
        inventory = InventoryCommitment(_inventory);
    }

    /// @notice Set the reservation timeout (admin only)
    /// @param timeout New timeout in seconds
    function setReservationTimeout(uint256 timeout) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(timeout >= 3600, "Timeout too short"); // min 1 hour
        require(timeout <= 604800, "Timeout too long"); // max 7 days
        reservationTimeout = timeout;
    }

    /// @notice Lock a submission pending warehouse intake
    /// @param receiptId The receipt token ID
    /// @param contributor The contributor's address
    function lockOnSubmission(
        uint256 receiptId,
        address contributor
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        require(receiptId > 0, "Invalid receipt ID");
        require(contributor != address(0), "Invalid contributor");
        require(!_lockedReceipts[receiptId], "Already locked");

        _lockedReceipts[receiptId] = true;

        emit SubmissionLocked(receiptId, contributor);
    }

    /// @notice Unlock a claim after verified warehouse intake
    /// @param receiptId The receipt token that was verified
    /// @param contributor The contributor receiving the claim
    /// @param poolId The pool for the claim
    function unlockClaimOnVerifiedIntake(
        uint256 receiptId,
        address contributor,
        uint256 poolId
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        require(_lockedReceipts[receiptId], "Not locked");
        require(!_unlockedReceipts[receiptId], "Already unlocked");
        require(poolId > 0, "Invalid pool ID");

        _unlockedReceipts[receiptId] = true;

        // Interaction: mint claim for contributor
        claimToken.mintClaim(contributor, poolId, 1);

        emit ClaimUnlocked(receiptId, contributor, poolId);
    }

    /// @notice Create a reservation — locks a claim and reserves an inventory item
    /// @param user The user making the reservation
    /// @param poolId The pool to claim from
    /// @param inventoryId The specific inventory item to reserve
    /// @return reservationId The new reservation ID
    function consumeClaimForReservation(
        address user,
        uint256 poolId,
        uint256 inventoryId
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant returns (uint256 reservationId) {
        // Checks
        require(user != address(0), "Invalid user");
        require(claimToken.balanceOf(user, poolId) >= 1, "No claim balance");

        // Effects
        reservationId = _nextReservationId++;
        _reservations[reservationId] = Reservation({
            user: user,
            poolId: poolId,
            inventoryId: inventoryId,
            claimAmount: 1,
            createdAt: block.timestamp,
            expiresAt: block.timestamp + reservationTimeout,
            status: ReservationStatus.Active
        });

        // Interactions: burn claim and reserve inventory
        claimToken.burnClaim(user, poolId, 1);
        inventory.reserveInventory(inventoryId, reservationId);

        emit ReservationCreated(reservationId, user, poolId, inventoryId);
    }

    /// @notice Cancel an active reservation — returns claim to user and releases inventory
    /// @param reservationId The reservation to cancel
    function cancelReservation(
        uint256 reservationId
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        Reservation storage res = _reservations[reservationId];
        require(res.status == ReservationStatus.Active, "Not active");

        // Effects
        res.status = ReservationStatus.Cancelled;

        // Interactions: re-mint claim and release inventory
        claimToken.mintClaim(res.user, res.poolId, 1);
        inventory.releaseReservation(res.inventoryId);

        emit ReservationCancelled(reservationId);
    }

    /// @notice Finalize a reservation after delivery confirmation
    /// @param reservationId The reservation to finalize
    function finalizeWithdrawal(
        uint256 reservationId
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        Reservation storage res = _reservations[reservationId];
        require(res.status == ReservationStatus.Active, "Not active");

        // Check that inventory has been delivered
        require(
            inventory.getInventoryState(res.inventoryId) == InventoryCommitment.InventoryState.Delivered,
            "Item not delivered"
        );

        // Effects
        res.status = ReservationStatus.Finalized;

        emit ReservationFinalized(reservationId);
    }

    /// @notice Expire a reservation that has timed out — permissionless cleanup
    /// @param reservationId The reservation to expire
    function expireReservation(
        uint256 reservationId
    ) external whenNotPaused nonReentrant {
        Reservation storage res = _reservations[reservationId];
        require(res.status == ReservationStatus.Active, "Not active");
        require(block.timestamp > res.expiresAt, "Not expired yet");

        // Effects
        res.status = ReservationStatus.Expired;

        // Interactions: re-mint claim and release inventory
        claimToken.mintClaim(res.user, res.poolId, 1);
        inventory.releaseReservation(res.inventoryId);

        emit ReservationExpired(reservationId);
    }

    /// @notice Emergency pause
    function emergencyPause() external onlyRole(GradientRoles.EMERGENCY_ADMIN) {
        _pause();
    }

    /// @notice Emergency unpause
    function emergencyUnpause() external onlyRole(GradientRoles.EMERGENCY_ADMIN) {
        _unpause();
    }

    // --- View Functions ---

    function getReservation(uint256 reservationId) external view returns (Reservation memory) {
        return _reservations[reservationId];
    }

    function isReceiptLocked(uint256 receiptId) external view returns (bool) {
        return _lockedReceipts[receiptId];
    }

    function isReceiptUnlocked(uint256 receiptId) external view returns (bool) {
        return _unlockedReceipts[receiptId];
    }

    function nextReservationId() external view returns (uint256) {
        return _nextReservationId;
    }
}
