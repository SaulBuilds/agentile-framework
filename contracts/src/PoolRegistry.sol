// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "./GradientRoles.sol";

/// @title PoolRegistry — Manages pool definitions for the Gradient Barter Protocol
/// @notice Pools are defined by hue (category), band (value range), region, and quality tier
contract PoolRegistry is AccessControl, Pausable, ReentrancyGuard {
    // --- Events ---
    event PoolCreated(uint256 indexed poolId, uint8 hue, uint8 band, string region);
    event PoolRulesUpdated(uint256 indexed poolId, address updatedBy);
    event PoolPaused(uint256 indexed poolId, string reason);
    event PoolUnpaused(uint256 indexed poolId);

    // --- Structs ---
    struct PoolConfig {
        uint8 hue;          // 0=Green, 1=Blue, 2=Amber, 3=Violet, 4=Teal, 5=Red
        uint8 band;         // 1-5
        string region;
        uint8 qualityTier;  // 0=New, 1=Refurbished, 2=UsedGood, 3=Collectible
        bool active;
        bool paused;
    }

    struct PoolRules {
        uint8 transferPolicy;    // 0=blocked, 1=allowlist
        uint256 maxCapacity;
        uint256 holdingPeriodSeconds;
    }

    // --- State ---
    uint256 private _nextPoolId = 1;
    mapping(uint256 => PoolConfig) private _pools;
    mapping(uint256 => PoolRules) private _poolRules;

    constructor(address admin) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(GradientRoles.POOL_ADMIN, admin);
        _grantRole(GradientRoles.EMERGENCY_ADMIN, admin);
    }

    /// @notice Create a new pool
    /// @param hue Category domain (0-5)
    /// @param band Value band (1-5)
    /// @param region Geographic scope
    /// @param qualityTier Condition classification (0-3)
    /// @param rules Initial pool rules
    /// @return poolId The ID of the newly created pool
    function createPool(
        uint8 hue,
        uint8 band,
        string calldata region,
        uint8 qualityTier,
        PoolRules calldata rules
    ) external onlyRole(GradientRoles.POOL_ADMIN) whenNotPaused nonReentrant returns (uint256 poolId) {
        // Checks
        require(hue <= 5, "Invalid hue");
        require(band >= 1 && band <= 5, "Invalid band");
        require(qualityTier <= 3, "Invalid quality tier");

        // Effects
        poolId = _nextPoolId++;
        _pools[poolId] = PoolConfig({
            hue: hue,
            band: band,
            region: region,
            qualityTier: qualityTier,
            active: true,
            paused: false
        });
        _poolRules[poolId] = rules;

        emit PoolCreated(poolId, hue, band, region);
    }

    /// @notice Update rules for an existing pool
    /// @param poolId The pool to update
    /// @param rules The new rules
    function updatePoolRules(
        uint256 poolId,
        PoolRules calldata rules
    ) external onlyRole(GradientRoles.POOL_ADMIN) whenNotPaused nonReentrant {
        require(_pools[poolId].active, "Pool does not exist");

        _poolRules[poolId] = rules;

        emit PoolRulesUpdated(poolId, msg.sender);
    }

    /// @notice Pause a specific pool
    /// @param poolId The pool to pause
    /// @param reason Why the pool is being paused
    function pausePool(uint256 poolId, string calldata reason) external nonReentrant {
        require(
            hasRole(GradientRoles.POOL_ADMIN, msg.sender) ||
            hasRole(GradientRoles.EMERGENCY_ADMIN, msg.sender),
            "Not authorized"
        );
        require(_pools[poolId].active, "Pool does not exist");
        require(!_pools[poolId].paused, "Pool already paused");

        _pools[poolId].paused = true;

        emit PoolPaused(poolId, reason);
    }

    /// @notice Unpause a specific pool
    /// @param poolId The pool to unpause
    function unpausePool(uint256 poolId) external onlyRole(GradientRoles.POOL_ADMIN) nonReentrant {
        require(_pools[poolId].active, "Pool does not exist");
        require(_pools[poolId].paused, "Pool not paused");

        _pools[poolId].paused = false;

        emit PoolUnpaused(poolId);
    }

    /// @notice Global emergency pause — stops all pool creation and updates
    function emergencyPause() external onlyRole(GradientRoles.EMERGENCY_ADMIN) {
        _pause();
    }

    /// @notice Resume from global emergency pause
    function emergencyUnpause() external onlyRole(GradientRoles.EMERGENCY_ADMIN) {
        _unpause();
    }

    // --- View Functions ---

    function getPool(uint256 poolId) external view returns (PoolConfig memory) {
        return _pools[poolId];
    }

    function getPoolRules(uint256 poolId) external view returns (PoolRules memory) {
        return _poolRules[poolId];
    }

    function isPoolActive(uint256 poolId) external view returns (bool) {
        return _pools[poolId].active && !_pools[poolId].paused;
    }

    function nextPoolId() external view returns (uint256) {
        return _nextPoolId;
    }
}
