// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./GradientRoles.sol";

/// @title DisputeResolver — On-chain dispute state for the Gradient Barter Protocol
/// @notice Lightweight on-chain dispute tracking. Bulk of dispute logic lives off-chain.
///         Records dispute opening, evidence attachment, and resolution.
contract DisputeResolver is AccessControl, Pausable, ReentrancyGuard {
    // --- Enums ---
    enum ObjectType { Receipt, Inventory, Claim, Shipment, CourierTask }
    enum DisputeStatus { Open, Resolved }
    enum Resolution { None, RefundClaim, Replacement, Denied, GoodwillCredit }

    // --- Events ---
    event DisputeOpened(uint256 indexed disputeId, address indexed actor, ObjectType objectType, uint256 objectId);
    event EvidenceSubmitted(uint256 indexed disputeId, bytes32 cidHash);
    event DisputeResolved(uint256 indexed disputeId, Resolution resolution);

    // --- Structs ---
    struct Dispute {
        address actor;
        ObjectType objectType;
        uint256 objectId;
        DisputeStatus status;
        Resolution resolution;
        uint256 openedAt;
        uint256 resolvedAt;
        bytes32[] evidenceHashes;
    }

    // --- State ---
    uint256 private _nextDisputeId = 1;
    mapping(uint256 => Dispute) private _disputes;
    // Track active disputes per object to prevent duplicates
    mapping(ObjectType => mapping(uint256 => bool)) private _activeDisputes;

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(GradientRoles.DISPUTE_ADMIN, admin);
        _grantRole(GradientRoles.EMERGENCY_ADMIN, admin);
    }

    /// @notice Open a dispute against an object
    /// @param actor The user filing the dispute
    /// @param objectType The type of object being disputed
    /// @param objectId The ID of the disputed object
    /// @param reason Not stored on-chain, but included for event indexing
    /// @return disputeId The new dispute ID
    function openDispute(
        address actor,
        ObjectType objectType,
        uint256 objectId,
        string calldata reason
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused nonReentrant returns (uint256 disputeId) {
        require(actor != address(0), "Invalid actor");
        require(objectId > 0, "Invalid object ID");
        require(!_activeDisputes[objectType][objectId], "Dispute already open");
        require(bytes(reason).length > 0, "Reason required");

        disputeId = _nextDisputeId++;
        _disputes[disputeId].actor = actor;
        _disputes[disputeId].objectType = objectType;
        _disputes[disputeId].objectId = objectId;
        _disputes[disputeId].status = DisputeStatus.Open;
        _disputes[disputeId].resolution = Resolution.None;
        _disputes[disputeId].openedAt = block.timestamp;

        _activeDisputes[objectType][objectId] = true;

        emit DisputeOpened(disputeId, actor, objectType, objectId);
    }

    /// @notice Submit evidence for a dispute
    /// @param disputeId The dispute to attach evidence to
    /// @param cidHash IPFS CID or hash of the evidence
    function submitEvidence(
        uint256 disputeId,
        bytes32 cidHash
    ) external onlyRole(GradientRoles.ESCROW_OPERATOR) whenNotPaused {
        require(_disputes[disputeId].status == DisputeStatus.Open, "Dispute not open");
        require(cidHash != bytes32(0), "Invalid evidence hash");

        _disputes[disputeId].evidenceHashes.push(cidHash);

        emit EvidenceSubmitted(disputeId, cidHash);
    }

    /// @notice Resolve a dispute
    /// @param disputeId The dispute to resolve
    /// @param resolution The resolution outcome
    function resolveDispute(
        uint256 disputeId,
        Resolution resolution
    ) external onlyRole(GradientRoles.DISPUTE_ADMIN) whenNotPaused nonReentrant {
        Dispute storage d = _disputes[disputeId];
        require(d.status == DisputeStatus.Open, "Dispute not open");
        require(resolution != Resolution.None, "Invalid resolution");

        d.status = DisputeStatus.Resolved;
        d.resolution = resolution;
        d.resolvedAt = block.timestamp;

        _activeDisputes[d.objectType][d.objectId] = false;

        emit DisputeResolved(disputeId, resolution);
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

    function getDispute(uint256 disputeId) external view returns (
        address actor,
        ObjectType objectType,
        uint256 objectId,
        DisputeStatus status,
        Resolution resolution,
        uint256 openedAt,
        uint256 resolvedAt,
        uint256 evidenceCount
    ) {
        Dispute storage d = _disputes[disputeId];
        return (d.actor, d.objectType, d.objectId, d.status, d.resolution, d.openedAt, d.resolvedAt, d.evidenceHashes.length);
    }

    function getEvidenceHash(uint256 disputeId, uint256 index) external view returns (bytes32) {
        return _disputes[disputeId].evidenceHashes[index];
    }

    function hasActiveDispute(ObjectType objectType, uint256 objectId) external view returns (bool) {
        return _activeDisputes[objectType][objectId];
    }

    function nextDisputeId() external view returns (uint256) {
        return _nextDisputeId;
    }
}
