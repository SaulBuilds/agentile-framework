// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/PoolRegistry.sol";
import "../src/GradientRoles.sol";

contract PoolRegistryTest is Test {
    PoolRegistry public registry;
    address public admin = address(1);
    address public poolAdmin = address(2);
    address public unauthorized = address(3);
    address public emergencyAdmin = address(4);

    function setUp() public {
        vm.startPrank(admin);
        registry = new PoolRegistry(admin);
        registry.grantRole(GradientRoles.POOL_ADMIN, poolAdmin);
        registry.grantRole(GradientRoles.EMERGENCY_ADMIN, emergencyAdmin);
        vm.stopPrank();
    }

    function _defaultRules() internal pure returns (PoolRegistry.PoolRules memory) {
        return PoolRegistry.PoolRules({
            transferPolicy: 0,
            maxCapacity: 100,
            holdingPeriodSeconds: 86400
        });
    }

    // --- Pool Creation ---

    function test_createPool_success() public {
        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Pacific NW", 0, _defaultRules());

        assertEq(poolId, 1);

        PoolRegistry.PoolConfig memory pool = registry.getPool(poolId);
        assertEq(pool.hue, 0);
        assertEq(pool.band, 1);
        assertEq(keccak256(bytes(pool.region)), keccak256(bytes("Pacific NW")));
        assertEq(pool.qualityTier, 0);
        assertTrue(pool.active);
        assertFalse(pool.paused);
    }

    function test_createPool_emitsEvent() public {
        vm.expectEmit(true, false, false, true);
        emit PoolRegistry.PoolCreated(1, 0, 1, "Pacific NW");

        vm.prank(poolAdmin);
        registry.createPool(0, 1, "Pacific NW", 0, _defaultRules());
    }

    function test_createPool_incrementsId() public {
        vm.startPrank(poolAdmin);
        uint256 id1 = registry.createPool(0, 1, "Region A", 0, _defaultRules());
        uint256 id2 = registry.createPool(1, 2, "Region B", 1, _defaultRules());
        vm.stopPrank();

        assertEq(id1, 1);
        assertEq(id2, 2);
    }

    function test_createPool_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        registry.createPool(0, 1, "Region", 0, _defaultRules());
    }

    function test_createPool_revertsForInvalidHue() public {
        vm.prank(poolAdmin);
        vm.expectRevert("Invalid hue");
        registry.createPool(6, 1, "Region", 0, _defaultRules());
    }

    function test_createPool_revertsForInvalidBand_zero() public {
        vm.prank(poolAdmin);
        vm.expectRevert("Invalid band");
        registry.createPool(0, 0, "Region", 0, _defaultRules());
    }

    function test_createPool_revertsForInvalidBand_six() public {
        vm.prank(poolAdmin);
        vm.expectRevert("Invalid band");
        registry.createPool(0, 6, "Region", 0, _defaultRules());
    }

    function test_createPool_revertsForInvalidQualityTier() public {
        vm.prank(poolAdmin);
        vm.expectRevert("Invalid quality tier");
        registry.createPool(0, 1, "Region", 4, _defaultRules());
    }

    // --- Pool Rules Update ---

    function test_updatePoolRules_success() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        PoolRegistry.PoolRules memory newRules = PoolRegistry.PoolRules({
            transferPolicy: 1,
            maxCapacity: 200,
            holdingPeriodSeconds: 172800
        });
        registry.updatePoolRules(poolId, newRules);
        vm.stopPrank();

        PoolRegistry.PoolRules memory stored = registry.getPoolRules(poolId);
        assertEq(stored.transferPolicy, 1);
        assertEq(stored.maxCapacity, 200);
        assertEq(stored.holdingPeriodSeconds, 172800);
    }

    function test_updatePoolRules_revertsForNonexistentPool() public {
        vm.prank(poolAdmin);
        vm.expectRevert("Pool does not exist");
        registry.updatePoolRules(999, _defaultRules());
    }

    function test_updatePoolRules_emitsEvent() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        vm.expectEmit(true, false, false, true);
        emit PoolRegistry.PoolRulesUpdated(poolId, poolAdmin);
        registry.updatePoolRules(poolId, _defaultRules());
        vm.stopPrank();
    }

    // --- Pool Pause/Unpause ---

    function test_pausePool_byPoolAdmin() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());
        registry.pausePool(poolId, "Maintenance");
        vm.stopPrank();

        PoolRegistry.PoolConfig memory pool = registry.getPool(poolId);
        assertTrue(pool.paused);
        assertFalse(registry.isPoolActive(poolId));
    }

    function test_pausePool_byEmergencyAdmin() public {
        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        vm.prank(emergencyAdmin);
        registry.pausePool(poolId, "Emergency");

        assertTrue(registry.getPool(poolId).paused);
    }

    function test_pausePool_emitsEvent() public {
        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        vm.expectEmit(true, false, false, true);
        emit PoolRegistry.PoolPaused(poolId, "Test reason");

        vm.prank(poolAdmin);
        registry.pausePool(poolId, "Test reason");
    }

    function test_pausePool_revertsForUnauthorized() public {
        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        vm.prank(unauthorized);
        vm.expectRevert("Not authorized");
        registry.pausePool(poolId, "Unauthorized");
    }

    function test_pausePool_revertsIfAlreadyPaused() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());
        registry.pausePool(poolId, "First");

        vm.expectRevert("Pool already paused");
        registry.pausePool(poolId, "Second");
        vm.stopPrank();
    }

    function test_unpausePool_success() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());
        registry.pausePool(poolId, "Maintenance");
        registry.unpausePool(poolId);
        vm.stopPrank();

        assertFalse(registry.getPool(poolId).paused);
        assertTrue(registry.isPoolActive(poolId));
    }

    function test_unpausePool_revertsIfNotPaused() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        vm.expectRevert("Pool not paused");
        registry.unpausePool(poolId);
        vm.stopPrank();
    }

    // --- Emergency Global Pause ---

    function test_emergencyPause_blocksPoolCreation() public {
        vm.prank(emergencyAdmin);
        registry.emergencyPause();

        vm.prank(poolAdmin);
        vm.expectRevert();
        registry.createPool(0, 1, "Region", 0, _defaultRules());
    }

    function test_emergencyUnpause_resumesPoolCreation() public {
        vm.prank(emergencyAdmin);
        registry.emergencyPause();

        vm.prank(emergencyAdmin);
        registry.emergencyUnpause();

        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());
        assertEq(poolId, 1);
    }

    function test_emergencyPause_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        registry.emergencyPause();
    }

    // --- View Functions ---

    function test_isPoolActive_trueForActivePool() public {
        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());

        assertTrue(registry.isPoolActive(poolId));
    }

    function test_isPoolActive_falseForPausedPool() public {
        vm.startPrank(poolAdmin);
        uint256 poolId = registry.createPool(0, 1, "Region", 0, _defaultRules());
        registry.pausePool(poolId, "Paused");
        vm.stopPrank();

        assertFalse(registry.isPoolActive(poolId));
    }

    function test_isPoolActive_falseForNonexistentPool() public view {
        assertFalse(registry.isPoolActive(999));
    }

    // --- Fuzz Tests ---

    function testFuzz_createPool_validParams(uint8 hue, uint8 band, uint8 tier) public {
        hue = uint8(bound(hue, 0, 5));
        band = uint8(bound(band, 1, 5));
        tier = uint8(bound(tier, 0, 3));

        vm.prank(poolAdmin);
        uint256 poolId = registry.createPool(hue, band, "Fuzz Region", tier, _defaultRules());

        PoolRegistry.PoolConfig memory pool = registry.getPool(poolId);
        assertEq(pool.hue, hue);
        assertEq(pool.band, band);
        assertEq(pool.qualityTier, tier);
    }
}
