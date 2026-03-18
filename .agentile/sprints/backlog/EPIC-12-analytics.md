# Epic 12: Analytics & Transparency

**Priority**: Low
**Size**: M
**Scope Reference**: SCOPE.md — Feature 13 (Analytics Dashboard)
**Phase**: 5 (Sprint 14)

## Stories

### Story 12.1: User Dashboard
As a user, I want to see my trade history so I trust the system.

**Acceptance Criteria**
- Dashboard shows contribution history, claims, reservations, deliveries
- Status of all in-progress items visible
- Historical data with timestamps

### Story 12.2: Pool Health Metrics
As a user, I want to see pool health so I know the system is active.

**Acceptance Criteria**
- Pool metrics show available inventory, recent fills, average dwell time
- Data updates from indexed on-chain events and warehouse events
- Public-facing (no auth required for pool-level metrics)

### Story 12.3: Operator Metrics
As an admin, I want SLA and operational metrics.

**Acceptance Criteria**
- Intake acceptance rate
- Average time to claim unlock
- Pool fill rate, dwell time
- Reservation-to-delivery success rate
- Dispute rate, courier completion rate
- Shrinkage rate
