# Sprint 11 — Community Courier Mode

**Start Date**: 2026-03-06
**End Date**: 2026-03-06
**Status**: COMPLETED

## Sprint Goal
Implement community courier onboarding, task board, QR pickup and proof-of-delivery flows, earnings/payout ledger, and safety reporting.

## Items

### 1. Courier Onboarding & Profile
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: Courier approval gate via requireApprovedCourier() checks role=COURIER and kycStatus=APPROVED. Unapproved couriers blocked from all task board operations. Operators approve couriers through existing KYC workflow.

### 2. Courier Task Board & Lifecycle
- **Priority**: High
- **Size**: L
- **Status**: DONE
- **Notes**: Full lifecycle implemented in CourierService: GET /courier/tasks (paginated, region filter), POST accept (locks to courier), POST pickup (QR scan or PIN verification with SHA-256 proof hash), POST milestone (location + notes tracking), POST deliver (qr/signature/photo/pin proof methods). Status transitions: POSTED→ACCEPTED→PICKUP_VERIFIED→IN_TRANSIT_COURIER→DELIVERED_COURIER→COMPLETED. Delivery confirmation auto-updates Shipment and InventoryItem to DELIVERED.

### 3. CommunityProvider Adapter
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: CommunityProvider implements DeliveryProvider interface. createShipment generates COM- tracking numbers with pending task data. createCourierTask() posts task to board after shipment record exists. getRates returns $9.99 base community rate. getTrackingEvents maps CourierEvents to ProviderTrackingEvents. cancelShipment marks task as FAILED. Registered in DeliveryService with PrismaService injection, routed via deliveryMethod='community'.

### 4. Courier Earnings & Payout Ledger
- **Priority**: Medium
- **Size**: M
- **Status**: DONE
- **Notes**: GET /courier/earnings returns paginated earnings with summary (totalEarned, totalPending, completedTasks, pendingTasks). Aggregates fee + tip from completed and delivered tasks. Payout notification sent via NotificationsService on task completion.

### 5. Safety Reporting
- **Priority**: Medium
- **Size**: S
- **Status**: DONE
- **Notes**: Address redaction in task board (only city/region/state visible, precise street hidden until acceptance). Emergency reporting via POST /courier/tasks/:id/emergency with type, description, location. All admins/operators notified on emergency. CourierEvent EXCEPTION recorded.

### 6. Tests
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: 21 new tests in courier.service.spec.ts covering: task board listing with address redaction, unapproved/non-courier rejection, task acceptance with locking, pickup verification (QR + PIN), milestone reporting with status transitions, delivery confirmation with proof hash + shipment/inventory updates, task completion with payout notification, earnings with summary aggregation, emergency reporting with admin notification.

## Exit Criteria
- [x] Courier onboarding with approval workflow
- [x] Task board lists available tasks with filters
- [x] Full task lifecycle: accept → pickup → milestone → deliver
- [x] CommunityProvider implements DeliveryProvider
- [x] Earnings endpoint returns payout history
- [x] Safety: address redaction, emergency reporting
- [x] All tests pass

## Metrics (updated at sprint end)
- Features completed: 6/6
- Tests passing: 187/187
- Blockers encountered: 0
- Blockers resolved: 0
