// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/ItemReceiptNFT.sol";
import "../src/GradientRoles.sol";

contract ItemReceiptNFTTest is Test {
    ItemReceiptNFT public nft;
    address public admin = address(1);
    address public warehouse = address(2);
    address public contributor = address(3);
    address public unauthorized = address(4);
    address public emergencyAdmin = address(5);

    bytes32 constant HASH_A = keccak256("submission-A");
    bytes32 constant HASH_B = keccak256("submission-B");
    bytes32 constant EVIDENCE_HASH = keccak256("evidence-photo-cid");

    function setUp() public {
        vm.startPrank(admin);
        nft = new ItemReceiptNFT(admin);
        nft.grantRole(GradientRoles.WAREHOUSE_ROLE, warehouse);
        nft.grantRole(GradientRoles.EMERGENCY_ADMIN, emergencyAdmin);
        vm.stopPrank();
    }

    // --- Minting ---

    function test_mintReceipt_success() public {
        vm.prank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        assertEq(tokenId, 1);
        assertEq(nft.ownerOf(tokenId), contributor);

        ItemReceiptNFT.ReceiptData memory data = nft.getReceiptData(tokenId);
        assertEq(data.submissionHash, HASH_A);
        assertEq(data.poolId, 1);
        assertEq(uint8(data.intakeStatus), uint8(ItemReceiptNFT.IntakeStatus.Pending));
        assertEq(data.conditionGrade, 0);
    }

    function test_mintReceipt_emitsEvent() public {
        vm.expectEmit(true, true, true, true);
        emit ItemReceiptNFT.ReceiptMinted(1, contributor, 1, HASH_A);

        vm.prank(warehouse);
        nft.mintReceipt(contributor, HASH_A, 1);
    }

    function test_mintReceipt_incrementsId() public {
        vm.startPrank(warehouse);
        uint256 id1 = nft.mintReceipt(contributor, HASH_A, 1);
        uint256 id2 = nft.mintReceipt(contributor, HASH_B, 2);
        vm.stopPrank();

        assertEq(id1, 1);
        assertEq(id2, 2);
    }

    function test_mintReceipt_revertsForUnauthorized() public {
        vm.prank(unauthorized);
        vm.expectRevert();
        nft.mintReceipt(contributor, HASH_A, 1);
    }

    function test_mintReceipt_revertsForZeroAddress() public {
        vm.prank(warehouse);
        vm.expectRevert("Invalid contributor");
        nft.mintReceipt(address(0), HASH_A, 1);
    }

    function test_mintReceipt_revertsForZeroHash() public {
        vm.prank(warehouse);
        vm.expectRevert("Invalid submission hash");
        nft.mintReceipt(contributor, bytes32(0), 1);
    }

    function test_mintReceipt_revertsForZeroPoolId() public {
        vm.prank(warehouse);
        vm.expectRevert("Invalid pool ID");
        nft.mintReceipt(contributor, HASH_A, 0);
    }

    // --- Intake Status ---

    function test_updateIntakeStatus_toAccepted() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);
        nft.updateIntakeStatus(tokenId, ItemReceiptNFT.IntakeStatus.Accepted);
        vm.stopPrank();

        ItemReceiptNFT.ReceiptData memory data = nft.getReceiptData(tokenId);
        assertEq(uint8(data.intakeStatus), uint8(ItemReceiptNFT.IntakeStatus.Accepted));
    }

    function test_updateIntakeStatus_emitsEvent() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.expectEmit(true, false, false, true);
        emit ItemReceiptNFT.IntakeStatusUpdated(tokenId, ItemReceiptNFT.IntakeStatus.Accepted);
        nft.updateIntakeStatus(tokenId, ItemReceiptNFT.IntakeStatus.Accepted);
        vm.stopPrank();
    }

    function test_updateIntakeStatus_revertsForNonexistentToken() public {
        vm.prank(warehouse);
        vm.expectRevert("Token does not exist");
        nft.updateIntakeStatus(999, ItemReceiptNFT.IntakeStatus.Accepted);
    }

    // --- Condition Grade ---

    function test_setConditionGrade_success() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);
        nft.setConditionGrade(tokenId, 8);
        vm.stopPrank();

        assertEq(nft.getReceiptData(tokenId).conditionGrade, 8);
    }

    function test_setConditionGrade_revertsForZero() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.expectRevert("Grade must be 1-10");
        nft.setConditionGrade(tokenId, 0);
        vm.stopPrank();
    }

    function test_setConditionGrade_revertsForEleven() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.expectRevert("Grade must be 1-10");
        nft.setConditionGrade(tokenId, 11);
        vm.stopPrank();
    }

    // --- Evidence ---

    function test_attachEvidence_success() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);
        nft.attachEvidence(tokenId, EVIDENCE_HASH);
        vm.stopPrank();

        assertEq(nft.getReceiptData(tokenId).evidenceHash, EVIDENCE_HASH);
    }

    function test_attachEvidence_emitsEvent() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.expectEmit(true, false, false, true);
        emit ItemReceiptNFT.EvidenceAttached(tokenId, EVIDENCE_HASH);
        nft.attachEvidence(tokenId, EVIDENCE_HASH);
        vm.stopPrank();
    }

    function test_attachEvidence_revertsForZeroHash() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.expectRevert("Invalid evidence hash");
        nft.attachEvidence(tokenId, bytes32(0));
        vm.stopPrank();
    }

    // --- Quarantine ---

    function test_markQuarantined_success() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);
        nft.markQuarantined(tokenId, "Suspected counterfeit");
        vm.stopPrank();

        assertEq(
            uint8(nft.getReceiptData(tokenId).intakeStatus),
            uint8(ItemReceiptNFT.IntakeStatus.Quarantined)
        );
    }

    function test_markQuarantined_emitsEvent() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.expectEmit(true, false, false, true);
        emit ItemReceiptNFT.ItemQuarantined(tokenId, "Safety concern");
        nft.markQuarantined(tokenId, "Safety concern");
        vm.stopPrank();
    }

    function test_markQuarantined_revertsForRejectedItem() public {
        vm.startPrank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);
        nft.updateIntakeStatus(tokenId, ItemReceiptNFT.IntakeStatus.Rejected);

        vm.expectRevert("Cannot quarantine rejected item");
        nft.markQuarantined(tokenId, "Too late");
        vm.stopPrank();
    }

    // --- Soul-bound (Non-Transferable) ---

    function test_transfer_reverts() public {
        vm.prank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.prank(contributor);
        vm.expectRevert("ItemReceiptNFT: non-transferable");
        nft.transferFrom(contributor, unauthorized, tokenId);
    }

    function test_safeTransfer_reverts() public {
        vm.prank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);

        vm.prank(contributor);
        vm.expectRevert("ItemReceiptNFT: non-transferable");
        nft.safeTransferFrom(contributor, unauthorized, tokenId);
    }

    // --- Emergency Pause ---

    function test_emergencyPause_blocksMinting() public {
        vm.prank(emergencyAdmin);
        nft.emergencyPause();

        vm.prank(warehouse);
        vm.expectRevert();
        nft.mintReceipt(contributor, HASH_A, 1);
    }

    function test_emergencyUnpause_resumesMinting() public {
        vm.prank(emergencyAdmin);
        nft.emergencyPause();

        vm.prank(emergencyAdmin);
        nft.emergencyUnpause();

        vm.prank(warehouse);
        uint256 tokenId = nft.mintReceipt(contributor, HASH_A, 1);
        assertEq(tokenId, 1);
    }

    // --- View ---

    function test_getReceiptData_revertsForNonexistent() public {
        vm.expectRevert("Token does not exist");
        nft.getReceiptData(999);
    }
}
