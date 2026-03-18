# Epic 7: Disputes & Trust

**Priority**: Medium
**Size**: L
**Scope Reference**: SCOPE.md — Feature 9 (Dispute Center)
**Phase**: 4 (Sprint 12)

## Stories

### Story 7.1: Open Dispute
As a user, I want to dispute a grading or delivery outcome so unfair results can be reviewed.

**Acceptance Criteria**
- User can open dispute against approved object types (submission, receipt, inventory, claim, shipment, courier task)
- Required reason and evidence fields enforced
- SLA deadlines visible (configurable per dispute type)
- Dispute creates audit trail entry

### Story 7.2: Evidence Submission
As a dispute participant, I want to submit photos, videos, and text as evidence.

**Acceptance Criteria**
- Multiple evidence types supported (photo, video, text, document)
- Evidence stored in S3 with CID hash
- Both parties can submit evidence within window
- Evidence is immutable once submitted

### Story 7.3: Admin Resolution
As an admin, I want to review disputes and issue resolutions.

**Acceptance Criteria**
- Admin review queue shows open disputes sorted by SLA urgency
- Resolution options: refund claim, replacement, deny, goodwill credit
- Resolution requires reason capture
- Resolution triggers appropriate state changes (re-mint claim, release inventory, etc.)
- User notified of outcome
