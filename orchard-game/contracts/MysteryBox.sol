// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @dev MysteryBox provides randomized reward boxes.
 * Per MysteryBoxRNG.tla specifications.
 */
contract MysteryBox is Ownable {
    using Counters for Counters.Counter;

    // ============ Constants ============
    uint256 public constant MIN_BOX_VALUE = 10 ether;
    uint256 public constant MAX_BOX_VALUE = 10000 ether;
    uint256 public constant REVEAL_DELAY = 1 hours;
    uint256 public constant BOX_EXPIRY = 7 days;

    // ============ Box State ============
    enum BoxState {
        CREATED,
        AWARDED,
        OPENED,
        CLAIMED,
        EXPIRED
    }

    // ============ State Variables ============
    Counters.Counter private _boxIds;
    Counters.Counter private _rewardIds;

    // Box definitions
    struct BoxType {
        uint256 boxTypeId;
        string name;
        uint256 minValue;
        uint256 maxValue;
        uint256 probability; // 0-10000 (0.01% - 100%)
        uint256 quantity; // total boxes of this type
        uint256 claimed;
    }
    
    mapping(uint256 => BoxType) public boxTypes;
    uint256[] private _activeBoxTypes;

    // Awarded boxes
    struct MysteryBox {
        uint256 boxId;
        uint256 boxTypeId;
        address recipient;
        uint256 awardedAt;
        uint256 openedAt;
        uint256 claimedAt;
        uint256 revealAt;
        uint256 rewardAmount;
        bytes32 commitHash;
        bool revealed;
        BoxState state;
    }
    
    mapping(uint256 => MysteryBox) public boxes;
    mapping(address => uint256[]) private _recipientBoxes;

    // Reward pool
    uint256 public rewardPool;
    uint256 public totalDistributed;

    // RNG state
    uint256 private _seed;
    mapping(uint256 => bool) private _usedSeeds;

    // ============ Events ============
    event BoxTypeCreated(
        uint256 indexed boxTypeId,
        string name,
        uint256 minValue,
        uint256 maxValue,
        uint256 probability,
        uint256 quantity
    );

    event BoxAwarded(
        uint256 indexed boxId,
        uint256 indexed boxTypeId,
        address indexed recipient,
        bytes32 commitHash
    );

    event BoxOpened(
        uint256 indexed boxId,
        uint256 rewardAmount
    );

    event BoxClaimed(
        uint256 indexed boxId,
        address indexed recipient,
        uint256 amount
    );

    event BoxExpired(
        uint256 indexed boxId
    );

    event RewardPoolFunded(
        address indexed funder,
        uint256 amount
    );

    event RewardsDistributed(
        uint256 indexed boxId,
        address indexed recipient,
        uint256 amount
    );

    // ============ Constructor ============
    constructor() {
        _seed = block.timestamp;
    }

    // ============ Admin Functions ============

    /**
     * @dev Create a new box type.
     */
    function createBoxType(
        string calldata name,
        uint256 minValue,
        uint256 maxValue,
        uint256 probability,
        uint256 quantity
    ) public onlyOwner {
        require(minValue >= MIN_BOX_VALUE, "Min value too low");
        require(maxValue <= MAX_BOX_VALUE, "Max value too high");
        require(minValue <= maxValue, "Invalid value range");
        require(probability <= 10000, "Probability too high");
        require(quantity > 0, "Quantity must be positive");
        
        uint256 boxTypeId = _boxIds.current();
        _boxIds.increment();
        
        boxTypes[boxTypeId] = BoxType({
            boxTypeId: boxTypeId,
            name: name,
            minValue: minValue,
            maxValue: maxValue,
            probability: probability,
            quantity: quantity,
            claimed: 0
        });
        
        _activeBoxTypes.push(boxTypeId);
        
        emit BoxTypeCreated(boxTypeId, name, minValue, maxValue, probability, quantity);
    }

    /**
     * @dev Fund the reward pool.
     */
    function fundRewardPool() public payable {
        require(msg.value > 0, "Must send ETH");
        rewardPool += msg.value;
        
        emit RewardPoolFunded(msg.sender, msg.value);
    }

    /**
     * @dev Withdraw excess funds (only if pool > distributed).
     */
    function withdrawExcess(uint256 amount) public onlyOwner {
        require(amount <= rewardPool - totalDistributed, "Cannot withdraw distributed funds");
        rewardPool -= amount;
        payable(owner()).transfer(amount);
    }

    // ============ Box Awarding ============

    /**
     * @dev Award a mystery box to a recipient.
     * @param recipient The recipient address.
     * @param boxTypeId The type of box to award.
     * @param commitHash Hash to prevent front-running.
     */
    function awardBox(address recipient, uint256 boxTypeId, bytes32 commitHash) public onlyOwner {
        BoxType storage boxType = boxTypes[boxTypeId];
        require(boxType.quantity > 0, "Box type not found");
        require(boxType.claimed < boxType.quantity, "All boxes claimed");
        
        uint256 boxId = _rewardIds.current();
        _rewardIds.increment();
        
        uint256 revealAt = block.timestamp + REVEAL_DELAY;
        
        boxes[boxId] = MysteryBox({
            boxId: boxId,
            boxTypeId: boxTypeId,
            recipient: recipient,
            awardedAt: block.timestamp,
            openedAt: 0,
            claimedAt: 0,
            revealAt: revealAt,
            rewardAmount: 0,
            commitHash: commitHash,
            revealed: false,
            state: BoxState.AWARDED
        });
        
        boxType.claimed++;
        _recipientBoxes[recipient].push(boxId);
        
        emit BoxAwarded(boxId, boxTypeId, recipient, commitHash);
    }

    /**
     * @dev Batch award boxes.
     */
    function batchAwardBoxes(address[] calldata recipients, uint256 boxTypeId) public onlyOwner {
        require(recipients.length > 0, "No recipients");
        
        bytes32 baseCommit = keccak256(abi.encodePacked(block.timestamp, msg.sender));
        
        for (uint256 i = 0; i < recipients.length; i++) {
            bytes32 commitHash = keccak256(abi.encodePacked(baseCommit, i));
            awardBox(recipients[i], boxTypeId, commitHash);
        }
    }

    // ============ Box Opening ============

    /**
     * @dev Open a box and reveal the reward.
     * @param boxId The box ID.
     * @param revealData Data to verify commit hash.
     */
    function openBox(uint256 boxId, bytes32 revealData) public {
        MysteryBox storage box = boxes[boxId];
        require(box.recipient == msg.sender, "Not recipient");
        require(box.state == BoxState.AWARDED, "Box not awarded");
        require(block.timestamp >= box.revealAt, "Too soon to reveal");
        
        // Verify commit
        bytes32 computedHash = keccak256(abi.encodePacked(revealData, box.recipient, box.boxId));
        require(computedHash == box.commitHash, "Invalid reveal data");
        
        // Generate reward
        BoxType storage boxType = boxTypes[box.boxTypeId];
        uint256 reward = _generateReward(boxType);
        
        require(rewardPool >= reward, "Insufficient reward pool");
        
        box.rewardAmount = reward;
        box.openedAt = block.timestamp;
        box.revealed = true;
        box.state = BoxState.OPENED;
        
        rewardPool -= reward;
        
        emit BoxOpened(boxId, reward);
    }

    /**
     * @dev Claim the reward from an opened box.
     */
    function claimBoxReward(uint256 boxId) public {
        MysteryBox storage box = boxes[boxId];
        require(box.recipient == msg.sender, "Not recipient");
        require(box.state == BoxState.OPENED, "Box not opened");
        require(box.revealed, "Box not revealed");
        
        uint256 reward = box.rewardAmount;
        require(reward > 0, "No reward");
        
        box.state = BoxState.CLAIMED;
        box.claimedAt = block.timestamp;
        totalDistributed += reward;
        
        payable(msg.sender).transfer(reward);
        
        emit BoxClaimed(boxId, msg.sender, reward);
    }

    /**
     * @dev Expire unclaimed boxes.
     */
    function expireBox(uint256 boxId) public onlyOwner {
        MysteryBox storage box = boxes[boxId];
        require(box.state == BoxState.AWARDED, "Box not awarded");
        require(block.timestamp >= box.awardedAt + BOX_EXPIRY, "Box not expired");
        
        box.state = BoxState.EXPIRED;
        
        emit BoxExpired(boxId);
    }

    // ============ View Functions ============

    /**
     * @dev Get box details.
     */
    function getBoxDetails(uint256 boxId) public view returns (
        uint256 boxTypeId,
        address recipient,
        uint256 awardedAt,
        uint256 openedAt,
        uint256 rewardAmount,
        bool revealed,
        BoxState state
    ) {
        MysteryBox storage box = boxes[boxId];
        return (
            box.boxTypeId,
            box.recipient,
            box.awardedAt,
            box.openedAt,
            box.rewardAmount,
            box.revealed,
            box.state
        );
    }

    /**
     * @dev Get recipient's boxes.
     */
    function getRecipientBoxes(address recipient) public view returns (uint256[] memory) {
        return _recipientBoxes[recipient];
    }

    /**
     * @dev Get active box types.
     */
    function getActiveBoxTypes() public view returns (uint256[] memory) {
        return _activeBoxTypes;
    }

    /**
     * @dev Get box type details.
     */
    function getBoxTypeDetails(uint256 boxTypeId) public view returns (
        string memory name,
        uint256 minValue,
        uint256 maxValue,
        uint256 probability,
        uint256 quantity,
        uint256 claimed
    ) {
        BoxType storage bt = boxTypes[boxTypeId];
        return (
            bt.name,
            bt.minValue,
            bt.maxValue,
            bt.probability,
            bt.quantity,
            bt.claimed
        );
    }

    /**
     * @dev Check if a box can be claimed.
     */
    function canClaim(uint256 boxId) public view returns (bool) {
        MysteryBox storage box = boxes[boxId];
        return box.state == BoxState.OPENED && box.revealed && box.recipient == msg.sender;
    }

    // ============ Internal Functions ============

    function _generateReward(BoxType storage boxType) internal returns (uint256) {
        // Use pseudo-random based on block data
        _seed = uint256(keccak256(abi.encodePacked(_seed, block.timestamp, block.difficulty)));
        
        // Check if lucky (within probability)
        uint256 luckyRoll = _seed % 10000;
        
        if (luckyRoll < boxType.probability) {
            // Lucky - return max value
            return boxType.maxValue;
        } else {
            // Not lucky - return random value in range
            uint256 range = boxType.maxValue - boxType.minValue;
            uint256 randomValue = boxType.minValue + (_seed % range);
            return randomValue;
        }
    }

    // ============ Fallback ============

    receive() external payable {
        fundRewardPool();
    }
}
