# Sprint 8 — Claim Activation & Inventory Browsing

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: COMPLETE

## Sprint Goal
Implement pool browsing, user claims dashboard, inventory browsing for claim holders, and the full reservation lifecycle (reserve → cancel/finalize). Activate claims at grading time for offline-first flow.

## Items

### 1. Pools Module
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: GET /pools (public, filterable by hue/band/region/status), GET /pools/:id (public), GET /pools/:id/inventory (auth + active claim required). PoolsService + PoolsController.

### 2. Claims Module
- **Priority**: High
- **Size**: L
- **Status**: DONE
- **Notes**: GET /claims (user's claims, filterable by status/poolId), GET /claims/:id (claim details with pool info). ClaimsService + ClaimsController.

### 3. Reservation Lifecycle
- **Priority**: High
- **Size**: XL
- **Status**: DONE
- **Notes**: POST /claims/:claimId/reserve (reserve inventory item, 24h expiry), POST /claims/:claimId/reserve/cancel (release item + claim), POST /claims/:claimId/reserve/finalize (consume claim, create outbound shipment). Atomic claim+inventory state transitions.

### 4. Claim Activation at Grading
- **Priority**: High
- **Size**: S
- **Status**: DONE
- **Notes**: Updated warehouse grading to create claims as ACTIVE with activatedAt timestamp instead of PENDING.

### 5. Tests
- **Priority**: High
- **Size**: L
- **Status**: DONE
- **Notes**: pools.service.spec.ts (7 tests), claims.service.spec.ts (21 tests). Covers pool listing/filtering, inventory browsing with claim check, claim listing, full reservation lifecycle, all edge cases.

## Exit Criteria
- [x] Public pool listing with filters works
- [x] Pool inventory browsable by claim holders only
- [x] User can list and view their claims
- [x] Reserve creates reservation with 24h expiry
- [x] Cancel releases both claim and inventory
- [x] Finalize consumes claim and creates outbound shipment
- [x] Claims activated at grading time
- [x] All tests pass

## Metrics (updated at sprint end)
- Features completed: 5/5
- Tests passing: 121/121 (28 new + 93 existing)
- Blockers encountered: 0
- Blockers resolved: 0
