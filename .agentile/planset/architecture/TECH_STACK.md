# TECH_STACK.md — Gradient Barter Protocol

## Smart Contracts

| Technology | Choice | Rationale |
|------------|--------|-----------|
| Language | Solidity ^0.8.26 | Industry standard for EVM, mature tooling, large talent pool |
| Framework | Foundry | Fastest test execution, native fuzz/invariant testing, Solidity-native tests |
| Libraries | OpenZeppelin | Battle-tested access control, ERC-721, ERC-1155, Pausable, ReentrancyGuard |
| Static Analysis | Slither | Automated vulnerability detection, CI-friendly |
| Symbolic Execution | Mythril | Deep vulnerability detection for critical paths |
| Fuzzing | Echidna + Foundry fuzz | Property-based testing for invariants (one-claim-per-item) |
| Target Chain | TBD (EVM-compatible) | L2 preferred for cost; Base, Arbitrum, or Polygon candidates |

### ADR: Why Foundry over Hardhat
Foundry provides faster compilation and test execution, native Solidity test writing (no JavaScript context-switching), built-in fuzz and invariant testing, and better gas reporting. The protocol's invariant testing requirements (one-claim-per-verified-item) make Foundry's property-based testing essential.

## Backend

| Technology | Choice | Rationale |
|------------|--------|-----------|
| Language | TypeScript | Shared language with frontend, strong typing, large ecosystem |
| Framework | NestJS | Modular architecture fits service boundaries, built-in DI, guards, interceptors |
| ORM | Prisma | Type-safe queries, migration management, schema-as-code |
| Database | PostgreSQL | ACID compliance critical for inventory/claim state, JSON support for metadata, mature |
| Cache/Queue | Redis + BullMQ | Job queues for async processing (claim activation, notifications), rate limiting, session cache |
| Chain Interface | viem/wagmi | Modern, type-safe EVM interaction, active maintenance |
| Object Storage | S3-compatible | Item photos, QR labels, delivery proofs, dispute evidence |
| Observability | OpenTelemetry | Vendor-neutral tracing, critical for debugging multi-service flows |

### ADR: Why NestJS over Express/Fastify
The protocol has 10+ distinct service modules (identity, submission, warehouse, claims, logistics, disputes, compliance, pools, notifications, analytics). NestJS's modular architecture, dependency injection, and built-in guard/interceptor patterns map directly to these boundaries. Raw Express/Fastify would require building these patterns from scratch.

### ADR: Why PostgreSQL over MongoDB
The data model has strong relational requirements: items belong to pools, claims reference items, reservations lock inventory, disputes reference multiple entities. ACID transactions are critical for inventory reservation (double-claim prevention). PostgreSQL's JSON columns handle flexible metadata without sacrificing relational integrity.

## Frontend

| Technology | Choice | Rationale |
|------------|--------|-----------|
| Framework | Next.js | SSR for pool explorer SEO, API routes for BFF layer, React ecosystem |
| State Management | TanStack Query | Server-state focused (most data comes from API), caching, optimistic updates |
| Styling | Tailwind CSS | Utility-first for rapid UI development, consistent design tokens |
| Components | shadcn/ui (Radix) | Accessible primitives, customizable, not a heavy dependency |
| Mobile | React Native / Expo | Code sharing with web, QR scanning support, camera access for proofs |

### ADR: Why Tailwind + shadcn over a full component library
The brand direction (warm earth tones, soft gradients, custom pool hue system) requires deep visual customization. Pre-built libraries like MUI or Ant Design impose their own design language. Tailwind + shadcn provides accessible primitives with full control over visual identity.

## Infrastructure

| Technology | Choice | Rationale |
|------------|--------|-----------|
| Web Hosting | Vercel | Native Next.js support, edge functions, preview deployments |
| Backend Hosting | Containerized (ECS/Fly/Render) | NestJS needs long-running processes for queues and indexer |
| Database | Managed PostgreSQL | Automated backups, failover, scaling |
| Object Storage | S3 or R2 | Cost-effective media storage with signed URL access |
| Log Aggregation | TBD | SIEM-ready audit trail required for compliance |
| CI/CD | GitHub Actions | Integrated with repo, free tier sufficient for early development |

## Dependencies Summary

### Core (v1 required)
- `@nestjs/core`, `@nestjs/platform-fastify`
- `prisma`, `@prisma/client`
- `bullmq`, `ioredis`
- `viem`, `wagmi`
- `next`, `react`, `react-dom`
- `@tanstack/react-query`
- `tailwindcss`
- `@radix-ui/*` (via shadcn)
- OpenZeppelin contracts
- Foundry toolchain

### Development
- `vitest` (backend + frontend testing)
- `cucumber-js` (BDD step definitions)
- `c8` (coverage)
- `eslint`, `prettier`
- `forge` (contract testing)
