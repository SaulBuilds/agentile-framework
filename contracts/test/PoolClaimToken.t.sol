// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/PoolClaimToken.sol";
import "../src/GradientRoles.sol";

contract PoolClaimTokenTest is Test {
    PoolClaimToken public token;
    address public admin = address(1);
    address public minter = address(2);
    address public escrow = address(3);
    address public contributor = address(4);
    address public recipient = address(5);
    address public unauthorized = address(6);
    address public emergencyAdmin = address(7);

    uint256 constant POOL_1 = 1;
    uint256 constant POOL_2 = 2;

    function setUp() public {
        vm.startPrank(admin);
        token = new PoolClaimToken(admin);
        token.grantRole(GradientRoles.CLAIM_MINTER, minter);
        token.grantRole(GradientRoles.ESCROW_OPERATOR, escrow);
        token.grantRole(GradientRoles.EMERGENCY_ADMIN, emergencyAdmin);
        vm.stopPrank();
    }

    // --- Minting ---

    function test_mintClaim_success() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        assertEq(token.balanceOf(contributor, POOL_1), 1);
        assertEq(token.totalSupplyForPool(POOL_1), 1);
    }

    function test_mintClaim_emitsEvent() public {
        vm.expectEmit(true, true, false, true);
        emit PoolClaimToken.ClaimMinted(contributor, POOL_1, 1);

        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);
    }

    function test_mintClaim_multipleForSamePool() public {
        vm.startPrank(minter);
        token.mintClaim(contributor, POOL_1, 1);
        token.mintClaim(contributor, POOL_1, 2);
        vm.stopPrank();

        assertEq(token.balanceOf(contributor, POOL_1), 3);
        assertEq(token.totalSupplyForPool(POOL_1), 3);
    }

    function test_mintClaim_differentPools() public {
        vm.startPrank(minter);
        token.mintClaim(contributor, POOL_1, 1);
        token.mintClaim(contributor, POOL_2, 3);
        vm.stopPrank();

        assertEq(token.balanceOf(contributor, POOL_1), 1);
        assertEq(token.balanceOf(contributor, POOL_2), 3);
    }

    function test_mintClaim_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        token.mintClaim(contributor, POOL_1, 1);
    }

    function test_mintClaim_revertsForZeroAddress() public {
        vm.prank(minter);
        vm.expectRevert("Invalid recipient");
        token.mintClaim(address(0), POOL_1, 1);
    }

    function test_mintClaim_revertsForZeroPoolId() public {
        vm.prank(minter);
        vm.expectRevert("Invalid pool ID");
        token.mintClaim(contributor, 0, 1);
    }

    function test_mintClaim_revertsForZeroQuantity() public {
        vm.prank(minter);
        vm.expectRevert("Quantity must be > 0");
        token.mintClaim(contributor, POOL_1, 0);
    }

    // --- Burning ---

    function test_burnClaim_success() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 3);

        vm.prank(escrow);
        token.burnClaim(contributor, POOL_1, 1);

        assertEq(token.balanceOf(contributor, POOL_1), 2);
        assertEq(token.totalSupplyForPool(POOL_1), 2);
    }

    function test_burnClaim_emitsEvent() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.expectEmit(true, true, false, true);
        emit PoolClaimToken.ClaimBurned(contributor, POOL_1, 1);

        vm.prank(escrow);
        token.burnClaim(contributor, POOL_1, 1);
    }

    function test_burnClaim_revertsForInsufficientBalance() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.prank(escrow);
        vm.expectRevert("Insufficient claim balance");
        token.burnClaim(contributor, POOL_1, 2);
    }

    function test_burnClaim_revertsForUnauthorized() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.prank(unauthorized);
        vm.expectRevert();
        token.burnClaim(contributor, POOL_1, 1);
    }

    // --- Transfer Restrictions (Default: Blocked) ---

    function test_transfer_blockedByDefault() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.prank(contributor);
        vm.expectRevert("PoolClaimToken: transfers blocked for this pool");
        token.safeTransferFrom(contributor, recipient, POOL_1, 1, "");
    }

    function test_batchTransfer_blockedByDefault() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        uint256[] memory ids = new uint256[](1);
        ids[0] = POOL_1;
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1;

        vm.prank(contributor);
        vm.expectRevert("PoolClaimToken: transfers blocked for this pool");
        token.safeBatchTransferFrom(contributor, recipient, ids, amounts, "");
    }

    // --- Transfer with Allowlist Policy ---

    function test_transfer_allowedWithAllowlist() public {
        vm.startPrank(admin);
        token.setTransferPolicy(POOL_1, 1); // allowlist mode
        token.setAllowlistEntry(POOL_1, contributor, true);
        vm.stopPrank();

        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.prank(contributor);
        token.safeTransferFrom(contributor, recipient, POOL_1, 1, "");

        assertEq(token.balanceOf(recipient, POOL_1), 1);
        assertEq(token.balanceOf(contributor, POOL_1), 0);
    }

    function test_transfer_revertsIfNotOnAllowlist() public {
        vm.prank(admin);
        token.setTransferPolicy(POOL_1, 1); // allowlist mode, but nobody added

        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.prank(contributor);
        vm.expectRevert("PoolClaimToken: not on allowlist");
        token.safeTransferFrom(contributor, recipient, POOL_1, 1, "");
    }

    // --- Transfer Policy ---

    function test_setTransferPolicy_emitsEvent() public {
        vm.expectEmit(true, false, false, true);
        emit PoolClaimToken.TransferPolicyUpdated(POOL_1, 1);

        vm.prank(admin);
        token.setTransferPolicy(POOL_1, 1);
    }

    function test_setTransferPolicy_revertsForInvalidPolicy() public {
        vm.prank(admin);
        vm.expectRevert("Invalid policy");
        token.setTransferPolicy(POOL_1, 2);
    }

    function test_setTransferPolicy_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        token.setTransferPolicy(POOL_1, 1);
    }

    // --- Emergency Pause ---

    function test_emergencyPause_blocksMinting() public {
        vm.prank(emergencyAdmin);
        token.emergencyPause();

        vm.prank(minter);
        vm.expectRevert();
        token.mintClaim(contributor, POOL_1, 1);
    }

    function test_emergencyPause_blocksBurning() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);

        vm.prank(emergencyAdmin);
        token.emergencyPause();

        vm.prank(escrow);
        vm.expectRevert();
        token.burnClaim(contributor, POOL_1, 1);
    }

    function test_emergencyUnpause_resumesOperations() public {
        vm.prank(emergencyAdmin);
        token.emergencyPause();

        vm.prank(emergencyAdmin);
        token.emergencyUnpause();

        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 1);
        assertEq(token.balanceOf(contributor, POOL_1), 1);
    }

    // --- View Functions ---

    function test_claimBalance() public {
        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, 5);

        assertEq(token.claimBalance(contributor, POOL_1), 5);
    }

    function test_totalSupplyForPool() public {
        vm.startPrank(minter);
        token.mintClaim(contributor, POOL_1, 3);
        token.mintClaim(recipient, POOL_1, 2);
        vm.stopPrank();

        assertEq(token.totalSupplyForPool(POOL_1), 5);
    }

    // --- Fuzz Tests ---

    function testFuzz_mintAndBurn_balanceConsistent(uint256 mintQty, uint256 burnQty) public {
        mintQty = bound(mintQty, 1, 1000);
        burnQty = bound(burnQty, 0, mintQty);

        vm.prank(minter);
        token.mintClaim(contributor, POOL_1, mintQty);

        if (burnQty > 0) {
            vm.prank(escrow);
            token.burnClaim(contributor, POOL_1, burnQty);
        }

        assertEq(token.balanceOf(contributor, POOL_1), mintQty - burnQty);
        assertEq(token.totalSupplyForPool(POOL_1), mintQty - burnQty);
    }
}
