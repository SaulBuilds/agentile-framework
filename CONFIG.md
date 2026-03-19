# Project Configuration -- Canonical Values

> **This file is the single source of truth** for project constants. When in doubt, this file wins.

## Identity

| Key | Value |
|-----|-------|
| **Project Name** | {{PROJECT_NAME}} |
| **Token Name** | {{TOKEN_NAME}} |
| **Token Symbol** | {{TOKEN_SYMBOL}} |
| **Token Decimals** | {{TOKEN_DECIMALS}} |
| **Total Supply** | {{TOKEN_TOTAL_SUPPLY}} |
| **Chain ID (devnet)** | {{CHAIN_ID_DEVNET}} |
| **Chain ID (testnet)** | {{CHAIN_ID_TESTNET}} |
| **Chain ID (mainnet)** | {{CHAIN_ID_MAINNET}} |
| **VM Name** | {{VM_NAME}} |
| **Consensus** | {{CONSENSUS_NAME}} |
| **Consensus Parameter k** | {{CONSENSUS_K}} |
| **Max Parents** | {{MAX_PARENTS}} |

## Network Parameters

| Key | Value | Notes |
|-----|-------|-------|
| **Block Time (devnet)** | {{BLOCK_TIME_DEVNET}} | {{BLOCK_TIME_DEVNET_CONFIG_PATH}} |
| **Block Time (testnet)** | {{BLOCK_TIME_TESTNET}} | {{BLOCK_TIME_TESTNET_CONFIG_PATH}} |
| **Target Finality** | {{TARGET_FINALITY}} | {{FINALITY_MECHANISM}} |
| **Checkpoint Interval** | {{CHECKPOINT_INTERVAL}} | {{CHECKPOINT_NOTES}} |
| **Committee Size** | {{COMMITTEE_SIZE}} | Quorum: {{QUORUM_SIZE}} |
| **Min Gas Price** | {{MIN_GAS_PRICE}} | |
| **Block Gas Limit** | {{BLOCK_GAS_LIMIT}} | |

## Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | {{PRIMARY_LANGUAGE}} |
| **EVM** | {{EVM_IMPLEMENTATION}} |
| **Storage** | {{STORAGE_ENGINE}} |
| **Networking** | {{NETWORKING_STACK}} |
| **Crypto** | {{CRYPTO_LIBRARIES}} |
| **GUI Framework** | {{GUI_FRAMEWORK}} |
| **GUI Styling** | {{GUI_STYLING}} |
| **Smart Contracts** | {{SMART_CONTRACT_FRAMEWORK}} |
| **Formal Verification** | {{FORMAL_VERIFICATION_TOOL}} |
| **Testing** | {{TESTING_FRAMEWORKS}} |

## Workspace Layout

| Crate | Path | Purpose |
|-------|------|---------|
| `{{CRATE_1_NAME}}` | `{{CRATE_1_PATH}}` | {{CRATE_1_PURPOSE}} |
| `{{CRATE_2_NAME}}` | `{{CRATE_2_PATH}}` | {{CRATE_2_PURPOSE}} |
| `{{CRATE_3_NAME}}` | `{{CRATE_3_PATH}}` | {{CRATE_3_PURPOSE}} |
| `{{CRATE_4_NAME}}` | `{{CRATE_4_PATH}}` | {{CRATE_4_PURPOSE}} |
| `{{CRATE_5_NAME}}` | `{{CRATE_5_PATH}}` | {{CRATE_5_PURPOSE}} |

_(Add or remove crate rows as needed for your project.)_

## Commands

```bash
# Build
{{BUILD_COMMAND}}

# Test (core crates)
{{TEST_COMMAND}}

# Lint
{{LINT_COMMAND}}

# Format
{{FORMAT_COMMAND}}

# GUI tests
{{GUI_TEST_COMMAND}}

# Contract tests
{{CONTRACT_TEST_COMMAND}}

# Run devnet node
{{RUN_DEVNET_COMMAND}}
```

## Naming Conventions

- **DO** say: {{CANONICAL_NAMES}}
- **DO NOT** say: {{DEPRECATED_NAMES}}
- **Addresses**: {{ADDRESS_FORMAT}}
- **Signatures**: {{SIGNATURE_FORMAT}}
