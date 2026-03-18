// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "forge-std/Script.sol";
import "../src/PoolRegistry.sol";
import "../src/ItemReceiptNFT.sol";
import "../src/PoolClaimToken.sol";
import "../src/InventoryCommitment.sol";
import "../src/EscrowSettlement.sol";
import "../src/DisputeResolver.sol";
import "../src/GradientRoles.sol";

contract Deploy is Script {
    function run() external {
        uint256 deployerPk = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address deployer = vm.addr(deployerPk);

        console.log("Deployer:", deployer);
        console.log("Chain ID:", block.chainid);

        vm.startBroadcast(deployerPk);

        // --- Phase 1: Independent contracts ---
        PoolRegistry poolRegistry = new PoolRegistry(deployer);
        console.log("PoolRegistry:", address(poolRegistry));

        ItemReceiptNFT receiptNFT = new ItemReceiptNFT(deployer);
        console.log("ItemReceiptNFT:", address(receiptNFT));

        PoolClaimToken claimToken = new PoolClaimToken(deployer);
        console.log("PoolClaimToken:", address(claimToken));

        InventoryCommitment inventory = new InventoryCommitment(deployer);
        console.log("InventoryCommitment:", address(inventory));

        DisputeResolver disputeResolver = new DisputeResolver(deployer);
        console.log("DisputeResolver:", address(disputeResolver));

        // --- Phase 2: Dependent contract ---
        EscrowSettlement escrow = new EscrowSettlement(
            deployer,
            address(claimToken),
            address(inventory)
        );
        console.log("EscrowSettlement:", address(escrow));

        // --- Phase 3: Role grants ---
        // EscrowSettlement needs CLAIM_MINTER + ESCROW_OPERATOR on PoolClaimToken
        claimToken.grantRole(GradientRoles.CLAIM_MINTER, address(escrow));
        claimToken.grantRole(GradientRoles.ESCROW_OPERATOR, address(escrow));

        // EscrowSettlement needs ESCROW_OPERATOR on InventoryCommitment
        inventory.grantRole(GradientRoles.ESCROW_OPERATOR, address(escrow));

        // Deployer acts as operator for all roles initially
        receiptNFT.grantRole(GradientRoles.WAREHOUSE_ROLE, deployer);
        inventory.grantRole(GradientRoles.INVENTORY_OPERATOR, deployer);
        inventory.grantRole(GradientRoles.WAREHOUSE_ROLE, deployer);
        disputeResolver.grantRole(GradientRoles.DISPUTE_ADMIN, deployer);
        disputeResolver.grantRole(GradientRoles.ESCROW_OPERATOR, deployer);

        vm.stopBroadcast();

        console.log("");
        console.log("=== Deployment Complete ===");
        console.log("PoolRegistry:         ", address(poolRegistry));
        console.log("ItemReceiptNFT:       ", address(receiptNFT));
        console.log("PoolClaimToken:       ", address(claimToken));
        console.log("InventoryCommitment:  ", address(inventory));
        console.log("DisputeResolver:      ", address(disputeResolver));
        console.log("EscrowSettlement:     ", address(escrow));
    }
}
