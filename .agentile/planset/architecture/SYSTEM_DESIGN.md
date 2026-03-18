# SYSTEM_DESIGN.md — Gradient Barter Protocol

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          CLIENT LAYER                               │
│                                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐     │
│  │  Web App      │  │  Mobile App  │  │  Admin Console         │     │
│  │  (Next.js)    │  │  (React      │  │  (Next.js /            │     │
│  │              │  │   Native)    │  │   warehouse ops)       │     │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬─────────────┘     │
│         │                 │                     │                   │
│         └─────────────────┼─────────────────────┘                   │
│                           │                                         │
│                     ┌─────▼─────┐                                   │
│                     │  API GW   │                                   │
│                     └─────┬─────┘                                   │
└───────────────────────────┼─────────────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────────────┐
│                     BACKEND LAYER (NestJS)                          │
│                           │                                         │
│  ┌────────────┐  ┌────────▼───────┐  ┌──────────────┐              │
│  │ Identity   │  │ Submission     │  │ Claim        │              │
│  │ Service    │  │ Service        │  │ Service      │              │
│  │            │  │                │  │              │              │
│  │ - auth     │  │ - intake       │  │ - activation │              │
│  │ - wallet   │  │ - media        │  │ - browsing   │              │
│  │ - KYC      │  │ - classify     │  │ - reservation│              │
│  └────────────┘  │ - label/QR     │  │ - consumption│              │
│                  └────────────────┘  └──────────────┘              │
│                                                                     │
│  ┌────────────┐  ┌────────────────┐  ┌──────────────┐              │
│  │ Warehouse  │  │ Logistics      │  │ Dispute      │              │
│  │ Service    │  │ Service        │  │ Service      │              │
│  │            │  │                │  │              │              │
│  │ - scan     │  │ - pickup       │  │ - open       │              │
│  │ - grade    │  │ - delivery     │  │ - evidence   │              │
│  │ - store    │  │ - proof events │  │ - resolve    │              │
│  │ - quarant. │  │ - 3P integrate │  │ - SLA        │              │
│  └────────────┘  └────────────────┘  └──────────────┘              │
│                                                                     │
│  ┌────────────┐  ┌────────────────┐  ┌──────────────┐              │
│  │ Pool       │  │ Compliance     │  │ Notification │              │
│  │ Engine     │  │ Service        │  │ Service      │              │
│  │            │  │                │  │              │              │
│  │ - taxonomy │  │ - restricted   │  │ - email      │              │
│  │ - bands    │  │   goods        │  │ - SMS        │              │
│  │ - rules    │  │ - recall check │  │ - push       │              │
│  │ - analytics│  │ - audit logs   │  │              │              │
│  └────────────┘  └────────────────┘  └──────────────┘              │
│                                                                     │
└───────────────────────────┬─────────────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────────────┐
│                      DATA LAYER                                     │
│                           │                                         │
│  ┌────────────┐  ┌────────▼───────┐  ┌──────────────┐              │
│  │ PostgreSQL │  │ Redis          │  │ S3-compat    │              │
│  │ (Prisma)   │  │ (BullMQ)       │  │ Object Store │              │
│  │            │  │                │  │              │              │
│  │ - users    │  │ - job queues   │  │ - photos     │              │
│  │ - pools    │  │ - rate limits  │  │ - labels     │              │
│  │ - items    │  │ - cache        │  │ - proofs     │              │
│  │ - claims   │  │ - sessions     │  │ - evidence   │              │
│  │ - disputes │  │                │  │              │              │
│  └────────────┘  └────────────────┘  └──────────────┘              │
│                                                                     │
└───────────────────────────┬─────────────────────────────────────────┘
                            │
