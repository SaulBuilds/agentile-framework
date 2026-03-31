// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev IdeaNFT uses Citrea's AI precompiles to create unique NFT ideas.
 * Uses MODEL_INFERENCE (0x0101) to generate embeddings and MODEL_METADATA (0x0103)
 * to store AI-generated metadata on-chain.
 */
contract IdeaNFT is ERC721URIStorage, Ownable {
    using Counters for Counters.Counter;

    // AI Precompile addresses
    address public constant MODEL_INFERENCE = address(0x0101);
    address public constant MODEL_METADATA = address(0x0103);
    address public constant MODEL_ENCRYPTION = address(0x0106);

    // Token state
    Counters.Counter private _tokenIds;

    struct Idea {
        bytes32 ideaHash;
        bytes32 embeddingHash;
        string prompt;
        bytes32 modelId;
        uint256 qualityScore;
        uint256 rarity;
        bool encrypted;
        uint256 createdAt;
    }

    mapping(uint256 => Idea) public ideas;

    // Embedding collection for similarity search
    mapping(bytes32 => uint256[]) public embeddingIndex;
    bytes32[] public allEmbeddings;

    // Quality tiers
    enum QualityTier { COMMON, UNCOMMON, RARE, EPIC, LEGENDARY }
    mapping(QualityTier => uint256) public qualityThresholds;

    // Events
    event IdeaMinted(
        uint256 indexed tokenId,
        address indexed creator,
        bytes32 ideaHash,
        QualityTier tier
    );
    event IdeaEnhanced(
        uint256 indexed tokenId,
        bytes32 indexed newEmbedding,
        uint256 newQualityScore
    );
    event IdeaEncrypted(
        uint256 indexed tokenId,
        address indexed owner
    );

    constructor() ERC721("IdeaNFT", "IDEA") {
        // Set quality thresholds
        qualityThresholds[QualityTier.COMMON] = 20;
        qualityThresholds[QualityTier.UNCOMMON] = 40;
        qualityThresholds[QualityTier.RARE] = 60;
        qualityThresholds[QualityTier.EPIC] = 80;
        qualityThresholds[QualityTier.LEGENDARY] = 95;
    }

    /**
     * @dev Mint a new idea NFT with AI-generated embedding
     */
    function mintIdea(
        string memory prompt,
        bytes32 modelId,
        bytes memory contextData
    ) public returns (uint256) {
        // Generate embedding using AI inference
        bytes memory payload = abi.encodePacked(
            modelId,
            bytes20(msg.sender),
            abi.encodePacked(prompt, contextData)
        );

        (bool ok, bytes memory embedding) = MODEL_INFERENCE.call(payload);

        bytes32 embeddingHash = keccak256(abi.encodePacked(prompt, block.timestamp));
        uint256 qualityScore = 50;

        if (ok && embedding.length > 0) {
            embeddingHash = keccak256(embedding);
            qualityScore = extractQualityScore(embedding);
        }

        // Determine rarity
        QualityTier tier = _determineTier(qualityScore);
        uint256 rarity = uint256(tier) + 1;

        // Mint token
        uint256 tokenId = _tokenIds.current();
        _tokenIds.increment();

        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, _generateMetadata(prompt, qualityScore, tier));

        // Store idea
        ideas[tokenId] = Idea({
            ideaHash: keccak256(abi.encodePacked(prompt)),
            embeddingHash: embeddingHash,
            prompt: prompt,
            modelId: modelId,
            qualityScore: qualityScore,
            rarity: rarity,
            encrypted: false,
            createdAt: block.timestamp
        });

        // Index embedding
        embeddingIndex[embeddingHash].push(tokenId);
        allEmbeddings.push(embeddingHash);

        emit IdeaMinted(tokenId, msg.sender, ideas[tokenId].ideaHash, tier);

        return tokenId;
    }

    /**
     * @dev Enhance existing idea with AI
     */
    function enhanceIdea(uint256 tokenId, bytes memory newContext) public {
        require(ownerOf(tokenId) == msg.sender, "Not owner");

        Idea storage idea = ideas[tokenId];
        bytes32 modelId = idea.modelId;

        // Get new embedding
        bytes memory payload = abi.encodePacked(
            modelId,
            bytes20(msg.sender),
            abi.encodePacked(idea.prompt, newContext)
        );

        (bool ok, bytes memory embedding) = MODEL_INFERENCE.call(payload);

        if (ok && embedding.length > 0) {
            bytes32 newEmbeddingHash = keccak256(embedding);
            uint256 newScore = extractQualityScore(embedding);

            // Only improve
            if (newScore > idea.qualityScore) {
                idea.embeddingHash = newEmbeddingHash;
                idea.qualityScore = newScore;

                // Update rarity if needed
                QualityTier newTier = _determineTier(newScore);
                idea.rarity = uint256(newTier) + 1;

                emit IdeaEnhanced(tokenId, newEmbeddingHash, newScore);
            }
        }
    }

    /**
     * @dev Encrypt idea metadata using MODEL_ENCRYPTION
     */
    function encryptIdea(uint256 tokenId, address recipient) public {
        require(ownerOf(tokenId) == msg.sender, "Not owner");

        bytes memory payload = abi.encodePacked(
            uint8(0), // encrypt tag
            ideas[tokenId].embeddingHash,
            bytes20(recipient),
            ideas[tokenId].prompt
        );

        (bool ok,) = MODEL_ENCRYPTION.call(payload);

        if (ok) {
            ideas[tokenId].encrypted = true;
            emit IdeaEncrypted(tokenId, recipient);
        }
    }

    /**
     * @dev Find similar ideas using embedding comparison
     */
    function findSimilarIdeas(uint256 tokenId, uint256 limit) public view returns (uint256[] memory) {
        Idea storage targetIdea = ideas[tokenId];
        bytes32 targetEmbedding = targetIdea.embeddingHash;

        // Simple similarity based on XOR distance
        uint256[] memory similar = new uint256[](limit);
        uint256[] memory scores = new uint256[](limit);

        for (uint256 i = 0; i < limit; i++) {
            if (i < allEmbeddings.length) {
                bytes32 compareHash = allEmbeddings[i];
                uint256 similarity = _calculateSimilarity(targetEmbedding, compareHash);
                similar[i] = embeddingIndex[compareHash][0];
                scores[i] = similarity;
            }
        }

        // Sort by similarity (simple bubble sort)
        for (uint256 i = 0; i < limit; i++) {
            for (uint256 j = i + 1; j < limit; j++) {
                if (scores[j] > scores[i]) {
                    (similar[i], similar[j]) = (similar[j], similar[i]);
                    (scores[i], scores[j]) = (scores[j], scores[i]);
                }
            }
        }

        return similar;
    }

    /**
     * @dev Get idea quality score
     */
    function getQualityScore(uint256 tokenId) public view returns (uint256) {
        return ideas[tokenId].qualityScore;
    }

    /**
     * @dev Get quality tier
     */
    function getQualityTier(uint256 tokenId) public view returns (QualityTier) {
        return _determineTier(ideas[tokenId].qualityScore);
    }

    /**
     * @dev Get rarity
     */
    function getRarity(uint256 tokenId) public view returns (uint256) {
        return ideas[tokenId].rarity;
    }

    // Internal helpers
    function _determineTier(uint256 score) internal view returns (QualityTier) {
        if (score >= qualityThresholds[QualityTier.LEGENDARY]) return QualityTier.LEGENDARY;
        if (score >= qualityThresholds[QualityTier.EPIC]) return QualityTier.EPIC;
        if (score >= qualityThresholds[QualityTier.RARE]) return QualityTier.RARE;
        if (score >= qualityThresholds[QualityTier.UNCOMMON]) return QualityTier.UNCOMMON;
        return QualityTier.COMMON;
    }

    function _calculateSimilarity(bytes32 a, bytes32 b) internal pure returns (uint256) {
        bytes32 xorResult = a ^ b;
        uint256 matching = 0;
        for (uint256 i = 0; i < 256; i++) {
            if ((xorResult >> i) == 0) {
                matching++;
            }
        }
        return (matching * 100) / 256;
    }

    function _generateMetadata(string memory prompt, uint256 score, QualityTier tier) 
        internal pure returns (string memory) {
        string memory tierName;
        if (tier == QualityTier.LEGENDARY) tierName = "Legendary";
        else if (tier == QualityTier.EPIC) tierName = "Epic";
        else if (tier == QualityTier.RARE) tierName = "Rare";
        else if (tier == QualityTier.UNCOMMON) tierName = "Uncommon";
        else tierName = "Common";

        return string(abi.encodePacked(
            "data:application/json,{\"name\":\"Idea #",
            _uintToString(score),
            "\",\"description\":\"",
            prompt,
            "\",\"attributes\":[",
            "{\"trait_type\":\"Quality\",\"value\":",
            _uintToString(score),
            "},{\"trait_type\":\"Tier\",\"value\":\"",
            tierName,
            "\"}]"
            "}"
        ));
    }

    function _uintToString(uint256 value) internal pure returns (string memory) {
        if (value == 0) return "0";
        uint256 temp = value;
        uint256 digits;
        while (temp > 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value > 0) {
            digits--;
            buffer[digits] = bytes1(48 + uint8(value % 10));
            value /= 10;
        }
        return string(buffer);
    }

    function extractQualityScore(bytes memory embedding) internal pure returns (uint256) {
        if (embedding.length == 0) return 50;
        if (embedding.length >= 32) {
            uint256 score = uint256(bytes32(embedding)) % 101;
            return score;
        }
        return uint256(uint8(embedding[0])) % 101;
    }

    receive() external payable {}
}
