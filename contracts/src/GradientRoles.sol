// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title GradientRoles — Shared role constants for the Gradient Barter Protocol
library GradientRoles {
    bytes32 public constant POOL_ADMIN = keccak256("POOL_ADMIN");
    bytes32 public constant WAREHOUSE_ROLE = keccak256("WAREHOUSE_ROLE");
    bytes32 public constant CLAIM_MINTER = keccak256("CLAIM_MINTER");
    bytes32 public constant ESCROW_OPERATOR = keccak256("ESCROW_OPERATOR");
    bytes32 public constant INVENTORY_OPERATOR = keccak256("INVENTORY_OPERATOR");
    bytes32 public constant DISPUTE_ADMIN = keccak256("DISPUTE_ADMIN");
    bytes32 public constant COURIER_ADMIN = keccak256("COURIER_ADMIN");
    bytes32 public constant EMERGENCY_ADMIN = keccak256("EMERGENCY_ADMIN");
}
