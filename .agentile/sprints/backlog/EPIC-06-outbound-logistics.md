# Epic 6: Outbound Logistics

**Priority**: High
**Size**: L
**Scope Reference**: SCOPE.md — Feature 8 (Outbound Shipping)
**Phase**: 2 (Sprint 9)

## Stories

### Story 6.1: Delivery Method Selection
As a claim holder, I want to choose shipping or local delivery so I receive my item conveniently.

**Acceptance Criteria**
- User sees eligible delivery methods for item and region
- Fees, tip option, ETA, and restrictions displayed
- Method selection persisted with reservation finalization
- Address validation before shipment creation

### Story 6.2: Shipment Tracking
As a user, I want to track my shipments in a visual timeline.

**Acceptance Criteria**
- Timeline shows all tracking events (created, picked up, in transit, delivered)
- Tracking updates from carrier API synced
- Estimated delivery shown and updated
- Push/email notification on status changes

### Story 6.3: Proof of Delivery
As the protocol, delivery must be confirmed with proof before the transaction is complete.

**Acceptance Criteria**
- Proof-of-delivery recorded (signature, photo, or QR scan)
- On-chain InventoryCommitment confirms delivery
- Proof hash anchored on-chain
- Delivered status visible to both parties
