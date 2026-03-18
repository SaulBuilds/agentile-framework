// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./GradientRoles.sol";

/// @title ItemReceiptNFT — Soul-bound ERC-721 representing verified item contributions
/// @notice Each token represents a physical item submitted to the Gradient Barter Protocol.
///         Tokens are non-transferable (soul-bound to the contributor).
contract ItemReceiptNFT is ERC721, AccessControl, Pausable, ReentrancyGuard {
    // --- Enums ---
    enum IntakeStatus { Pending, Accepted, Rejected, Quarantined }

    // --- Events ---
    event ReceiptMinted(uint256 indexed tokenId, address indexed contributor, uint256 indexed poolId, bytes32 submissionHash);
    event IntakeStatusUpdated(uint256 indexed tokenId, IntakeStatus newStatus);
    event EvidenceAttached(uint256 indexed tokenId, bytes32 cidHash);
    event ItemQuarantined(uint256 indexed tokenId, string reason);

    // --- Structs ---
    struct ReceiptData {
        bytes32 submissionHash;
        uint256 poolId;
        IntakeStatus intakeStatus;
        uint8 conditionGrade;
        bytes32 evidenceHash;
        uint256 mintedAt;
    }

    // --- State ---
    uint256 private _nextTokenId = 1;
    mapping(uint256 => ReceiptData) private _receipts;

    constructor(address admin) ERC721("Gradient Item Receipt", "GRCPT") {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(GradientRoles.WAREHOUSE_ROLE, admin);
        _grantRole(GradientRoles.EMERGENCY_ADMIN, admin);
    }

    /// @notice Mint a receipt for a verified item submission
    /// @param contributor The wallet that submitted the item
    /// @param submissionHash Hash of the off-chain submission data
    /// @param poolId The pool this item is assigned to
    /// @return tokenId The minted token ID
    function mintReceipt(
        address contributor,
        bytes32 submissionHash,
        uint256 poolId
    ) external onlyRole(GradientRoles.WAREHOUSE_ROLE) whenNotPaused nonReentrant returns (uint256 tokenId) {
        // Checks
        require(contributor != address(0), "Invalid contributor");
        require(submissionHash != bytes32(0), "Invalid submission hash");
        require(poolId > 0, "Invalid pool ID");

        // Effects
        tokenId = _nextTokenId++;
        _receipts[tokenId] = ReceiptData({
            submissionHash: submissionHash,
            poolId: poolId,
            intakeStatus: IntakeStatus.Pending,
            conditionGrade: 0,
            evidenceHash: bytes32(0),
            mintedAt: block.timestamp
        });

        // Interactions
        _safeMint(contributor, tokenId);

        emit ReceiptMinted(tokenId, contributor, poolId, submissionHash);
    }

    /// @notice Update the intake status of a receipt
    /// @param tokenId The receipt token to update
    /// @param status The new intake status
    function updateIntakeStatus(
        uint256 tokenId,
        IntakeStatus status
    ) external onlyRole(GradientRoles.WAREHOUSE_ROLE) whenNotPaused nonReentrant {
        require(_ownerOf(tokenId) != address(0), "Token does not exist");

        _receipts[tokenId].intakeStatus = status;

        emit IntakeStatusUpdated(tokenId, status);
    }

    /// @notice Update condition grade for a receipt
    /// @param tokenId The receipt token
    /// @param grade Condition grade (1-10 scale)
    function setConditionGrade(
        uint256 tokenId,
        uint8 grade
    ) external onlyRole(GradientRoles.WAREHOUSE_ROLE) whenNotPaused {
        require(_ownerOf(tokenId) != address(0), "Token does not exist");
        require(grade >= 1 && grade <= 10, "Grade must be 1-10");

        _receipts[tokenId].conditionGrade = grade;
    }

    /// @notice Attach evidence hash (IPFS CID or similar) to a receipt
    /// @param tokenId The receipt token
    /// @param cidHash Hash of the evidence data
    function attachEvidence(
        uint256 tokenId,
        bytes32 cidHash
    ) external onlyRole(GradientRoles.WAREHOUSE_ROLE) whenNotPaused {
        require(_ownerOf(tokenId) != address(0), "Token does not exist");
        require(cidHash != bytes32(0), "Invalid evidence hash");

        _receipts[tokenId].evidenceHash = cidHash;

        emit EvidenceAttached(tokenId, cidHash);
    }

    /// @notice Quarantine an item for safety/authenticity concerns
    /// @param tokenId The receipt token
    /// @param reason Why the item is being quarantined
    function markQuarantined(
        uint256 tokenId,
        string calldata reason
    ) external onlyRole(GradientRoles.WAREHOUSE_ROLE) whenNotPaused nonReentrant {
        require(_ownerOf(tokenId) != address(0), "Token does not exist");
        require(
            _receipts[tokenId].intakeStatus != IntakeStatus.Rejected,
            "Cannot quarantine rejected item"
        );

        _receipts[tokenId].intakeStatus = IntakeStatus.Quarantined;

        emit ItemQuarantined(tokenId, reason);
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

    function getReceiptData(uint256 tokenId) external view returns (ReceiptData memory) {
        require(_ownerOf(tokenId) != address(0), "Token does not exist");
        return _receipts[tokenId];
    }

    function nextTokenId() external view returns (uint256) {
        return _nextTokenId;
    }

    // --- Soul-bound: Block all transfers ---

    function _update(
        address to,
        uint256 tokenId,
        address auth
    ) internal virtual override returns (address) {
        address from = _ownerOf(tokenId);
        // Allow minting (from == address(0)) but block transfers
        if (from != address(0) && to != address(0)) {
            revert("ItemReceiptNFT: non-transferable");
        }
        return super._update(to, tokenId, auth);
    }

    // --- Interface support ---

    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC721, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
