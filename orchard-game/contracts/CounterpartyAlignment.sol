// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev CounterpartyAlignment tracks player relationships based on:
 * - Input similarity (seed payload hashing)
 * - Ideology matching (philosophical approach tags)
 * - Federation membership (collaborative play)
 * - Duel outcomes (win/loss creates rivalry/alliance)
 */
contract CounterpartyAlignment is Ownable {
    struct PlayerProfile {
        bytes32 inputHash;
        bytes32 ideologyHash;
        uint256 ideologyScore;
        uint256[] federationIds;
        uint256 totalDuels;
        uint256 wins;
        uint256 losses;
    }
    
    struct Alignment {
        address playerA;
        address playerB;
        uint256 similarityScore;
        uint256 alignmentStrength;
        bool isRival;
        bool isAlly;
        uint256 lastInteraction;
    }
    
    mapping(address => PlayerProfile) public playerProfiles;
    mapping(bytes32 => Alignment) public alignments;
    
    uint256 public constant MAX_IDEOLOGY_SCORE = 100;
    uint256 public constant SIMILARITY_THRESHOLD = 70;
    uint256 public constant RIVALRY_THRESHOLD = 30;
    
    mapping(address => address[]) public playerConnections;
    mapping(address => uint256[]) public rivalryScores;
    mapping(address => uint256[]) public allyScores;
    
    event ProfileUpdated(address indexed player);
    event AlignmentDiscovered(address indexed playerA, address indexed playerB, uint256 score);
    event RivalryCreated(address indexed playerA, address indexed playerB);
    event AllyCreated(address indexed playerA, address indexed playerB);

    function updateInputHash(bytes32 inputHash) external {
        playerProfiles[msg.sender].inputHash = inputHash;
        emit ProfileUpdated(msg.sender);
    }
    
    function updateIdeology(bytes32 ideologyHash, uint256 ideologyScore) external {
        require(ideologyScore <= MAX_IDEOLOGY_SCORE, "Score too high");
        
        PlayerProfile storage profile = playerProfiles[msg.sender];
        profile.ideologyHash = ideologyHash;
        profile.ideologyScore = ideologyScore;
        
        emit ProfileUpdated(msg.sender);
    }
    
    function joinFederation(uint256 federationId) external {
        PlayerProfile storage profile = playerProfiles[msg.sender];
        profile.federationIds.push(federationId);
        
        emit ProfileUpdated(msg.sender);
    }
    
    function recordDuelOutcome(address opponent, bool won) external {
        PlayerProfile storage profile = playerProfiles[msg.sender];
        profile.totalDuels++;
        
        if (won) {
            profile.wins++;
        } else {
            profile.losses++;
        }
        
        _updateAlignment(msg.sender, opponent, won);
    }
    
    function _updateAlignment(address playerA, address playerB, bool won) internal {
        bytes32 alignmentKey = _getAlignmentKey(playerA, playerB);
        
        Alignment storage alignment = alignments[alignmentKey];
        alignment.playerA = playerA;
        alignment.playerB = playerB;
        alignment.lastInteraction = block.timestamp;
        
        if (won) {
            alignment.isRival = true;
            alignment.alignmentStrength = 100;
            rivalryScores[playerA].push(100);
            emit RivalryCreated(playerA, playerB);
        } else {
            alignment.isAlly = true;
            alignment.alignmentStrength = 50;
            allyScores[playerA].push(50);
            emit AllyCreated(playerA, playerB);
        }
    }
    
    function calculateSimilarity(address playerA, address playerB) public view returns (uint256) {
        PlayerProfile storage profileA = playerProfiles[playerA];
        PlayerProfile storage profileB = playerProfiles[playerB];
        
        if (profileA.inputHash == 0 || profileB.inputHash == 0) {
            return 0;
        }
        
        uint256 matchingBits = 0;
        bytes32 hashA = profileA.inputHash;
        bytes32 hashB = profileB.inputHash;
        
        for (uint256 i = 0; i < 256; i++) {
            if ((hashA >> i) == (hashB >> i)) {
                matchingBits++;
            }
        }
        
        return (matchingBits * 100) / 256;
    }
    
    function discoverAlignment(address playerA, address playerB) external {
        uint256 similarity = calculateSimilarity(playerA, playerB);
        
        if (similarity >= SIMILARITY_THRESHOLD) {
            bytes32 alignmentKey = _getAlignmentKey(playerA, playerB);
            
            Alignment storage alignment = alignments[alignmentKey];
            alignment.playerA = playerA;
            alignment.playerB = playerB;
            alignment.similarityScore = similarity;
            alignment.alignmentStrength = similarity;
            alignment.lastInteraction = block.timestamp;
            
            playerConnections[playerA].push(playerB);
            playerConnections[playerB].push(playerA);
            
            emit AlignmentDiscovered(playerA, playerB, similarity);
        }
    }
    
    function getPlayerAlignment(address player, address target) external view returns (
        uint256 similarityScore,
        uint256 alignmentStrength,
        bool isRival,
        bool isAlly
    ) {
        bytes32 alignmentKey = _getAlignmentKey(player, target);
        Alignment storage alignment = alignments[alignmentKey];
        
        return (
            alignment.similarityScore,
            alignment.alignmentStrength,
            alignment.isRival,
            alignment.isAlly
        );
    }
    
    function getConnectedPlayers(address player) external view returns (address[] memory) {
        return playerConnections[player];
    }
    
    function getPlayerStats(address player) external view returns (
        uint256 totalDuels,
        uint256 wins,
        uint256 losses,
        uint256 winRate
    ) {
        PlayerProfile storage profile = playerProfiles[player];
        uint256 winRate = profile.totalDuels > 0 
            ? (profile.wins * 100) / profile.totalDuels 
            : 0;
        
        return (
            profile.totalDuels,
            profile.wins,
            profile.losses,
            winRate
        );
    }
    
    function _getAlignmentKey(address playerA, address playerB) internal pure returns (bytes32) {
        return playerA < playerB
            ? keccak256(abi.encodePacked(playerA, playerB))
            : keccak256(abi.encodePacked(playerB, playerA));
    }
}
