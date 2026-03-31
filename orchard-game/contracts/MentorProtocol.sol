// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev MentorProtocol handles the LoRA-like knowledge transfer system.
 * High-performing seeds can mentor developing seeds, sharing knowledge adapters.
 * Per MentorPropagation.tla specifications.
 */
contract MentorProtocol is Ownable {
    using Counters for Counters.Counter;

    // ============ Constants ============
    uint256 public constant MAX_MENTORS_PER_MENTEE = 5;
    uint256 public constant MAX_MENTEES_PER_MENTOR = 20;
    uint256 public constant MIN_MENTOR_SCORE_THRESHOLD = 70;
    uint256 public constant MAX_INFLUENCE_PER_MENTOR = 1000;
    uint256 public constant PROPAGATION_DELAY = 1 days;
    uint256 public constant ADAPTER_WEIGHT_DECAY = 100;

    // ============ State Variables ============
    Counters.Counter private _mentorshipIds;
    Counters.Counter private _adapterIds;

    // Mentor tracking
    mapping(address => bool) public isMentor;
    mapping(address => uint256) public mentorScore;
    mapping(address => uint256) public mentorInfluence;
    mapping(address => uint256) public mentorTotalEarnings;
    mapping(address => uint256[]) private _mentorMentees; // mentor -> mentee IDs

    // Mentee tracking
    mapping(address => bool) public isMentee;
    mapping(address => uint256[]) private _menteeMentors; // mentee -> mentor IDs

    // Active mentorships
    struct Mentorship {
        uint256 id;
        address mentor;
        address mentee;
        uint256 startTime;
        uint256 influence;
        bool active;
        uint256 adapterWeight;
    }
    
    mapping(uint256 => Mentorship) public mentorships;
    mapping(address => uint256[]) private _playerMentorships; // player -> mentorship IDs

    // Knowledge adapters
    struct KnowledgeAdapter {
        uint256 adapterId;
        address creator;
        uint256 quality;
        uint256 creationTime;
        uint256 propagationCount;
    }
    
    mapping(uint256 => KnowledgeAdapter) public adapters;
    mapping(address => uint256[]) private _playerAdapters; // player -> adapter IDs
    
    // Influence history
    struct InfluenceRecord {
        uint256 timestamp;
        uint256 influence;
    }
    mapping(address => InfluenceRecord[]) private _mentorHistory;

    // ============ Events ============
    event MentorRegistered(address indexed mentor, uint256 initialScore);
    event MentorDeregistered(address indexed mentor);
    event MentorshipCreated(
        uint256 indexed mentorshipId,
        address indexed mentor,
        address indexed mentee,
        uint256 influence
    );
    event MentorshipEnded(
        uint256 indexed mentorshipId,
        address indexed mentor,
        address indexed mentee
    );
    event KnowledgeAdapterCreated(
        uint256 indexed adapterId,
        address indexed creator,
        uint256 quality
    );
    event KnowledgePropagated(
        uint256 indexed mentorshipId,
        uint256 adapterId,
        uint256 weight
    );
    event MentorRewardClaimed(
        address indexed mentor,
        uint256 amount
    );
    event InfluenceUpdated(
        address indexed mentor,
        uint256 newInfluence
    );

    // ============ Constructor ============
    constructor() {}

    // ============ Mentor Management ============

    /**
     * @dev Register as a mentor (requires minimum score).
     * @param initialScore The initial mentor score.
     */
    function registerAsMentor(uint256 initialScore) public {
        require(initialScore >= MIN_MENTOR_SCORE_THRESHOLD, "Score below threshold");
        require(!isMentor[msg.sender], "Already a mentor");
        
        isMentor[msg.sender] = true;
        mentorScore[msg.sender] = initialScore;
        mentorInfluence[msg.sender] = 0;
        
        emit MentorRegistered(msg.sender, initialScore);
    }

    /**
     * @dev Deregister as a mentor.
     */
    function deregisterAsMentor() public {
        require(isMentor[msg.sender], "Not a mentor");
        
        // End all active mentorships
        uint256[] storage myMentorships = _playerMentorships[msg.sender];
        for (uint256 i = 0; i < myMentorships.length; i++) {
            uint256 mentorshipId = myMentorships[i];
            if (mentorships[mentorshipId].active) {
                _endMentorship(mentorshipId);
            }
        }
        
        isMentor[msg.sender] = false;
        
        emit MentorDeregistered(msg.sender);
    }

    /**
     * @dev Update mentor score (called by external validation).
     * @param mentor The mentor address.
     * @param newScore The new score.
     */
    function updateMentorScore(address mentor, uint256 newScore) public onlyOwner {
        require(isMentor[mentor], "Not a mentor");
        require(newScore <= 100, "Score too high");
        
        uint256 oldScore = mentorScore[mentor];
        mentorScore[mentor] = newScore;
        
        // Update influence based on score change
        if (newScore > oldScore) {
            mentorInfluence[mentor] = _min(
                mentorInfluence[mentor] + (newScore - oldScore) * 10,
                MAX_INFLUENCE_PER_MENTOR
            );
        }
        
        emit InfluenceUpdated(mentor, mentorInfluence[mentor]);
    }

    // ============ Mentorship Management ============

    /**
     * @dev Request a mentorship from a mentor.
     * @param mentor The mentor address.
     */
    function requestMentorship(address mentor) public {
        require(isMentor[mentor], "Not a valid mentor");
        require(!isMentor[msg.sender], "Mentors cannot have mentors");
        require(isMentee[msg.sender] == false || _menteeMentors[msg.sender].length < MAX_MENTORS_PER_MENTEE, "Too many mentors");
        
        // Check mentor capacity
        uint256 mentorMenteeCount = _mentorMentees[mentor].length;
        require(mentorMenteeCount < MAX_MENTEES_PER_MENTOR, "Mentor at capacity");
        
        // Create mentorship
        uint256 mentorshipId = _mentorshipIds.current();
        _mentorshipIds.increment();
        
        mentorships[mentorshipId] = Mentorship({
            id: mentorshipId,
            mentor: mentor,
            mentee: msg.sender,
            startTime: block.timestamp,
            influence: mentorInfluence[mentor],
            active: true,
            adapterWeight: 0
        });
        
        // Track relationships
        _playerMentorships[msg.sender].push(mentorshipId);
        _mentorMentees[mentor].push(mentorshipId);
        
        if (!isMentee[msg.sender]) {
            isMentee[msg.sender] = true;
        }
        
        emit MentorshipCreated(mentorshipId, mentor, msg.sender, mentorInfluence[mentor]);
    }

    /**
     * @dev End a mentorship.
     * @param mentorshipId The ID of the mentorship to end.
     */
    function endMentorship(uint256 mentorshipId) public {
        require(mentorships[mentorshipId].active, "Not active");
        require(
            mentorships[mentorshipId].mentor == msg.sender || 
            mentorships[mentorshipId].mentee == msg.sender ||
            msg.sender == owner(),
            "Not authorized"
        );
        
        _endMentorship(mentorshipId);
    }

    /**
     * @dev End mentorship internally.
     */
    function _endMentorship(uint256 mentorshipId) internal {
        Mentorship storage m = mentorships[mentorshipId];
        address mentor = m.mentor;
        address mentee = m.mentee;
        
        // Record final influence in history
        _mentorHistory[mentor].push(InfluenceRecord({
            timestamp: block.timestamp,
            influence: m.influence
        }));
        
        m.active = false;
        
        emit MentorshipEnded(mentorshipId, mentor, mentee);
    }

    // ============ Knowledge Adapter Management ============

    /**
     * @dev Create a knowledge adapter.
     * @param quality The quality of the adapter (0-100).
     * @return adapterId The ID of the created adapter.
     */
    function createAdapter(uint256 quality) public returns (uint256) {
        require(quality <= 100, "Quality too high");
        
        uint256 adapterId = _adapterIds.current();
        _adapterIds.increment();
        
        adapters[adapterId] = KnowledgeAdapter({
            adapterId: adapterId,
            creator: msg.sender,
            quality: quality,
            creationTime: block.timestamp,
            propagationCount: 0
        });
        
        _playerAdapters[msg.sender].push(adapterId);
        
        emit KnowledgeAdapterCreated(adapterId, msg.sender, quality);
        
        return adapterId;
    }

    /**
     * @dev Propagate knowledge adapter to mentee.
     * @param mentorshipId The mentorship ID.
     * @param adapterId The adapter ID to propagate.
     */
    function propagateKnowledge(uint256 mentorshipId, uint256 adapterId) public {
        require(mentorships[mentorshipId].active, "Not active");
        require(mentorships[mentorshipId].mentor == msg.sender, "Not your mentorship");
        
        KnowledgeAdapter storage adapter = adapters[adapterId];
        require(adapter.creator == msg.sender, "Not your adapter");
        
        // Calculate weight based on adapter quality and mentor influence
        uint256 weight = (adapter.quality * mentorships[mentorshipId].influence) / 100;
        
        // Apply decay based on propagation count
        weight = weight * (1000 - adapter.propagationCount * ADAPTER_WEIGHT_DECAY) / 1000;
        
        mentorships[mentorshipId].adapterWeight += weight;
        adapter.propagationCount++;
        
        emit KnowledgePropagated(mentorshipId, adapterId, weight);
    }

    // ============ Reward Distribution ============

    /**
     * @dev Claim mentor rewards.
     * @param amount The amount to claim.
     */
    function claimMentorReward(uint256 amount) public {
        require(isMentor[msg.sender], "Not a mentor");
        
        // Calculate reward based on influence and active mentees
        uint256 rewardableInfluence = mentorInfluence[msg.sender];
        uint256 activeMenteeCount = 0;
        
        uint256[] storage myMentorships = _mentorMentees[msg.sender];
        for (uint256 i = 0; i < myMentorships.length; i++) {
            if (mentorships[myMentorships[i]].active) {
                activeMenteeCount++;
            }
        }
        
        require(activeMenteeCount > 0, "No active mentees");
        
        mentorTotalEarnings[msg.sender] += amount;
        
        emit MentorRewardClaimed(msg.sender, amount);
    }

    // ============ View Functions ============

    /**
     * @dev Get mentee's mentors.
     * @param mentee The mentee address.
     */
    function getMenteesMentors(address mentee) public view returns (address[] memory) {
        uint256[] storage mentorshipIds = _playerMentorships[mentee];
        address[] memory mentors = new address[](mentorshipIds.length);
        
        for (uint256 i = 0; i < mentorshipIds.length; i++) {
            mentors[i] = mentorships[mentorshipIds[i]].mentor;
        }
        
        return mentors;
    }

    /**
     * @dev Get mentor's mentees.
     * @param mentor The mentor address.
     */
    function getMentorsMentees(address mentor) public view returns (address[] memory) {
        uint256[] storage mentorshipIds = _mentorMentees[mentor];
        address[] memory mentees = new address[](mentorshipIds.length);
        
        for (uint256 i = 0; i < mentorshipIds.length; i++) {
            mentees[i] = mentorships[mentorshipIds[i]].mentee;
        }
        
        return mentees;
    }

    /**
     * @dev Get active mentorship count for a mentor.
     */
    function getActiveMentorshipCount(address mentor) public view returns (uint256) {
        uint256[] storage mentorshipIds = _mentorMentees[mentor];
        uint256 count = 0;
        
        for (uint256 i = 0; i < mentorshipIds.length; i++) {
            if (mentorships[mentorshipIds[i]].active) {
                count++;
            }
        }
        
        return count;
    }

    /**
     * @dev Get player's adapters.
     */
    function getPlayerAdapters(address player) public view returns (uint256[] memory) {
        return _playerAdapters[player];
    }

    /**
     * @dev Get mentorship details.
     */
    function getMentorshipDetails(uint256 mentorshipId) public view returns (
        address mentor,
        address mentee,
        uint256 startTime,
        uint256 influence,
        bool active,
        uint256 adapterWeight
    ) {
        Mentorship storage m = mentorships[mentorshipId];
        return (
            m.mentor,
            m.mentee,
            m.startTime,
            m.influence,
            m.active,
            m.adapterWeight
        );
    }

    /**
     * @dev Get mentor's total earnings.
     */
    function getMentorEarnings(address mentor) public view returns (uint256) {
        return mentorTotalEarnings[mentor];
    }

    // ============ Math Helpers ============
    function _min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }
}
