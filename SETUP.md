# Setup Guide -- Customizing the Agentile Framework

This document lists every `{{PLACEHOLDER}}` variable in the framework and explains what to replace it with.

---

## CONFIG.md Placeholders

### Identity

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{PROJECT_NAME}}` | The name of your project | `MyChain` |
| `{{TOKEN_NAME}}` | The name of your native token | `MYTOKEN` |
| `{{TOKEN_SYMBOL}}` | The ticker symbol | `MYT` |
| `{{TOKEN_DECIMALS}}` | Number of decimal places | `18` |
| `{{TOKEN_TOTAL_SUPPLY}}` | Total token supply | `1,000,000,000 MYT` |
| `{{CHAIN_ID_DEVNET}}` | Chain ID for local development | `31337` |
| `{{CHAIN_ID_TESTNET}}` | Chain ID for public testnet | `80001` |
| `{{CHAIN_ID_MAINNET}}` | Chain ID for mainnet | `1` |
| `{{VM_NAME}}` | Name of the virtual machine | `My Virtual Machine (MVM)` |
| `{{CONSENSUS_NAME}}` | Consensus algorithm name | `MyConsensus` |
| `{{CONSENSUS_K}}` | Primary consensus parameter | `18` |
| `{{MAX_PARENTS}}` | Maximum parent blocks | `10` |

### Network Parameters

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{BLOCK_TIME_DEVNET}}` | Block time on devnet | `1 second` |
| `{{BLOCK_TIME_DEVNET_CONFIG_PATH}}` | Path to devnet config | `node/config/devnet.toml` |
| `{{BLOCK_TIME_TESTNET}}` | Block time on testnet | `2 seconds` |
| `{{BLOCK_TIME_TESTNET_CONFIG_PATH}}` | Path to testnet config | `node/config/testnet.toml` |
| `{{TARGET_FINALITY}}` | Target finality time | `12 seconds` |
| `{{FINALITY_MECHANISM}}` | How finality is achieved | `Committee BFT checkpoints` |
| `{{CHECKPOINT_INTERVAL}}` | Blocks between checkpoints | `50 blocks` |
| `{{CHECKPOINT_NOTES}}` | Additional checkpoint info | `~25s at devnet speed` |
| `{{COMMITTEE_SIZE}}` | Number of validators in committee | `100 validators` |
| `{{QUORUM_SIZE}}` | Quorum requirement | `67 (2/3 + 1)` |
| `{{MIN_GAS_PRICE}}` | Minimum gas price | `1 gwei` |
| `{{BLOCK_GAS_LIMIT}}` | Gas limit per block | `30,000,000` |

### Technology Stack

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{PRIMARY_LANGUAGE}}` | Main programming language | `Rust (edition 2021)` |
| `{{EVM_IMPLEMENTATION}}` | EVM engine used | `MyEVM` |
| `{{STORAGE_ENGINE}}` | Database engine | `MyDB` |
| `{{NETWORKING_STACK}}` | P2P networking library | `MyP2P (Noise + Gossipsub)` |
| `{{CRYPTO_LIBRARIES}}` | Cryptography libraries | `ed25519-dalek, k256, sha3` |
| `{{GUI_FRAMEWORK}}` | GUI framework | `Electron + React` |
| `{{GUI_STYLING}}` | CSS/styling approach | `CSS variables + inline styles` |
| `{{SMART_CONTRACT_FRAMEWORK}}` | Smart contract tooling | `Solidity (Foundry)` |
| `{{FORMAL_VERIFICATION_TOOL}}` | Formal verification tool | `TLA+ (TLC model checker)` |
| `{{TESTING_FRAMEWORKS}}` | Testing tools | `cargo test, vitest, forge test` |

### Workspace Layout

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{CRATE_N_NAME}}` | Module/crate name | `my-consensus` |
| `{{CRATE_N_PATH}}` | Path to the module | `core/consensus/` |
| `{{CRATE_N_PURPOSE}}` | What the module does | `Consensus engine, tip selection` |

Add or remove crate rows as needed. The template includes 5 slots.

