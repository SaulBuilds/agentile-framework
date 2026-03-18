# ADR-001: Token & Legal Posture

**Status**: Accepted
**Date**: 2026-03-06
**Decision Makers**: Protocol operator (SaulBuilds)

## Context

The Gradient Barter Protocol issues tokens representing item receipts (ERC-721) and pool claims (ERC-1155). The legal characterization of these tokens determines regulatory exposure, marketing language, and feature constraints.

Key risks:
- If pool claims are broadly tradeable, pooled for profit, or cash-redeemable, they may be classified as securities
- If the protocol facilitates cash-equivalent exchanges, it may trigger money-transmission regulations
- Barter exchanges have specific tax reporting obligations in the US (IRS Form 1099-B)

## Decision

### Pool Claim Tokens (ERC-1155)

1. **Characterization**: Pool claims are **barter entitlements** — a specific contractual right to withdraw one item from a defined pool. They are NOT securities, NOT investment contracts, NOT money.

2. **Non-cash redeemable**: Claims cannot be redeemed for cash, cryptocurrency, or any monetary equivalent.

3. **Non-transferable in v1**: Claims use allowlist-transferable ERC-1155 with the transfer policy set to block all transfers in v1. Secondary markets are explicitly not supported.

4. **No profit promise**: The protocol never markets claims as appreciating assets, investment vehicles, or profit-generating instruments.

5. **One-to-one backing**: Every active claim is backed by exactly one verified physical item in a warehouse. Claims cannot exceed verified inventory.

### Item Receipt Tokens (ERC-721)

1. **Characterization**: Receipts are **proof of contribution** — an on-chain record that an item was submitted, received, and graded.

2. **Non-transferable**: Receipts are soul-bound to the contributing user's wallet. They serve as audit trail, not tradeable assets.

### Language Standards

**Use**: contribute, claim, pool, withdraw, entitlement, barter, exchange, trade
**Never use**: loan, collateral, interest, redemption, investment, returns, yield, profit, appreciation

### Compliance Disclosures

- Onboarding includes acknowledgment that barter may be taxable
- The protocol does not provide tax advice
- Identity information is collected as appropriate for reporting obligations
- The protocol does not market itself as tax-free or profit-generating

## Consequences

- No secondary claim marketplace in v1 (limits liquidity but reduces regulatory risk)
- No algorithmic incentives or reward tokens in v1
- No cross-pool claim swaps
- Future secondary transfer (v2+) must be compliance-gated with restricted allowlists
- Separate utility claims from any future fee/reward token

## Alternatives Considered

1. **Fully transferable claims**: Higher liquidity but significant regulatory risk. Rejected for v1.
2. **Cash-redeemable claims**: Would likely trigger money-transmission classification. Rejected entirely.
3. **No tokens at all (pure database)**: Loses on-chain transparency and audit trail benefits. Rejected.

## Supersedes

None (first decision on this topic).
