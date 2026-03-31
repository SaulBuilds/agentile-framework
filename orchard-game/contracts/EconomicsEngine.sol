// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Counters.sol";
import "./ORTToken.sol";

/**
 * @dev EconomicsEngine handles farming strategies, rewards distribution, and economic mechanics.
 */
contract EconomicsEngine is Ownable {
    using Counters for Counters.Counter;

    ORTToken public ortToken;
    
    uint256 public constant BASE_REWARD_MULTIPLIER = 1e18;
    uint256 public constant MAX_FARMING_MULTIPLIER = 2e18; // 2x max
    uint256 public constant DAILY_FARMING_BONUS = 0.01e18; // 1% per day
    uint256 public constant FEDERATION_BONUS_PER_MEMBER = 0.1e18; // 10% per member
    uint256 public constant DUEL_WIN_BONUS = 0.25e18; // 25% bonus for winning duels
    
    struct PlayerEconomics {
        uint256 totalStaked;
        uint256 stakeStartTime;
        uint256 lastHarvestTime;
        uint256 totalRewardsClaimed;
        uint256 farmingMultiplier;
    }
    
    mapping(address => PlayerEconomics) public playerEconomics;
    
    struct SeedStake {
        uint256 tokenId;
        uint256 stakeAmount;
        uint256 stakeTime;
        bool harvested;
    }
    
    mapping(address => SeedStake[]) public playerSeeds;
    
    uint256 public totalValueLocked;
    uint256 public seasonalRewardPool;
    uint256 public currentSeasonId;
    
    event StakeDeposited(address indexed player, uint256 amount, uint256 timestamp);
    event StakeWithdrawn(address indexed player, uint256 amount);
    event RewardsClaimed(address indexed player, uint256 amount, uint256 multiplier);
    event FarmingMultiplierUpdated(address indexed player, uint256 newMultiplier);
    event SeasonalRewardPoolAdded(uint256 amount);

    constructor(address _ortToken) {
        ortToken = ORTToken(_ortToken);
    }

    function depositStake(uint256 amount) external {
        require(amount > 0, "Amount must be positive");
        require(ortToken.balanceOf(msg.sender) >= amount, "Insufficient balance");
        
        ortToken.transferFrom(msg.sender, address(this), amount);
        
        PlayerEconomics storage econ = playerEconomics[msg.sender];
        
        if (econ.totalStaked == 0) {
            econ.stakeStartTime = block.timestamp;
        }
        
        econ.totalStaked += amount;
        totalValueLocked += amount;
        
        _updateFarmingMultiplier(msg.sender);
        
        emit StakeDeposited(msg.sender, amount, block.timestamp);
    }

    function withdrawStake(uint256 amount) external {
        PlayerEconomics storage econ = playerEconomics[msg.sender];
        require(econ.totalStaked >= amount, "Insufficient staked amount");
        
        econ.totalStaked -= amount;
        totalValueLocked -= amount;
        
        _updateFarmingMultiplier(msg.sender);
        
        ortToken.transfer(msg.sender, amount);
        
        emit StakeWithdrawn(msg.sender, amount);
    }

    function calculateReward(
        uint256 stakeAmount,
        uint256 growthScore,
        uint256 memberCount,
        bool isDuelWinner,
        uint256 farmingMultiplier
    ) public pure returns (uint256) {
        require(growthScore <= 100, "Invalid growth score");
        
        uint256 baseReward = (stakeAmount * growthScore * BASE_REWARD_MULTIPLIER) / 100;
        
        uint256 federationBonus = baseReward * (FEDERATION_BONUS_PER_MEMBER * memberCount) / BASE_REWARD_MULTIPLIER;
        
        uint256 farmingBonus = baseReward * (farmingMultiplier - BASE_REWARD_MULTIPLIER) / BASE_REWARD_MULTIPLIER;
        
        uint256 duelBonus = isDuelWinner 
            ? (baseReward * DUEL_WIN_BONUS) / BASE_REWARD_MULTIPLIER 
            : 0;
        
        return baseReward + federationBonus + farmingBonus + duelBonus;
    }

    function claimRewards(uint256 seedTokenId) external returns (uint256) {
        PlayerEconomics storage econ = playerEconomics[msg.sender];
        require(econ.totalStaked > 0, "No stake to claim rewards on");
        
        SeedStake[] storage seeds = playerSeeds[msg.sender];
        
        uint256 totalReward = 0;
        
        for (uint256 i = 0; i < seeds.length; i++) {
            if (!seeds[i].harvested) {
                uint256 reward = calculateReward(
                    seeds[i].stakeAmount,
                    50, // Would come from GrowthEngine
                    1,
                    false,
                    econ.farmingMultiplier
                );
                
                seeds[i].harvested = true;
                totalReward += reward;
            }
        }
        
        require(totalReward > 0, "No harvestable seeds");
        
        econ.totalRewardsClaimed += totalReward;
        econ.lastHarvestTime = block.timestamp;
        
        if (seasonalRewardPool >= totalReward) {
            seasonalRewardPool -= totalReward;
            ortToken.transfer(msg.sender, totalReward);
        } else {
            ortToken.transfer(msg.sender, seasonalRewardPool);
            seasonalRewardPool = 0;
        }
        
        emit RewardsClaimed(msg.sender, totalReward, econ.farmingMultiplier);
        
        return totalReward;
    }

    function _updateFarmingMultiplier(address player) internal {
        PlayerEconomics storage econ = playerEconomics[player];
        
        if (econ.totalStaked == 0) {
            econ.farmingMultiplier = BASE_REWARD_MULTIPLIER;
            return;
        }
        
        uint256 daysStaked = (block.timestamp - econ.stakeStartTime) / 1 days;
        uint256 newMultiplier = BASE_REWARD_MULTIPLIER + (daysStaked * DAILY_FARMING_BONUS);
        
        if (newMultiplier > MAX_FARMING_MULTIPLIER) {
            newMultiplier = MAX_FARMING_MULTIPLIER;
        }
        
        econ.farmingMultiplier = newMultiplier;
        
        emit FarmingMultiplierUpdated(player, newMultiplier);
    }

    function getPlayerEconomics(address player) external view returns (
        uint256 totalStaked,
        uint256 stakeStartTime,
        uint256 totalRewardsClaimed,
        uint256 farmingMultiplier
    ) {
        PlayerEconomics storage econ = playerEconomics[player];
        return (
            econ.totalStaked,
            econ.stakeStartTime,
            econ.totalRewardsClaimed,
            econ.farmingMultiplier
        );
    }

    function addToSeasonalPool(uint256 amount) external onlyOwner {
        require(ortToken.transferFrom(msg.sender, address(this), amount), "Transfer failed");
        seasonalRewardPool += amount;
        emit SeasonalRewardPoolAdded(amount);
    }

    function getFarmingMultiplier(address player) external view returns (uint256) {
        return playerEconomics[player].farmingMultiplier;
    }
}
