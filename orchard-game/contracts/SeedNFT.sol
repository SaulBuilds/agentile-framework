// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev SeedNFT represents a planted seed in the Orchard Game.
 * Each seed is an NFT that tracks its growth through checkpoints.
 */
contract SeedNFT is ERC721URIStorage, Ownable {
    using Counters for Counters.Counter;

    // ============ Constants ============
    uint256 public constant MAX_CHECKPOINTS = 1000;
    uint256 public constant MIN_STAKE = 10; // minimum ORT to plant a seed

    // ============ State Variables ============
    Counters.Counter private _tokenIds;

    // Seed properties
    mapping(uint256 => address) private _seedPlanter; // planter address
    mapping(uint256 => uint256) private _seedStake; // stake amount
    mapping(uint256 => uint256) private _seedFederation; // federation ID
    mapping(uint256 => uint256) private _seedCheckpoint; // current checkpoint (0-maxCheckpoints)
    mapping(uint256 => uint256) private _seedMaxCheckpoint; // max checkpoints for growth
    mapping(uint256 => uint256) private _seedGrowthScore; // final score at harvest (0-100)
    mapping(uint256 => bool) private _seedIsHarvested; // whether seed has been harvested
    mapping(uint256 => bool) private _seedIsFailed; // whether seed has failed validation

    // ============ Events ============
    event SeedPlanted(
        uint256 indexed tokenId,
        address indexed planter,
        uint256 stake,
        uint256 federation,
        uint256 maxCheckpoints
    );

    event SeedAdvanced(
        uint256 indexed tokenId,
        uint256 newCheckpoint
    );

    event SeedHarvested(
        uint256 indexed tokenId,
        uint256 growthScore
    );

    event SeedFailed(
        uint256 indexed tokenId,
        string reason
    );

    // ============ Constructor ============
    constructor() ERC721("SeedNFT", "SEED") {}

    // ============ Public Functions ============

    /**
     * @dev Plant a new seed.
     * @param _payload The IPFS hash or content identifier for the seed's prompt/data.
     * @param _stake The amount of ORT to stake (must be >= MIN_STAKE).
     * @param _federation The federation ID where the seed is planted.
     * @param _maxCheckpoints The number of checkpoints required for growth (1-MAX_CHECKPOINTS).
     * @return tokenId The ID of the newly planted seed.
     */
    function plantSeed(
        string calldata _payload,
        uint256 _stake,
        uint256 _federation,
        uint256 _maxCheckpoints
    ) public returns (uint256) {
        require(_stake >= MIN_STAKE, "Stake too low");
        require(_maxCheckpoints > 0 && _maxCheckpoints <= MAX_CHECKPOINTS, "Invalid max checkpoints");

        uint256 tokenId = _tokenIds.current();
        _tokenIds.increment();

        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, _payload);

        _seedPlanter[tokenId] = msg.sender;
        _seedStake[tokenId] = _stake;
        _seedFederation[tokenId] = _federation;
        _seedCheckpoint[tokenId] = 0;
        _seedMaxCheckpoint[tokenId] = _maxCheckpoints;
        _seedGrowthScore[tokenId] = 0;
        _seedIsHarvested[tokenId] = false;
        _seedIsFailed[tokenId] = false;

        emit SeedPlanted(tokenId, msg.sender, _stake, _federation, _maxCheckpoints);

        return tokenId;
    }

    /**
     * @dev Advance the seed by one checkpoint.
     * @param tokenId The ID of the seed to advance.
     */
    function advanceCheckpoint(uint256 tokenId) public {
        require(_exists(tokenId), "Seed does not exist");
        require(!_seedIsHarvested[tokenId], "Seed already harvested");
        require(!_seedIsFailed[tokenId], "Seed has failed");
        require(_seedCheckpoint[tokenId] < _seedMaxCheckpoint[tokenId], "Seed already at max checkpoint");

        unchecked {
            _seedCheckpoint[tokenId] += 1;
        }

        emit SeedAdvanced(tokenId, _seedCheckpoint[tokenId]);

        // If we've reached max checkpoint, the seed is ready for harvest
        // (In a full implementation, validation would happen here via GrowthEngine)
    }

    /**
     * @dev Harvest a mature seed.
     * @param tokenId The ID of the seed to harvest.
     * @param _growthScore The final growth score (0-100) determined by validation.
     */
    function harvestSeed(uint256 tokenId, uint256 _growthScore) public {
        require(_exists(tokenId), "Seed does not exist");
        require(_seedCheckpoint[tokenId] == _seedMaxCheckpoint[tokenId], "Seed not mature");
        require(!_seedIsHarvested[tokenId], "Seed already harvested");
        require(!_seedIsFailed[tokenId], "Seed has failed");
        require(_growthScore <= 100, "Invalid growth score");

        _seedIsHarvested[tokenId] = true;
        _seedGrowthScore[tokenId] = _growthScore;

        emit SeedHarvested(tokenId, _growthScore);
    }

    /**
     * @dev Mark a seed as failed validation.
     * @param tokenId The ID of the seed that failed.
     * @param _reason The reason for failure.
     */
    function failSeed(uint256 tokenId, string memory _reason) public {
        require(_exists(tokenId), "Seed does not exist");
        require(!_seedIsHarvested[tokenId], "Cannot harvest a failed seed");
        require(!_seedIsFailed[tokenId], "Seed already failed");

        _seedIsFailed[tokenId] = true;

        emit SeedFailed(tokenId, _reason);
    }

    // ============ View Functions ============

    function planterOf(uint256 tokenId) public view returns (address) {
        return _seedPlanter[tokenId];
    }

    function stakeOf(uint256 tokenId) public view returns (uint256) {
        return _seedStake[tokenId];
    }

    function federationOf(uint256 tokenId) public view returns (uint256) {
        return _seedFederation[tokenId];
    }

    function checkpointOf(uint256 tokenId) public view returns (uint256) {
        return _seedCheckpoint[tokenId];
    }

    function maxCheckpointOf(uint256 tokenId) public view returns (uint256) {
        return _seedMaxCheckpoint[tokenId];
    }

    function growthScoreOf(uint256 tokenId) public view returns (uint256) {
        return _seedGrowthScore[tokenId];
    }

    function isHarvested(uint256 tokenId) public view returns (bool) {
        return _seedIsHarvested[tokenId];
    }

    function isFailed(uint256 tokenId) public view returns (bool) {
        return _seedIsFailed[tokenId];
    }

    // ============ Internal Overrides ============

    function _burn(uint256 tokenId) internal override(ERC721) {
        super._burn(tokenId);
    }

    // ============ ERC721URIStorage Overrides ============

    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string)
    {
        return super.tokenURI(tokenId);
    }
}