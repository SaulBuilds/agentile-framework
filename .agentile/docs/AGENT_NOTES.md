# Agent Notes

> This is the agent's working memory. Update this file after every significant decision, blocker, or insight.
> This file persists between sessions and serves as continuity for the agent.

## Format
```
### [YYYY-MM-DD] [Topic]
[Note content]
```

---

### 2026-03-05 Framework Hardening — Gate Enforcement Overhaul

**Context**: During first use of the Agentile framework, the agent (Claude Code) skipped the INIT_WORKFLOW entirely. It read the repo, saw untracked files, committed and pushed them — but never detected the empty planset or ran initialization. The framework's guidance was too passive.

**Root cause**: `AGENT_ENTRY.md` described what to do as suggestions, not hard gates. An agent could acknowledge the framework and still proceed directly to the user's request. The rule files (`BDD_RULES.md`, `TDD_RULES.md`, etc.) similarly lacked enforcement language.

**Changes made**:
- `AGENT_ENTRY.md`: Rewritten with MANDATORY PRE-FLIGHT CHECK, HARD GATE system, and guided discovery protocol
- `rules/CORE_RULES.md`: Added Rule 0 (Initialize Before Everything) as highest-priority rule. All rules now have explicit GATE checkpoints.
- `rules/BDD_RULES.md`: Added Feature Completeness Gate, Feature Lifecycle Gate, Tagging Gate
- `rules/TDD_RULES.md`: Added gates at every RED/GREEN/REFACTOR phase transition. Added Prohibited Practices section. Added Test Completeness Gate.
- `rules/DOCUMENTATION_RULES.md`: Added gates for every documentation event. Documentation debt treated with same urgency as test debt.
- `rules/GIT_RULES.md`: Added Pre-Commit Gate, Merge Gate, Sprint Merge Gate. Added Incomplete Work Protocol. Prohibited skipped/pending tests in commits.
- `workflows/INIT_WORKFLOW.md`: Every step now has pass/fail gate with verification criteria
- `workflows/FEATURE_WORKFLOW.md`: Entry gate + phase transition gates + exit gate with full traceability chain verification
- `workflows/SPRINT_WORKFLOW.md`: Entry gate, planning gate, execution tracking gate, review gates, completion checklist
- `workflows/RETROFIT_WORKFLOW.md`: Entry gate, audit gate, planset gate, safety-net gate, backlog gate
- `workflows/REVIEW_WORKFLOW.md`: Every section is an enforceable gate with failure protocol

**New files**:
- `docs/EVALUATION.md`: 7 benchmarks for testing agent compliance (cold start detection, guided discovery, feature traceability, gate enforcement, sprint boundary respect, review completeness, retrofit detection)
- `templates/AGENT_HOOKS.template.md`: Hook file templates for Claude, Cursor, Copilot, Windsurf

**Key design decision**: Gates use "DO NOT PROCEED" language rather than "should" or "recommended." The framework now treats initialization like a hard dependency — nothing downstream works without it. This mirrors how CI/CD pipelines work: a failed gate blocks the pipeline.

**Benchmark for validation**: The failure case (Benchmark 1 in EVALUATION.md) directly mirrors what happened in this session. Future agents should be tested against all 7 benchmarks.

### 2026-03-06 Sprint 1 — Product Framing & Rules (COMPLETE)

**Context**: First sprint of Gradient Barter Protocol. Definition sprint — no code, all planning artifacts.

**Deliverables produced**:
- `planset/executive-summary/POOL_TAXONOMY.md` — 6 hues, 5 bands, 4 quality tiers, regional scoping. V1 launches with 4 hues × 3 bands.
- `planset/executive-summary/ITEM_POLICY.md` — 10 allowed categories, 11 excluded, 5 restricted patterns. Pre-submission declarations required.
- `planset/architecture/ADR-001-token-posture.md` — Claims are non-transferable, non-cash-redeemable barter entitlements. Language standards defined.
- `planset/executive-summary/WAREHOUSE_SOP.md` — Full 10-section SOP: receiving, grading, storage, quarantine, outbound, high-value protocol, role separation, incidents.
- `planset/executive-summary/SUCCESS_METRICS.md` — 10 KPIs with pilot targets, health indicators, reporting cadence.
- 6 Gherkin feature files in `features/protocol/` and `features/warehouse/` and `features/ux/`

