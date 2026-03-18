# VISION.md — Gradient Barter Protocol

## Project Purpose

Gradient Barter Protocol is a warehouse-backed, token-assisted barter network for real-world items. It enables users to contribute idle physical goods to categorized value pools, have them received and verified by authorized sorting centers, and receive non-fungible pool claims entitling them to withdraw one item from the same pool.

The protocol is intentionally **80% utility / barter infrastructure** and **20% market infrastructure**.

## Core Design Principle

**One deposited item = one entitled claim in the same pool.** Warehouse verification and delivery confirmation act as the unlock conditions.

This is not a generalized on-chain marketplace. It is a **custodied barter clearinghouse with on-chain state transparency**.

## Target Users

| Role | Description |
|------|-------------|
| Contributor / Trader | Households and small businesses with idle goods they want to exchange |
| Claim Holder | Users who have earned pool claims and want to browse/reserve items |
| Warehouse Operator | Staff at sorting centers who receive, grade, and store items |
| Quality Grader | Specialized operators who assess item condition and assign pool bands |
| Courier / Driver | Local delivery personnel (3P or community) |
| Operations Admin | Platform staff managing pools, categories, and policies |
| Compliance Reviewer | Staff handling disputes, restricted goods, and safety |
| Super Admin | Protocol-level operator with emergency controls |

## Why It Can Work

Most households and small businesses hold dormant assets that never reach resale markets because the process is inconvenient, low-trust, time-consuming, or too low-value relative to the effort. The protocol reduces friction by turning:

- idle goods into a pool contribution
- warehouse custody into trust
- transparent claim rules into fairness
- on-chain tracking into auditable logistics
- local delivery into a flexible last-mile network

## Key Differentiators

1. **Not a marketplace** — no open price discovery, no speculation, no cash redemption
2. **Warehouse-backed trust** — physical verification before claims unlock
3. **Pool-based matching** — items grouped by category + value band, not individual listings
4. **On-chain transparency** — claim entitlements and custody events are auditable
5. **Safety-first logistics** — bounded delivery scope, proof events, restricted goods enforcement

## Success Criteria (v1 Pilot)

- Successfully process item submissions across 3-5 categories
- Warehouse intake → claim unlock pipeline working end-to-end
- Same-pool reservation and delivery completing without manual intervention
- Dispute resolution workflow handling edge cases
- 1-2 warehouses operational in 1 region
- Claims non-transferable, no secondary market
- Third-party delivery integrated

## Non-Goals for v1

- Open price discovery marketplace for every item
- Unrestricted financialization of claims
- Cross-pool borrowing or leveraged trading
- Support for regulated, dangerous, or hard-to-verify items
- International shipping
- Luxury authentication categories
- Complex DAO governance
- Algorithmic liquidity incentives

## Strategic Stance

> If a feature increases speculation more than utility, it belongs after product-market fit, or not at all.
