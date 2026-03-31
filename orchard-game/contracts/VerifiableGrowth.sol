// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev VerifiableGrowth uses Citrea's AI precompiles for verifiable seed growth.
 * Integrates PROOF_VERIFY (0x0104) to prove that growth scores were computed correctly.
 */
contract VerifiableGrowth is Ownable {
    // AI Precompile addresses
    address public constant PROOF_VERIFY = address(0x0104);
    address public constant MODEL_INFERENCE = address(0x0101);

    // Growth state
    struct GrowthRecord {
        bytes32 seedHash;
        bytes32 modelId;
        uint256 checkpoint;
        uint256 growthScore;
        bytes32 commitment;
        bytes32 response;
        bool verified;
        uint256 timestamp;
    }

    mapping(bytes32 => GrowthRecord[]) public growthHistory;
    mapping(bytes32 => uint256) public currentCheckpoint;

    // Proof verification thresholds
    uint256 public constant MIN_PROOFS_FOR_VERIFICATION = 2;
    uint256 public verificationThreshold = 1;

    // Growth model configuration
    mapping(bytes32 => GrowthModelConfig) public growthModels;

    // Events
    event GrowthRecorded(
        bytes32 indexed seedHash,
        bytes32 indexed modelId,
        uint256 checkpoint,
        uint256 growthScore
    );
    event GrowthVerified(
        bytes32 indexed seedHash,
        bool indexed success,
        address verifier
    );
    event ModelConfigured(
        bytes32 indexed modelId,
        string name,
        uint256 baseMultiplier,
        uint256 complexityFactor
    );

    struct GrowthModelConfig {
        string name;
        uint256 baseMultiplier;
        uint256 complexityFactor;
        bool isActive;
    }

    constructor() {
        // Default growth model
        bytes32 defaultModel = keccak256("orchard-growth-v1");
        growthModels[defaultModel] = GrowthModelConfig({
            name: "Orchard Default Growth",
            baseMultiplier: 100,
            complexityFactor: 10,
            isActive: true
        });
    }

    /**
     * @dev Configure a growth model
     */
    function configureGrowthModel(
        bytes32 modelId,
        string memory name,
        uint256 baseMultiplier,
        uint256 complexityFactor
    ) public onlyOwner {
        growthModels[modelId] = GrowthModelConfig({
            name: name,
            baseMultiplier: baseMultiplier,
            complexityFactor: complexityFactor,
            isActive: true
        });

        emit ModelConfigured(modelId, name, baseMultiplier, complexityFactor);
    }

    /**
     * @dev Record growth with AI inference and create proof commitment
     */
    function recordGrowth(
        bytes32 seedHash,
        bytes32 modelId,
        bytes memory inputData
    ) public returns (uint256) {
        GrowthModelConfig storage model = growthModels[modelId];
        require(model.isActive, "Model not active");

        // Call AI inference to get growth score
        bytes memory payload = abi.encodePacked(
            modelId,
            bytes20(msg.sender),
            inputData
        );

        (bool ok, bytes memory output) = MODEL_INFERENCE.call(payload);

        uint256 growthScore = 0;
        if (ok && output.length > 0) {
            growthScore = parseGrowthScore(output);
        }

        // Create commitment for proof
        bytes32 commitment = keccak256(abi.encodePacked(
            seedHash,
            modelId,
            growthScore,
            block.timestamp,
            block.prevrandao
        ));

        // Store growth record
        uint256 checkpoint = currentCheckpoint[seedHash] + 1;
        currentCheckpoint[seedHash] = checkpoint;

        GrowthRecord memory record = GrowthRecord({
            seedHash: seedHash,
            modelId: modelId,
            checkpoint: checkpoint,
            growthScore: growthScore,
            commitment: commitment,
            response: bytes32(0),
            verified: false,
            timestamp: block.timestamp
        });

        growthHistory[seedHash].push(record);

        emit GrowthRecorded(seedHash, modelId, checkpoint, growthScore);

        return growthScore;
    }

    /**
     * @dev Verify growth proof using PROOF_VERIFY precompile
     */
    function verifyGrowth(
        bytes32 seedHash,
        uint256 recordIndex,
        bytes32 response,
        bytes memory statement
    ) public returns (bool) {
        require(recordIndex < growthHistory[seedHash].length, "Invalid record index");

        GrowthRecord storage record = growthHistory[seedHash][recordIndex];

        bytes memory proofPayload = abi.encodePacked(
            record.modelId,
            record.commitment,
            response,
            statement
        );

        (bool ok, bytes memory result) = PROOF_VERIFY.staticcall(proofPayload);

        bool valid = ok && result.length == 1 && result[0] == 0x01;

        if (valid) {
            record.verified = true;
        }

        emit GrowthVerified(seedHash, valid, msg.sender);

        return valid;
    }

    /**
     * @dev Batch verify multiple growth records
     */
    function batchVerifyGrowth(
        bytes32 seedHash,
        uint256[] memory recordIndices,
        bytes32[] memory responses,
        bytes[] memory statements
    ) public returns (uint256) {
        require(
            recordIndices.length == responses.length &&
            responses.length == statements.length,
            "Array length mismatch"
        );

        uint256 verifiedCount = 0;

        for (uint256 i = 0; i < recordIndices.length; i++) {
            if (verifyGrowth(seedHash, recordIndices[i], responses[i], statements[i])) {
                verifiedCount++;
            }
        }

        return verifiedCount;
    }

    /**
     * @dev Get growth score with verification
     */
    function getVerifiedGrowthScore(bytes32 seedHash) public view returns (uint256, bool) {
        GrowthRecord[] storage records = growthHistory[seedHash];

        if (records.length == 0) return (0, false);

        uint256 totalScore = 0;
        uint256 verifiedCount = 0;

        for (uint256 i = 0; i < records.length; i++) {
            totalScore += records[i].growthScore;
            if (records[i].verified) {
                verifiedCount++;
            }
        }

        bool verified = verifiedCount >= MIN_PROOFS_FOR_VERIFICATION;

        return (totalScore / records.length, verified);
    }

    /**
     * @dev Get checkpoint count
     */
    function getCheckpointCount(bytes32 seedHash) public view returns (uint256) {
        return currentCheckpoint[seedHash];
    }

    /**
     * @dev Get growth history length
     */
    function getGrowthHistoryLength(bytes32 seedHash) public view returns (uint256) {
        return growthHistory[seedHash].length;
    }

    /**
     * @dev Calculate expected score based on checkpoint and model
     */
    function calculateExpectedScore(
        bytes32 modelId,
        uint256 checkpoint,
        uint256 baseStake
    ) public view returns (uint256) {
        GrowthModelConfig storage model = growthModels[modelId];
        require(model.isActive, "Model not active");

        // Score increases with checkpoint progression
        uint256 checkpointBonus = checkpoint * model.complexityFactor;
        uint256 baseScore = model.baseMultiplier + checkpointBonus;

        // Cap at 100
        return baseScore > 100 ? 100 : baseScore;
    }

    // Helper to parse growth score from AI output
    function parseGrowthScore(bytes memory output) internal pure returns (uint256) {
        if (output.length == 0) return 50; // Default middle score

        if (output.length >= 32) {
            uint256 score = uint256(bytes32(output));
            return score > 100 ? 100 : score;
        }

        // Use first byte, map to 0-100
        return uint256(uint8(output[0])) % 101;
    }

    receive() external payable {}
}