**Decisions**:
- V1 launches with reduced scope: 4 hues (green, blue, amber, teal), 3 bands (1-3), 2 quality tiers (new, used-good), 1 region
- Band 5 ($500+) requires KYC — deferred to after ops mature
- Collectible grading tier deferred to v2
- Claims are soul-bound in v1 (no transfers, no marketplace)
- Warehouse SOP requires dual-control for Band 4-5 items

**Next**: Sprint 2 — Architecture & Security Baselines

### 2026-03-06 Sprint 2 — Architecture & Security Baselines (COMPLETE)

**Deliverables produced**:
- `planset/architecture/CONTRACT_INTERFACES.md` — 7 Solidity interfaces with full function sigs, events, access control, CEI annotations, 5 critical invariants
- `planset/architecture/PRISMA_SCHEMA_DRAFT.md` — 25 Prisma models, 18 enums, all relations, composite indexes
- `planset/architecture/THREAT_MODEL.md` — STRIDE analysis, 22 threats across 5 layers, all mitigated, top 5 residual risks
- `planset/architecture/PRIVACY_POLICY.md` — Full data lifecycle policy with retention periods and breach response
- `planset/architecture/ADR-002-delivery-partners.md` — ShipEngine/EasyPost as primary, Uber Direct as premium fallback
- 5 Gherkin feature files in `features/contracts/`, `features/backend/`, `features/security/`, `features/compliance/`, `features/logistics/`

**Decisions**:
- Standard carriers via ShipEngine abstraction as primary delivery partner (not Uber Direct) — nationwide coverage, predictable cost, flexible package sizes
- Abstraction layer interface designed so providers can be swapped without touching claim/reservation logic
- EscrowSettlement is the most critical contract — handles claim locking, reservation timeout, and finalization. Heaviest invariant testing needed here.
- Prisma schema uses event-sourced pattern for inventory status (InventoryStatusEvent table)
- Privacy policy requires on-chain permanence disclosure during onboarding

**Next**: Sprint 3 — Solidity Core Scaffolding

### 2026-03-06 Sprint 3 — Solidity Core Scaffolding (COMPLETE)

**Deliverables produced**:
- `contracts/src/GradientRoles.sol` — 8 role constants shared across all contracts
- `contracts/src/PoolRegistry.sol` — Pool management with CRUD, pause, access control
- `contracts/src/ItemReceiptNFT.sol` — Soul-bound ERC-721, intake status, grading, quarantine
- `contracts/src/PoolClaimToken.sol` — Non-transferable ERC-1155, transfer policy, allowlist support
- `contracts/test/PoolRegistry.t.sol` — 25 tests including fuzz
- `contracts/test/ItemReceiptNFT.t.sol` — 24 tests
- `contracts/test/PoolClaimToken.t.sol` — 25 tests including fuzz

**Test results**: 74/74 passing, 0 failures, 2 fuzz suites (256 runs each)

**Decisions**:
- Soul-bound implementation uses `_update` override — cleaner than custom transfer hooks
- Condition grade uses 1-10 scale (simpler than string grades, sufficient for v1)
- PoolClaimToken uses pool ID as ERC-1155 token ID — clean mapping, no extra registry
- Transfer policy supports future v2 allowlist mode but defaults to fully blocked
- All contracts use all three OpenZeppelin guards: AccessControl + Pausable + ReentrancyGuard

**Next**: Sprint 4 — Settlement & Reservation Logic (EscrowSettlement, InventoryCommitment, CEI hardening, invariant/fuzz tests)

### 2026-03-06 Sprint 4 — Settlement & Reservation Logic (COMPLETE)

