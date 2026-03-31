// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev SchoolSafety handles content filtering, privacy protection, and panic controls.
 * Per SchoolSafety.tla specifications for educational environments.
 */
contract SchoolSafety is Ownable {
    using Counters for Counters.Counter;

    // ============ Constants ============
    uint256 public constant MAX_CONTENT_LENGTH = 10000;
    uint256 public constant PANIC_COOLDOWN = 1 hours;

    // ============ Panic State ============
    bool public panicActive;
    uint256 public panicStartTime;
    address public panicInitiator;

    // ============ Content State ============
    enum ContentState {
        PENDING,
        APPROVED,
        REJECTED,
        FLAGGED
    }

    struct ContentRecord {
        uint256 contentId;
        address submitter;
        string contentHash;
        ContentState state;
        uint256 submittedAt;
        uint256 reviewedAt;
        address reviewer;
        string reviewNotes;
    }

    mapping(uint256 => ContentRecord) public contentRecords;
    Counters.Counter private _contentIds;

    // Approved educators
    mapping(address => bool) public approvedEducators;
    mapping(address => uint256) public educatorPermissions; // 0=none, 1=review, 2=admin

    // Privacy settings
    bool public privacyByDefault = true;
    mapping(address => bool) public privacyExemptions;

    // ============ Events ============
    event PanicActivated(address indexed initiator, uint256 timestamp);
    event PanicDeactivated(address indexed initiator, uint256 duration);
    event ContentSubmitted(uint256 indexed contentId, address indexed submitter, string contentHash);
    event ContentApproved(uint256 indexed contentId, address indexed reviewer);
    event ContentRejected(uint256 indexed contentId, address indexed reviewer, string reason);
    event ContentFlagged(uint256 indexed contentId, address indexed flagger, string reason);
    event EducatorApproved(address indexed educator, uint256 permissions);
    event EducatorRevoked(address indexed educator);
    event PrivacySettingChanged(address indexed user, bool exempted);
    event SafetyIncidentReported(address indexed reporter, string description, uint256 timestamp);

    // ============ Constructor ============
    constructor() {
        panicActive = false;
    }

    // ============ Panic Functions ============

    /**
     * @dev Activate panic mode - stops all interactions
     */
    function activatePanic(string calldata reason) public onlyOwner {
        require(!panicActive, "Panic already active");
        
        panicActive = true;
        panicStartTime = block.timestamp;
        panicInitiator = msg.sender;
        
        emit PanicActivated(msg.sender, block.timestamp);
    }

    /**
     * @dev Deactivate panic mode
     */
    function deactivatePanic() public onlyOwner {
        require(panicActive, "No active panic");
        require(block.timestamp >= panicStartTime + PANIC_COOLDOWN, "Panic cooldown active");
        
        uint256 duration = block.timestamp - panicStartTime;
        panicActive = false;
        
        emit PanicDeactivated(msg.sender, duration);
    }

    /**
     * @dev Check if operations are allowed
     */
    function _checkPanic() internal view {
        require(!panicActive, "System in panic mode");
    }

    // ============ Content Management ============

    /**
     * @dev Submit content for review
     */
    function submitContent(string calldata contentHash) public {
        _checkPanic();
        require(bytes(contentHash).length > 0, "Content hash required");
        require(bytes(contentHash).length <= MAX_CONTENT_LENGTH, "Content too long");
        
        uint256 contentId = _contentIds.current();
        _contentIds.increment();
        
        contentRecords[contentId] = ContentRecord({
            contentId: contentId,
            submitter: msg.sender,
            contentHash: contentHash,
            state: ContentState.PENDING,
            submittedAt: block.timestamp,
            reviewedAt: 0,
            reviewer: address(0),
            reviewNotes: ""
        });
        
        emit ContentSubmitted(contentId, msg.sender, contentHash);
    }

    /**
     * @dev Approve content
     */
    function approveContent(uint256 contentId, string calldata notes) public {
        _checkPanic();
        require(educatorPermissions[msg.sender] >= 1, "Not authorized to review");
        require(contentRecords[contentId].contentId == contentId, "Content not found");
        require(contentRecords[contentId].state == ContentState.PENDING, "Content not pending");
        
        ContentRecord storage record = contentRecords[contentId];
        record.state = ContentState.APPROVED;
        record.reviewedAt = block.timestamp;
        record.reviewer = msg.sender;
        record.reviewNotes = notes;
        
        emit ContentApproved(contentId, msg.sender);
    }

    /**
     * @dev Reject content
     */
    function rejectContent(uint256 contentId, string calldata reason) public {
        _checkPanic();
        require(educatorPermissions[msg.sender] >= 1, "Not authorized to review");
        require(contentRecords[contentId].contentId == contentId, "Content not found");
        require(contentRecords[contentId].state == ContentState.PENDING, "Content not pending");
        
        ContentRecord storage record = contentRecords[contentId];
        record.state = ContentState.REJECTED;
        record.reviewedAt = block.timestamp;
        record.reviewer = msg.sender;
        record.reviewNotes = reason;
        
        emit ContentRejected(contentId, msg.sender, reason);
    }

    /**
     * @dev Flag content for review
     */
    function flagContent(uint256 contentId, string calldata reason) public {
        _checkPanic();
        require(contentRecords[contentId].contentId == contentId, "Content not found");
        
        ContentRecord storage record = contentRecords[contentId];
        record.state = ContentState.FLAGGED;
        
        emit ContentFlagged(contentId, msg.sender, reason);
    }

    // ============ Educator Management ============

    /**
     * @dev Approve an educator
     */
    function approveEducator(address educator, uint256 permissions) public onlyOwner {
        require(educator != address(0), "Invalid educator address");
        require(permissions >= 1 && permissions <= 2, "Invalid permissions level");
        
        approvedEducators[educator] = true;
        educatorPermissions[educator] = permissions;
        
        emit EducatorApproved(educator, permissions);
    }

    /**
     * @dev Revoke educator
     */
    function revokeEducator(address educator) public onlyOwner {
        approvedEducators[educator] = false;
        educatorPermissions[educator] = 0;
        
        emit EducatorRevoked(educator);
    }

    // ============ Privacy Functions ============

    /**
     * @dev Set privacy exemption for a user
     */
    function setPrivacyExemption(address user, bool exempted) public onlyOwner {
        privacyExemptions[user] = exempted;
        
        emit PrivacySettingChanged(user, exempted);
    }

    /**
     * @dev Check if user data can be viewed
     */
    function canViewUserData(address user) public view returns (bool) {
        if (!privacyByDefault) return true;
        if (privacyExemptions[user]) return true;
        if (educatorPermissions[msg.sender] >= 1) return true;
        return msg.sender == user;
    }

    // ============ Safety Reporting ============

    /**
     * @dev Report a safety incident
     */
    function reportIncident(string calldata description) public {
        require(bytes(description).length > 0, "Description required");
        
        emit SafetyIncidentReported(msg.sender, description, block.timestamp);
    }

    // ============ View Functions ============

    /**
     * @dev Get content details
     */
    function getContent(uint256 contentId) public view returns (
        address submitter,
        string memory contentHash,
        ContentState state,
        uint256 submittedAt,
        uint256 reviewedAt,
        address reviewer
    ) {
        ContentRecord storage record = contentRecords[contentId];
        return (
            record.submitter,
            record.contentHash,
            record.state,
            record.submittedAt,
            record.reviewedAt,
            record.reviewer
        );
    }

    /**
     * @dev Get panic status
     */
    function getPanicStatus() public view returns (
        bool active,
        uint256 startTime,
        address initiator,
        uint256 timeRemaining
    ) {
        uint256 timeRemainingCalc = 0;
        if (panicActive && panicStartTime + PANIC_COOLDOWN > block.timestamp) {
            timeRemainingCalc = panicStartTime + PANIC_COOLDOWN - block.timestamp;
        }
        return (panicActive, panicStartTime, panicInitiator, timeRemainingCalc);
    }

    /**
     * @dev Check if content is approved
     */
    function isContentApproved(uint256 contentId) public view returns (bool) {
        return contentRecords[contentId].state == ContentState.APPROVED;
    }
}
