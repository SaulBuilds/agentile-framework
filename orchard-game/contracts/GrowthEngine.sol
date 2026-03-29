// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./SeedNFT.sol";

/**
 * @dev GrowthEngine handles seed validation, Belnap logic computation, and growth score calculation.
 * In solo mode, this uses a simplified validation mechanism. In federated mode, it would
 * integrate with the MCP marketplace for inference and validator consensus.
 */
contract GrowthEngine is Ownable {
    SeedNFT public seedNFT;

    // ============ Constants ============
    uint256 public constant VALIDATION_THRESHOLD = 50; // minimum agreement percentage for T state
    uint256 public constant CONTRADICTION_THRESHOLD = 30; // below this is considered F state
    // Between VALIDATION_THRESHOLD and CONTRADICTION_THRESHOLD is B state (both/contradiction)
    // Above VALIDATION_THRESHOLD is T state (true)
    // Note: This is a simplified version; actual Belnap logic is more nuanced

    // ============ Events ============
    event ValidationProcessed(
        uint256 indexed tokenId,
        uint256 checkpoint,
        uint8 belnapState, // 0=N, 1=T, 2=F, 3=B
        uint256 agreementPercentage
    );

    event GrowthScoreCalculated(
        uint256 indexed tokenId,
        uint256 growthScore
    );

    // ============ Constructor ============
    constructor(SeedNFT _seedNFT) {
        seedNFT = _seedNFT;
    }

    // ============ Public Functions ============

    /**
     * @dev Process validation for a seed at a specific checkpoint.
     * In solo mode, this uses a mock validation. In reality, this would
     * collect validator responses and compute Belnap state.
     * @param tokenId The ID of the seed to validate.
     * @param checkpoint The checkpoint number to validate (should match current checkpoint).
     */
    function processValidation(uint256 tokenId, uint256 checkpoint) public {
        require(address(seedNFT) != address(0), "SeedNFT not set");
        require(seedNFT.exists(tokenId), "Seed does not exist");
        require(!seedNFT.isHarvested(tokenId), "Cannot validate harvested seed");
        require(!seedNFT.isFailed(tokenId), "Cannot validate failed seed");
        require(seedNFT.checkpointOf(tokenId) == checkpoint, "Checkpoint mismatch");

        // In solo mode, we simulate validation with a deterministic pseudo-random value
        // based on block timestamp and tokenId to get repeatable results for testing
        uint256 validationValue = uint256(keccak256(abi.encodePacked(block.timestamp, tokenId, checkpoint))) % 100;

        uint8 belnapState;
        if (validationValue >= VALIDATION_THRESHOLD) {
            belnapState = 1; // T (True)
        } else if (validationValue <= CONTRADICTION_THRESHOLD) {
            belnapState = 2; // F (False)
        } else {
            belnapState = 3; // B (Both/Contradiction)
        }
        // N (No information) would be when no validations are present, but we always simulate

        emit ValidationProcessed(tokenId, checkpoint, belnapState, validationValue);

        // If this is the final checkpoint, calculate and set the growth score
        if (checkpoint == seedNFT.maxCheckpointOf(tokenId)) {
            // Growth score is based on validation value (0-100 scale)
            uint256 growthScore = validationValue;
            seedNFT.harvestSeed(tokenId, growthScore);
            emit GrowthScoreCalculated(tokenId, growthScore);
        }
    }

    // ============ View Functions ============

    /**
     * @dev Get the Belnap state description for a validation value.
     * @param validationValue The validation percentage (0-100).
     * @return stateDesc Description of the Belnap state.
     */
    function getBelnapStateDescription(uint256 validationValue)
        public
        pure
        returns (string memory)
    {
        if (validationValue >= VALIDATION_THRESHOLD) {
            return "True (T)";
        } else if (validationValue <= CONTRADICTION_THRESHOLD) {
            return "False (F)";
        } else {
            return "Both/Contradiction (B)";
        }
    }
}