# Epic 3: Item Submission

**Priority**: High
**Size**: XL
**Scope Reference**: SCOPE.md — Features 3-4 (Item Submission, QR/Label Generation)
**Phase**: 1 (Sprint 6)

## Stories

### Story 3.1: Submit Item
As a contributor, I want to submit an item with photos and condition details so the platform can classify it.

**Acceptance Criteria**
- Submission requires mandatory media (min 2 photos) and condition fields
- User selects category and estimated band
- Rules engine flags prohibited categories before submission completes
- Submission receives a unique ID
- Draft state allows editing before final submit

### Story 3.2: QR Label & Packing Instructions
As a contributor, I want packing instructions and a printable QR label so my item can be tracked.

**Acceptance Criteria**
- System generates QR/label tied to submission ID
- User can print PDF label or save mobile QR
- Packing instructions are category-specific
- Every label includes fallback human-readable code
- Label includes warehouse address and handling notes

### Story 3.3: Media Upload
As a contributor, I want to upload multiple photos of my item from different angles.

**Acceptance Criteria**
- Signed URL upload to S3-compatible storage
- Min 2, max 10 photos per submission
- Accepted formats: JPEG, PNG, HEIC
- Max file size enforced (10MB per photo)
- Malware scanning on uploaded media
