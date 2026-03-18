# Sprint 6 — Submission & Labels

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: COMPLETE

## Sprint Goal
Implement the item submission flow: CRUD APIs, media upload via signed URLs, restricted item screening, QR/label generation, PDF label download, and category-specific packing instructions.

## Items

### 1. Submission CRUD APIs
- **Priority**: High
- **Size**: L
- **Feature File**: `features/protocol/item-policy.feature`
- **Status**: DONE
- **Notes**: POST /submissions (create draft), GET /submissions/:id, GET /submissions (list), POST /submissions/:id/submit (finalize with screening). Validation, status transitions, user ownership checks.

### 2. Media Upload Module
- **Priority**: High
- **Size**: M
- **Feature File**: `features/protocol/item-policy.feature`
- **Status**: DONE
- **Notes**: POST /submissions/:id/media. Min 2, max 10 photos. JPEG/PNG/HEIC. Max 10MB. Local storage for dev, S3-compatible for prod. File validation.

### 3. Restricted Item Screening
- **Priority**: High
- **Size**: M
- **Feature File**: `features/protocol/item-policy.feature`
- **Status**: DONE
- **Notes**: Rules engine checks category against RestrictedItemRule table. Auto-block excluded categories, flag restricted for manual review. Pre-submission declarations validation.

### 4. QR Label & Packing Instructions
- **Priority**: High
- **Size**: M
- **Feature File**: `features/protocol/item-policy.feature`
- **Status**: DONE
- **Notes**: GET /submissions/:id/label. QR data generation, human-readable code, category-specific packing instructions. GET /submissions/:id/label/pdf for printable PDF.

## Exit Criteria
- [x] Submission CRUD works end-to-end (create → upload media → submit)
- [x] Restricted item screening blocks excluded categories
- [x] Restricted items flagged for manual review
- [x] QR label generated with human-readable fallback
- [x] PDF label endpoint returns valid PDF
- [x] Packing instructions vary by category
- [x] All tests pass

## Blockers
| Blocker | Feature | Status | Resolution |
|---------|---------|--------|------------|
| — | — | — | — |

## Metrics (updated at sprint end)
- Features completed: 4/4
- Test coverage: —%
- Tests passing: 61/61 (35 new + 26 existing)
- Blockers encountered: 1 (wildcard pattern matching bug)
- Blockers resolved: 1
