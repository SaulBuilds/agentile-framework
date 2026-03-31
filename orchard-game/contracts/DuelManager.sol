// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "./SeedNFT.sol";
import "./GrowthEngine.sol";

/**
 * @dev DuelManager handles pollination duels between players' seeds.
 */
contract DuelManager is Ownable {
    // ============ Constants ============
    uint256 public constant DUEL_RESPONSE_TIME = 60; // seconds to respond to a duel
    uint256 public constant DUEL_COOLDOWN = 86400;   // 24 hours cooldown after a duel

    // ============ State Variables ============
    struct Duel {
        uint256 seedIdA;          // ID of the first seed
        uint256 seedIdB;          // ID of the second seed
        address playerA;          // Address of the first player
        address playerB;          // Address of the second player
        uint256 startTime;        // Block timestamp when duel started
        uint256 responseDeadline; // Block timestamp by which playerB must respond
        bool accepted;            // Whether the duel has been accepted
        bool completed;           // Whether the duel has been completed
        uint8 result;             // 0 = draw, 1 = playerA wins, 2 = playerB wins
        bool growthBonusApplied;  // Whether the growth bonus has been applied
    }

    mapping(uint256 => Duel) public duels; // duelId => Duel
    uint256 public nextDuelId;

    SeedNFT private seedNFT;
    GrowthEngine private growthEngine;

    // Cooldown tracking: player => last duel timestamp
    mapping(address => uint256) public lastDuelTime;

    // ============ Events ============
    event DuelInitiated(
        uint256 indexed duelId,
        address indexed initiator,
        uint256 seedIdA,
        uint256 indexed target,
        uint256 seedIdB
    );

    event DuelAccepted(
        uint256 indexed duelId,
        address indexed responder
    );

    event DuelRejected(
        uint256 indexed duelId,
        address indexed responder
    );

    event DuelCompleted(
        uint256 indexed duelId,
        uint8 result, // 0 = draw, 1 = playerA wins, 2 = playerB wins
        uint256 growthBonusAmount
    );

    event DuelTimedOut(
        uint256 indexed duelId,
        address indexed initiator
    );

    event DuelCooldownActive(
        address indexed player,
        uint256 secondsRemaining
    );

    // ============ Constructor ============
    constructor(SeedNFT _seedNFT, GrowthEngine _growthEngine) {
        seedNFT = _seedNFT;
        growthEngine = _growthEngine;
    }

    // ============ Public Functions ============

    /**
     * @dev Initiate a pollination duel with another player's seed.
     * @param seedIdA The ID of the initiator's seed.
     * @param target The address of the target player.
     * @param seedIdB The ID of the target player's seed.
     * @return duelId The ID of the newly created duel.
     */
    function initiateDuel(
        uint256 seedIdA,
        address target,
        uint256 seedIdB
    ) public returns (uint256) {
        require(target != address(0), "Target cannot be zero address");
        require(target != msg.sender, "Cannot duel yourself");
        require(!isOnCooldown(msg.sender), "You are on cooldown");
        require(!isOnCooldown(target), "Target is on cooldown");
        require(seedNFT.ownerOf(seedIdA) != address(0), "Your seed does not exist");
        require(seedNFT.ownerOf(seedIdB) != address(0), "Target's seed does not exist");
        require(seedNFT.planterOf(seedIdA) == msg.sender, "You do not own seed A");
        require(seedNFT.planterOf(seedIdB) == target, "Target does not own seed B");
        require(seedNFT.checkpointOf(seedIdA) > 0, "Seed A must have advanced at least one checkpoint");
        require(seedNFT.checkpointOf(seedIdB) > 0, "Seed B must have advanced at least one checkpoint");
        require(!seedNFT.isHarvested(seedIdA), "Seed A has already been harvested");
        require(!seedNFT.isHarvested(seedIdB), "Seed B has already been harvested");
        require(!seedNFT.isFailed(seedIdA), "Seed A has failed");
        require(!seedNFT.isFailed(seedIdB), "Seed B has failed");

        uint256 duelId = nextDuelId++;
        duels[duelId] = Duel({
            seedIdA: seedIdA,
            seedIdB: seedIdB,
            playerA: msg.sender,
            playerB: target,
            startTime: block.timestamp,
            responseDeadline: block.timestamp + DUEL_RESPONSE_TIME,
            accepted: false,
            completed: false,
            result: 0,
            growthBonusApplied: false
        });

        emit DuelInitiated(duelId, msg.sender, seedIdA, uint256(uint160(target)), seedIdB);

        return duelId;
    }

    /**
     * @dev Accept a duel that has been initiated with you.
     * @param duelId The ID of the duel to accept.
     */
    function acceptDuel(uint256 duelId) public {
        Duel storage duel = duels[duelId];
        require(duel.playerB == msg.sender, "Not the target of this duel");
        require(!duel.accepted, "Duel already accepted");
        require(!duel.completed, "Duel already completed");
        require(block.timestamp <= duel.responseDeadline, "Duel response time has expired");

        duel.accepted = true;

        emit DuelAccepted(duelId, msg.sender);
    }

    /**
     * @dev Reject a duel that has been initiated with you.
     * @param duelId The ID of the duel to reject.
     */
    function rejectDuel(uint256 duelId) public {
        Duel storage duel = duels[duelId];
        require(duel.playerB == msg.sender, "Not the target of this duel");
        require(!duel.accepted, "Duel already accepted");
        require(!duel.completed, "Duel already completed");
        require(block.timestamp <= duel.responseDeadline, "Duel response time has expired");

        // We don't mark the duel as completed, but we can consider it inactive.
        // For simplicity, we'll just emit an event and leave the duel in the mapping.
        // In a real implementation, we might want to clean up or mark as rejected.

        emit DuelRejected(duelId, msg.sender);
    }

    /**
     * @dev Complete a duel after both players have submitted their seeds for validation.
     * This function should be called after validation has been processed for both seeds.
     * @param duelId The ID of the duel to complete.
     * @param growthScoreA The growth score of seed A (from validation).
     * @param growthScoreB The growth score of seed B (from validation).
     */
    function completeDuel(
        uint256 duelId,
        uint256 growthScoreA,
        uint256 growthScoreB
    ) public {
        Duel storage duel = duels[duelId];
        require(duel.accepted, "Duel not accepted");
        require(!duel.completed, "Duel already completed");
        require(duel.playerA == msg.sender || duel.playerB == msg.sender, "Not a participant in this duel");
        require(growthScoreA <= 100, "Invalid growth score for seed A");
        require(growthScoreB <= 100, "Invalid growth score for seed B");

        // Determine the winner based on growth scores
        uint8 result;
        uint256 bonusAmount = 0;
        if (growthScoreA > growthScoreB) {
            result = 1; // playerA wins
            bonusAmount = growthScoreA - growthScoreB;
        } else if (growthScoreB > growthScoreA) {
            result = 2; // playerB wins
            bonusAmount = growthScoreB - growthScoreA;
        } else {
            result = 0; // draw
            bonusAmount = 0;
        }

        duel.result = result;
        duel.completed = true;

        // Apply growth bonus to the winner's seed (if any)
        if (bonusAmount > 0) {
            if (result == 1) {
                // Player A wins, increase growth score of seed A by bonusAmount
                // In a real implementation, we would update the seed's growth score in SeedNFT.
                // For now, we note that the bonus is applied and would be used in reward calculations.
                // We'll set a flag to indicate the bonus has been applied.
                duel.growthBonusApplied = true;
            } else if (result == 2) {
                // Player B wins, increase growth score of seed B by bonusAmount
                duel.growthBonusApplied = true;
            }
        }

        // Set cooldown for both players
        lastDuelTime[duel.playerA] = block.timestamp;
        lastDuelTime[duel.playerB] = block.timestamp;

        emit DuelCompleted(duelId, result, bonusAmount);
    }

    /**
     * @dev Call this function when a duel times out (no response from target).
     * @param duelId The ID of the duel that timed out.
     */
    function duelTimedOut(uint256 duelId) public {
        Duel storage duel = duels[duelId];
        require(duel.playerB == msg.sender, "Not the target of this duel");
        require(!duel.accepted, "Duel already accepted");
        require(!duel.completed, "Duel already completed");
        require(block.timestamp > duel.responseDeadline, "Duel has not timed out yet");

        // Mark the duel as completed due to timeout
        duel.completed = true;
        duel.result = 0; // treat as draw for timeout? Or maybe no bonus and no win.

        // Set cooldown for the initiator (the target didn't respond, so only initiator gets cooldown?)
        // Actually, both players should get cooldown to prevent spamming.
        lastDuelTime[duel.playerA] = block.timestamp;
        lastDuelTime[duel.playerB] = block.timestamp;

        emit DuelTimedOut(duelId, msg.sender);
    }

    // ============ View Functions ============

    function isOnCooldown(address player)
        public
        view
        returns (bool)
    {
        uint256 lastTime = lastDuelTime[player];
        if (lastTime == 0) {
            return false;
        }
        return (block.timestamp - lastTime) < DUEL_COOLDOWN;
    }

    function timeUntilCooldownOver(address player)
        public
        view
        returns (uint256)
    {
        uint256 lastTime = lastDuelTime[player];
        if (lastTime == 0) {
            return 0;
        }
        uint256 elapsed = block.timestamp - lastTime;
        if (elapsed >= DUEL_COOLDOWN) {
            return 0;
        }
        return DUEL_COOLDOWN - elapsed;
    }

    function getDuel(uint256 duelId)
        public
        view
        returns (
            uint256 seedIdA,
            uint256 seedIdB,
            address playerA,
            address playerB,
            uint256 startTime,
            uint256 responseDeadline,
            bool accepted,
            bool completed,
            uint8 result,
            bool growthBonusApplied
        )
    {
        Duel storage duel = duels[duelId];
        return (
            duel.seedIdA,
            duel.seedIdB,
            duel.playerA,
            duel.playerB,
            duel.startTime,
            duel.responseDeadline,
            duel.accepted,
            duel.completed,
            duel.result,
            duel.growthBonusApplied
        );
    }
}