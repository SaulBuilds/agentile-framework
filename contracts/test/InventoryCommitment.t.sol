// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/InventoryCommitment.sol";
import "../src/GradientRoles.sol";

contract InventoryCommitmentTest is Test {
    InventoryCommitment public inventory;
    address public admin = address(1);
    address public inventoryOp = address(2);
    address public escrowOp = address(3);
    address public emergencyAdmin = address(4);
    address public unauthorized = address(5);

    uint256 constant RECEIPT_1 = 1;
    uint256 constant RECEIPT_2 = 2;
    uint256 constant POOL_1 = 1;
    uint256 constant POOL_2 = 2;

    function setUp() public {
        vm.startPrank(admin);
        inventory = new InventoryCommitment(admin);
        inventory.grantRole(GradientRoles.INVENTORY_OPERATOR, inventoryOp);
        inventory.grantRole(GradientRoles.ESCROW_OPERATOR, escrowOp);
        inventory.grantRole(GradientRoles.EMERGENCY_ADMIN, emergencyAdmin);
        vm.stopPrank();
    }

    // =============================================================
    //                     REGISTER INVENTORY
    // =============================================================

    function test_registerInventory_success() public {
        vm.prank(inventoryOp);
        uint256 id = inventory.registerInventory(RECEIPT_1, POOL_1);

        assertEq(id, 1);
        InventoryCommitment.InventoryRecord memory rec = inventory.getInventory(id);
        assertEq(rec.receiptId, RECEIPT_1);
        assertEq(rec.poolId, POOL_1);
        assertEq(uint256(rec.state), uint256(InventoryCommitment.InventoryState.Available));
        assertEq(rec.reservationId, 0);
    }

    function test_registerInventory_emitsEvent() public {
        vm.expectEmit(true, true, true, true);
        emit InventoryCommitment.InventoryRegistered(1, RECEIPT_1, POOL_1);

        vm.prank(inventoryOp);
        inventory.registerInventory(RECEIPT_1, POOL_1);
    }

    function test_registerInventory_incrementsId() public {
        vm.startPrank(inventoryOp);
        uint256 id1 = inventory.registerInventory(RECEIPT_1, POOL_1);
        uint256 id2 = inventory.registerInventory(RECEIPT_2, POOL_1);
        vm.stopPrank();

        assertEq(id1, 1);
        assertEq(id2, 2);
    }

    function test_registerInventory_incrementsAvailableCount() public {
        vm.startPrank(inventoryOp);
        inventory.registerInventory(RECEIPT_1, POOL_1);
        inventory.registerInventory(RECEIPT_2, POOL_1);
        vm.stopPrank();

        assertEq(inventory.getAvailableCount(POOL_1), 2);
    }

    function test_registerInventory_revertsForZeroReceiptId() public {
        vm.prank(inventoryOp);
        vm.expectRevert("Invalid receipt ID");
        inventory.registerInventory(0, POOL_1);
    }

    function test_registerInventory_revertsForZeroPoolId() public {
        vm.prank(inventoryOp);
        vm.expectRevert("Invalid pool ID");
        inventory.registerInventory(RECEIPT_1, 0);
    }

    function test_registerInventory_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        inventory.registerInventory(RECEIPT_1, POOL_1);
    }

    // =============================================================
    //                     RESERVE INVENTORY
    // =============================================================

    function test_reserveInventory_success() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        InventoryCommitment.InventoryRecord memory rec = inventory.getInventory(invId);
        assertEq(uint256(rec.state), uint256(InventoryCommitment.InventoryState.Reserved));
        assertEq(rec.reservationId, 100);
        assertEq(inventory.getAvailableCount(POOL_1), 0);
    }

    function test_reserveInventory_emitsEvent() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.expectEmit(true, true, false, true);
        emit InventoryCommitment.InventoryReserved(invId, 100);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);
    }

    function test_reserveInventory_revertsIfNotAvailable() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(escrowOp);
        vm.expectRevert("Not available");
        inventory.reserveInventory(invId, 101);
    }

    function test_reserveInventory_revertsForUnauthorized() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(unauthorized);
        vm.expectRevert();
        inventory.reserveInventory(invId, 100);
    }

    // =============================================================
    //                     RELEASE RESERVATION
    // =============================================================

    function test_releaseReservation_success() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(escrowOp);
        inventory.releaseReservation(invId);

        InventoryCommitment.InventoryRecord memory rec = inventory.getInventory(invId);
        assertEq(uint256(rec.state), uint256(InventoryCommitment.InventoryState.Available));
        assertEq(rec.reservationId, 0);
        assertEq(inventory.getAvailableCount(POOL_1), 1);
    }

    function test_releaseReservation_emitsEvent() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.expectEmit(true, false, false, true);
        emit InventoryCommitment.InventoryReleased(invId);

        vm.prank(escrowOp);
        inventory.releaseReservation(invId);
    }

    function test_releaseReservation_revertsIfNotReserved() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        vm.expectRevert("Not reserved");
        inventory.releaseReservation(invId);
    }

    // =============================================================
    //                     MARK OUTBOUND
    // =============================================================

    function test_markOutbound_success() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        bytes32 trackingRef = keccak256("TRACK-001");
        vm.prank(inventoryOp);
        inventory.markOutbound(invId, trackingRef);

        InventoryCommitment.InventoryRecord memory rec = inventory.getInventory(invId);
        assertEq(uint256(rec.state), uint256(InventoryCommitment.InventoryState.Outbound));
        assertEq(rec.trackingRef, trackingRef);
    }

    function test_markOutbound_emitsEvent() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        bytes32 trackingRef = keccak256("TRACK-001");
        vm.expectEmit(true, false, false, true);
        emit InventoryCommitment.InventoryOutbound(invId, trackingRef);

        vm.prank(inventoryOp);
        inventory.markOutbound(invId, trackingRef);
    }

    function test_markOutbound_revertsIfNotReserved() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(inventoryOp);
        vm.expectRevert("Not reserved");
        inventory.markOutbound(invId, keccak256("TRACK"));
    }

    function test_markOutbound_revertsForZeroTrackingRef() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(inventoryOp);
        vm.expectRevert("Invalid tracking ref");
        inventory.markOutbound(invId, bytes32(0));
    }

    // =============================================================
    //                     CONFIRM DELIVERED
    // =============================================================

    function test_confirmDelivered_success() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        bytes32 trackingRef = keccak256("TRACK-001");
        vm.prank(inventoryOp);
        inventory.markOutbound(invId, trackingRef);

        bytes32 proofHash = keccak256("DELIVERY-PROOF");
        vm.prank(inventoryOp);
        inventory.confirmDelivered(invId, proofHash);

        InventoryCommitment.InventoryRecord memory rec = inventory.getInventory(invId);
        assertEq(uint256(rec.state), uint256(InventoryCommitment.InventoryState.Delivered));
        assertEq(rec.proofHash, proofHash);
    }

    function test_confirmDelivered_emitsEvent() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(inventoryOp);
        inventory.markOutbound(invId, keccak256("TRACK"));

        bytes32 proofHash = keccak256("PROOF");
        vm.expectEmit(true, false, false, true);
        emit InventoryCommitment.InventoryDelivered(invId, proofHash);

        vm.prank(inventoryOp);
        inventory.confirmDelivered(invId, proofHash);
    }

    function test_confirmDelivered_revertsIfNotOutbound() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(inventoryOp);
        vm.expectRevert("Not outbound");
        inventory.confirmDelivered(invId, keccak256("PROOF"));
    }

    function test_confirmDelivered_revertsForZeroProofHash() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(inventoryOp);
        inventory.markOutbound(invId, keccak256("TRACK"));

        vm.prank(inventoryOp);
        vm.expectRevert("Invalid proof hash");
        inventory.confirmDelivered(invId, bytes32(0));
    }

    // =============================================================
    //                     QUARANTINE
    // =============================================================

    function test_quarantine_fromAvailable() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);
        assertEq(inventory.getAvailableCount(POOL_1), 1);

        vm.prank(inventoryOp);
        inventory.quarantineInventory(invId, "Damaged");

        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Quarantined));
        assertEq(inventory.getAvailableCount(POOL_1), 0);
    }

    function test_quarantine_fromReserved() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(inventoryOp);
        inventory.quarantineInventory(invId, "Safety concern");

        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Quarantined));
    }

    function test_quarantine_emitsEvent() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.expectEmit(true, false, false, true);
        emit InventoryCommitment.InventoryQuarantined(invId, "Test reason");

        vm.prank(inventoryOp);
        inventory.quarantineInventory(invId, "Test reason");
    }

    function test_quarantine_revertsFromOutbound() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(inventoryOp);
        inventory.markOutbound(invId, keccak256("TRACK"));

        vm.prank(inventoryOp);
        vm.expectRevert("Cannot quarantine from current state");
        inventory.quarantineInventory(invId, "Too late");
    }

    function test_quarantine_revertsFromDelivered() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 100);

        vm.prank(inventoryOp);
        inventory.markOutbound(invId, keccak256("TRACK"));

        vm.prank(inventoryOp);
        inventory.confirmDelivered(invId, keccak256("PROOF"));

        vm.prank(inventoryOp);
        vm.expectRevert("Cannot quarantine from current state");
        inventory.quarantineInventory(invId, "Already delivered");
    }

    // =============================================================
    //                     EMERGENCY PAUSE
    // =============================================================

    function test_emergencyPause_blocksRegistration() public {
        vm.prank(emergencyAdmin);
        inventory.emergencyPause();

        vm.prank(inventoryOp);
        vm.expectRevert();
        inventory.registerInventory(RECEIPT_1, POOL_1);
    }

    function test_emergencyUnpause_resumesOperations() public {
        vm.prank(emergencyAdmin);
        inventory.emergencyPause();

        vm.prank(emergencyAdmin);
        inventory.emergencyUnpause();

        vm.prank(inventoryOp);
        uint256 id = inventory.registerInventory(RECEIPT_1, POOL_1);
        assertEq(id, 1);
    }

    // =============================================================
    //                     VIEW FUNCTIONS
    // =============================================================

    function test_getInventoryState_returnsCorrectState() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Available));
    }

    function test_getInventoryState_noneForNonexistent() public view {
        assertEq(uint256(inventory.getInventoryState(999)), uint256(InventoryCommitment.InventoryState.None));
    }

    function test_nextInventoryId() public {
        assertEq(inventory.nextInventoryId(), 1);

        vm.prank(inventoryOp);
        inventory.registerInventory(RECEIPT_1, POOL_1);

        assertEq(inventory.nextInventoryId(), 2);
    }

    // =============================================================
    //                     FULL LIFECYCLE
    // =============================================================

    function test_fullLifecycle_availableToDelivered() public {
        // Register
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);
        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Available));
        assertEq(inventory.getAvailableCount(POOL_1), 1);

        // Reserve
        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 1);
        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Reserved));
        assertEq(inventory.getAvailableCount(POOL_1), 0);

        // Outbound
        vm.prank(inventoryOp);
        inventory.markOutbound(invId, keccak256("SHIP-001"));
        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Outbound));

        // Delivered
        vm.prank(inventoryOp);
        inventory.confirmDelivered(invId, keccak256("PROOF-001"));
        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Delivered));
    }

    function test_lifecycle_reserveAndRelease() public {
        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(RECEIPT_1, POOL_1);

        vm.prank(escrowOp);
        inventory.reserveInventory(invId, 1);
        assertEq(inventory.getAvailableCount(POOL_1), 0);

        vm.prank(escrowOp);
        inventory.releaseReservation(invId);
        assertEq(inventory.getAvailableCount(POOL_1), 1);
        assertEq(uint256(inventory.getInventoryState(invId)), uint256(InventoryCommitment.InventoryState.Available));
    }

    // =============================================================
    //                     FUZZ TESTS
    // =============================================================

    function testFuzz_registerInventory_validParams(uint256 receiptId, uint256 poolId) public {
        receiptId = bound(receiptId, 1, type(uint128).max);
        poolId = bound(poolId, 1, type(uint128).max);

        vm.prank(inventoryOp);
        uint256 invId = inventory.registerInventory(receiptId, poolId);

        InventoryCommitment.InventoryRecord memory rec = inventory.getInventory(invId);
        assertEq(rec.receiptId, receiptId);
        assertEq(rec.poolId, poolId);
        assertEq(uint256(rec.state), uint256(InventoryCommitment.InventoryState.Available));
    }

    function testFuzz_availableCount_consistent(uint8 count) public {
        count = uint8(bound(count, 1, 50));

        vm.startPrank(inventoryOp);
        for (uint256 i = 0; i < count; i++) {
            inventory.registerInventory(i + 1, POOL_1);
        }
        vm.stopPrank();

        assertEq(inventory.getAvailableCount(POOL_1), count);
    }
}
