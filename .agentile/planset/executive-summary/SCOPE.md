# SCOPE.md — Gradient Barter Protocol

## In-Scope Features (MVP — Prioritized)

### Priority: High (Must Ship)

1. **Identity & Wallet Linking** — User registration (email/phone), EVM wallet connection, nonce-based wallet verification
2. **Pool Explorer** — Browse pool hues, value bands, category examples, availability, filter by category/band/condition
3. **Item Submission** — Photo upload, condition details, category selection, estimated band, prohibited item screening, unique submission ID
4. **QR/Label Generation** — Printable PDF labels, mobile QR, category-specific packing instructions, fallback human-readable codes
5. **Warehouse Intake** — Inbound scan, condition grading, accept/reject/quarantine, final pool band assignment, inventory location, claim service event emission
6. **Claim Activation** — Automatic claim unlock on verified intake, active claim status, pool ID and withdrawal rights display
7. **Same-Pool Reservation** — Claim balance check, item reservation with timeout, claim consumption on finalization
8. **Outbound Shipping** — Delivery method selection, fee/ETA display, tracking timeline, proof-of-delivery recording

### Priority: Medium (Should Ship)

9. **Dispute Center** — Open disputes against grading/delivery, evidence upload, SLA deadlines, resolution workflows (refund claim, replacement, deny, goodwill credit)
10. **Restricted Goods Enforcement** — Submission screening against restricted rule sets, warehouse quarantine, claim blocking for rejected items, admin alerts
11. **Admin Console** — Pool configuration, threshold management, pause controls (pool-level and global), role-based access, audit logging

### Priority: Low (Nice to Have for v1)

12. **Buy-on-Site Whitelist** — Public whitelist of items the platform will buy on receipt, condition windows, warehouse conversion workflow
13. **Analytics Dashboard** — Contribution history, claims, reservations, pool health metrics, average dwell time
14. **Notification System** — Email, SMS, push notifications for submission status, claim unlock, delivery updates

## Explicitly Out of Scope (v1)

| Feature | Reason |
|---------|--------|
| Open claim marketplace / secondary trading | Increases speculation, regulatory risk |
| Algorithmic liquidity incentives | Premature financialization |
| Broad token rewards | Not needed for MVP validation |
| Complex DAO governance | Operational complexity too high for pilot |
| International shipping | Logistics complexity; start with 1 region |
| Luxury item authentication | Requires specialized ops maturity |
| Cross-pool borrowing or leverage | Against core design principle |
| Community courier mode | Build after 3P delivery is stable (Phase 3) |
| Mobile native app | Web-first MVP; React Native in Phase 5 |

## MVP vs Future Phases

### MVP (Phases 0-2, Sprints 1-9)
Core barter loop: submit → verify → claim → reserve → deliver. One region, 3-5 categories, 3 value bands, 1-2 warehouses, 3P delivery only.

### Phase 3 (Sprints 10-11)
Logistics sidecar: 3P courier integration, community courier mode, task board, safety reporting.

### Phase 4 (Sprints 12-13)
Safety & compliance: dispute center (full), restricted goods engine, buy-on-site program.

### Phase 5 (Sprints 14-16)
Polish & launch: analytics, mobile native app, performance, QA, pen testing, pilot launch.
