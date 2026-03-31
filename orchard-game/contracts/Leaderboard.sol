// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev Leaderboard tracks player and federation rankings.
 * Rankings are based on growth scores, harvest counts, and participation.
 */
contract Leaderboard is Ownable {
    using Counters for Counters.Counter;

    // ============ Constants ============
    uint256 public constant MAX_LEADERBOARD_SIZE = 1000;
    uint256 public constant MIN_SCORE_THRESHOLD = 1;

    // ============ State Variables ============
    Counters.Counter private _seasonIds;

    // Player rankings
    struct PlayerRecord {
        address player;
        uint256 totalScore;
        uint256 harvestCount;
        uint256 averageScore;
        uint256 participationStreak;
        uint256 lastActiveSeason;
    }
    
    mapping(uint256 => mapping(address => PlayerRecord)) private _seasonPlayerRecords;
    mapping(uint256 => address[]) private _seasonPlayerRanking;
    mapping(bytes32 => uint256) private _playerSeasonRank;

    // Federation rankings
    struct FederationRecord {
        uint256 federationId;
        uint256 totalScore;
        uint256 memberCount;
        uint256 averageScore;
        uint256 harvestCount;
    }
    
    mapping(uint256 => mapping(uint256 => FederationRecord)) private _seasonFederationRecords;
    mapping(uint256 => uint256[]) private _seasonFederationRanking;
    mapping(bytes32 => uint256) private _federationSeasonRank;

    // Global (all-time) rankings
    address[] private _globalPlayerRanking;
    mapping(address => uint256) private _globalPlayerScore;
    mapping(address => uint256) private _globalHarvestCount;
    mapping(address => uint256) private _globalPlayerRank;

    uint256[] private _globalFederationRanking;
    mapping(uint256 => uint256) private _globalFederationScore;
    mapping(uint256 => uint256) private _globalFederationRank;

    // Current season tracking
    uint256 public currentLeaderboardSeason;
    bool public seasonalResetsEnabled;

    // ============ Events ============
    event PlayerScoreUpdated(
        uint256 indexed seasonId,
        address indexed player,
        uint256 newScore,
        uint256 newRank
    );

    event FederationScoreUpdated(
        uint256 indexed seasonId,
        uint256 indexed federationId,
        uint256 newScore,
        uint256 newRank
    );

    event SeasonReset(
        uint256 indexed oldSeasonId,
        uint256 indexed newSeasonId
    );

    event GlobalRankingUpdated(
        address indexed player,
        uint256 newGlobalRank
    );

    // ============ Constructor ============
    constructor() {
        currentLeaderboardSeason = 0;
        seasonalResetsEnabled = true;
    }

    // ============ Player Functions ============

    /**
     * @dev Update player score for current season.
     */
    function updatePlayerScore(address player, uint256 score, uint256 federationId) public onlyOwner {
        require(score >= MIN_SCORE_THRESHOLD, "Score too low");
        
        uint256 seasonId = currentLeaderboardSeason;
        
        PlayerRecord storage record = _seasonPlayerRecords[seasonId][player];
        
        record.totalScore += score;
        record.harvestCount++;
        record.averageScore = record.totalScore / record.harvestCount;
        record.lastActiveSeason = seasonId;
        
        _updatePlayerRanking(seasonId, player);
        
        _globalPlayerScore[player] += score;
        _globalHarvestCount[player]++;
        _updateGlobalPlayerRanking(player);
        
        if (federationId > 0) {
            _updateFederationScore(seasonId, federationId, score);
        }
        
        bytes32 rankKey = keccak256(abi.encodePacked(seasonId, player));
        emit PlayerScoreUpdated(seasonId, player, record.totalScore, _playerSeasonRank[rankKey]);
    }

    /**
     * @dev Get player rank for a season.
     */
    function getPlayerSeasonRank(uint256 seasonId, address player) public view returns (uint256) {
        bytes32 rankKey = keccak256(abi.encodePacked(seasonId, player));
        return _playerSeasonRank[rankKey];
    }

    /**
     * @dev Get top N players for a season.
     */
    function getTopPlayers(uint256 seasonId, uint256 n) public view returns (address[] memory, uint256[] memory) {
        address[] storage ranking = _seasonPlayerRanking[seasonId];
        uint256 limit = n > ranking.length ? ranking.length : n;
        
        address[] memory topPlayers = new address[](limit);
        uint256[] memory scores = new uint256[](limit);
        
        for (uint256 i = 0; i < limit; i++) {
            topPlayers[i] = ranking[i];
            scores[i] = _seasonPlayerRecords[seasonId][ranking[i]].totalScore;
        }
        
        return (topPlayers, scores);
    }

    /**
     * @dev Get player record for a season.
     */
    function getPlayerRecord(uint256 seasonId, address player) public view returns (
        uint256 totalScore,
        uint256 harvestCount,
        uint256 averageScore,
        uint256 participationStreak
    ) {
        PlayerRecord storage record = _seasonPlayerRecords[seasonId][player];
        return (
            record.totalScore,
            record.harvestCount,
            record.averageScore,
            record.participationStreak
        );
    }

    // ============ Federation Functions ============

    function _updateFederationScore(uint256 seasonId, uint256 federationId, uint256 score) internal {
        FederationRecord storage record = _seasonFederationRecords[seasonId][federationId];
        
        record.totalScore += score;
        record.harvestCount++;
        
        if (record.harvestCount > 0) {
            record.averageScore = record.totalScore / record.harvestCount;
        }
        
        _updateFederationRanking(seasonId, federationId);
        
        _globalFederationScore[federationId] += score;
        _updateGlobalFederationRanking(federationId);
        
        bytes32 rankKey = keccak256(abi.encodePacked(seasonId, federationId));
        emit FederationScoreUpdated(seasonId, federationId, record.totalScore, _federationSeasonRank[rankKey]);
    }

    /**
     * @dev Update federation member count.
     */
    function updateFederationMemberCount(uint256 seasonId, uint256 federationId, uint256 memberCount) public onlyOwner {
        FederationRecord storage record = _seasonFederationRecords[seasonId][federationId];
        record.memberCount = memberCount;
    }

    /**
     * @dev Get top N federations for a season.
     */
    function getTopFederations(uint256 seasonId, uint256 n) public view returns (uint256[] memory, uint256[] memory) {
        uint256[] storage ranking = _seasonFederationRanking[seasonId];
        uint256 limit = n > ranking.length ? ranking.length : n;
        
        uint256[] memory topFederations = new uint256[](limit);
        uint256[] memory scores = new uint256[](limit);
        
        for (uint256 i = 0; i < limit; i++) {
            topFederations[i] = ranking[i];
            scores[i] = _seasonFederationRecords[seasonId][ranking[i]].totalScore;
        }
        
        return (topFederations, scores);
    }

    /**
     * @dev Get federation record.
     */
    function getFederationRecord(uint256 seasonId, uint256 federationId) public view returns (
        uint256 totalScore,
        uint256 memberCount,
        uint256 averageScore,
        uint256 harvestCount
    ) {
        FederationRecord storage record = _seasonFederationRecords[seasonId][federationId];
        return (
            record.totalScore,
            record.memberCount,
            record.averageScore,
            record.harvestCount
        );
    }

    // ============ Global Rankings ============

    /**
     * @dev Get top N players globally.
     */
    function getGlobalTopPlayers(uint256 n) public view returns (address[] memory, uint256[] memory) {
        uint256 limit = n > _globalPlayerRanking.length ? _globalPlayerRanking.length : n;
        
        address[] memory topPlayers = new address[](limit);
        uint256[] memory scores = new uint256[](limit);
        
        for (uint256 i = 0; i < limit; i++) {
            topPlayers[i] = _globalPlayerRanking[i];
            scores[i] = _globalPlayerScore[_globalPlayerRanking[i]];
        }
        
        return (topPlayers, scores);
    }

    /**
     * @dev Get player's global rank.
     */
    function getPlayerGlobalRank(address player) public view returns (uint256) {
        return _globalPlayerRank[player];
    }

    /**
     * @dev Get top N federations globally.
     */
    function getGlobalTopFederations(uint256 n) public view returns (uint256[] memory, uint256[] memory) {
        uint256 limit = n > _globalFederationRanking.length ? _globalFederationRanking.length : n;
        
        uint256[] memory topFederations = new uint256[](limit);
        uint256[] memory scores = new uint256[](limit);
        
        for (uint256 i = 0; i < limit; i++) {
            topFederations[i] = _globalFederationRanking[i];
            scores[i] = _globalFederationScore[_globalFederationRanking[i]];
        }
        
        return (topFederations, scores);
    }

    // ============ Season Management ============

    /**
     * @dev Start a new leaderboard season.
     */
    function startNewSeason() public onlyOwner {
        uint256 newSeasonId = _seasonIds.current();
        _seasonIds.increment();
        
        uint256 oldSeason = currentLeaderboardSeason;
        currentLeaderboardSeason = newSeasonId;
        
        emit SeasonReset(oldSeason, newSeasonId);
    }

    /**
     * @dev Toggle seasonal resets.
     */
    function toggleSeasonalResets() public onlyOwner {
        seasonalResetsEnabled = !seasonalResetsEnabled;
    }

    // ============ Internal Ranking Functions ============

    function _updatePlayerRanking(uint256 seasonId, address player) internal {
        address[] storage ranking = _seasonPlayerRanking[seasonId];
        
        uint256 currentRank = 0;
        bool found = false;
        
        for (uint256 i = 0; i < ranking.length; i++) {
            if (ranking[i] == player) {
                currentRank = i;
                found = true;
                break;
            }
        }
        
        if (!found) {
            ranking.push(player);
            currentRank = ranking.length - 1;
        }
        
        while (currentRank > 0) {
            address above = ranking[currentRank - 1];
            if (_seasonPlayerRecords[seasonId][player].totalScore > _seasonPlayerRecords[seasonId][above].totalScore) {
                ranking[currentRank - 1] = player;
                ranking[currentRank] = above;
                bytes32 rankKey1 = keccak256(abi.encodePacked(seasonId, player));
                bytes32 rankKey2 = keccak256(abi.encodePacked(seasonId, above));
                _playerSeasonRank[rankKey1] = currentRank - 1;
                _playerSeasonRank[rankKey2] = currentRank;
                currentRank--;
            } else {
                break;
            }
        }
        
        bytes32 rankKey = keccak256(abi.encodePacked(seasonId, player));
        _playerSeasonRank[rankKey] = currentRank;
    }

    function _updateFederationRanking(uint256 seasonId, uint256 federationId) internal {
        uint256[] storage ranking = _seasonFederationRanking[seasonId];
        
        uint256 currentRank = 0;
        bool found = false;
        
        for (uint256 i = 0; i < ranking.length; i++) {
            if (ranking[i] == federationId) {
                currentRank = i;
                found = true;
                break;
            }
        }
        
        if (!found) {
            ranking.push(federationId);
            currentRank = ranking.length - 1;
        }
        
        while (currentRank > 0) {
            uint256 above = ranking[currentRank - 1];
            if (_seasonFederationRecords[seasonId][federationId].totalScore > _seasonFederationRecords[seasonId][above].totalScore) {
                ranking[currentRank - 1] = federationId;
                ranking[currentRank] = above;
                bytes32 rankKey1 = keccak256(abi.encodePacked(seasonId, federationId));
                bytes32 rankKey2 = keccak256(abi.encodePacked(seasonId, above));
                _federationSeasonRank[rankKey1] = currentRank - 1;
                _federationSeasonRank[rankKey2] = currentRank;
                currentRank--;
            } else {
                break;
            }
        }
        
        bytes32 rankKey = keccak256(abi.encodePacked(seasonId, federationId));
        _federationSeasonRank[rankKey] = currentRank;
    }

    function _updateGlobalPlayerRanking(address player) internal {
        uint256 currentRank = 0;
        bool found = false;
        
        for (uint256 i = 0; i < _globalPlayerRanking.length; i++) {
            if (_globalPlayerRanking[i] == player) {
                currentRank = i;
                found = true;
                break;
            }
        }
        
        if (!found) {
            _globalPlayerRanking.push(player);
            currentRank = _globalPlayerRanking.length - 1;
        }
        
        while (currentRank > 0) {
            address above = _globalPlayerRanking[currentRank - 1];
            if (_globalPlayerScore[player] > _globalPlayerScore[above]) {
                _globalPlayerRanking[currentRank - 1] = player;
                _globalPlayerRanking[currentRank] = above;
                _globalPlayerRank[player] = currentRank - 1;
                _globalPlayerRank[above] = currentRank;
                currentRank--;
            } else {
                break;
            }
        }
        
        _globalPlayerRank[player] = currentRank;
        
        emit GlobalRankingUpdated(player, currentRank);
    }

    function _updateGlobalFederationRanking(uint256 federationId) internal {
        uint256 currentRank = 0;
        bool found = false;
        
        for (uint256 i = 0; i < _globalFederationRanking.length; i++) {
            if (_globalFederationRanking[i] == federationId) {
                currentRank = i;
                found = true;
                break;
            }
        }
        
        if (!found) {
            _globalFederationRanking.push(federationId);
            currentRank = _globalFederationRanking.length - 1;
        }
        
        while (currentRank > 0) {
            uint256 above = _globalFederationRanking[currentRank - 1];
            if (_globalFederationScore[federationId] > _globalFederationScore[above]) {
                _globalFederationRanking[currentRank - 1] = federationId;
                _globalFederationRanking[currentRank] = above;
                _globalFederationRank[federationId] = currentRank - 1;
                _globalFederationRank[above] = currentRank;
                currentRank--;
            } else {
                break;
            }
        }
        
        _globalFederationRank[federationId] = currentRank;
    }
}
