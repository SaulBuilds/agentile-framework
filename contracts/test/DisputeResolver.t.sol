// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/DisputeResolver.sol";
import "../src/GradientRoles.sol";

contract DisputeResolverTest is Test {
    DisputeResolver public resolver;
    address public admin = address(1);
    address public escrowOp = address(2);
    address public disputeAdmin = address(3);
    address public emergencyAdmin = address(4);
    address public unauthorized = address(5);
    address public actor = address(6);

    function setUp() public {
        vm.startPrank(admin);
        resolver = new DisputeResolver(admin);
        resolver.grantRole(GradientRoles.ESCROW_OPERATOR, escrowOp);
        resolver.grantRole(GradientRoles.DISPUTE_ADMIN, disputeAdmin);
        resolver.grantRole(GradientRoles.EMERGENCY_ADMIN, emergencyAdmin);
        vm.stopPrank();
    }

    // =============================================================
    //                     OPEN DISPUTE
    // =============================================================

    function test_openDispute_success() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(
            actor,
            DisputeResolver.ObjectType.Receipt,
            1,
            "Item not as described"
        );

        assertEq(disputeId, 1);

        (
            address dActor,
            DisputeResolver.ObjectType objectType,
            uint256 objectId,
            DisputeResolver.DisputeStatus status,
            DisputeResolver.Resolution resolution,
            uint256 openedAt,
            uint256 resolvedAt,
            uint256 evidenceCount
        ) = resolver.getDispute(disputeId);

        assertEq(dActor, actor);
        assertEq(uint256(objectType), uint256(DisputeResolver.ObjectType.Receipt));
        assertEq(objectId, 1);
        assertEq(uint256(status), uint256(DisputeResolver.DisputeStatus.Open));
        assertEq(uint256(resolution), uint256(DisputeResolver.Resolution.None));
        assertGt(openedAt, 0);
        assertEq(resolvedAt, 0);
        assertEq(evidenceCount, 0);
    }

    function test_openDispute_emitsEvent() public {
        vm.expectEmit(true, true, false, true);
        emit DisputeResolver.DisputeOpened(1, actor, DisputeResolver.ObjectType.Receipt, 1);

        vm.prank(escrowOp);
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");
    }

    function test_openDispute_incrementsId() public {
        vm.startPrank(escrowOp);
        uint256 id1 = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason 1");
        uint256 id2 = resolver.openDispute(actor, DisputeResolver.ObjectType.Inventory, 2, "Reason 2");
        vm.stopPrank();

        assertEq(id1, 1);
        assertEq(id2, 2);
    }

    function test_openDispute_tracksActiveDispute() public {
        vm.prank(escrowOp);
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        assertTrue(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));
    }

    function test_openDispute_revertsForDuplicate() public {
        vm.startPrank(escrowOp);
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "First dispute");

        vm.expectRevert("Dispute already open");
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Second dispute");
        vm.stopPrank();
    }

    function test_openDispute_allowsDifferentObjectTypes() public {
        vm.startPrank(escrowOp);
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Receipt dispute");
        resolver.openDispute(actor, DisputeResolver.ObjectType.Inventory, 1, "Inventory dispute");
        vm.stopPrank();

        assertTrue(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));
        assertTrue(resolver.hasActiveDispute(DisputeResolver.ObjectType.Inventory, 1));
    }

    function test_openDispute_revertsForZeroActor() public {
        vm.prank(escrowOp);
        vm.expectRevert("Invalid actor");
        resolver.openDispute(address(0), DisputeResolver.ObjectType.Receipt, 1, "Reason");
    }

    function test_openDispute_revertsForZeroObjectId() public {
        vm.prank(escrowOp);
        vm.expectRevert("Invalid object ID");
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 0, "Reason");
    }

    function test_openDispute_revertsForEmptyReason() public {
        vm.prank(escrowOp);
        vm.expectRevert("Reason required");
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "");
    }

    function test_openDispute_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");
    }

    // =============================================================
    //                     SUBMIT EVIDENCE
    // =============================================================

    function test_submitEvidence_success() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        bytes32 cidHash = keccak256("QmEvidenceHash1");
        vm.prank(escrowOp);
        resolver.submitEvidence(disputeId, cidHash);

        (, , , , , , , uint256 evidenceCount) = resolver.getDispute(disputeId);
        assertEq(evidenceCount, 1);
        assertEq(resolver.getEvidenceHash(disputeId, 0), cidHash);
    }

    function test_submitEvidence_multipleEvidence() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        bytes32 cid1 = keccak256("Evidence1");
        bytes32 cid2 = keccak256("Evidence2");
        bytes32 cid3 = keccak256("Evidence3");

        vm.startPrank(escrowOp);
        resolver.submitEvidence(disputeId, cid1);
        resolver.submitEvidence(disputeId, cid2);
        resolver.submitEvidence(disputeId, cid3);
        vm.stopPrank();

        (, , , , , , , uint256 evidenceCount) = resolver.getDispute(disputeId);
        assertEq(evidenceCount, 3);
        assertEq(resolver.getEvidenceHash(disputeId, 0), cid1);
        assertEq(resolver.getEvidenceHash(disputeId, 1), cid2);
        assertEq(resolver.getEvidenceHash(disputeId, 2), cid3);
    }

    function test_submitEvidence_emitsEvent() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        bytes32 cidHash = keccak256("Evidence");
        vm.expectEmit(true, false, false, true);
        emit DisputeResolver.EvidenceSubmitted(disputeId, cidHash);

        vm.prank(escrowOp);
        resolver.submitEvidence(disputeId, cidHash);
    }

    function test_submitEvidence_revertsForResolvedDispute() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(disputeAdmin);
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.Denied);

        vm.prank(escrowOp);
        vm.expectRevert("Dispute not open");
        resolver.submitEvidence(disputeId, keccak256("Late evidence"));
    }

    function test_submitEvidence_revertsForZeroHash() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(escrowOp);
        vm.expectRevert("Invalid evidence hash");
        resolver.submitEvidence(disputeId, bytes32(0));
    }

    function test_submitEvidence_revertsForUnauthorized() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(unauthorized);
        vm.expectRevert();
        resolver.submitEvidence(disputeId, keccak256("Evidence"));
    }

    // =============================================================
    //                     RESOLVE DISPUTE
    // =============================================================

    function test_resolveDispute_success() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(disputeAdmin);
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.RefundClaim);

        (
            ,
            ,
            ,
            DisputeResolver.DisputeStatus status,
            DisputeResolver.Resolution resolution,
            ,
            uint256 resolvedAt,
        ) = resolver.getDispute(disputeId);

        assertEq(uint256(status), uint256(DisputeResolver.DisputeStatus.Resolved));
        assertEq(uint256(resolution), uint256(DisputeResolver.Resolution.RefundClaim));
        assertGt(resolvedAt, 0);
    }

    function test_resolveDispute_clearsActiveFlag() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        assertTrue(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));

        vm.prank(disputeAdmin);
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.Denied);

        assertFalse(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));
    }

    function test_resolveDispute_allowsNewDisputeAfterResolution() public {
        vm.prank(escrowOp);
        uint256 id1 = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "First");

        vm.prank(disputeAdmin);
        resolver.resolveDispute(id1, DisputeResolver.Resolution.Denied);

        vm.prank(escrowOp);
        uint256 id2 = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Second");
        assertGt(id2, id1);
    }

    function test_resolveDispute_emitsEvent() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.expectEmit(true, false, false, true);
        emit DisputeResolver.DisputeResolved(disputeId, DisputeResolver.Resolution.Replacement);

        vm.prank(disputeAdmin);
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.Replacement);
    }

    function test_resolveDispute_allResolutionTypes() public {
        // RefundClaim
        vm.prank(escrowOp);
        uint256 id1 = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "R1");
        vm.prank(disputeAdmin);
        resolver.resolveDispute(id1, DisputeResolver.Resolution.RefundClaim);

        // Replacement
        vm.prank(escrowOp);
        uint256 id2 = resolver.openDispute(actor, DisputeResolver.ObjectType.Inventory, 1, "R2");
        vm.prank(disputeAdmin);
        resolver.resolveDispute(id2, DisputeResolver.Resolution.Replacement);

        // Denied
        vm.prank(escrowOp);
        uint256 id3 = resolver.openDispute(actor, DisputeResolver.ObjectType.Claim, 1, "R3");
        vm.prank(disputeAdmin);
        resolver.resolveDispute(id3, DisputeResolver.Resolution.Denied);

        // GoodwillCredit
        vm.prank(escrowOp);
        uint256 id4 = resolver.openDispute(actor, DisputeResolver.ObjectType.Shipment, 1, "R4");
        vm.prank(disputeAdmin);
        resolver.resolveDispute(id4, DisputeResolver.Resolution.GoodwillCredit);
    }

    function test_resolveDispute_revertsIfAlreadyResolved() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(disputeAdmin);
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.Denied);

        vm.prank(disputeAdmin);
        vm.expectRevert("Dispute not open");
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.RefundClaim);
    }

    function test_resolveDispute_revertsForNoneResolution() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(disputeAdmin);
        vm.expectRevert("Invalid resolution");
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.None);
    }

    function test_resolveDispute_revertsForUnauthorized() public {
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        vm.prank(unauthorized);
        vm.expectRevert();
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.Denied);
    }

    // =============================================================
    //                     EMERGENCY PAUSE
    // =============================================================

    function test_emergencyPause_blocksOpenDispute() public {
        vm.prank(emergencyAdmin);
        resolver.emergencyPause();

        vm.prank(escrowOp);
        vm.expectRevert();
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");
    }

    function test_emergencyUnpause_resumesOperations() public {
        vm.prank(emergencyAdmin);
        resolver.emergencyPause();

        vm.prank(emergencyAdmin);
        resolver.emergencyUnpause();

        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");
        assertEq(disputeId, 1);
    }

    // =============================================================
    //                     VIEW FUNCTIONS
    // =============================================================

    function test_nextDisputeId() public {
        assertEq(resolver.nextDisputeId(), 1);

        vm.prank(escrowOp);
        resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Reason");

        assertEq(resolver.nextDisputeId(), 2);
    }

    function test_hasActiveDispute_falseByDefault() public view {
        assertFalse(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));
    }

    // =============================================================
    //                     FULL LIFECYCLE
    // =============================================================

    function test_fullLifecycle_openSubmitResolve() public {
        // Open dispute
        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, 1, "Missing item");
        assertTrue(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));

        // Submit evidence
        vm.startPrank(escrowOp);
        resolver.submitEvidence(disputeId, keccak256("Photo of empty box"));
        resolver.submitEvidence(disputeId, keccak256("Shipping label scan"));
        vm.stopPrank();

        // Verify evidence
        (, , , , , , , uint256 evidenceCount) = resolver.getDispute(disputeId);
        assertEq(evidenceCount, 2);

        // Resolve
        vm.prank(disputeAdmin);
        resolver.resolveDispute(disputeId, DisputeResolver.Resolution.RefundClaim);

        // Verify resolved
        assertFalse(resolver.hasActiveDispute(DisputeResolver.ObjectType.Receipt, 1));
        (
            ,
            ,
            ,
            DisputeResolver.DisputeStatus status,
            DisputeResolver.Resolution resolution,
            ,
            ,
        ) = resolver.getDispute(disputeId);
        assertEq(uint256(status), uint256(DisputeResolver.DisputeStatus.Resolved));
        assertEq(uint256(resolution), uint256(DisputeResolver.Resolution.RefundClaim));
    }

    // =============================================================
    //                     FUZZ TESTS
    // =============================================================

    function testFuzz_openDispute_validObjectIds(uint256 objectId) public {
        objectId = bound(objectId, 1, type(uint128).max);

        vm.prank(escrowOp);
        uint256 disputeId = resolver.openDispute(actor, DisputeResolver.ObjectType.Receipt, objectId, "Fuzz test");

        (
            address dActor,
            ,
            uint256 dObjectId,
            ,
            ,
            ,
            ,
        ) = resolver.getDispute(disputeId);

        assertEq(dActor, actor);
        assertEq(dObjectId, objectId);
    }
}
