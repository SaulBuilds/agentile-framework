# Epic 1: Identity, Onboarding & Wallet Linking

**Priority**: High
**Size**: L
**Scope Reference**: SCOPE.md — Feature 1 (Identity & Wallet Linking)
**Phase**: 1 (Sprint 5)

## Stories

### Story 1.1: User Registration
As a new user, I want to create an account with email or phone so that I can participate in the barter network.

**Acceptance Criteria**
- User can register with email or phone
- Password is hashed and stored securely
- JWT token returned on successful registration
- Duplicate email/phone rejected
- Input validation on all fields

### Story 1.2: Wallet Linking
As a registered user, I want to connect an EVM wallet so that my claims and receipts are on-chain.

**Acceptance Criteria**
- User can connect an EVM wallet
- Backend generates nonce for signature verification
- User signs nonce to prove wallet ownership
- Wallet bound to user profile on verification
- Duplicate wallet linking prevented
- Audit log records link event

### Story 1.3: KYC Flow (Medium priority)
As a user accessing higher-value pools, I want to complete identity verification when prompted.

**Acceptance Criteria**
- Threshold-based KYC prompt triggers at appropriate pool bands
- User sees reason and data usage disclosure
- Verification result stored with status and timestamp
- Restricted features blocked until approved
