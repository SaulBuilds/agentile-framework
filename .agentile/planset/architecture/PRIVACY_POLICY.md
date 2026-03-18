# Privacy & Data Retention Policy — Draft

## 1. Data Collection

### What We Collect

| Data Type | Collection Point | Purpose | Required? |
|-----------|-----------------|---------|-----------|
| Email / phone | Registration | Account identity, notifications | Yes (one of) |
| Password (hashed) | Registration | Authentication | Yes |
| Wallet address | Wallet linking | On-chain claim association | Yes |
| Item photos/video | Submission | Grading, evidence, dispute resolution | Yes |
| Item description | Submission | Classification, pool assignment | Yes |
| Shipping address | Reservation checkout | Delivery | Yes (for delivery) |
| KYC documents | Higher-value pools | Identity verification, compliance | Conditional |
| Device/IP metadata | All interactions | Security, fraud detection | Automatic |
| Location (couriers) | Active delivery tasks | Routing, proof, safety | Courier role only |

### What We Don't Collect
- Social security numbers (unless legally required for tax reporting thresholds)
- Financial account details (no cash transactions in v1)
- Biometric data
- Data from minors (18+ required)

---

## 2. Data Storage

| Data Type | Storage Location | Encryption |
|-----------|-----------------|------------|
| User profiles | PostgreSQL | At rest (AES-256) |
| Passwords | PostgreSQL | bcrypt/argon2 hashed |
| Item media | S3-compatible | At rest + in transit (TLS) |
| Grading evidence | S3-compatible | At rest + in transit |
| Dispute evidence | S3-compatible | At rest + in transit |
| Wallet addresses | PostgreSQL + on-chain | At rest (DB), public (chain) |
| Shipping addresses | PostgreSQL | At rest, access-controlled |
| KYC documents | Third-party KYC provider | Provider's policy applies |
| Audit logs | Append-only store | At rest, immutable |

---

## 3. Data Retention Periods

| Data Type | Retention Period | Trigger for Deletion |
|-----------|-----------------|---------------------|
| Active user profile | Duration of account | Account deletion request |
| Inactive user profile | 2 years after last activity | Auto-archive, then deletion |
| Item photos (submission) | 2 years after trade completion | Automated cleanup |
| Grading evidence (warehouse) | 2 years after trade completion | Automated cleanup |
| Dispute evidence | 3 years after resolution | Automated cleanup |
| CCTV footage | 90 days minimum | Rolling deletion |
| Audit logs (operator actions) | 5 years | Regulatory minimum |
| Courier location data | 90 days after task completion | Automated cleanup |
| Shipping addresses | 1 year after delivery | Automated cleanup |
| KYC documents | Per KYC provider policy | Provider-managed |
| Tax reporting records | 7 years (IRS requirement) | Regulatory minimum |

---

## 4. Data Deletion & User Rights

### Account Deletion
- Users can request account deletion at any time
- Upon request:
  - Profile data deleted within 30 days
  - Item media deleted within 30 days
  - Active claims must be resolved first (consumed or expired)
  - Audit log entries referencing the user are anonymized, not deleted
  - On-chain data (wallet address, transaction hashes) cannot be deleted

### Data Export
- Users can request a machine-readable export of their data
- Export includes: profile, submission history, claim history, trade history
- Export excludes: internal grading notes, audit logs, other users' data

### Correction
- Users can update their profile information at any time
- Submission data is immutable after submission (correction requires new submission)

---

## 5. On-Chain Data Permanence

**Important disclosure (shown during onboarding):**

The following data is stored on a public blockchain and **cannot be deleted**:
- Wallet addresses
- Transaction hashes
- Pool claim minting/burning records
- Item receipt token IDs
- Evidence hashes (not the evidence itself — just the hash)

**No PII is stored on-chain.** Only identifiers, hashes, and state are recorded. The off-chain database and storage can be purged per retention policy while on-chain hashes remain as non-reversible references.

---

## 6. Data Access Controls

| Role | Access Scope |
|------|-------------|
| Contributor | Own profile, submissions, claims, trades |
| Warehouse Operator | Submission details for items at their warehouse, grading data |
| Courier | Task details, pickup/dropoff locations (after acceptance) |
| Admin | All data within their operational scope |
| Super Admin | All data, audit logs, compliance tools |
| Support | User profiles, submission history, disputes (read-only) |

All access is logged. No role has unrestricted database access. Queries go through the application layer.

---

## 7. Third-Party Data Sharing

| Third Party | Data Shared | Purpose |
|-------------|------------|---------|
| KYC provider | User identity documents | Identity verification |
| Shipping carriers | Shipping address, item dimensions/weight | Delivery |
| Payment processor (if applicable) | Transaction amounts | Fee processing |
| Law enforcement | As legally required | Subpoena/warrant compliance |

No data is sold to advertisers or data brokers. No data is shared for marketing purposes.

---

## 8. Breach Response

1. Identify and contain the breach within 24 hours
2. Assess scope: what data was exposed, how many users affected
3. Notify affected users within 72 hours
4. Notify regulators as required by jurisdiction
5. Publish post-mortem and remediation plan
6. Implement fixes and verify