### Commands

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{BUILD_COMMAND}}` | How to build the project | `cargo build --release` |
| `{{TEST_COMMAND}}` | How to run all tests | `cargo test --workspace` |
| `{{LINT_COMMAND}}` | How to lint | `cargo clippy --all-targets -D warnings` |
| `{{FORMAT_COMMAND}}` | How to format | `cargo fmt --all` |
| `{{GUI_TEST_COMMAND}}` | How to test the GUI | `cd gui && npx vitest run` |
| `{{CONTRACT_TEST_COMMAND}}` | How to test contracts | `cd contracts && forge test` |
| `{{RUN_DEVNET_COMMAND}}` | How to run a local node | `cargo run --bin node -- devnet` |

### Naming Conventions

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{CANONICAL_NAMES}}` | Official terms to use | `MYT, MVM, MyConsensus` |
| `{{DEPRECATED_NAMES}}` | Terms to avoid | `CITE, Old VM Name, GHOST` |
| `{{ADDRESS_FORMAT}}` | Address format description | `20-byte EVM or 32-byte ed25519` |
| `{{SIGNATURE_FORMAT}}` | Signature scheme description | `ed25519 (native) or ECDSA (EVM)` |

---

## AGENT_ENTRY.md Placeholders

| Placeholder | Description | Example |
|-------------|-------------|---------|
| `{{PROJECT_DESCRIPTION}}` | One-paragraph project description | `MyChain is a Layer-1 blockchain...` |
| `{{TOKEN_NAME}}` | Same as CONFIG.md | `MYTOKEN` |
| `{{TOKEN_TOTAL_SUPPLY}}` | Same as CONFIG.md | `1B` |
| `{{TOKEN_DECIMALS}}` | Same as CONFIG.md | `18` |
| `{{CHAIN_ID_PRIMARY}}` | Primary chain ID to reference | `80001 (testnet)` |
| `{{VM_NAME}}` | Same as CONFIG.md | `MVM` |
| `{{VM_DESCRIPTION}}` | Short VM description | `EVM-compatible execution engine` |
| `{{CONSENSUS_NAME}}` | Same as CONFIG.md | `MyConsensus` |
| `{{CONSENSUS_PARAMS}}` | Key consensus parameters | `k=18, ECVRF election, BFT checkpoints` |
| `{{TEST_SUITE_SUMMARY}}` | Summary of test counts | `2,484 Rust tests, 596 GUI tests` |

---

## QUIZ_SPEC.md Placeholders

The quiz has 5 categories with 3 questions each (Easy, Medium, Hard).

| Placeholder | Description |
|-------------|-------------|
| `{{QUIZ_CATEGORY_A_NAME}}` | Name of category A (e.g., "Rust & Systems Programming") |
| `{{QUIZ_A1_EASY}}` | Full Easy question for category A (question + 4 options + correct answer + rationale) |
| `{{QUIZ_A2_MEDIUM}}` | Full Medium question for category A |
| `{{QUIZ_A3_HARD}}` | Full Hard question for category A |
| `{{QUIZ_CATEGORY_B_NAME}}` | Name of category B (e.g., "Blockchain & Consensus") |
| `{{QUIZ_B1_EASY}}` | Full Easy question for category B |
| `{{QUIZ_B2_MEDIUM}}` | Full Medium question for category B |
| `{{QUIZ_B3_HARD}}` | Full Hard question for category B |
| `{{QUIZ_CATEGORY_C_NAME}}` | Name of category C (e.g., "Testing & Quality") |
| `{{QUIZ_C1_EASY}}` through `{{QUIZ_C3_HARD}}` | Questions for category C |
| `{{QUIZ_CATEGORY_D_NAME}}` | Name of category D (e.g., "Architecture & Design") |
| `{{QUIZ_D1_EASY}}` through `{{QUIZ_D3_HARD}}` | Questions for category D |
| `{{QUIZ_CATEGORY_E_NAME}}` | Name of category E (e.g., "Formal Methods") |
| `{{QUIZ_E1_EASY}}` through `{{QUIZ_E3_HARD}}` | Questions for category E |

### Question Format

Each question placeholder should be replaced with:

```markdown
**Question text here?**

- (a) Option A
- (b) Option B
- (c) Option C
- (d) Option D

**Correct: (X)**

*Rationale: Explanation of why the correct answer is right and why distractors are wrong.*
```

---

## Customization Checklist

- [ ] Replace all placeholders in `CONFIG.md`
- [ ] Replace all placeholders in `AGENT_ENTRY.md`
- [ ] Write 15 quiz questions in `onboarding/QUIZ_SPEC.md` (5 categories x 3 difficulties)
- [ ] Update `coverage/BASELINE.md` with initial test counts
- [ ] Update `coverage/GATES.md` with per-module coverage targets
- [ ] Create your first sprint in `sprints/active/`
- [ ] Update `sprints/CURRENT.md` to reference the sprint
- [ ] Verify all links in `MANIFEST.md` point to existing files
