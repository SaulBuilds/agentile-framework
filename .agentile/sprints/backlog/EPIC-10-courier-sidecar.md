# Epic 10: Courier Sidecar

**Priority**: Medium
**Size**: XL
**Scope Reference**: SCOPE.md — Out of scope for MVP core, Phase 3 (Sprints 10-11)
**Phase**: 3

## Stories

### Story 10.1: Third-Party Courier Integration
As the logistics service, I want to dispatch deliveries through 3P couriers (Uber Direct, DoorDash Drive).

**Acceptance Criteria**
- Abstraction layer for multiple courier providers
- Pickup request and status sync
- Delivery exceptions handled
- Fee calculation from provider APIs
- Tracking events from provider mapped to internal model

### Story 10.2: Community Courier Onboarding
As a prospective courier, I want to apply and get approved to deliver items.

**Acceptance Criteria**
- Courier profile with identity verification
- Insurance and background check integration
- Approval workflow for operators
- Courier cannot accept tasks until approved

### Story 10.3: Courier Task Board
As an approved courier, I want to see and accept eligible delivery tasks.

**Acceptance Criteria**
- Tasks show distance, payout, item constraints, time window
- Accept locks task to courier
- Pickup requires QR scan or PIN verification
- Drop-off requires approved proof method
- Payout released only after successful completion or admin resolution

### Story 10.4: Courier Safety
As a courier, I want safety protections during deliveries.

**Acceptance Criteria**
- No cash handling by default
- No high-risk or prohibited goods in courier tasks
- Precise addresses redacted until task acceptance
- In-app emergency reporting
- Time-window and route logging
- Escalation paths for unsafe interactions
