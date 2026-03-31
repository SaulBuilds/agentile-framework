// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../contracts/Federation.sol";

contract FederationFuzzTest is Test {
    Federation public federation;
    address public alice = address(0x1);
    address public bob = address(0x2);
    address public charlie = address(0x3);

    function setUp() public {
        federation = new Federation();
    }

    function testCreateFederationFuzz(uint256 minStake) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);

        uint256 fedId = federation.createFederation(minStake);

        assertEq(fedId, 0);
        assertEq(federation.federationCreator(fedId), address(this));
        assertEq(federation.federationMinStake(fedId), minStake);
    }

    function testJoinFederationFuzz(uint256 minStake) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        federation.joinFederation(fedId);

        assertTrue(federation.isMember(fedId, alice));
    }

    function testLeaveFederationFuzz(uint256 minStake) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        federation.joinFederation(fedId);
        
        vm.prank(alice);
        federation.leaveFederation(fedId);

        assertTrue(!federation.isMember(fedId, alice));
    }

    function testStakeSeedFuzz(uint256 minStake, uint256 stakeAmount) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(stakeAmount >= minStake);
        vm.assume(stakeAmount <= 100000);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        federation.joinFederation(fedId);
        
        vm.prank(alice);
        federation.stakeSeed(fedId, 1, stakeAmount);

        assertEq(federation.memberStake(fedId, alice), stakeAmount);
    }

    function testUnstakeSeedFuzz(uint256 minStake, uint256 stakeAmount) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(stakeAmount >= minStake);
        vm.assume(stakeAmount <= 100000);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        federation.joinFederation(fedId);
        
        vm.prank(alice);
        federation.stakeSeed(fedId, 1, stakeAmount);
        
        vm.prank(alice);
        federation.unstakeSeed(fedId, 1, stakeAmount);

        assertEq(federation.memberStake(fedId, alice), 0);
    }

    function testAddRewardFuzz(uint256 minStake, uint256 rewardAmount) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(rewardAmount > 0);
        vm.assume(rewardAmount <= 1000000);

        uint256 fedId = federation.createFederation(minStake);
        federation.addReward(fedId, rewardAmount);

        assertEq(federation.federationRewardPool(fedId), rewardAmount);
    }

    function testUpdateTotalScoreFuzz(uint256 minStake, uint256 score) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(score <= 1000000);

        uint256 fedId = federation.createFederation(minStake);
        federation.updateTotalScore(fedId, score);

        assertEq(federation.getTotalScore(fedId), score);
    }

    function testCannotJoinAlreadyMemberFuzz(uint256 minStake) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        federation.joinFederation(fedId);
        
        vm.prank(alice);
        vm.expectRevert("Already a member");
        federation.joinFederation(fedId);
    }

    function testCannotLeaveWithoutBeingMemberFuzz(uint256 minStake) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        vm.expectRevert("Not a member");
        federation.leaveFederation(fedId);
    }

    function testMultipleMembersFuzz(uint256 minStake, uint256 memberCount) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(memberCount >= 2);
        vm.assume(memberCount <= 10);

        uint256 fedId = federation.createFederation(minStake);

        for (uint256 i = 0; i < memberCount; i++) {
            address member = address(uint160(0x100 + i));
            vm.prank(member);
            federation.joinFederation(fedId);
            assertTrue(federation.isMember(fedId, member));
        }
    }

    function testDistributeRewardsFuzz(uint256 minStake, uint256 rewardAmount) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(rewardAmount > 0);
        vm.assume(rewardAmount <= 1000000);

        uint256 fedId = federation.createFederation(minStake);
        federation.addReward(fedId, rewardAmount);
        federation.distributeRewards(fedId);

        assertEq(federation.federationRewardPool(fedId), 0);
    }

    function testCannotCreateFederationWithLowStake(uint256 minStake) public {
        vm.assume(minStake < 100);
        vm.assume(minStake > 0);

        vm.expectRevert("Minimum stake too low");
        federation.createFederation(minStake);
    }

    function testFederationCreatorViewFuzz(uint256 minStake) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);

        uint256 fedId = federation.createFederation(minStake);
        
        assertEq(federation.federationCreator(fedId), address(this));
    }

    function testCannotStakeBelowMinimumFuzz(uint256 minStake, uint256 stakeAmount) public {
        vm.assume(minStake >= 100);
        vm.assume(minStake <= 10000);
        vm.assume(stakeAmount > 0);
        vm.assume(stakeAmount < minStake);

        uint256 fedId = federation.createFederation(minStake);
        
        vm.prank(alice);
        federation.joinFederation(fedId);
        
        vm.prank(alice);
        vm.expectRevert("Stake below minimum");
        federation.stakeSeed(fedId, 1, stakeAmount);
    }
}
