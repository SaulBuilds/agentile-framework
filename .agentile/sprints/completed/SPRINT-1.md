# Sprint 1 — Product Framing & Rules

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: ACTIVE

## Sprint Goal
Finalize the product definition, pool taxonomy, allowed categories, token/legal posture, and warehouse SOPs so that development can begin with a validated foundation.

## Items

### 1. Define Pool Taxonomy & Category Matrix
- **Priority**: High
- **Size**: M
- **Feature File**: `features/protocol/pool-taxonomy.feature`
- **Deliverable**: `planset/executive-summary/POOL_TAXONOMY.md`
- **Status**: DONE
- **Notes**: 6 hues defined (4 active for v1 launch), 5 bands (3 active for v1), 4 quality tiers (2 active for v1), regional scoping defined. V1 launches with ~8-12 pools.

### 2. Define Item Policy & Restricted Goods Matrix
- **Priority**: High
- **Size**: M
- **Feature File**: `features/protocol/item-policy.feature`
- **Deliverable**: `planset/executive-summary/ITEM_POLICY.md`
- **Status**: DONE
- **Notes**: 10 allowed categories, 11 excluded categories, 5 restricted patterns requiring manual review. Pre-submission declaration required. Recall screening integrated.

### 3. Define Token & Legal Posture
- **Priority**: High
- **Size**: S
- **Feature File**: `features/protocol/token-posture.feature`
- **Deliverable**: `planset/architecture/ADR-001-token-posture.md`
- **Status**: DONE
- **Notes**: Claims are non-transferable, non-cash-redeemable barter entitlements. Language standards defined. Compliance disclosures required at onboarding.

### 4. Draft Warehouse SOP v1
- **Priority**: High
- **Size**: L
- **Feature File**: `features/warehouse/warehouse-sop.feature`
- **Deliverable**: `planset/executive-summary/WAREHOUSE_SOP.md`
- **Status**: DONE
- **Notes**: Full SOP covering receiving, grading, storage, quarantine, outbound, high-value protocol, role separation, and incident procedures. 10 sections.

### 5. Create User Flow Maps
- **Priority**: Medium
- **Size**: M
- **Feature File**: `features/ux/user-flows.feature`
- **Deliverable**: `features/ux/user-flows.feature` (flows documented as Gherkin scenarios)
- **Status**: DONE
- **Notes**: All 9 core user flows mapped as scenarios: onboard, submit, label, ship, track, claim, browse, reserve, confirm.

### 6. Define Success Metrics & KPIs
- **Priority**: Medium
- **Size**: S
- **Feature File**: `features/protocol/success-metrics.feature`
- **Deliverable**: `planset/executive-summary/SUCCESS_METRICS.md`
- **Status**: DONE
- **Notes**: 10 KPIs with pilot targets, health indicators (green/yellow/red), measurement infrastructure, and reporting cadence defined.

## Exit Criteria
- [x] Pool taxonomy approved (hues, bands, tiers, regions)
- [x] Category matrix approved (allowed + excluded + restricted)
- [x] Token/legal posture documented as ADR
- [x] Warehouse SOP v1 drafted and reviewed
- [x] User flow maps for all 9 core flows
- [x] KPIs defined with pilot targets

## Blockers
| Blocker | Feature | Status | Resolution |
|---------|---------|--------|------------|
| — | — | — | — |

## Metrics (updated at sprint end)
- Features completed: 6/6
- Test coverage: N/A (definition sprint — no code)
- Tests passing: N/A
- Blockers encountered: 0
- Blockers resolved: 0
