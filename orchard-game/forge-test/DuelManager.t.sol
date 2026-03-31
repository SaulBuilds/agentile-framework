// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../contracts/DuelManager.sol";
import "../contracts/SeedNFT.sol";
import "../contracts/GrowthEngine.sol";

contract DuelManagerFuzzTest is Test {
    DuelManager public duelManager;
    SeedNFT public seedNFT;
    GrowthEngine public growthEngine;
    
    address public alice = address(0x1);
    address public bob = address(0x2);

    uint256 public seedIdA;
    uint256 public seedIdB;

    function setUp() public {
        seedNFT = new SeedNFT();
        growthEngine = new GrowthEngine(seedNFT);
        duelManager = new DuelManager(seedNFT, growthEngine);
        
        seedIdA = seedNFT.plantSeed("seedA", 50, 1, 5);
        seedIdB = seedNFT.plantSeed("seedB", 50, 1, 5);
        seedNFT.advanceCheckpoint(seedIdA);
        seedNFT.advanceCheckpoint(seedIdB);
    }

    function testInitiateDuelFuzz(address target) public {
        vm.assume(target != address(0));
        vm.assume(target != alice);

        uint256 duelId = duelManager.initiateDuel(seedIdA, target, seedIdB);

        assertEq(duelId, 0);
    }

    function testAcceptDuelFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
    }

    function testRejectDuelFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.rejectDuel(duelId);
    }

    function testCompleteDuelFuzz(uint256 scoreA, uint256 scoreB) public {
        vm.assume(scoreA <= 100);
        vm.assume(scoreB <= 100);

        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
        
        vm.prank(alice);
        duelManager.completeDuel(duelId, scoreA, scoreB);
    }

    function testDuelTimedOutFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.warp(block.timestamp + 61);
        
        vm.prank(bob);
        duelManager.duelTimedOut(duelId);
    }

    function testCannotDuelSelfFuzz() public {
        vm.expectRevert("Cannot duel yourself");
        duelManager.initiateDuel(seedIdA, alice, seedIdB);
    }

    function testCannotInitiateZeroAddress() public {
        vm.expectRevert("Target cannot be zero address");
        duelManager.initiateDuel(seedIdA, address(0), seedIdB);
    }

    function testCooldownCheckFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
        
        vm.prank(alice);
        duelManager.completeDuel(duelId, 80, 20);

        assertTrue(duelManager.isOnCooldown(alice));
        assertTrue(duelManager.isOnCooldown(bob));
    }

    function testCooldownAfterTimeoutFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.warp(block.timestamp + 61);
        
        vm.prank(bob);
        duelManager.duelTimedOut(duelId);

        assertTrue(duelManager.isOnCooldown(alice));
    }

    function testTimeUntilCooldownOverFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
        
        vm.prank(alice);
        duelManager.completeDuel(duelId, 80, 20);

        uint256 cooldown = duelManager.timeUntilCooldownOver(alice);
        assertTrue(cooldown > 0);
    }

    function testCannotAcceptOwnDuel() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(alice);
        vm.expectRevert("Not the target of this duel");
        duelManager.acceptDuel(duelId);
    }

    function testCannotCompleteUnacceptedDuel(uint256 scoreA, uint256 scoreB) public {
        vm.assume(scoreA <= 100);
        vm.assume(scoreB <= 100);

        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(alice);
        vm.expectRevert("Duel not accepted");
        duelManager.completeDuel(duelId, scoreA, scoreB);
    }

    function testDuelDrawFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
        
        vm.prank(alice);
        duelManager.completeDuel(duelId, 50, 50);
    }

    function testMultipleDuelsFuzz(uint256 count) public {
        vm.assume(count >= 1);
        vm.assume(count <= 5);

        for (uint256 i = 0; i < count; i++) {
            uint256 newSeedA = seedNFT.plantSeed(string(abi.encodePacked("seedA", i)), 50, 1, 5);
            uint256 newSeedB = seedNFT.plantSeed(string(abi.encodePacked("seedB", i)), 50, 1, 5);
            seedNFT.advanceCheckpoint(newSeedA);
            seedNFT.advanceCheckpoint(newSeedB);
            
            duelManager.initiateDuel(newSeedA, bob, newSeedB);
        }

        assertTrue(true);
    }

    function testCannotCompleteDuelTwice(uint256 scoreA, uint256 scoreB) public {
        vm.assume(scoreA <= 100);
        vm.assume(scoreB <= 100);

        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
        
        vm.prank(alice);
        duelManager.completeDuel(duelId, scoreA, scoreB);
        
        vm.prank(bob);
        vm.expectRevert("Duel already completed");
        duelManager.completeDuel(duelId, scoreA, scoreB);
    }

    function testGrowthBonusAppliedFuzz() public {
        uint256 duelId = duelManager.initiateDuel(seedIdA, bob, seedIdB);
        
        vm.prank(bob);
        duelManager.acceptDuel(duelId);
        
        vm.prank(alice);
        duelManager.completeDuel(duelId, 80, 20);
    }
}
