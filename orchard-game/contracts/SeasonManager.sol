// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev SeasonManager handles temporal governance for the Orchard Game.
 * It manages seasonal cycles, epoch transitions, and knowledge frontier progression.
 * Per SeasonTransition.tla specifications.
 */
contract SeasonManager is Ownable {
    using Counters for Counters.Counter;

    // ============ Constants ============
    uint256 public constant MIN_SEASON_DURATION = 1 days;
    uint256 public constant MAX_SEASON_DURATION = 365 days;
    uint256 public constant DEFAULT_SEASON_DURATION = 30 days;
    uint256 public constant EPOCH_DURATION = 1 days;
    uint256 public constant MAX_KNOWLEDGE_FRONTIER_SIZE = 10000;
    uint256 public constant MIN_KNOWLEDGE_FRONTIER_SIZE = 10;

    // ============ Season State ============
    enum SeasonState {
        INACTIVE,
        ACTIVE,
        ENDING_SOON,
        TRANSITIONING
    }

    // ============ State Variables ============
    Counters.Counter private _seasonIds;
    
    // Current season tracking
    uint256 public currentSeasonId;
    SeasonState public currentSeasonState;
    uint256 public seasonStartTime;
    uint256 public seasonEndTime;
    uint256 public currentEpoch;
    uint256 public lastEpochTransition;
    
    // Knowledge frontier (concepts that can be explored this season)
    uint256 public knowledgeFrontierSize;
    mapping(uint256 => uint256) public conceptMastery; // conceptId -> mastery level
    
    // Season data
    mapping(uint256 => uint256) private _seasonDurations;
    mapping(uint256 => uint256) private _seasonStartTimes;
    mapping(uint256 => uint256) private _seasonEndTimes;
    mapping(uint256 => uint256) private _seasonTotalHarvests;
    mapping(uint256 => uint256) private _seasonTotalScore;
    mapping(uint256 => uint256) private _seasonParticipantCount;
    mapping(uint256 => bool) private _seasonCompleted;
    
    // Per-season knowledge frontiers
    mapping(uint256 => uint256) private _seasonKnowledgeFrontierSize;
    mapping(uint256 => mapping(uint256 => uint256)) private _seasonConceptMastery; // seasonId -> conceptId -> mastery

    // ============ Events ============
    event SeasonStarted(
        uint256 indexed seasonId,
        uint256 duration,
        uint256 knowledgeFrontierSize
    );

    event SeasonEnded(
        uint256 indexed seasonId,
        uint256 totalHarvests,
        uint256 totalScore,
        uint256 participantCount
    );

    event SeasonExtended(
        uint256 indexed seasonId,
        uint256 newEndTime
    );

    event EpochTransition(
        uint256 indexed seasonId,
        uint256 indexed epoch,
        uint256 timestamp
    );

    event KnowledgeFrontierExpanded(
        uint256 indexed seasonId,
        uint256 newSize
    );

    event ConceptMasteryUpdated(
        uint256 indexed seasonId,
        uint256 indexed conceptId,
        uint256 newMasteryLevel
    );

    event RewardDistributed(
        uint256 indexed seasonId,
        address indexed recipient,
        uint256 amount
    );

    // ============ Constructor ============
    constructor() {
        currentSeasonId = 0;
        currentSeasonState = SeasonState.INACTIVE;
        knowledgeFrontierSize = MIN_KNOWLEDGE_FRONTIER_SIZE;
    }

    // ============ Admin Functions ============

    /**
     * @dev Start a new season with specified duration.
     * @param duration Duration of the season in seconds.
     * @param knowledgeFrontierSize Initial size of the knowledge frontier.
     */
    function startSeason(uint256 duration, uint256 knowledgeFrontierSize) public onlyOwner {
        require(duration >= MIN_SEASON_DURATION && duration <= MAX_SEASON_DURATION, "Invalid season duration");
        require(knowledgeFrontierSize >= MIN_KNOWLEDGE_FRONTIER_SIZE && knowledgeFrontierSize <= MAX_KNOWLEDGE_FRONTIER_SIZE, "Invalid frontier size");
        
        // End current season if active
        if (currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON) {
            _endCurrentSeason();
        }
        
        uint256 seasonId = _seasonIds.current();
        _seasonIds.increment();
        
        currentSeasonId = seasonId;
        currentSeasonState = SeasonState.ACTIVE;
        seasonStartTime = block.timestamp;
        seasonEndTime = block.timestamp + duration;
        currentEpoch = 0;
        lastEpochTransition = block.timestamp;
        
        _seasonDurations[seasonId] = duration;
        _seasonStartTimes[seasonId] = block.timestamp;
        _seasonEndTimes[seasonId] = seasonEndTime;
        _seasonKnowledgeFrontierSize[seasonId] = knowledgeFrontierSize;
        
        // Initialize knowledge frontier
        for (uint256 i = 0; i < knowledgeFrontierSize; i++) {
            _seasonConceptMastery[seasonId][i] = 0;
        }
        
        knowledgeFrontierSize = knowledgeFrontierSize;
        
        emit SeasonStarted(seasonId, duration, knowledgeFrontierSize);
    }

    /**
     * @dev Extend the current season.
     * @param additionalDuration Additional duration in seconds.
     */
    function extendSeason(uint256 additionalDuration) public onlyOwner {
        require(currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON, "No active season");
        require(additionalDuration > 0 && additionalDuration <= MAX_SEASON_DURATION, "Invalid duration");
        
        seasonEndTime += additionalDuration;
        _seasonEndTimes[currentSeasonId] = seasonEndTime;
        
        if (currentSeasonState == SeasonState.ENDING_SOON) {
            currentSeasonState = SeasonState.ACTIVE;
        }
        
        emit SeasonExtended(currentSeasonId, seasonEndTime);
    }

    /**
     * @dev End the current season early.
     */
    function endSeasonEarly() public onlyOwner {
        require(currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON, "No active season");
        _endCurrentSeason();
    }

    /**
     * @dev Transition to a new epoch within the current season.
     */
    function transitionEpoch() public onlyOwner {
        require(currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON, "No active season");
        require(block.timestamp >= lastEpochTransition + EPOCH_DURATION, "Too soon for epoch transition");
        
        currentEpoch++;
        lastEpochTransition = block.timestamp;
        
        emit EpochTransition(currentSeasonId, currentEpoch, block.timestamp);
    }

    /**
     * @dev Expand the knowledge frontier for the current season.
     * @param newSize The new size of the knowledge frontier.
     */
    function expandKnowledgeFrontier(uint256 newSize) public onlyOwner {
        require(currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON, "No active season");
        require(newSize > knowledgeFrontierSize, "Can only expand");
        require(newSize <= MAX_KNOWLEDGE_FRONTIER_SIZE, "Exceeds maximum");
        
        // Initialize new concepts
        for (uint256 i = knowledgeFrontierSize; i < newSize; i++) {
            _seasonConceptMastery[currentSeasonId][i] = 0;
        }
        
        _seasonKnowledgeFrontierSize[currentSeasonId] = newSize;
        knowledgeFrontierSize = newSize;
        
        emit KnowledgeFrontierExpanded(currentSeasonId, newSize);
    }

    /**
     * @dev Update mastery level for a specific concept.
     * @param conceptId The ID of the concept to update.
     * @param masteryLevel The new mastery level (0-1000).
     */
    function updateConceptMastery(uint256 conceptId, uint256 masteryLevel) public onlyOwner {
        require(currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON, "No active season");
        require(conceptId < knowledgeFrontierSize, "Concept out of bounds");
        require(masteryLevel <= 1000, "Mastery level too high");
        
        _seasonConceptMastery[currentSeasonId][conceptId] = masteryLevel;
        conceptMastery[conceptId] = masteryLevel;
        
        emit ConceptMasteryUpdated(currentSeasonId, conceptId, masteryLevel);
    }

    /**
     * @dev Record a harvest event for the current season.
     * @param player The address of the player.
     * @param score The growth score of the harvested seed.
     */
    function recordHarvest(address player, uint256 score) public onlyOwner {
        require(currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON, "No active season");
        
        _seasonTotalHarvests[currentSeasonId]++;
        _seasonTotalScore[currentSeasonId] += score;
        
        // Track unique participants (simplified - in production would use a set)
        _seasonParticipantCount[currentSeasonId]++;
    }

    /**
     * @dev Distribute rewards at season end.
     * @param recipients Array of recipient addresses.
     * @param amounts Array of reward amounts (must match recipients length).
     */
    function distributeRewards(address[] calldata recipients, uint256[] calldata amounts) public onlyOwner {
        require(currentSeasonState == SeasonState.TRANSITIONING, "Season not transitioning");
        require(recipients.length == amounts.length, "Length mismatch");
        
        for (uint256 i = 0; i < recipients.length; i++) {
            emit RewardDistributed(currentSeasonId, recipients[i], amounts[i]);
        }
    }

    // ============ View Functions ============

    /**
     * @dev Get the current season information.
     */
    function getCurrentSeasonInfo() public view returns (
        uint256 seasonId,
        SeasonState state,
        uint256 startTime,
        uint256 endTime,
        uint256 epoch,
        uint256 frontierSize
    ) {
        return (
            currentSeasonId,
            currentSeasonState,
            seasonStartTime,
            seasonEndTime,
            currentEpoch,
            knowledgeFrontierSize
        );
    }

    /**
     * @dev Get season statistics.
     * @param seasonId The ID of the season.
     */
    function getSeasonStats(uint256 seasonId) public view returns (
        uint256 duration,
        uint256 totalHarvests,
        uint256 totalScore,
        uint256 participantCount,
        uint256 knowledgeFrontierSize,
        bool completed
    ) {
        return (
            _seasonDurations[seasonId],
            _seasonTotalHarvests[seasonId],
            _seasonTotalScore[seasonId],
            _seasonParticipantCount[seasonId],
            _seasonKnowledgeFrontierSize[seasonId],
            _seasonCompleted[seasonId]
        );
    }

    /**
     * @dev Get concept mastery for current season.
     * @param conceptId The ID of the concept.
     */
    function getConceptMastery(uint256 conceptId) public view returns (uint256) {
        require(conceptId < knowledgeFrontierSize, "Concept out of bounds");
        return _seasonConceptMastery[currentSeasonId][conceptId];
    }

    /**
     * @dev Check if season is active.
     */
    function isSeasonActive() public view returns (bool) {
        return currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON;
    }

    /**
     * @dev Get time remaining in current season.
     */
    function getTimeRemainingInSeason() public view returns (uint256) {
        if (currentSeasonState == SeasonState.INACTIVE || currentSeasonState == SeasonState.TRANSITIONING) {
            return 0;
        }
        if (block.timestamp >= seasonEndTime) {
            return 0;
        }
        return seasonEndTime - block.timestamp;
    }

    /**
     * @dev Get average score per harvest for a season.
     * @param seasonId The ID of the season.
     */
    function getAverageScore(uint256 seasonId) public view returns (uint256) {
        if (_seasonTotalHarvests[seasonId] == 0) {
            return 0;
        }
        return _seasonTotalScore[seasonId] / _seasonTotalHarvests[seasonId];
    }

    // ============ Internal Functions ============

    function _endCurrentSeason() internal {
        currentSeasonState = SeasonState.TRANSITIONING;
        
        _seasonTotalHarvests[currentSeasonId] = _seasonTotalHarvests[currentSeasonId];
        _seasonCompleted[currentSeasonId] = true;
        
        emit SeasonEnded(
            currentSeasonId,
            _seasonTotalHarvests[currentSeasonId],
            _seasonTotalScore[currentSeasonId],
            _seasonParticipantCount[currentSeasonId]
        );
        
        currentSeasonState = SeasonState.INACTIVE;
    }

    // ============ Auto-transition ============
    
    /**
     * @dev Check and perform automatic season end if time has passed.
     * Can be called by anyone to trigger auto-transition.
     */
    function checkSeasonEnd() public {
        if ((currentSeasonState == SeasonState.ACTIVE || currentSeasonState == SeasonState.ENDING_SOON) 
            && block.timestamp >= seasonEndTime) {
            _endCurrentSeason();
        }
    }
}
