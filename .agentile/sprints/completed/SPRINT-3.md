# Sprint 3 — Solidity Core Scaffolding

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: ACTIVE

## Sprint Goal
Implement the three foundational smart contracts (PoolRegistry, ItemReceiptNFT, PoolClaimToken) with role-based access control, pause controls, and comprehensive unit tests following Red-Green-Refactor TDD.

## Items

### 1. Initialize Foundry Project
- **Priority**: High
- **Size**: S
- **Feature File**: N/A (infrastructure setup)
- **Status**: DONE
- **Coverage**: N/A
- **Notes**: Foundry initialized in `contracts/`, OpenZeppelin installed, `foundry.toml` configured with Solc 0.8.26, remappings, fuzz config (256 runs).

### 2. Implement PoolRegistry Contract
- **Priority**: High
- **Size**: L
- **Feature File**: `features/contracts/contract-interfaces.feature`
- **Status**: DONE
- **Coverage**: 25 tests passing
- **Notes**: Pool creation with hue/band/region/tier validation, pool rules update, per-pool pause/unpause, global emergency pause, AccessControl + Pausable + ReentrancyGuard. Fuzz tested with bounded params.

### 3. Implement ItemReceiptNFT Contract
- **Priority**: High
- **Size**: L
- **Feature File**: `features/contracts/contract-interfaces.feature`
- **Status**: DONE
- **Coverage**: 24 tests passing
- **Notes**: ERC-721 soul-bound (all transfers revert), mint receipt, update intake status, set condition grade (1-10), attach evidence hash, quarantine with reason. Emergency pause. All warehouse functions gated by WAREHOUSE_ROLE.

### 4. Implement PoolClaimToken Contract
- **Priority**: High
- **Size**: M
- **Feature File**: `features/contracts/contract-interfaces.feature`
- **Status**: DONE
- **Coverage**: 25 tests passing (including 256-run fuzz)
- **Notes**: ERC-1155 with transfer restrictions. Default policy = blocked (v1). Allowlist policy available. Mint by CLAIM_MINTER, burn by ESCROW_OPERATOR. Total supply tracking per pool. Emergency pause. Fuzz tested mint/burn balance consistency.

### 5. Implement Shared Access Control & Pause
- **Priority**: High
- **Size**: S
- **Feature File**: `features/contracts/contract-interfaces.feature`
- **Status**: DONE
- **Coverage**: Tested across all 3 contracts
- **Notes**: `GradientRoles.sol` library with 8 role constants. Each contract uses AccessControl + Pausable + ReentrancyGuard. Emergency pause tested on all contracts.

## Exit Criteria
- [x] Foundry project builds with `forge build`
- [x] All 3 contracts compile without errors
- [x] Unit tests cover all public functions (74 tests total)
- [x] All tests pass with `forge test` (74/74 passing)
- [x] Role-based access control enforced on all state-changing functions
- [x] Emergency pause stops all mutable operations
- [x] Events emitted for all state changes

## Blockers
| Blocker | Feature | Status | Resolution |
|---------|---------|--------|------------|
| — | — | — | — |

## Metrics
- Features completed: 5/5
- Test coverage: 74 tests (25 PoolRegistry + 24 ItemReceiptNFT + 25 PoolClaimToken)
- Tests passing: 74/74
- Fuzz tests: 2 (256 runs each)
- Blockers encountered: 0
- Blockers resolved: 0
