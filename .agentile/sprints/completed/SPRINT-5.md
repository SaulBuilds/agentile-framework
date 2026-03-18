# Sprint 5 — Backend Foundation

**Start Date**: 2026-03-06
**End Date**: 2026-03-20
**Status**: COMPLETE

## Sprint Goal
Scaffold the NestJS backend API and Next.js frontend. Set up PostgreSQL with Prisma schema, implement auth/wallet-linking endpoints, and create the event indexer scaffold for syncing on-chain state.

## Items

### 1. Project Scaffolding (NestJS + Next.js)
- **Priority**: High
- **Size**: M
- **Feature File**: `features/backend/db-schema.feature`
- **Status**: DONE
- **Notes**: Monorepo with npm workspaces. `apps/api` (NestJS), `apps/web` (Next.js + Tailwind v4), `packages/shared-types`. TypeScript strict mode. ESLint config. Health check endpoint.

### 2. Prisma Schema & Database Setup
- **Priority**: High
- **Size**: L
- **Feature File**: `features/backend/db-schema.feature`
- **Status**: DONE
- **Notes**: 25 models, 20 enums, all relations with cascade rules, composite indexes. Schema validates with `prisma format` and `prisma generate`. Matches PRISMA_SCHEMA_DRAFT.md spec exactly.

### 3. Auth Module (Register, Login, JWT)
- **Priority**: High
- **Size**: M
- **Feature File**: `features/backend/db-schema.feature`
- **Status**: DONE
- **Coverage**: 8 unit tests
- **Notes**: POST /auth/register, POST /auth/login. bcrypt password hashing (12 rounds). JWT with configurable secret/expiry. ValidationPipe with whitelist. Duplicate email/phone detection.

### 4. Wallet Linking Module
- **Priority**: High
- **Size**: M
- **Feature File**: `features/backend/db-schema.feature`
- **Status**: DONE
- **Coverage**: 5 unit tests
- **Notes**: POST /auth/wallet/link (nonce generation), POST /auth/wallet/verify (EIP-191 via viem), GET /auth/wallet (list). JWT-protected. Duplicate wallet detection. Nonce cleared after verification.

### 5. Event Indexer Scaffold
- **Priority**: Medium
- **Size**: M
- **Feature File**: `features/backend/db-schema.feature`
- **Status**: DONE
- **Coverage**: 3 unit tests
- **Notes**: Scaffold with handlers for PoolCreated, ReceiptMinted, ClaimMinted/Burned, InventoryRegistered. Placeholder block tracking. Logs warning when RPC_URL not configured.

## Exit Criteria
- [x] NestJS app boots and responds to health check
- [x] Next.js app boots with landing page
- [x] Prisma schema validates and client generates
- [x] Auth register/login returns JWT
- [x] Wallet link/verify flow works with mock signatures
- [x] Event indexer scaffold processes mock events
- [x] All tests pass (19/19)

## Blockers
| Blocker | Feature | Status | Resolution |
|---------|---------|--------|------------|
| — | — | — | — |

## Metrics (updated at sprint end)
- Features completed: 5/5
- Test coverage: 19 unit tests
- Tests passing: 19/19
- Blockers encountered: 0
- Blockers resolved: 0
