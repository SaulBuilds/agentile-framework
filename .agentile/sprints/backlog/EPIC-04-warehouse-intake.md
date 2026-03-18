# Epic 4: Warehouse Intake & Processing

**Priority**: High
**Size**: XL
**Scope Reference**: SCOPE.md — Feature 5 (Warehouse Intake)
**Phase**: 2 (Sprint 7)

## Stories

### Story 4.1: Inbound Scan & Grading
As a warehouse operator, I want to scan inbound items and complete grading so claims unlock only after verification.

**Acceptance Criteria**
- Operator can scan QR label and load submission record
- Operator can accept, reject, or quarantine
- Operator records condition, photos, notes, and final pool band
- Inventory location (bin) is assigned
- Intake completion emits event to claim service
- On-chain ItemReceiptNFT minted on acceptance

### Story 4.2: Warehouse Admin Queue
As a warehouse operator, I want to see a queue of pending intake items.

**Acceptance Criteria**
- Queue shows all items awaiting grading at this warehouse
- Items sorted by arrival time
- Status indicators for urgent/overdue items
- Search by submission ID or QR code

### Story 4.3: Quarantine Flow
As a warehouse operator, I want to quarantine suspicious or damaged items.

**Acceptance Criteria**
- Quarantine action requires reason
- Quarantined items are blocked from claim activation
- Admin receives alert for quarantined items
- Quarantine can be resolved (release to pool or reject/return)