**Deliverables produced**:
- `contracts/src/InventoryCommitment.sol` — State machine: Available → Reserved → Outbound → Delivered, plus Quarantined. Tracks `_availableCount` per pool.
- `contracts/src/EscrowSettlement.sol` — Core settlement orchestration. Integrates PoolClaimToken + InventoryCommitment. Reservation timeout (default 24h, min 1h, max 7d). Permissionless expiry.
- `contracts/src/DisputeResolver.sol` — Lightweight on-chain dispute tracking. 5 object types, 4 resolution types, evidence hash array, duplicate prevention.
- `contracts/test/InventoryCommitment.t.sol` — 36 tests including 2 fuzz suites
- `contracts/test/EscrowSettlement.t.sol` — 41 tests including 2 fuzz suites, 3 full lifecycle tests
- `contracts/test/DisputeResolver.t.sol` — 30 tests including 1 fuzz suite
- `contracts/test/Invariants.t.sol` — 5 invariant tests (256 runs, ~3840 calls each, 0 reverts)

**Test results**: 186/186 passing, 0 failures, 5 fuzz suites + 5 invariant suites

**5 Critical Invariants verified**:
1. Claims in circulation never exceed registered inventory
2. Available count never exceeds total registered
3. Finalized count equals delivered count (1:1 mapping)
4. Expired/cancelled reservations correctly release resources (mint/burn accounting matches balances)
5. No double-settlement (settled ≤ reserved)

**Decisions**:
- EscrowSettlement holds immutable references to PoolClaimToken and InventoryCommitment — no proxy pattern needed for v1
- `expireReservation` is permissionless (no role check) — anyone can clean up expired reservations
- Invariant handler exercises all 5 state transitions (register+unlock, consume, cancel, deliver+finalize, expire) with random actor selection
- CEI pattern confirmed on all state-changing functions: checks first, state mutations second, external calls last

**Next**: Sprint 5 — Backend Foundation

### 2026-03-06 Sprint 5 — Backend Foundation (COMPLETE)

**Deliverables produced**:
- Monorepo scaffold: `apps/api` (NestJS), `apps/web` (Next.js + Tailwind v4), `packages/shared-types`
- `apps/api/prisma/schema.prisma` — 25 models, 20 enums, all relations, validates with `prisma format` + `prisma generate`
- Auth module: register (email/phone + bcrypt), login, JWT strategy, guards
- Wallet module: link (nonce generation), verify (EIP-191 via viem), list
- Event indexer scaffold: handlers for PoolCreated, ReceiptMinted, ClaimMinted/Burned, InventoryRegistered
- 4 test files: 19/19 passing

**Decisions**:
- Monorepo uses npm workspaces (no Turborepo/Lerna — simpler for v1)
- Tailwind v4 with @tailwindcss/postcss plugin (new config-free approach)
- Prisma client generated into node_modules (standard approach, no custom output)
- Auth uses generic `BadRequestException` for missing email+phone (clear error without leaking info)
- Wallet nonce cleared after successful verification (one-time use)
- Event indexer is synchronous scaffold — BullMQ queue integration deferred to when RPC is configured

**Next**: Sprint 6 — Submission & Labels (item submission APIs, media upload, QR/label generation)

### 2026-03-06 Sprint 6 — Submission & Labels (COMPLETE)

**Deliverables produced**:
- `apps/api/src/submissions/submissions.service.ts` — CRUD + submit flow with status transitions (DRAFT → SUBMITTED/QUARANTINED/REJECTED)
- `apps/api/src/submissions/submissions.controller.ts` — REST endpoints with JWT guard
- `apps/api/src/submissions/screening.service.ts` — Pattern-matching rules engine (BLOCKED, RECALL, MANUAL_REVIEW) with wildcard support
- `apps/api/src/submissions/media.service.ts` — File upload validation (type, size, count), local storage for dev
- `apps/api/src/submissions/labels.service.ts` — QR data generation, human-readable codes (GBP-XXXX-XXXX), category-specific packing instructions, PDF placeholder
- `apps/api/src/submissions/labels.controller.ts` + `media.controller.ts` — REST endpoints
- `apps/api/src/submissions/dto/` — CreateSubmissionDto, SubmitItemDto (5 declarations), QuerySubmissionsDto
- 4 test files: submissions.service.spec.ts (12), screening.service.spec.ts (8), labels.service.spec.ts (8), media.service.spec.ts (7)