┌───────────────────────────┼─────────────────────────────────────────┐
│                     CHAIN LAYER                                     │
│                           │                                         │
│  ┌────────────┐  ┌────────▼───────┐  ┌──────────────┐              │
│  │ Event      │  │ Smart          │  │              │              │
│  │ Indexer    │◄─┤ Contracts      │  │              │              │
│  │ (viem)     │  │                │  │              │              │
│  │            │  │ - PoolRegistry │  │              │              │
│  │ - sync     │  │ - ItemReceipt  │  │              │              │
│  │ - index    │  │   NFT (721)    │  │              │              │
│  │ - replay   │  │ - PoolClaim    │  │              │              │
│  │            │  │   Token (1155) │  │              │              │
│  └────────────┘  │ - Escrow       │  │              │              │
│                  │   Settlement   │  │              │              │
│                  │ - Inventory    │  │              │              │
│                  │   Commitment   │  │              │              │
│                  │ - Dispute      │  │              │              │
│                  │   Resolver     │  │              │              │
│                  │ - Courier      │  │              │              │
│                  │   Rewards      │  │              │              │
│                  └────────────────┘  │              │              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Responsibilities

### Client Layer
- **Web App (Next.js)**: Primary user interface — pool browsing, item submission, claim management, delivery tracking
- **Mobile App (React Native)**: Same core flows optimized for mobile; QR scanning for couriers and warehouse ops
- **Admin Console**: Warehouse operations, pool management, dispute resolution, compliance tools

### Backend Layer (NestJS)
- **Identity Service**: Auth (email/phone + wallet), nonce-based wallet verification, KYC triggers, sanctions screening
- **Submission Service**: Item submission lifecycle, photo/video upload to S3, category classification, band estimation, QR/label generation, packing instructions
- **Warehouse Service**: Inbound scan processing, condition grading, accept/reject/quarantine decisions, bin assignment, intake event emission
- **Claim Service**: Listens for verified intake events, activates claims, exposes claimable inventory, handles reservation with timeout, claim consumption
- **Pool Engine**: Pool taxonomy management, band definitions, regional rules, saturation analytics
- **Logistics Service**: Pickup creation, 3P courier dispatch, tracking integration, proof event monitoring, exception handling
- **Dispute Service**: Dispute lifecycle, evidence collection, SLA tracking, resolution execution
- **Compliance Service**: Restricted item rule sets, recall screening, audit logging, operator action tracking
- **Notification Service**: Multi-channel notifications (email, SMS, push) for status changes

### Data Layer
- **PostgreSQL (Prisma)**: All persistent state — users, pools, items, claims, disputes, logistics events
- **Redis (BullMQ)**: Job queues (claim activation, notification dispatch, indexer sync), rate limiting, session cache
- **S3-compatible Storage**: Item photos, QR labels, delivery proofs, dispute evidence

### Chain Layer
- **Smart Contracts**: On-chain entitlement state, pool definitions, claim minting/burning, inventory commitment, escrow settlement
- **Event Indexer (viem)**: Syncs on-chain events to PostgreSQL, enables replay, provides API layer with chain state

## Data Flow — Core Barter Loop

```
User submits item
  → Submission Service creates record + uploads media
  → Label Service generates QR
  → User ships item to warehouse

Warehouse scans QR
  → Warehouse Service loads submission, grades item
  → Warehouse Service accepts → emits intake event
  → Event triggers on-chain: ItemReceiptNFT minted

Claim activation
  → Claim Service detects verified intake
  → On-chain: PoolClaimToken minted for user
  → User sees active claim in dashboard

User reserves item
  → Claim Service checks balance + item availability
  → On-chain: EscrowSettlement locks claim
  → On-chain: InventoryCommitment reserves item
  → Reservation timeout starts

Delivery
  → Logistics Service creates shipment
  → 3P courier picks up from warehouse
  → Proof-of-delivery recorded
  → On-chain: InventoryCommitment confirms delivery
  → Claim consumed, item delivered
```

## Hybrid On-Chain / Off-Chain Boundary

| Concern | On-Chain | Off-Chain |
|---------|----------|-----------|
| Pool definitions and rules | Yes | Cached in Postgres |
| Item receipt proof | NFT minted | Full grading data in Postgres + S3 |
| Claim entitlement | Token balance | Claim metadata in Postgres |
| Reservation lock | Escrow state | Reservation timeout in Redis |
| Inventory status | Commitment contract | Full status history in Postgres |
| Delivery proof | Hash anchored | Full proof data in S3 |
| Disputes | Resolution hash | Evidence, timeline in Postgres + S3 |
| User identity | Wallet address | Profile, KYC in Postgres |
| Media/photos | IPFS CID reference | S3 storage |
| Analytics | Indexed events | Postgres aggregations |
