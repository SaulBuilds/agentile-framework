# MILESTONES.md — Gradient Barter Protocol

## Phase 0: Definition & Validation (Sprints 1-2)

### Sprint 1: Product Framing & Rules
- Finalize product requirements
- Define pool taxonomy and allowed categories
- Define token/legal posture
- Warehouse SOP v1
- User flow maps
- Success metrics

**Exit**: PRD approved, category matrix approved, pool model approved

### Sprint 2: Architecture & Security Baselines
- System architecture diagrams
- Smart contract interfaces
- DB schema draft
- Threat model
- Privacy/data retention policy draft
- Delivery partner integration assessment

**Exit**: ADRs approved, threat model signed off, initial backlog ready

---

## Phase 1: Core Protocol & Backend Foundation (Sprints 3-6)

### Sprint 3: Solidity Core Scaffolding
- PoolRegistry, ItemReceiptNFT, ClaimToken contracts
- Role model and pause controls
- Unit tests

### Sprint 4: Settlement & Reservation Logic
- EscrowSettlement, InventoryCommitment contracts
- CEI hardening
- Invariant and fuzz tests

### Sprint 5: Backend Foundation
- Next.js app scaffold
- NestJS API scaffold
- Postgres + Prisma setup
- Auth/wallet link
- Event indexer scaffold

### Sprint 6: Submission & Labels
- Item submission APIs
- Media upload
- QR/label generation
- PDF print flow
- Packing instruction engine

---

## Phase 2: Warehouse & Claims (Sprints 7-9)

### Sprint 7: Warehouse Admin Console
- Inbound scan UI
- Grading workflow
- Accept/reject/quarantine flows
- Storage location assignment

### Sprint 8: Claim Activation & Inventory Browsing
- Claim activation jobs
- User claims dashboard
- Browse claimable inventory
- Reservation API and UI

### Sprint 9: Outbound Shipping
- Shipment creation
- Proof events model
- Delivery timeline UI
- Notifications

**MVP Complete** — Core barter loop functional end-to-end.

---

## Phase 3: Logistics Sidecar (Sprints 10-11)

### Sprint 10: Third-Party Courier Integration
- Uber Direct / DoorDash option abstraction
- Pickup request and status sync
- Delivery exceptions
- Fee calculation

### Sprint 11: Community Courier Mode
- Courier onboarding
- Task board
- QR pickup and POD flows
- Tip and payout ledger
- Safety reporting

---

## Phase 4: Safety, Disputes & Compliance (Sprints 12-13)

### Sprint 12: Dispute Center
- Dispute APIs
- Evidence upload
- Admin review queue
- Resolution workflows

### Sprint 13: Restricted Goods & Buy-on-Site
- Restricted goods rules engine
- Recall check workflow hooks
- Buy-on-site whitelist UI and admin
- Item conversion policies

---

## Phase 5: Analytics, Polish & Launch (Sprints 14-16)

### Sprint 14: Analytics & Transparency
- Pool health dashboards
- Dwell time analytics
- Operator SLA metrics
- Fraud/risk monitoring basics

### Sprint 15: Performance, Mobile & QA
- Native app parity on key flows
- Load testing
- Accessibility fixes
- Offline scan support
- Bug bash

### Sprint 16: Launch Readiness
- Penetration test fixes
- Warehouse training
- Incident runbooks
- Customer support macros
- Staged rollout and pilot launch

---

## Pilot Launch Target

- 1-2 warehouses
- 3-5 categories
- 3 value bands
- 1 region
- Claims non-transferable
- Third-party delivery first, community courier second
