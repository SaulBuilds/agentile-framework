# Threat Model — Gradient Barter Protocol

## Methodology
STRIDE analysis per system layer. Each threat identifies: category, attack vector, impact, likelihood, mitigation, and residual risk.

---

## 1. Smart Contract Layer

### T1.1 — Reentrancy on Settlement
- **Category**: Tampering
- **Vector**: Malicious contract calls back into EscrowSettlement during claim consumption or payout
- **Impact**: Critical — double-spend claims, drain escrow
- **Likelihood**: Medium (well-known attack pattern)
- **Mitigation**: ReentrancyGuard on all state-changing functions, CEI pattern enforced, pull-payment for CourierRewards
- **Residual Risk**: Low

### T1.2 — Claim Inflation
- **Category**: Elevation of Privilege
- **Vector**: Attacker finds way to mint claims without a corresponding verified receipt
- **Impact**: Critical — breaks one-claim-per-item invariant, unbacked claims enter system
- **Mitigation**: CLAIM_MINTER role restricted to backend escrow service only, invariant fuzz tested, mintClaim requires verified receipt reference
- **Residual Risk**: Low (if invariant tests are comprehensive)

### T1.3 — Unauthorized Minting/Burning
- **Category**: Elevation of Privilege
- **Vector**: Attacker gains WAREHOUSE_ROLE or CLAIM_MINTER role
- **Impact**: Critical — fake receipts, unauthorized claims
- **Mitigation**: Role-based access control via OpenZeppelin, multi-sig for role grants, timelocked admin operations
- **Residual Risk**: Low

### T1.4 — Signature Replay
- **Category**: Spoofing
- **Vector**: Replaying a valid signature to re-execute an authorized action
- **Impact**: High — duplicate operations
- **Mitigation**: EIP-712 typed data with nonce tracking, chain ID binding
- **Residual Risk**: Low

### T1.5 — Front-Running Reservations
- **Category**: Tampering
- **Vector**: Attacker observes a reservation transaction in mempool and front-runs to claim the same inventory item
- **Impact**: Medium — user loses desired item
- **Mitigation**: Reservation via backend (off-chain commit, on-chain confirm), not direct user txn. Alternatively: commit-reveal scheme.
- **Residual Risk**: Low (backend-mediated)

### T1.6 — Admin Key Compromise
- **Category**: Elevation of Privilege
- **Vector**: EMERGENCY_ADMIN or POOL_ADMIN private key stolen
- **Impact**: Critical — attacker can pause/unpause, modify pools, disrupt operations
- **Mitigation**: Multi-sig wallets for admin roles, timelocked operations for critical changes, key rotation procedures
- **Residual Risk**: Medium (operational discipline dependent)

---

## 2. Backend Layer

### T2.1 — SQL Injection
- **Category**: Tampering
- **Vector**: Malicious input in API parameters executing arbitrary SQL
- **Impact**: Critical — data breach, data manipulation
- **Mitigation**: Prisma ORM uses parameterized queries exclusively, no raw SQL without explicit escaping
- **Residual Risk**: Low

### T2.2 — Unauthorized API Access
- **Category**: Elevation of Privilege
- **Vector**: Attacker accesses admin or operator endpoints without proper role
- **Impact**: High — unauthorized pool changes, dispute resolution, inventory manipulation
- **Mitigation**: NestJS guards on every endpoint, role-based access control, JWT validation, wallet-signature verification for sensitive operations
- **Residual Risk**: Low

### T2.3 — Malicious File Upload
- **Category**: Tampering
- **Vector**: User uploads malware disguised as item photo
- **Impact**: Medium — server compromise, malware distribution
- **Mitigation**: Signed URL uploads (never touch server), S3 bucket policy prevents execution, malware scanning on upload, content-type validation, file size limits
- **Residual Risk**: Low

### T2.4 — API Abuse / DDoS
- **Category**: Denial of Service
- **Vector**: Flood of requests to exhaust resources or create spam submissions
- **Impact**: Medium — service degradation
- **Mitigation**: Redis-backed rate limiting per user and per endpoint, request validation, CAPTCHA on submission flow, queue-based processing for heavy operations
- **Residual Risk**: Medium (sophisticated DDoS requires infrastructure-level protection)

### T2.5 — Secret Exposure
- **Category**: Information Disclosure
- **Vector**: Database credentials, API keys, or private keys leaked via logs, error messages, or repo
- **Impact**: Critical — full system compromise
- **Mitigation**: Environment variable management, no secrets in code, secret rotation policy, structured logging (no sensitive data), .gitignore enforcement
- **Residual Risk**: Low (with proper operational hygiene)

