// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../contracts/ORTToken.sol";

contract ORTTokenFuzzTest is Test {
    ORTToken public ort;
    address public alice = address(0x1);
    address public bob = address(0x2);

    function setUp() public {
        ort = new ORTToken();
    }

    function testStakeFuzz(uint256 amount) public {
        vm.assume(amount > 0);
        vm.assume(amount <= ort.balanceOf(address(this)));

        uint256 balanceBefore = ort.balanceOf(address(this));
        ort.stake(amount);
        
        assertEq(ort.balanceOf(address(this)), balanceBefore - amount);
    }

    function testUnstakeFuzz(uint256 amount) public {
        vm.assume(amount > 0);
        vm.assume(amount <= ort.balanceOf(address(this)));

        ort.stake(amount);
        uint256 balanceBefore = ort.balanceOf(address(this));
        ort.unstake(amount);
        
        assertEq(ort.balanceOf(address(this)), balanceBefore + amount);
    }

    function testRewardFuzz(uint256 amount) public {
        vm.assume(amount > 0);
        vm.assume(amount <= ort.stakedBalance());

        uint256 balanceBefore = ort.balanceOf(alice);
        ort.reward(alice, amount);
        
        assertEq(ort.balanceOf(alice), balanceBefore + amount);
    }

    function testStakeSeedInFederationFuzz(uint256 federationId, uint256 tokenId, uint256 amount) public {
        vm.assume(amount > 0);
        vm.assume(amount <= ort.balanceOf(address(this)));

        uint256 balanceBefore = ort.balanceOf(address(this));
        ort.stakeSeedInFederation(federationId, tokenId, amount);
        
        assertEq(ort.balanceOf(address(this)), balanceBefore - amount);
    }

    function testUnstakeSeedFromFederationFuzz(uint256 federationId, uint256 tokenId, uint256 amount) public {
        vm.assume(amount > 0);
        
        ort.stakeSeedInFederation(federationId, tokenId, amount);
        uint256 balanceBefore = ort.balanceOf(address(this));
        ort.unstakeSeedFromFederation(federationId, tokenId, amount);
        
        assertEq(ort.balanceOf(address(this)), balanceBefore + amount);
    }

    function testCannotStakeZero() public {
        vm.expectRevert("Amount must be greater than zero");
        ort.stake(0);
    }

    function testCannotUnstakeZero() public {
        vm.expectRevert("Amount must be greater than zero");
        ort.unstake(0);
    }

    function testCannotRewardZero() public {
        vm.expectRevert("Amount must be greater than zero");
        ort.reward(alice, 0);
    }

    function testCannotStakeMoreThanBalance(uint256 amount) public {
        vm.assume(amount > ort.balanceOf(address(this)));

        vm.expectRevert("Insufficient balance");
        ort.stake(amount);
    }

    function testMultipleStakesFuzz(uint256 count) public {
        vm.assume(count >= 1);
        vm.assume(count <= 10);
        
        uint256 totalAmount = count * 100;
        vm.assume(totalAmount <= ort.balanceOf(address(this)));

        for (uint256 i = 0; i < count; i++) {
            ort.stake(100);
        }
        
        assertEq(ort.stakedBalance(), count * 100);
    }

    function testStakeAndUnstakeFuzz(uint256 amount) public {
        vm.assume(amount > 0);
        vm.assume(amount <= ort.balanceOf(address(this)) / 2);

        ort.stake(amount);
        ort.unstake(amount);
        
        assertEq(ort.balanceOf(address(this)), ort.INITIAL_SUPPLY());
    }

    function testInitialSupplyFuzz() public {
        assertEq(ort.totalSupply(), ort.INITIAL_SUPPLY());
    }

    function testTokenDecimals() public {
        assertEq(ort.decimals(), 18);
    }

    function testTokenName() public {
        assertEq(ort.name(), "Orchard Token");
    }

    function testTokenSymbol() public {
        assertEq(ort.symbol(), "ORT");
    }
}
