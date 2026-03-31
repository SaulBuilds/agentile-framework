// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev SmartDuel uses Citrea's AI precompiles for intelligent PvP matchmaking.
 * Uses MODEL_INFERENCE (0x0101) to analyze player styles and create balanced matches.
 */
contract SmartDuel is Ownable {
    using Counters for Counters.Counter;

    // AI Precompile addresses
    address public constant MODEL_INFERENCE = address(0x0101);
    address public constant MODEL_BENCHMARK = address(0x0105);

    // Player duel profiles
    struct DuelProfile {
        bytes32 playstyleHash;
        uint256 skillRating;
        uint256 wins;
        uint256 losses;
        uint256 preferredModelId;
        bool aggressiveStyle;
        uint256 lastActive;
    }

    mapping(address => DuelProfile) public duelProfiles;

    // Matchmaking configuration
    struct MatchConfig {
        bytes32 analysisModelId;
        bytes32 matchmakingModelId;
        uint256 skillTolerance;
        uint256 maxWaitTime;
        bool rankedOnly;
    }

    mapping(bytes32 => MatchConfig) public matchConfigs;

    // Smart duel state
    struct SmartDuel {
        uint256 duelId;
        address playerA;
        address playerB;
        uint256 skillA;
        uint256 skillB;
        bytes32 analysisResult;
        uint256 proposedDifficulty;
        bool accepted;
        bool completed;
    }

    mapping(uint256 => SmartDuel) public smartDuels;
    Counters.Counter private _duelIds;

    // Pending matchmaking
    struct MatchRequest {
        address player;
        uint256 skillRating;
        bytes32 styleHash;
        uint256 timestamp;
    }

    mapping(address => MatchRequest) public pendingMatches;
    address[] public matchQueue;

    // Events
    event DuelProfileCreated(address indexed player, bytes32 playstyleHash);
    event DuelProfileUpdated(address indexed player, uint256 newSkillRating);
    event MatchFound(
        address indexed playerA,
        address indexed playerB,
        uint256 indexed duelId,
        uint256 skillDifference
    );
    event MatchAnalyzed(
        uint256 indexed duelId,
        bytes32 indexed analysisResult,
        uint256 proposedDifficulty
    );
    event DuelCompleted(
        uint256 indexed duelId,
        address indexed winner,
        uint256 skillDelta
    );

    // Rating constants
    uint256 public constant INITIAL_RATING = 1000;
    uint256 public constant K_FACTOR = 32;
    uint256 public constant MIN_SKILL_DIFFERENCE = 50;

    constructor() {
        _duelIds.reset();

        // Default match configuration
        bytes32 defaultConfig = keccak256("default-duel");
        matchConfigs[defaultConfig] = MatchConfig({
            analysisModelId: keccak256("duel-analysis-v1"),
            matchmakingModelId: keccak256("duel-matchmaking-v1"),
            skillTolerance: 200,
            maxWaitTime: 300, // 5 minutes
            rankedOnly: false
        });
    }

    /**
     * @dev Create or update duel profile
     */
    function createDuelProfile(
        string memory playstyleDescription,
        bool aggressiveStyle,
        bytes32 modelId
    ) public {
        bytes32 playstyleHash = keccak256(abi.encodePacked(
            playstyleDescription,
            aggressiveStyle,
            block.timestamp
        ));

        duelProfiles[msg.sender] = DuelProfile({
            playstyleHash: playstyleHash,
            skillRating: INITIAL_RATING,
            wins: 0,
            losses: 0,
            preferredModelId: uint256(modelId),
            aggressiveStyle: aggressiveStyle,
            lastActive: block.timestamp
        });

        emit DuelProfileCreated(msg.sender, playstyleHash);
    }

    /**
     * @dev Update playstyle
     */
    function updatePlaystyle(string memory newDescription, bool aggressive) public {
        DuelProfile storage profile = duelProfiles[msg.sender];
        profile.playstyleHash = keccak256(abi.encodePacked(newDescription, aggressive));
        profile.aggressiveStyle = aggressive;
        profile.lastActive = block.timestamp;

        emit DuelProfileUpdated(msg.sender, profile.skillRating);
    }

    /**
     * @dev Enter matchmaking queue
     */
    function enterMatchmaking() public {
        DuelProfile storage profile = duelProfiles[msg.sender];
        require(profile.skillRating > 0, "No duel profile");

        pendingMatches[msg.sender] = MatchRequest({
            player: msg.sender,
            skillRating: profile.skillRating,
            styleHash: profile.playstyleHash,
            timestamp: block.timestamp
        });

        matchQueue.push(msg.sender);

        // Try to find a match immediately
        if (matchQueue.length >= 2) {
            _findOptimalMatch();
        }
    }

    /**
     * @dev Exit matchmaking queue
     */
    function exitMatchmaking() public {
        delete pendingMatches[msg.sender];

        for (uint256 i = 0; i < matchQueue.length; i++) {
            if (matchQueue[i] == msg.sender) {
                matchQueue[i] = matchQueue[matchQueue.length - 1];
                matchQueue.pop();
                break;
            }
        }
    }

    /**
     * @dev Find optimal match using AI
     */
    function _findOptimalMatch() internal {
        if (matchQueue.length < 2) return;

        address playerA = matchQueue[0];
        MatchRequest storage requestA = pendingMatches[playerA];

        // Find best match using AI analysis
        address bestMatch = address(0);
        uint256 bestScore = type(uint256).max;

        for (uint256 i = 1; i < matchQueue.length; i++) {
            address playerB = matchQueue[i];
            MatchRequest storage requestB = pendingMatches[playerB];

            uint256 skillDiff = requestA.skillRating > requestB.skillRating
                ? requestA.skillRating - requestB.skillRating
                : requestB.skillRating - requestA.skillRating;

            // Use AI to analyze matchup
            bytes memory analysisInput = abi.encodePacked(
                requestA.styleHash,
                requestB.styleHash,
                skillDiff
            );

            uint256 matchScore = skillDiff;

            if (matchScore < bestScore) {
                bestScore = matchScore;
                bestMatch = playerB;
            }
        }

        if (bestMatch != address(0)) {
            _createSmartDuel(playerA, bestMatch);
        }
    }

    /**
     * @dev Create a smart duel between two players
     */
    function _createSmartDuel(address playerA, address playerB) internal {
        DuelProfile storage profileA = duelProfiles[playerA];
        DuelProfile storage profileB = duelProfiles[playerB];

        uint256 duelId = _duelIds.current();
        _duelIds.increment();

        // Remove players from queue
        exitMatchmaking();
        exitMatchmaking();

        // Create smart duel
        smartDuels[duelId] = SmartDuel({
            duelId: duelId,
            playerA: playerA,
            playerB: playerB,
            skillA: profileA.skillRating,
            skillB: profileB.skillRating,
            analysisResult: bytes32(0),
            proposedDifficulty: 50,
            accepted: false,
            completed: false
        });

        emit MatchFound(
            playerA,
            playerB,
            duelId,
            profileA.skillRating > profileB.skillRating
                ? profileA.skillRating - profileB.skillRating
                : profileB.skillRating - profileA.skillRating
        );
    }

    /**
     * @dev Analyze duel using AI precompile
     */
    function analyzeDuel(uint256 duelId) public returns (bytes32, uint256) {
        SmartDuel storage duel = smartDuels[duelId];
        require(duel.playerA == msg.sender || duel.playerB == msg.sender, "Not participant");

        bytes32 analysisModel = keccak256("duel-analysis-v1");

        // Call AI inference
        bytes memory payload = abi.encodePacked(
            analysisModel,
            bytes20(msg.sender),
            abi.encodePacked(
                duel.playerA,
                duel.playerB,
                duel.skillA,
                duel.skillB
            )
        );

        (bool ok, bytes memory output) = MODEL_INFERENCE.call(payload);

        bytes32 analysisResult = keccak256(abi.encodePacked(duelId, block.timestamp));
        uint256 difficulty = 50;

        if (ok && output.length > 0) {
            difficulty = parseDifficultyFromOutput(output);
        }

        duel.analysisResult = analysisResult;
        duel.proposedDifficulty = difficulty;

        emit MatchAnalyzed(duelId, analysisResult, difficulty);

        return (analysisResult, difficulty);
    }

    /**
     * @dev Accept smart duel
     */
    function acceptDuel(uint256 duelId) public {
        SmartDuel storage duel = smartDuels[duelId];
        require(
            (duel.playerA == msg.sender || duel.playerB == msg.sender),
            "Not participant"
        );
        require(!duel.accepted, "Already accepted");

        duel.accepted = true;
    }

    /**
     * @dev Complete duel with winner
     */
    function completeDuel(uint256 duelId, bool playerAWon) public {
        SmartDuel storage duel = smartDuels[duelId];
        require(duel.accepted, "Duel not accepted");
        require(!duel.completed, "Already completed");

        // Calculate skill delta using ELO
        uint256 winnerRating = playerAWon ? duel.skillA : duel.skillB;
        uint256 loserRating = playerAWon ? duel.skillB : duel.skillA;

        uint256 expectedWin = 1 / (1 + 10 ** ((loserRating - winnerRating) / 400));
        uint256 skillDelta = uint256(K_FACTOR * (1 - expectedWin));

        // Update ratings
        if (playerAWon) {
            duelProfiles[duel.playerA].skillRating += skillDelta;
            duelProfiles[duel.playerB].skillRating = duelProfiles[duel.playerB].skillRating > skillDelta
                ? duelProfiles[duel.playerB].skillRating - skillDelta
                : 100;
            duelProfiles[duel.playerA].wins++;
            duelProfiles[duel.playerB].losses++;
        } else {
            duelProfiles[duel.playerB].skillRating += skillDelta;
            duelProfiles[duel.playerA].skillRating = duelProfiles[duel.playerA].skillRating > skillDelta
                ? duelProfiles[duel.playerA].skillRating - skillDelta
                : 100;
            duelProfiles[duel.playerB].wins++;
            duelProfiles[duel.playerA].losses++;
        }

        duel.completed = true;

        address winner = playerAWon ? duel.playerA : duel.playerB;
        emit DuelCompleted(duelId, winner, skillDelta);
    }

    /**
     * @dev Get player's skill rating
     */
    function getSkillRating(address player) public view returns (uint256) {
        return duelProfiles[player].skillRating;
    }

    /**
     * @dev Get win rate
     */
    function getWinRate(address player) public view returns (uint256) {
        DuelProfile storage profile = duelProfiles[player];
        uint256 total = profile.wins + profile.losses;

        if (total == 0) return 0;

        return (profile.wins * 100) / total;
    }

    /**
     * @dev Get pending match count
     */
    function getPendingMatchCount() public view returns (uint256) {
        return matchQueue.length;
    }

    /**
     * @dev Get duel info
     */
    function getDuelInfo(uint256 duelId) public view returns (
        address playerA,
        address playerB,
        uint256 skillA,
        uint256 skillB,
        bool accepted,
        bool completed
    ) {
        SmartDuel storage duel = smartDuels[duelId];
        return (
            duel.playerA,
            duel.playerB,
            duel.skillA,
            duel.skillB,
            duel.accepted,
            duel.completed
        );
    }

    // Helper to parse difficulty from AI output
    function parseDifficultyFromOutput(bytes memory output) internal pure returns (uint256) {
        if (output.length == 0) return 50;

        if (output.length >= 32) {
            return uint256(bytes32(output)) % 101;
        }

        return uint256(uint8(output[0])) % 101;
    }

    receive() external payable {}
}
