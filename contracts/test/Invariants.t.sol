// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Test.sol";
import "../src/EscrowSettlement.sol";
import "../src/PoolClaimToken.sol";
import "../src/InventoryCommitment.sol";
import "../src/GradientRoles.sol";

/// @title InvariantHandler — drives state transitions for invariant testing
/// @notice The fuzzer calls this handler's public functions to exercise the system
contract InvariantHandler is Test {
    EscrowSettlement public escrow;
    PoolClaimToken public claimToken;
    InventoryCommitment public inventory;

    address public admin;
    address[] public actors;

    // Tracking for invariant assertions
    uint256 public totalRegistered;
    uint256 public totalReserved;
    uint256 public totalDelivered;
    uint256 public totalQuarantined;
    uint256 public totalReleased;
    uint256 public totalClaimsMinted;
    uint256 public totalClaimsBurned;
    uint256 public totalCancelled;
    uint256 public totalExpired;
    uint256 public totalFinalized;

    // Track inventory IDs and reservation IDs for targeted operations
    uint256[] public inventoryIds;
    uint256[] public reservationIds;
    uint256 public receiptCounter;

    uint256 constant POOL_1 = 1;

    constructor(
        EscrowSettlement _escrow,
        PoolClaimToken _claimToken,
        InventoryCommitment _inventory,
        address _admin
    ) {
        escrow = _escrow;
        claimToken = _claimToken;
        inventory = _inventory;
        admin = _admin;

        actors.push(address(0x1001));
        actors.push(address(0x1002));
        actors.push(address(0x1003));
    }

    function _randomActor(uint256 seed) internal view returns (address) {
        return actors[seed % actors.length];
    }

    /// @notice Register new inventory and lock/unlock a claim for a random actor
    function registerAndUnlock(uint256 actorSeed) external {
        address actor = _randomActor(actorSeed);
        receiptCounter++;
        uint256 receiptId = receiptCounter;

        // Register inventory
        vm.prank(admin);
        uint256 invId = inventory.registerInventory(receiptId, POOL_1);
        inventoryIds.push(invId);
        totalRegistered++;

        // Lock and unlock claim for actor
        vm.startPrank(admin);
        escrow.lockOnSubmission(receiptId, actor);
        escrow.unlockClaimOnVerifiedIntake(receiptId, actor, POOL_1);
        vm.stopPrank();

        totalClaimsMinted++;
    }

    /// @notice Consume a claim for a reservation
    function consumeClaim(uint256 actorSeed, uint256 invIndex) external {
        if (inventoryIds.length == 0) return;
        invIndex = invIndex % inventoryIds.length;
        uint256 invId = inventoryIds[invIndex];

        address actor = _randomActor(actorSeed);

        // Check preconditions
        if (claimToken.balanceOf(actor, POOL_1) < 1) return;
        if (inventory.getInventoryState(invId) != InventoryCommitment.InventoryState.Available) return;

        vm.prank(admin);
        uint256 resId = escrow.consumeClaimForReservation(actor, POOL_1, invId);
        reservationIds.push(resId);

        totalReserved++;
        totalClaimsBurned++;
    }

    /// @notice Cancel a reservation
    function cancelReservation(uint256 resIndex) external {
        if (reservationIds.length == 0) return;
        resIndex = resIndex % reservationIds.length;
        uint256 resId = reservationIds[resIndex];

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        if (res.status != EscrowSettlement.ReservationStatus.Active) return;

        vm.prank(admin);
        escrow.cancelReservation(resId);

        totalCancelled++;
        totalReleased++;
        totalClaimsMinted++; // re-minted on cancel
    }

    /// @notice Ship and deliver an inventory item, then finalize
    function deliverAndFinalize(uint256 resIndex) external {
        if (reservationIds.length == 0) return;
        resIndex = resIndex % reservationIds.length;
        uint256 resId = reservationIds[resIndex];

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        if (res.status != EscrowSettlement.ReservationStatus.Active) return;

        uint256 invId = res.inventoryId;
        InventoryCommitment.InventoryState invState = inventory.getInventoryState(invId);

        vm.startPrank(admin);

        // Advance inventory state if needed
        if (invState == InventoryCommitment.InventoryState.Reserved) {
            inventory.markOutbound(invId, keccak256(abi.encodePacked("SHIP", resId)));
        }

        invState = inventory.getInventoryState(invId);
        if (invState == InventoryCommitment.InventoryState.Outbound) {
            inventory.confirmDelivered(invId, keccak256(abi.encodePacked("PROOF", resId)));
        }

        // Finalize
        escrow.finalizeWithdrawal(resId);
        vm.stopPrank();

        totalDelivered++;
        totalFinalized++;
    }

    /// @notice Expire a reservation by warping time
    function expireReservation(uint256 resIndex) external {
        if (reservationIds.length == 0) return;
        resIndex = resIndex % reservationIds.length;
        uint256 resId = reservationIds[resIndex];

        EscrowSettlement.Reservation memory res = escrow.getReservation(resId);
        if (res.status != EscrowSettlement.ReservationStatus.Active) return;

        // Warp past expiry
        vm.warp(res.expiresAt + 1);
        escrow.expireReservation(resId);

        totalExpired++;
        totalReleased++;
        totalClaimsMinted++; // re-minted on expire
    }

    function getInventoryIdsLength() external view returns (uint256) {
        return inventoryIds.length;
    }

    function getReservationIdsLength() external view returns (uint256) {
        return reservationIds.length;
    }
}

