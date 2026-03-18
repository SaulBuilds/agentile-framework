// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/EscrowSettlement.sol";
import "../src/PoolClaimToken.sol";
import "../src/InventoryCommitment.sol";
import "../src/GradientRoles.sol";

contract EscrowSettlementTest is Test {
    EscrowSettlement public escrow;
    PoolClaimToken public claimToken;
    InventoryCommitment public inventory;

    address public admin = address(1);
    address public escrowOp = address(2);
    address public emergencyAdmin = address(3);
    address public unauthorized = address(4);
    address public contributor = address(5);
    address public user = address(6);

    uint256 constant RECEIPT_1 = 1;
    uint256 constant RECEIPT_2 = 2;
    uint256 constant POOL_1 = 1;

    function setUp() public {
        vm.startPrank(admin);

        // Deploy dependencies
        claimToken = new PoolClaimToken(admin);
        inventory = new InventoryCommitment(admin);

        // Deploy EscrowSettlement
        escrow = new EscrowSettlement(admin, address(claimToken), address(inventory));

        // Grant roles on EscrowSettlement
        escrow.grantRole(GradientRoles.ESCROW_OPERATOR, escrowOp);
        escrow.grantRole(GradientRoles.EMERGENCY_ADMIN, emergencyAdmin);

        // Grant EscrowSettlement the roles it needs on dependent contracts
        claimToken.grantRole(GradientRoles.CLAIM_MINTER, address(escrow));
        claimToken.grantRole(GradientRoles.ESCROW_OPERATOR, address(escrow));
        inventory.grantRole(GradientRoles.ESCROW_OPERATOR, address(escrow));

        // Grant inventory operator for test setup
        inventory.grantRole(GradientRoles.INVENTORY_OPERATOR, admin);

        vm.stopPrank();
    }

    // Helper: register available inventory
    function _registerInventory(uint256 receiptId, uint256 poolId) internal returns (uint256) {
        vm.prank(admin);
        return inventory.registerInventory(receiptId, poolId);
    }

    // Helper: full setup for reservation tests (lock, unlock, register inventory)
    function _setupForReservation() internal returns (uint256 inventoryId) {
        // Lock submission
        vm.prank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);

        // Unlock claim (mints claim to contributor)
        vm.prank(escrowOp);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);

        // Register inventory
        inventoryId = _registerInventory(RECEIPT_1, POOL_1);
    }

    // =============================================================
    //                     CONSTRUCTOR
    // =============================================================

    function test_constructor_revertsForZeroClaimToken() public {
        vm.prank(admin);
        vm.expectRevert("Invalid claim token");
        new EscrowSettlement(admin, address(0), address(inventory));
    }

    function test_constructor_revertsForZeroInventory() public {
        vm.prank(admin);
        vm.expectRevert("Invalid inventory");
        new EscrowSettlement(admin, address(claimToken), address(0));
    }

    // =============================================================
    //                     LOCK ON SUBMISSION
    // =============================================================

    function test_lockOnSubmission_success() public {
        vm.prank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);

        assertTrue(escrow.isReceiptLocked(RECEIPT_1));
    }

    function test_lockOnSubmission_emitsEvent() public {
        vm.expectEmit(true, true, false, true);
        emit EscrowSettlement.SubmissionLocked(RECEIPT_1, contributor);

        vm.prank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
    }

    function test_lockOnSubmission_revertsForDuplicate() public {
        vm.startPrank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);

        vm.expectRevert("Already locked");
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        vm.stopPrank();
    }

    function test_lockOnSubmission_revertsForZeroReceiptId() public {
        vm.prank(escrowOp);
        vm.expectRevert("Invalid receipt ID");
        escrow.lockOnSubmission(0, contributor);
    }

    function test_lockOnSubmission_revertsForZeroContributor() public {
        vm.prank(escrowOp);
        vm.expectRevert("Invalid contributor");
        escrow.lockOnSubmission(RECEIPT_1, address(0));
    }

    function test_lockOnSubmission_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        escrow.lockOnSubmission(RECEIPT_1, contributor);
    }

    // =============================================================
    //                     UNLOCK CLAIM
    // =============================================================

    function test_unlockClaimOnVerifiedIntake_success() public {
        vm.startPrank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
        vm.stopPrank();

        assertTrue(escrow.isReceiptUnlocked(RECEIPT_1));
        assertEq(claimToken.balanceOf(contributor, POOL_1), 1);
    }

    function test_unlockClaimOnVerifiedIntake_emitsEvent() public {
        vm.prank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);

        vm.expectEmit(true, true, false, true);
        emit EscrowSettlement.ClaimUnlocked(RECEIPT_1, contributor, POOL_1);

        vm.prank(escrowOp);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
    }

    function test_unlockClaim_revertsIfNotLocked() public {
        vm.prank(escrowOp);
        vm.expectRevert("Not locked");
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
    }

    function test_unlockClaim_revertsIfAlreadyUnlocked() public {
        vm.startPrank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);

        vm.expectRevert("Already unlocked");
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
        vm.stopPrank();
    }

    function test_unlockClaim_revertsForZeroPoolId() public {
        vm.startPrank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);

        vm.expectRevert("Invalid pool ID");
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, 0);
        vm.stopPrank();
    }

    // =============================================================
    //                     CONSUME CLAIM FOR RESERVATION
    // =============================================================

    function test_consumeClaimForReservation_success() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        assertEq(resId, 1);
        assertEq(claimToken.balanceOf(contributor, POOL_1), 0); // claim burned

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        assertEq(res.user, contributor);
        assertEq(res.poolId, POOL_1);
        assertEq(res.inventoryId, inventoryId);
        assertEq(res.claimAmount, 1);
        assertEq(uint256(res.status), uint256(EscrowSettlement.ReservationStatus.Active));
        assertGt(res.expiresAt, res.createdAt);
    }

    function test_consumeClaimForReservation_emitsEvent() public {
        uint256 inventoryId = _setupForReservation();

        vm.expectEmit(true, true, false, true);
        emit EscrowSettlement.ReservationCreated(1, contributor, POOL_1, inventoryId);

        vm.prank(escrowOp);
        escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);
    }

    function test_consumeClaimForReservation_revertsForNoClaim() public {
        uint256 inventoryId = _registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        vm.expectRevert("No claim balance");
        escrow.consumeClaimForReservation(user, POOL_1, inventoryId);
    }

    function test_consumeClaimForReservation_revertsForZeroUser() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        vm.expectRevert("Invalid user");
        escrow.consumeClaimForReservation(address(0), POOL_1, inventoryId);
    }

    function test_consumeClaimForReservation_revertsForUnauthorized() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(unauthorized);
        vm.expectRevert();
        escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);
    }

    // =============================================================
    //                     CANCEL RESERVATION
    // =============================================================

    function test_cancelReservation_success() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        // Claim was burned
        assertEq(claimToken.balanceOf(contributor, POOL_1), 0);

        vm.prank(escrowOp);
        escrow.cancelReservation(resId);

        // Claim re-minted
        assertEq(claimToken.balanceOf(contributor, POOL_1), 1);

        // Reservation cancelled
        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        assertEq(uint256(res.status), uint256(EscrowSettlement.ReservationStatus.Cancelled));

        // Inventory released back to available
        assertEq(uint256(inventory.getInventoryState(inventoryId)), uint256(InventoryCommitment.InventoryState.Available));
    }

    function test_cancelReservation_emitsEvent() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.expectEmit(true, false, false, true);
        emit EscrowSettlement.ReservationCancelled(resId);

        vm.prank(escrowOp);
        escrow.cancelReservation(resId);
    }

    function test_cancelReservation_revertsIfNotActive() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.prank(escrowOp);
        escrow.cancelReservation(resId);

        vm.prank(escrowOp);
        vm.expectRevert("Not active");
        escrow.cancelReservation(resId);
    }

    // =============================================================
    //                     FINALIZE WITHDRAWAL
    // =============================================================

    function test_finalizeWithdrawal_success() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        // Move inventory through outbound → delivered
        vm.startPrank(admin);
        inventory.grantRole(GradientRoles.INVENTORY_OPERATOR, admin);
        inventory.markOutbound(inventoryId, keccak256("SHIP-001"));
        inventory.confirmDelivered(inventoryId, keccak256("PROOF-001"));
        vm.stopPrank();

        vm.prank(escrowOp);
        escrow.finalizeWithdrawal(resId);

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        assertEq(uint256(res.status), uint256(EscrowSettlement.ReservationStatus.Finalized));
    }

    function test_finalizeWithdrawal_emitsEvent() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.startPrank(admin);
        inventory.markOutbound(inventoryId, keccak256("SHIP"));
        inventory.confirmDelivered(inventoryId, keccak256("PROOF"));
        vm.stopPrank();

        vm.expectEmit(true, false, false, true);
        emit EscrowSettlement.ReservationFinalized(resId);

        vm.prank(escrowOp);
        escrow.finalizeWithdrawal(resId);
    }

    function test_finalizeWithdrawal_revertsIfNotDelivered() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.prank(escrowOp);
        vm.expectRevert("Item not delivered");
        escrow.finalizeWithdrawal(resId);
    }

    function test_finalizeWithdrawal_revertsIfNotActive() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.prank(escrowOp);
        escrow.cancelReservation(resId);

        vm.prank(escrowOp);
        vm.expectRevert("Not active");
        escrow.finalizeWithdrawal(resId);
    }

    // =============================================================
    //                     EXPIRE RESERVATION
    // =============================================================

    function test_expireReservation_success() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        // Fast forward past expiry
        vm.warp(block.timestamp + 86401);

        // Permissionless — anyone can call
        escrow.expireReservation(resId);

        // Claim re-minted
        assertEq(claimToken.balanceOf(contributor, POOL_1), 1);

        // Reservation expired
        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        assertEq(uint256(res.status), uint256(EscrowSettlement.ReservationStatus.Expired));

        // Inventory released
        assertEq(uint256(inventory.getInventoryState(inventoryId)), uint256(InventoryCommitment.InventoryState.Available));
    }

    function test_expireReservation_emitsEvent() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.warp(block.timestamp + 86401);

        vm.expectEmit(true, false, false, true);
        emit EscrowSettlement.ReservationExpired(resId);

        escrow.expireReservation(resId);
    }

    function test_expireReservation_revertsIfNotExpiredYet() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.expectRevert("Not expired yet");
        escrow.expireReservation(resId);
    }

    function test_expireReservation_revertsIfNotActive() public {
        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.prank(escrowOp);
        escrow.cancelReservation(resId);

        vm.warp(block.timestamp + 86401);

        vm.expectRevert("Not active");
        escrow.expireReservation(resId);
    }

    // =============================================================
    //                     RESERVATION TIMEOUT CONFIG
    // =============================================================

    function test_setReservationTimeout_success() public {
        vm.prank(admin);
        escrow.setReservationTimeout(7200); // 2 hours

        assertEq(escrow.reservationTimeout(), 7200);
    }

    function test_setReservationTimeout_revertsIfTooShort() public {
        vm.prank(admin);
        vm.expectRevert("Timeout too short");
        escrow.setReservationTimeout(3599);
    }

    function test_setReservationTimeout_revertsIfTooLong() public {
        vm.prank(admin);
        vm.expectRevert("Timeout too long");
        escrow.setReservationTimeout(604801);
    }

    function test_setReservationTimeout_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        escrow.setReservationTimeout(7200);
    }

    // =============================================================
    //                     EMERGENCY PAUSE
    // =============================================================

    function test_emergencyPause_blocksLockOnSubmission() public {
        vm.prank(emergencyAdmin);
        escrow.emergencyPause();

        vm.prank(escrowOp);
        vm.expectRevert();
        escrow.lockOnSubmission(RECEIPT_1, contributor);
    }

    function test_emergencyUnpause_resumesOperations() public {
        vm.prank(emergencyAdmin);
        escrow.emergencyPause();

        vm.prank(emergencyAdmin);
        escrow.emergencyUnpause();

        vm.prank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        assertTrue(escrow.isReceiptLocked(RECEIPT_1));
    }

    // =============================================================
    //                     VIEW FUNCTIONS
    // =============================================================

    function test_nextReservationId() public {
        assertEq(escrow.nextReservationId(), 1);

        uint256 inventoryId = _setupForReservation();
        vm.prank(escrowOp);
        escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        assertEq(escrow.nextReservationId(), 2);
    }

    // =============================================================
    //                     FULL LIFECYCLE
    // =============================================================

    function test_fullLifecycle_lockToFinalize() public {
        // 1. Lock submission
        vm.prank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        assertTrue(escrow.isReceiptLocked(RECEIPT_1));

        // 2. Unlock claim (mints claim)
        vm.prank(escrowOp);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
        assertEq(claimToken.balanceOf(contributor, POOL_1), 1);

        // 3. Register inventory
        uint256 inventoryId = _registerInventory(RECEIPT_1, POOL_1);
        assertEq(inventory.getAvailableCount(POOL_1), 1);

        // 4. Consume claim for reservation (burns claim, reserves inventory)
        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);
        assertEq(claimToken.balanceOf(contributor, POOL_1), 0);
        assertEq(uint256(inventory.getInventoryState(inventoryId)), uint256(InventoryCommitment.InventoryState.Reserved));

        // 5. Ship (outbound) and deliver
        vm.startPrank(admin);
        inventory.markOutbound(inventoryId, keccak256("SHIP"));
        inventory.confirmDelivered(inventoryId, keccak256("PROOF"));
        vm.stopPrank();

        // 6. Finalize
        vm.prank(escrowOp);
        escrow.finalizeWithdrawal(resId);

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        assertEq(uint256(res.status), uint256(EscrowSettlement.ReservationStatus.Finalized));
    }

    function test_fullLifecycle_lockToCancel() public {
        // Lock → unlock → reserve → cancel
        vm.startPrank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
        vm.stopPrank();

        uint256 inventoryId = _registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        vm.prank(escrowOp);
        escrow.cancelReservation(resId);

        // Claim restored, inventory available
        assertEq(claimToken.balanceOf(contributor, POOL_1), 1);
        assertEq(uint256(inventory.getInventoryState(inventoryId)), uint256(InventoryCommitment.InventoryState.Available));
    }

    function test_fullLifecycle_lockToExpire() public {
        // Lock → unlock → reserve → expire
        vm.startPrank(escrowOp);
        escrow.lockOnSubmission(RECEIPT_1, contributor);
        escrow.unlockClaimOnVerifiedIntake(RECEIPT_1, contributor, POOL_1);
        vm.stopPrank();

        uint256 inventoryId = _registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        // Fast forward past expiry
        vm.warp(block.timestamp + 86401);
        escrow.expireReservation(resId);

        // Claim restored, inventory available
        assertEq(claimToken.balanceOf(contributor, POOL_1), 1);
        assertEq(uint256(inventory.getInventoryState(inventoryId)), uint256(InventoryCommitment.InventoryState.Available));
    }

    // =============================================================
    //                     FUZZ TESTS
    // =============================================================

    function testFuzz_reservationTimeout_appliedCorrectly(uint256 timeout) public {
        timeout = bound(timeout, 3600, 604800);

        vm.prank(admin);
        escrow.setReservationTimeout(timeout);

        uint256 inventoryId = _setupForReservation();

        vm.prank(escrowOp);
        uint256 resId = escrow.consumeClaimForReservation(contributor, POOL_1, inventoryId);

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        assertEq(res.expiresAt, res.createdAt + timeout);
    }

    function testFuzz_lockOnSubmission_multipleReceipts(uint8 count) public {
        count = uint8(bound(count, 1, 50));

        vm.startPrank(escrowOp);
        for (uint256 i = 1; i <= count; i++) {
            escrow.lockOnSubmission(i, contributor);
            assertTrue(escrow.isReceiptLocked(i));
        }
        vm.stopPrank();
    }
}
