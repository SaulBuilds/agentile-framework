# Sprint 7 — Warehouse Admin Console

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: COMPLETE

## Sprint Goal
Implement the warehouse intake backend: QR scan lookup, grading workflow (accept/reject/quarantine), intake queue, bin assignment, and role-based access. Also close out stub debt from prior sprints (indexer persistence, PDF generation).

## Items

### 0. Close Stub Debt
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: Fixed indexer event handler persistence (handleReceiptMinted, handleClaimMinted, handleClaimBurned, handleInventoryRegistered, block tracking via IndexerState model). Implemented real PDF label generation with pdfkit.

### 1. Warehouse Intake Scan API
- **Priority**: High
- **Size**: M
- **Feature File**: `features/warehouse/warehouse-sop.feature`
- **Status**: DONE
- **Notes**: POST /warehouse/intake/scan — scan QR/human-readable code, load submission details. GET /warehouse/intake/queue — list pending intake items. Role guard for OPERATOR/GRADER/ADMIN.

### 2. Grading Workflow API
- **Priority**: High
- **Size**: L
- **Feature File**: `features/warehouse/warehouse-sop.feature`
- **Status**: DONE
- **Notes**: POST /warehouse/intake/:submissionId/grade — accept (creates ItemReceipt + InventoryItem, activates claim), reject (updates status, records reason), quarantine (moves to quarantine, admin alert). Bin location assignment. Condition grade + final band.

### 3. Quarantine Management API
- **Priority**: High
- **Size**: M
- **Feature File**: `features/warehouse/warehouse-sop.feature`
- **Status**: DONE
- **Notes**: POST /warehouse/inventory/:id/quarantine — quarantine an inventory item. POST /warehouse/inventory/:id/quarantine/resolve — release or reject quarantined item. Requires reason and evidence.

### 4. Tests
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: Unit tests for WarehouseService covering all grading decisions, queue listing, quarantine flows, role validation. Tests for fixed indexer handlers and PDF generation.

## Exit Criteria
- [x] Warehouse scan loads submission by QR code or human-readable code
- [x] Intake queue lists SUBMITTED/RECEIVED items
- [x] Accept creates ItemReceipt + InventoryItem + activates claim
- [x] Reject updates status with reason
- [x] Quarantine blocks claim activation
- [x] Quarantine resolution works (release or reject)
- [x] Role guard restricts to OPERATOR/GRADER/ADMIN
- [x] Indexer handlers persist to database
- [x] PDF label generates real PDF
- [x] All tests pass

## Blockers
| Blocker | Feature | Status | Resolution |
|---------|---------|--------|------------|
| — | — | — | — |

## Metrics (updated at sprint end)
- Features completed: 5/5
- Test coverage: —%
- Tests passing: 93/93 (32 new + 61 existing)
- Blockers encountered: 0
- Blockers resolved: 0
