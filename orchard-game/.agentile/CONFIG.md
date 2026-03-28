# Project Configuration -- Canonical Values

> **This file is the single source of truth** for project constants. When in doubt, this file wins.

## Identity

| Key | Value |
|-----|-------|
| **Project Name** | Orchard Game |
| **Token Name** | Orchard Token |
| **Token Symbol** | ORT |
| **Token Decimals** | 18 |
| **Total Supply** | 1000000 |

## Technology Stack

| Layer | Technology |
|-------|------------|
| **Language** | Solidity |
| **Framework** | Hardhat |
| **Storage** | IPFS/Filecoin for game assets, Ethereum for state |
| **Testing** | Chai, Mocha, Waffle |
| **Formal Verification** | TLA+ (TLC model checker) |
| **Frontend** | React with p5.js canvas |
| **Chat Interface** | WebSocket-based |
| **Federated Learning** | TensorFlow.js with Web Workers |

## Workspace Layout

| Module | Path | Purpose |
|--------|------|---------|
| contracts | contracts/ | Solidity smart contracts for game logic |
| frontend | frontend/ | React application with p5.js integration |
| specs | specs/ | TLA+ modules and Gherkin feature files |
| scripts | scripts/ | Deployment and utility scripts |
| tests | tests/ | Unit, integration, and visual tests |

## Commands

```bash
# Build
npx hardhat compile

# Test
npx hardhat test

# Lint
npx hardhat solhint 'contracts/**/*.sol'
npx eslint frontend/src/**/*.js

# Format
npx hardhat prettier
npx prettier --write frontend/src/**/*.{js,jsx,ts,tsx}

# Run
npm run dev  # Starts frontend development server
```

## Naming Conventions

- Define canonical names for your project here
- List any commonly confused terms and their correct usage
