// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev ORTToken is the native token of the Orchard Game ecosystem.
 * It is used for staking, rewards, and economic interactions.
 */
contract ORTToken is ERC20, Ownable {
    // ============ Constants ============
    uint256 public constant INITIAL_SUPPLY = 1_000_000 * 10 ** 18; // 1 million tokens
    uint256 public constant MIN_STAKE_TO_CREATE = 1000 * 10 ** 18; // 1000 tokens
    uint256 public constant MIN_STAKE_TO_JOIN = 100 * 10 ** 18;   // 100 tokens

    // ============ Events ============
    event Staked(address indexed player, uint256 amount);
    event Unstaked(address indexed player, uint256 amount);
    event Rewarded(address indexed player, uint256 amount);
    event SeedStakedInFederation(address indexed player, uint256 federationId, uint256 tokenId, uint256 amount);
    event SeedUnstakedFromFederation(address indexed player, uint256 federationId, uint256 tokenId, uint256 amount);

    // ============ Constructor ============
    constructor() ERC20("Orchard Token", "ORT") {
        _mint(msg.sender, INITIAL_SUPPLY);
    }

    // ============ Public Functions ============

    /**
     * @dev Stake tokens for a player.
     * @param amount The amount of ORT to stake.
     */
    function stake(uint256 amount) public {
        require(amount > 0, "Amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");

        _transfer(msg.sender, address(this), amount);

        emit Staked(msg.sender, amount);
    }

    /**
     * @dev Unstake tokens for a player.
     * @param amount The amount of ORT to unstake.
     */
    function unstake(uint256 amount) public {
        require(amount > 0, "Amount must be greater than zero");
        require(balanceOf(address(this)) >= amount, "Insufficient staked balance");

        _transfer(address(this), msg.sender, amount);

        emit Unstaked(msg.sender, amount);
    }

    /**
     * @dev Reward tokens to a player (e.g., from harvest).
     * @param player The address of the player to reward.
     * @param amount The amount of ORT to reward.
     */
    function reward(address player, uint256 amount) public {
        require(amount > 0, "Amount must be greater than zero");
        require(balanceOf(address(this)) >= amount, "Insufficient reward balance");

        _transfer(address(this), player, amount);

        emit Rewarded(player, amount);
    }

    /**
     * @dev Stake a seed in a federation using ORT tokens.
     * This function should be called after the player has staked ORT tokens
     * and before calling the Federation contract's stakeSeed function.
     * @param federationId The ID of the federation.
     * @param tokenId The ID of the SeedNFT to stake.
     * @param amount The amount of ORT to stake.
     */
    function stakeSeedInFederation(
        uint256 federationId,
        uint256 tokenId,
        uint256 amount
    ) public {
        require(amount > 0, "Amount must be greater than zero");
        require(balanceOf(msg.sender) >= amount, "Insufficient balance");

        // Transfer tokens to this contract (which acts as a staking escrow)
        _transfer(msg.sender, address(this), amount);

        emit SeedStakedInFederation(msg.sender, federationId, tokenId, amount);
    }

    /**
     * @dev Unstake a seed from a federation and return ORT tokens to the player.
     * This function should be called after the Federation contract's unstakeSeed function.
     * @param federationId The ID of the federation.
     * @param tokenId The ID of the SeedNFT to unstake.
     * @param amount The amount of ORT to unstake.
     */
    function unstakeSeedFromFederation(
        uint256 federationId,
        uint256 tokenId,
        uint256 amount
    ) public {
        require(amount > 0, "Amount must be greater than zero");
        require(balanceOf(address(this)) >= amount, "Insufficient staked balance in contract");

        // Transfer tokens back to the player
        _transfer(address(this), msg.sender, amount);

        emit SeedUnstakedFromFederation(msg.sender, federationId, tokenId, amount);
    }

    // ============ View Functions ============

    /**
     * @dev Get the balance of ORT tokens staked in this contract (escrow).
     * @return The amount of ORT tokens currently staked.
     */
    function stakedBalance()
        public
        view
        returns (uint256)
    {
        return balanceOf(address(this));
    }
}