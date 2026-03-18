# Sprint 2 — Architecture & Security Baselines

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: ACTIVE

## Sprint Goal
Produce detailed smart contract interfaces, a Prisma DB schema, threat model, privacy policy, and delivery partner assessment so that implementation sprints have clear, security-reviewed specs to build against.

## Items

### 1. Define Smart Contract Interfaces & Invariants
- **Priority**: High
- **Size**: XL
- **Feature File**: `features/contracts/contract-interfaces.feature`
- **Deliverable**: `planset/architecture/CONTRACT_INTERFACES.md`
- **Status**: DONE
- **Notes**: All 7 contracts specified with full Solidity interfaces, events, access control roles, CEI annotations, and 5 critical invariants for fuzz testing. Security patterns documented (reentrancy guards, pausable, pull payment, EIP-712).

### 2. Draft Prisma DB Schema
- **Priority**: High
- **Size**: L
- **Feature File**: `features/backend/db-schema.feature`
- **Deliverable**: `planset/architecture/PRISMA_SCHEMA_DRAFT.md`
- **Status**: DONE
- **Notes**: Complete Prisma schema with 25 models, 18 enums, all relations with cascade rules, composite indexes for common query patterns. Ready to be copied into `prisma/schema.prisma` during Sprint 5.

### 3. Create Threat Model
- **Priority**: High
- **Size**: L
- **Feature File**: `features/security/threat-model.feature`
- **Deliverable**: `planset/architecture/THREAT_MODEL.md`
- **Status**: DONE
- **Notes**: STRIDE analysis across 5 layers: smart contracts (6 threats), backend (6 threats), warehouse (4 threats), logistics (3 threats), user-facing (3 threats). 22 threats total, all mitigated. Top 5 residual risks identified for ongoing monitoring.

### 4. Draft Privacy & Data Retention Policy
- **Priority**: Medium
- **Size**: M
- **Feature File**: `features/compliance/privacy-policy.feature`
- **Deliverable**: `planset/architecture/PRIVACY_POLICY.md`
- **Status**: DONE
- **Notes**: Covers data collection, storage, retention periods (ranging from 90 days to 7 years), deletion rights, on-chain permanence disclosure, access controls by role, third-party sharing, and breach response.

### 5. Delivery Partner Integration Assessment
- **Priority**: Medium
- **Size**: S
- **Feature File**: `features/logistics/delivery-partners.feature`
- **Deliverable**: `planset/architecture/ADR-002-delivery-partners.md`
- **Status**: DONE
- **Notes**: Evaluated Uber Direct, DoorDash Drive, and standard carriers. Recommended: ShipEngine/EasyPost abstraction over UPS/FedEx/USPS as primary (nationwide, flexible, predictable cost), Uber Direct as premium same-day fallback. Abstraction layer interface designed.

## Exit Criteria
- [x] All 7 contract interfaces specified with function signatures, events, and invariants
- [x] Prisma schema covers all entities from DATA_MODEL.md
- [x] Threat model identifies top risks per system layer with mitigations
- [x] Privacy policy defines data lifecycle for all PII and evidence data
- [x] Delivery partner recommendation with rationale (ADR)

## Blockers
| Blocker | Feature | Status | Resolution |
|---------|---------|--------|------------|
| — | — | — | — |

## Metrics (updated at sprint end)
- Features completed: 5/5
- Test coverage: N/A (architecture sprint — no code)
- Tests passing: N/A
- Blockers encountered: 0
- Blockers resolved: 0
