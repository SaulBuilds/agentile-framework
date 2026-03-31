// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev AIValidator uses Citrea's AI precompiles to validate seed content.
 * Integrates MODEL_INFERENCE (0x0101) to score and validate seed prompts.
 */
contract AIValidator is Ownable {
    using Counters for Counters.Counter;

    // AI Precompile addresses on Citrea
    address public constant MODEL_INFERENCE = address(0x0101);
    address public constant MODEL_METADATA = address(0x0103);
    address public constant PROOF_VERIFY = address(0x0104);

    // Model configurations
    mapping(bytes32 => ModelConfig) public models;
    bytes32[] public registeredModels;

    // Validation records
    struct Validation {
        bytes32 modelId;
        address validator;
        uint256 score;
        bool verified;
        uint256 timestamp;
        bytes32 proofCommitment;
    }

    mapping(bytes32 => Validation[]) public seedValidations;
    mapping(bytes32 => uint256) public validatedSeedCount;

    // Score thresholds
    uint256 public constant MIN_VALIDATION_SCORE = 30;
    uint256 public constant HIGH_QUALITY_THRESHOLD = 70;

    // Trust scoring for validators
    mapping(address => uint256) public validatorTrustScore;
    mapping(address => uint256) public validationCount;

    // Events
    event ModelRegistered(bytes32 indexed modelId, string name, string description);
    event SeedValidated(bytes32 indexed seedHash, bytes32 indexed modelId, uint256 score, address validator);
    event ProofVerified(bytes32 indexed seedHash, bool success, address verifier);
    event ValidatorTrusted(address indexed validator, uint256 newTrustScore);

    struct ModelConfig {
        string name;
        string description;
        bool isActive;
        uint256 minInputLength;
        uint256 maxInputLength;
    }

    constructor() {
        // Register default validation model
        bytes32 defaultModel = keccak256("orchard-default-v1");
        models[defaultModel] = ModelConfig({
            name: "Orchard Default Validator",
            description: "Default model for validating seed content quality",
            isActive: true,
            minInputLength: 1,
            maxInputLength: 1000
        });
        registeredModels.push(defaultModel);
    }

    /**
     * @dev Register a new AI model for validation
     */
    function registerModel(
        string memory name,
        string memory description,
        uint256 minInputLength,
        uint256 maxInputLength
    ) public onlyOwner returns (bytes32) {
        bytes32 modelId = keccak256(abi.encodePacked(name, block.timestamp));

        models[modelId] = ModelConfig({
            name: name,
            description: description,
            isActive: true,
            minInputLength: minInputLength,
            maxInputLength: maxInputLength
        });

        registeredModels.push(modelId);
        emit ModelRegistered(modelId, name, description);

        return modelId;
    }

    /**
     * @dev Validate seed content using AI inference precompile
     */
    function validateSeed(
        bytes32 seedHash,
        bytes memory inputData,
        bytes32 modelId
    ) public returns (uint256) {
        ModelConfig storage model = models[modelId];
        require(model.isActive, "Model not active");

        uint256 inputLen = inputData.length;
        require(inputLen >= model.minInputLength && inputLen <= model.maxInputLength, "Invalid input length");

        // Call MODEL_INFERENCE precompile
        bytes memory payload = abi.encodePacked(
            modelId,
            bytes20(msg.sender),
            inputData
        );

        (bool ok, bytes memory output) = MODEL_INFERENCE.call(payload);

        uint256 score = 0;
        if (ok && output.length > 0) {
            score = extractScoreFromOutput(output);
        }

        // Store validation
        Validation memory validation = Validation({
            modelId: modelId,
            validator: msg.sender,
            score: score,
            verified: score >= MIN_VALIDATION_SCORE,
            timestamp: block.timestamp,
            proofCommitment: keccak256(abi.encodePacked(seedHash, output))
        });

        seedValidations[seedHash].push(validation);
        validatedSeedCount[seedHash]++;

        // Update validator trust score
        if (score >= MIN_VALIDATION_SCORE) {
            validatorTrustScore[msg.sender] += 1;
        } else {
            validatorTrustScore[msg.sender] = validatorTrustScore[msg.sender] > 0 
                ? validatorTrustScore[msg.sender] - 1 
                : 0;
        }
        validationCount[msg.sender]++;

        emit SeedValidated(seedHash, modelId, score, msg.sender);

        return score;
    }

    /**
     * @dev Verify a proof of validation using PROOF_VERIFY precompile
     */
    function verifyProof(
        bytes32 seedHash,
        bytes32 modelId,
        bytes32 commitment,
        bytes32 response,
        bytes memory statement
    ) public returns (bool) {
        bytes memory proofPayload = abi.encodePacked(
            modelId,
            commitment,
            response,
            statement
        );

        (bool ok, bytes memory result) = PROOF_VERIFY.staticcall(proofPayload);

        bool valid = ok && result.length == 1 && result[0] == 0x01;

        emit ProofVerified(seedHash, valid, msg.sender);

        return valid;
    }

    /**
     * @dev Get model metadata using MODEL_METADATA precompile
     */
    function getModelMetadata(bytes32 modelId) public view returns (string memory) {
        (bool ok, bytes memory metadataBytes) = MODEL_METADATA.staticcall(abi.encodePacked(modelId));

        if (ok && metadataBytes.length > 0) {
            return string(metadataBytes);
        }

        return "{}";
    }

    /**
     * @dev Check if seed has high-quality validations
     */
    function isHighQualitySeed(bytes32 seedHash) public view returns (bool) {
        Validation[] storage validations = seedValidations[seedHash];

        uint256 highQualityCount = 0;
        for (uint256 i = 0; i < validations.length; i++) {
            if (validations[i].score >= HIGH_QUALITY_THRESHOLD) {
                highQualityCount++;
            }
        }

        return highQualityCount >= 3;
    }

    /**
     * @dev Get average validation score for a seed
     */
    function getAverageScore(bytes32 seedHash) public view returns (uint256) {
        Validation[] storage validations = seedValidations[seedHash];

        if (validations.length == 0) return 0;

        uint256 totalScore = 0;
        for (uint256 i = 0; i < validations.length; i++) {
            totalScore += validations[i].score;
        }

        return totalScore / validations.length;
    }

    /**
     * @dev Get validator trust score
     */
    function getValidatorTrustScore(address validator) public view returns (uint256) {
        uint256 base = validatorTrustScore[validator];
        uint256 count = validationCount[validator];

        if (count == 0) return 0;

        // Weighted by total validations
        return (base * 100) / count;
    }

    /**
     * @dev Get all registered models
     */
    function getRegisteredModels() public view returns (bytes32[] memory) {
        return registeredModels;
    }

    /**
     * @dev Get validation count for a seed
     */
    function getValidationCount(bytes32 seedHash) public view returns (uint256) {
        return validatedSeedCount[seedHash];
    }

    // Helper to extract score from model output
    function extractScoreFromOutput(bytes memory output) internal pure returns (uint256) {
        if (output.length == 0) return 0;

        // Try to parse as uint256 from the output
        if (output.length >= 32) {
            return uint256(bytes32(output));
        }

        // Fallback: use first byte as score (0-255)
        return uint256(uint8(output[0]));
    }

    // Fallback for receiving AI inference results
    receive() external payable {}
}