### T2.6 — Event Indexer Desync
- **Category**: Tampering / Denial of Service
- **Vector**: Event indexer falls behind or misses events, causing off-chain state to diverge from on-chain
- **Impact**: High — claims not activated, inventory not reflected, user sees stale data
- **Mitigation**: Indexer health monitoring, automatic replay from last confirmed block, consistency checks between on-chain and off-chain state, alerting on desync
- **Residual Risk**: Medium (eventual consistency is inherent)

---

## 3. Warehouse Layer

### T3.1 — Internal Theft
- **Category**: Tampering
- **Vector**: Warehouse employee steals an item and marks it as delivered or lost
- **Impact**: High — inventory shrinkage, user trust erosion
- **Mitigation**: CCTV on all areas, role separation (receiver ≠ grader ≠ picker), dual control for high-value items, cycle counting, shrinkage monitoring with alerts
- **Residual Risk**: Medium (physical security has inherent limits)

### T3.2 — Item Substitution
- **Category**: Spoofing
- **Vector**: Employee swaps a high-value item for a lower-value one
- **Impact**: High — user receives wrong item, trust broken
- **Mitigation**: Photographic evidence at intake and pick, serial number tracking, QR-based chain of custody, tamper-evident labels
- **Residual Risk**: Medium

### T3.3 — Grading Manipulation
- **Category**: Tampering
- **Vector**: Grader intentionally under/over-grades items for personal benefit
- **Impact**: Medium — pool imbalance, unfair claims
- **Mitigation**: Evidence-based grading (photos + checklist mandatory), grading audits, operator action logging, statistical monitoring for anomalies per grader
- **Residual Risk**: Medium

### T3.4 — Counterfeit Items
- **Category**: Spoofing
- **Vector**: User submits counterfeit item that passes automated screening
- **Impact**: Medium — recipient receives fake item
- **Mitigation**: Warehouse physical inspection, serial number verification, category-specific authentication training, dispute pathway for recipients
- **Residual Risk**: Medium (authentication is imperfect for some categories)

---

## 4. Logistics Layer

### T4.1 — Delivery Interception
- **Category**: Tampering
- **Vector**: Courier steals package during delivery
- **Impact**: High — item lost, user trust broken
- **Mitigation**: Proof-of-pickup QR scan, proof-of-delivery required, GPS tracking during transit, courier identity verification, insurance
- **Residual Risk**: Medium (3P couriers managed by their own policies)

### T4.2 — Address Exposure
- **Category**: Information Disclosure
- **Vector**: Precise user addresses visible to couriers before task acceptance
- **Impact**: Medium — privacy violation, potential safety risk
- **Mitigation**: Addresses redacted until task acceptance, approximate location shown for task selection, full address only after commitment
- **Residual Risk**: Low

### T4.3 — Fake Delivery Proof
- **Category**: Spoofing
- **Vector**: Courier submits false proof-of-delivery without actually delivering
- **Impact**: High — user doesn't receive item but system marks it delivered
- **Mitigation**: Multiple proof methods (QR scan, PIN, photo + geolocation), recipient confirmation window, dispute pathway with SLA
- **Residual Risk**: Medium

---

## 5. User-Facing Layer

### T5.1 — Account Takeover
- **Category**: Spoofing
- **Vector**: Attacker gains access to user account via credential stuffing, phishing, or session hijack
- **Impact**: High — attacker claims user's items, changes wallet
- **Mitigation**: Strong password requirements, rate-limited login attempts, wallet re-verification for sensitive operations, session management with short-lived tokens
- **Residual Risk**: Medium (MFA recommended for v2)

### T5.2 — Fraudulent Submissions
- **Category**: Spoofing
- **Vector**: User submits item that doesn't match photos, sends empty box, or uses someone else's item
- **Impact**: Medium — wasted warehouse resources, fraudulent claims
- **Mitigation**: Pre-submission declaration (legal acknowledgment), warehouse verification catches mismatches, serial number checks, repeat-offender tracking, account restrictions
- **Residual Risk**: Medium

### T5.3 — Dispute Abuse
- **Category**: Denial of Service
- **Vector**: User files frivolous disputes to tie up admin resources or block inventory
- **Impact**: Low-Medium — operational burden
- **Mitigation**: Required evidence for dispute filing, dispute rate monitoring per user, repeat abuser flagging, goodwill credit limits
- **Residual Risk**: Low

---

## Risk Summary

| Risk Level | Count | Action |
|------------|-------|--------|
| Critical (pre-mitigation) | 5 | All mitigated to Low-Medium |
| High (pre-mitigation) | 8 | All mitigated to Low-Medium |
| Medium (pre-mitigation) | 5 | Mitigated to Low-Medium |

### Top Residual Risks (require ongoing monitoring)
1. **Admin key compromise** — operational discipline, multi-sig required
2. **Internal warehouse theft** — physical security limits
3. **Event indexer desync** — eventual consistency inherent in hybrid architecture
4. **Counterfeit items** — authentication is imperfect for some categories
5. **Fake delivery proof** — multiple proof methods reduce but don't eliminate risk
