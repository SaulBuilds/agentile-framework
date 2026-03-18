# CONFIG.md — Project Configuration

> Edit this file to match your project. The agent reads this to understand tooling and conventions.

## Project Identity
```yaml
project_name: "Gradient Barter Protocol"
description: "A warehouse-backed, token-assisted barter network for real-world items. Users contribute items to categorized value pools, items are verified by sorting centers, and contributors receive pool claims to withdraw equivalent items."
version: "0.1.0"
repository: "https://github.com/SaulBuilds/agentile-framework"
owner: "SaulBuilds"
```

## Tech Stack
```yaml
# Smart Contracts
contract_language: "Solidity ^0.8.26"
contract_framework: "Foundry"
contract_libraries: "OpenZeppelin (access control, ERC-721, ERC-1155, pausables, reentrancy guards)"
contract_testing: "Foundry (unit, fuzz, invariant)"
contract_analysis: "Slither, Mythril, Echidna"

# Backend
language: "TypeScript"
framework: "NestJS"
runtime: "Node 20"
package_manager: "pnpm"
orm: "Prisma"
database: "PostgreSQL"
cache: "Redis + BullMQ"
chain_interface: "viem/wagmi"
observability: "OpenTelemetry"

# Frontend
frontend_framework: "Next.js"
frontend_state: "TanStack Query"
frontend_styling: "Tailwind CSS + shadcn/ui (Radix primitives)"
mobile: "React Native / Expo"
```

## Testing
```yaml
test_runner: "vitest"
bdd_framework: "cucumber-js"
coverage_target: 90
coverage_tool: "c8"
contract_test_runner: "forge test"
```

## CI/CD
```yaml
ci_platform: "GitHub Actions"
deployment_target: "Vercel (web), containerized backend (ECS/Fly/Render)"
containerized: true
```

## Documentation
```yaml
doc_format: "markdown"
api_docs: true
changelog: true
```

## Agent Preferences
```yaml
verbosity: "concise"
report_frequency: "per-task"
auto_commit: false
branch_per_feature: true
ask_before_refactor: false
```

## Custom Rules
```yaml
custom_rules:
  - "All smart contract methods must follow CEI (Checks-Effects-Interactions) pattern"
  - "No unbounded loops over dynamic arrays in contracts"
  - "All signatures must use EIP-712 typed data"
  - "Pool claims are non-transferable in v1 (allowlist-transferable ERC-1155)"
  - "One verified contribution = one pool claim (invariant must be tested)"
  - "No cash redemption of claims"
  - "Avoid loan/collateral/interest terminology — use barter/contribute/claim language"
  - "All physical events require: canonical DB row, signed actor, timestamp, evidence hash"
  - "Restricted items matrix must be checked before any submission completes"
  - "Proof-of-pickup and proof-of-delivery are mandatory for all logistics"
```
