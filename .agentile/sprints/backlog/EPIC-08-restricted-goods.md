# Epic 8: Restricted Goods & Safety

**Priority**: Medium
**Size**: M
**Scope Reference**: SCOPE.md — Feature 10 (Restricted Goods Enforcement)
**Phase**: 4 (Sprint 13)

## Stories

### Story 8.1: Submission Screening
As the protocol, prohibited and recalled items must be blocked at submission.

**Acceptance Criteria**
- Submission flow screens against restricted rule sets
- Blocked categories return clear rejection with reason
- Manual review items flagged for operator attention
- Recall database checked (CPSC or equivalent)

### Story 8.2: Warehouse Quarantine for Unsafe Items
As a warehouse operator, I want to quarantine items that pass submission but fail physical inspection.

**Acceptance Criteria**
- Warehouse can quarantine with mandatory reason
- Claim issuance blocked for quarantined items
- Admin receives alert for severe categories
- Quarantined items tracked separately with resolution workflow

### Story 8.3: Restricted Items Matrix
As an admin, I want to manage the restricted items rule set.

**Acceptance Criteria**
- Admin can add/edit/deactivate restriction rules
- Rules match against category patterns
- Rule types: blocked, manual_review, recall
- All changes audit-logged