**Test results**: 61/61 passing (35 new submission tests + 26 existing)

**Bug fixed**: `matchPattern()` wildcard matching — pattern `*ambiguous*` (wildcards at both ends) incorrectly entered the `endsWith('*')` branch. Fixed by checking for wildcards at both ends first.

**Decisions**:
- Submit requires all 5 declarations true + minimum 2 media files
- Screening runs on submit, not on draft creation — allows users to complete their submission before screening
- QR human-readable code format: GBP-XXXX-XXXX (hex), stored in QrLabel table
- Packing instructions are category-specific (power tools, hand tools, consumer electronics, gaming consoles, small appliances) with default fallback
- PDF generation uses text buffer placeholder — real PDF via pdfkit deferred to when visual design is ready
- Media storage uses local `uploads/` directory for dev; S3 integration deferred

**Next**: Sprint 7 — Warehouse Admin Console

### 2026-03-06 Sprint 7 — Warehouse Admin Console (COMPLETE)

**Stub debt closed**:
- `apps/api/src/indexer/indexer.service.ts` — All 4 event handlers now persist to database: handleReceiptMinted updates on-chain token/tx, handleClaimMinted activates pending claims, handleClaimBurned consumes active claims, handleInventoryRegistered creates inventory items. Block tracking via new IndexerState model.
- `apps/api/src/submissions/labels.service.ts` — Real PDF generation via pdfkit. Labels controller now serves `application/pdf` with proper content-type.
- `apps/api/prisma/schema.prisma` — Added IndexerState model for key-value block tracking.

**Deliverables produced**:
- `apps/api/src/warehouse/warehouse.service.ts` — Full warehouse intake service: QR scan lookup, intake queue, grading workflow (accept/reject/quarantine), quarantine management (quarantine + resolve).
- `apps/api/src/warehouse/warehouse.controller.ts` — REST endpoints with JWT + Roles guards.
- `apps/api/src/warehouse/warehouse.module.ts` — Module wiring.
- `apps/api/src/warehouse/dto/` — ScanIntakeDto, GradeSubmissionDto (accept/reject/quarantine with finalBand, conditionGrade, binLocation), QuarantineDto, ResolveQuarantineDto.
- `apps/api/src/auth/guards/roles.guard.ts` — Role-based access guard with `@Roles()` decorator.
- `apps/api/src/warehouse/warehouse.service.spec.ts` — 22 tests covering scan, queue, all 3 grading decisions, quarantine, and quarantine resolution.
- Updated `apps/api/src/indexer/indexer.service.spec.ts` — 14 tests covering all event handlers + block tracking.

**Test results**: 93/93 passing (32 new + 61 existing)

**Decisions**:
- Accept creates ItemReceipt + InventoryItem + ACTIVE Claim in a single transaction
- Grading requires OPERATOR/GRADER/ADMIN role via RolesGuard
- Quarantine supports two resolutions: RELEASE (back to AVAILABLE) and REJECT (RETURNED status)
- OperatorAction table logs every grading and quarantine decision with metadata
- InventoryStatusEvent table tracks all inventory status transitions with actor
- Scan endpoint searches by QR human-readable code first, then falls back to submission ID
- Intake queue returns SUBMITTED and RECEIVED status items ordered by arrival time

**Next**: Sprint 8 — Claim Activation & Inventory Browsing

### 2026-03-06 Sprint 8 — Claim Activation & Inventory Browsing (COMPLETE)

