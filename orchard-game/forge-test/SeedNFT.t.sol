// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../contracts/SeedNFT.sol";

contract SeedNFTfuzzTest is Test {
    SeedNFT public seedNFT;
    address public alice = address(0x1);
    address public bob = address(0x2);

    function setUp() public {
        seedNFT = new SeedNFT();
    }

    function testPlantSeedFuzz(string memory payload, uint256 stake, uint256 federation, uint256 maxCheckpoints) public {
        vm.assume(stake >= 10);
        vm.assume(stake <= 10000);
        vm.assume(federation >= 1);
        vm.assume(federation <= 1000);
        vm.assume(maxCheckpoints >= 1);
        vm.assume(maxCheckpoints <= 1000);
        vm.assume(bytes(payload).length > 0);
        vm.assume(bytes(payload).length <= 1000);

        uint256 tokenId = seedNFT.plantSeed(payload, stake, federation, maxCheckpoints);

        assertEq(seedNFT.planterOf(tokenId), address(this));
        assertEq(seedNFT.stakeOf(tokenId), stake);
        assertEq(seedNFT.federationOf(tokenId), federation);
        assertEq(seedNFT.checkpointOf(tokenId), 0);
        assertEq(seedNFT.maxCheckpointOf(tokenId), maxCheckpoints);
    }

    function testAdvanceCheckpointFuzz(uint256 numCheckpoints) public {
        vm.assume(numCheckpoints >= 1);
        vm.assume(numCheckpoints <= 100);
        
        uint256 tokenId = seedNFT.plantSeed("test", 50, 1, numCheckpoints);

        for (uint256 i = 0; i < numCheckpoints; i++) {
            seedNFT.advanceCheckpoint(tokenId);
        }

        assertEq(seedNFT.checkpointOf(tokenId), numCheckpoints);
    }

    function testHarvestFuzz(uint256 growthScore) public {
        vm.assume(growthScore <= 100);
        
        uint256 tokenId = seedNFT.plantSeed("test", 50, 1, 1);
        seedNFT.advanceCheckpoint(tokenId);
        seedNFT.harvestSeed(tokenId, growthScore);

        assertTrue(seedNFT.isHarvested(tokenId));
        assertEq(seedNFT.growthScoreOf(tokenId), growthScore);
    }

    function testFailSeedFuzz(string memory reason) public {
        vm.assume(bytes(reason).length > 0);
        vm.assume(bytes(reason).length <= 200);
        
        uint256 tokenId = seedNFT.plantSeed("test", 50, 1, 3);
        vm.expectRevert();
        seedNFT.failSeed(tokenId, reason);
    }

    function testFailSeedReverts(string memory reason) public {
        vm.assume(bytes(reason).length > 0);
        vm.assume(bytes(reason).length <= 200);
        
        uint256 tokenId = seedNFT.plantSeed("test", 50, 1, 3);
        seedNFT.failSeed(tokenId, reason);
        assertTrue(seedNFT.isFailed(tokenId));
    }

    function testMultipleSeedsFuzz(uint256 count) public {
        vm.assume(count >= 1);
        vm.assume(count <= 50);

        for (uint256 i = 0; i < count; i++) {
            seedNFT.plantSeed(string(abi.encodePacked("test", i)), 50, 1, 5);
        }

        for (uint256 i = 0; i < count; i++) {
            assertEq(seedNFT.planterOf(i), address(this));
        }
    }

    function testCheckpointBoundariesFuzz(uint256 maxCp) public {
        vm.assume(maxCp >= 1);
        vm.assume(maxCp <= 1000);
        
        uint256 tokenId = seedNFT.plantSeed("test", 50, 1, maxCp);
        
        for (uint256 i = 0; i < maxCp; i++) {
            seedNFT.advanceCheckpoint(tokenId);
        }
        
        vm.expectRevert("Seed already at max checkpoint");
        seedNFT.advanceCheckpoint(tokenId);
    }

    function testGrowthScoreBoundariesFuzz(uint256 score) public {
        vm.assume(score <= 100);
        
        uint256 tokenId = seedNFT.plantSeed("test", 50, 1, 1);
        seedNFT.advanceCheckpoint(tokenId);
        
        if (score <= 100) {
            seedNFT.harvestSeed(tokenId, score);
            assertEq(seedNFT.growthScoreOf(tokenId), score);
        }
    }

    function testOwnershipFuzz(address user, string memory payload) public {
        vm.assume(user != address(0));
        vm.assume(bytes(payload).length > 0);
        
        vm.prank(user);
        uint256 tokenId = seedNFT.plantSeed(payload, 50, 1, 5);
        
        assertEq(seedNFT.planterOf(tokenId), user);
    }
}