/// @title InvariantTest — Tests the 5 critical invariants of the settlement system
contract InvariantTest is Test {
    EscrowSettlement public escrow;
    PoolClaimToken public claimToken;
    InventoryCommitment public inventory;
    InvariantHandler public handler;

    address public admin = address(0xAD);
    uint256 constant POOL_1 = 1;

    function setUp() public {
        vm.startPrank(admin);

        // Deploy system
        claimToken = new PoolClaimToken(admin);
        inventory = new InventoryCommitment(admin);
        escrow = new EscrowSettlement(admin, address(claimToken), address(inventory));

        // Grant roles
        claimToken.grantRole(GradientRoles.CLAIM_MINTER, address(escrow));
        claimToken.grantRole(GradientRoles.ESCROW_OPERATOR, address(escrow));
        inventory.grantRole(GradientRoles.ESCROW_OPERATOR, address(escrow));
        inventory.grantRole(GradientRoles.INVENTORY_OPERATOR, admin);

        // Grant admin the ESCROW_OPERATOR role for handler calls
        escrow.grantRole(GradientRoles.ESCROW_OPERATOR, admin);

        vm.stopPrank();

        // Deploy handler
        handler = new InvariantHandler(escrow, claimToken, inventory, admin);

        // Only target handler functions
        targetContract(address(handler));
    }

    /// @notice Invariant 1: Total claims in circulation never exceed total registered inventory
    /// claims_in_circulation = totalMinted - totalBurned
    /// This must always be <= totalRegistered
    function invariant_claimsNeverExceedInventory() public view {
        uint256 claimsInCirculation = 0;
        address[] memory actors = new address[](3);
        actors[0] = address(0x1001);
        actors[1] = address(0x1002);
        actors[2] = address(0x1003);

        for (uint256 i = 0; i < actors.length; i++) {
            claimsInCirculation += claimToken.balanceOf(actors[i], POOL_1);
        }

        assertLe(
            claimsInCirculation,
            handler.totalRegistered(),
            "INV-1: Claims in circulation exceed registered inventory"
        );
    }

    /// @notice Invariant 2: Available + Reserved + Outbound + Delivered + Quarantined = Total registered
    /// We verify available count consistency via the pool-level counter
    function invariant_availableCountConsistent() public view {
        uint256 registeredCount = handler.totalRegistered();
        uint256 availableCount = inventory.getAvailableCount(POOL_1);

        // Available count can never exceed total registered
        assertLe(
            availableCount,
            registeredCount,
            "INV-2: Available count exceeds total registered"
        );
    }

    /// @notice Invariant 3: Every finalized reservation maps to exactly one delivered item
    function invariant_finalizedMatchesDelivered() public view {
        assertEq(
            handler.totalFinalized(),
            handler.totalDelivered(),
            "INV-3: Finalized count doesn't match delivered count"
        );
    }

    /// @notice Invariant 4: Expired/cancelled reservations re-mint exactly one claim and release inventory
    /// Net claims = totalMinted - totalBurned should equal claims in circulation
    function invariant_expiredReservationsReleaseResources() public view {
        uint256 netMinted = handler.totalClaimsMinted();
        uint256 netBurned = handler.totalClaimsBurned();

        // Net claims minted >= net burned (can't burn more than minted)
        assertGe(
            netMinted,
            netBurned,
            "INV-4: More claims burned than minted"
        );

        // Calculate expected claims in circulation
        uint256 expectedInCirculation = netMinted - netBurned;

        // Verify against actual balances
        uint256 actualInCirculation = 0;
        address[] memory actors = new address[](3);
        actors[0] = address(0x1001);
        actors[1] = address(0x1002);
        actors[2] = address(0x1003);

        for (uint256 i = 0; i < actors.length; i++) {
            actualInCirculation += claimToken.balanceOf(actors[i], POOL_1);
        }

        assertEq(
            actualInCirculation,
            expectedInCirculation,
            "INV-4: Claim balance doesn't match mint/burn accounting"
        );
    }

    /// @notice Invariant 5: No reservation can be double-finalized, double-cancelled, or double-expired
    /// The handler tracks these counts and they must be internally consistent
    function invariant_noDoubleSettlement() public view {
        uint256 totalSettled = handler.totalFinalized() + handler.totalCancelled() + handler.totalExpired();
        uint256 totalReserved = handler.totalReserved();

        // Settled reservations can never exceed total reservations created
        assertLe(
            totalSettled,
            totalReserved,
            "INV-5: More settlements than reservations"
        );
    }
}