**Deliverables produced**:
- `apps/api/src/pools/pools.service.ts` — Pool listing (public, filterable by hue/band/region/qualityTier/status), pool details with available inventory + active claim counts, inventory browsing (auth + active claim required, includes condition/photos/grading details).
- `apps/api/src/pools/pools.controller.ts` — Public GET /pools, GET /pools/:id. Auth-only GET /pools/:id/inventory.
- `apps/api/src/claims/claims.service.ts` — Claim listing (filterable by status/poolId), claim details with available actions, full reservation lifecycle: reserve (24h expiry), cancel (release item + delete consumption), finalize (consume claim + create outbound shipment).
- `apps/api/src/claims/claims.controller.ts` — GET /claims, GET /claims/:id, POST /claims/:claimId/reserve, POST /claims/:claimId/reserve/cancel, POST /claims/:claimId/reserve/finalize.
- `apps/api/src/claims/dto/` — ReserveDto (inventoryId), FinalizeDto (deliveryMethod, shippingAddress).
- Updated `apps/api/src/warehouse/warehouse.service.ts` — Claims now created as ACTIVE with activatedAt at grading time.

**Test results**: 121/121 passing (28 new — 7 pools + 21 claims)

**Decisions**:
- Pool listing and detail endpoints are public (no auth required). Inventory browsing requires auth + active claim in the pool.
- Reserve uses ClaimConsumption to link claim ↔ inventory. On cancel, the consumption record is deleted. On finalize, it's preserved with consumedAt timestamp.
- 24-hour reservation timeout enforced at finalize time (if reservation expired, finalize throws BadRequestException).
- Finalize creates OUTBOUND shipment record. Actual carrier integration deferred to Sprint 9.
- Claims created as ACTIVE at grading time (not PENDING) — enables offline-first flow without on-chain dependency. On-chain activation via indexer will update onChainTokenId when available.

**Next**: Sprint 9 — Outbound Shipping

### 2026-03-06 Sprint 9 — Outbound Shipping (COMPLETE) — MVP BARTER LOOP COMPLETE

**Deliverables produced**:
- `apps/api/src/shipments/shipments.service.ts` — Shipment listing (filter by direction/status, shows both inbound submissions and outbound reservations), shipment detail with tracking timeline, tracking event management (auto-updates shipment status), proof-of-delivery confirmation (SHA-256 proof hash, updates inventory to DELIVERED).
- `apps/api/src/shipments/shipments.controller.ts` — GET /shipments, GET /shipments/:id, GET /shipments/:id/tracking, POST /shipments/:id/tracking (operator-only), POST /shipments/:id/deliver.
- `apps/api/src/shipments/dto/` — AddTrackingDto, DeliverDto (proofMethod: qr/signature/photo/pin, proofData).
- `apps/api/src/notifications/notifications.service.ts` — Notification listing (filter by read/unread), mark as read, create (callable by other services).
- `apps/api/src/notifications/notifications.controller.ts` — GET /notifications, PATCH /notifications/:id/read.

**Test results**: 143/143 passing (22 new — 14 shipments + 8 notifications)

**Decisions**:
- Shipment ownership determined by submission.userId (inbound) or inventory.reservedById (outbound)
- Tracking events auto-update shipment status via statusMap (picked_up → PICKED_UP, in_transit → IN_TRANSIT, exception → EXCEPTION)
- Delivery proof hashed as SHA-256 of `${proofMethod}:${proofData}:${shipmentId}` — anchored on proofHash field
- Outbound delivery confirmation also updates InventoryItem to DELIVERED and creates InventoryStatusEvent
- NotificationsService.create() accepts channel param (EMAIL/SMS/PUSH), defaults to PUSH
- Carrier API integration (ShipEngine/EasyPost) deferred to Sprint 10

**MVP Complete**: The full barter loop is now functional end-to-end:
1. Contributor submits item → uploads media → generates QR label
2. Ships to warehouse → operator scans QR → grades (accept/reject/quarantine)
3. Acceptance creates receipt + inventory item + active claim
4. Claim holder browses pool inventory → reserves item (24h hold)
5. Finalizes reservation → claim consumed → outbound shipment created
6. Tracking events added → delivery confirmed with proof → inventory DELIVERED

**Next**: Phase 3 — Logistics Sidecar (Sprint 10: Third-Party Courier Integration)
