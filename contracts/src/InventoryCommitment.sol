// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./GradientRoles.sol";

/// @title InventoryCommitment — Binds warehouse inventory to on-chain availability
/// @notice Tracks state transitions for physical items in the warehouse.
///         State machine: Available → Reserved → Outbound → Delivered
///         Also supports: Quarantined (from any non-delivered state), Released (from Reserved)
contract InventoryCommitment is AccessControl, Pausable, ReentrancyGuard {
    // --- Enums ---
    enum InventoryState { None, Available, Reserved, Outbound, Delivered, Quarantined }

 
    event InventoryRegistered(uint256 indexed inventoryId, uint256 indexed receiptId, uint256 indexed poolId);
    event InventoryReserved(uint256 indexed inventoryId, uint256 indexed reservationId);
    event InventoryReleased(uint256 indexed inventoryId);
    event InventoryOutbound(uint256 indexed inventoryId, bytes32 trackingRef);
    event InventoryDelivered(uint256 indexed inventoryId, bytes32 proofHash);
    event InventoryQuarantined(uint256 indexed inventoryId, string reason);

    // --- Structs ---
    struct InventoryRecord {
        uint256 receiptId;
        uint256 poolId;
        uint256 reservationId;
        InventoryState state;
        bytes32 trackingRef;
        bytes32 proofHash;
    }

    // --- State ---
    uint256 private _nextInventoryId = 1;
    mapping(uint256 => InventoryRecord) private _inventory;
    mapping(uint256 => uint256) private _availableCount; // poolId → count

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(GradientRoles.INVENTORY_OPERATOR, admin);
        _grantRole(GradientRoles.ESCROW_OPERATOR, admin);
        _grantRole(GradientRoles.EMERGENCY_ADMIN, admin);
    }

    /// @notice Register a verified item as available inventory
    /// @param receiptId The on-chain receipt token ID
    /// @param poolId The pool this item belongs to
    /// @return inventoryId The new inventory ID
    function registerInventory(
        uint256 receiptId,
        uint256 poolId
    ) external onlyRole(GradientRoles.INVENTORY_OPERATOR) whenNotPaused nonReentrant returns (uint256 inventoryId) {
        require(receiptId > 0, "Invalid receipt ID");
        require(poolId > 0, "Invalid pool ID");

        inventoryId = _nextInventoryId++;
        _inventory[inventoryId] = InventoryRecord({
            receiptId: receiptId,
            poolId: poolId,
            reservationId: 0,
            state: InventoryState.Available,
            trackingRef: bytes32(0),
            proofHash: bytes32(0)
        });
        _availableCount[poolId]++;

        emit InventoryRegistered(inventoryId, receiptId, poolId);
    }

    /// @notice Reserve an available item (called by EscrowSettlement)
    /// @param inventoryId The item to reserve
    /// @param reservationId The reservation this item is locked for
    function reserveInventory(
        uint256 inventoryId,
        uint256 reservationId
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        InventoryRecord storage inv = _inventory[inventoryId];
        require(inv.state == InventoryState.Available, "Not available");

        inv.state = InventoryState.Reserved;
        inv.reservationId = reservationId;
        _availableCount[inv.poolId]--;

        emit InventoryReserved(inventoryId, reservationId);
    }

    /// @notice Release a reserved item back to available (reservation cancelled/expired)
    /// @param inventoryId The item to release
    function releaseReservation(
        uint256 inventoryId
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant {
        InventoryRecord storage inv = _inventory[inventoryId];
        require(inv.state == InventoryState.Reserved, "Not reserved");

        inv.state = InventoryState.Available;
        inv.reservationId = 0;
        _availableCount[inv.poolId]++;

        emit InventoryReleased(inventoryId);
    }

    /// @notice Mark a reserved item as outbound (shipped from warehouse)
    /// @param inventoryId The item being shipped
    /// @param trackingRef Hash or reference for the shipment
    function markOutbound(
        uint256 inventoryId,
        bytes32 trackingRef
    ) external onlyRole(GradientRoles.INVENTORY_OPERATOR) whenNotPaused nonReentrant {
        InventoryRecord storage inv = _inventory[inventoryId];
        require(inv.state == InventoryState.Reserved, "Not reserved");
        require(trackingRef != bytes32(0), "Invalid tracking ref");

        inv.state = InventoryState.Outbound;
        inv.trackingRef = trackingRef;

        emit InventoryOutbound(inventoryId, trackingRef);
    }

    /// @notice Confirm delivery of an outbound item
    /// @param inventoryId The item delivered
    /// @param proofHash Hash of the delivery proof (photo, signature, etc.)
    function confirmDelivered(
        uint256 inventoryId,
        bytes32 proofHash
    ) external onlyRole(GradientRoles.INVENTORY_OPERATOR) whenNotPaused nonReentrant {
        InventoryRecord storage inv = _inventory[inventoryId];
        require(inv.state == InventoryState.Outbound, "Not outbound");
        require(proofHash != bytes32(0), "Invalid proof hash");

        inv.state = InventoryState.Delivered;
        inv.proofHash = proofHash;

        emit InventoryDelivered(inventoryId, proofHash);
    }

    /// @notice Quarantine an item for safety or integrity concerns
    /// @param inventoryId The item to quarantine
    /// @param reason Why the item is being quarantined
    function quarantineInventory(
        uint256 inventoryId,
        string calldata reason
    ) external onlyRole(GradientRoles.INVENTORY_OPERATOR) whenNotPaused nonReentrant {
        InventoryRecord storage inv = _inventory[inventoryId];
        require(
            inv.state == InventoryState.Available || inv.state == InventoryState.Reserved,
            "Cannot quarantine from current state"
        );

        // If it was available, decrement count
        if (inv.state == InventoryState.Available) {
            _availableCount[inv.poolId]--;
        }

        inv.state = InventoryState.Quarantined;

        emit InventoryQuarantined(inventoryId, reason);
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

    function getInventory(uint256 inventoryId) external view returns (InventoryRecord memory) {
        return _inventory[inventoryId];
    }

    function getInventoryState(uint256 inventoryId) external view returns (InventoryState) {
        return _inventory[inventoryId].state;
    }

    function getAvailableCount(uint256 poolId) external view returns (uint256) {
        return _availableCount[poolId];
    }

    function nextInventoryId() external view returns (uint256) {
        return _nextInventoryId;
    }
}
