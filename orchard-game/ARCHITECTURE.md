# Orchard Game: x402 Payments, SDKs & Economic System

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                     ORCHARD GAME ECOSYSTEM                          │
├─────────────────────────────────────────────────────────────────────┤
│  AI AGENT LAYER                                                     │
│  ┌─────────────────┐    ┌─────────────────┐                         │
│  │  Python SDK     │    │  NPM/TS SDK     │                         │
│  │  (agents)       │    │  (JS agents)    │                         │
│  └────────┬────────┘    └────────┬────────┘                         │
│           │                      │                                  │
│           ▼                      ▼                                  │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │              x402 PAYMENT GATEWAY                            │   │
│  │   - HTTP 402 Payment Required flow                          │   │
│  │   - Per-request micropayments (USDC)                        │   │
│  │   - Agent wallet integration (Awal, Privy)                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────────┤
│  SMART CONTRACT LAYER                                               │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐               │
│  │ ORTToken.sol │ │ SeedNFT.sol  │ │ Federation.sol│               │
│  │ - Staking    │ │ - Planting   │ │ - Collab     │               │
│  │ - Rewards    │ │ - Growth     │ │ - Farming    │               │
│  └──────────────┘ └──────────────┘ └──────────────┘               │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐               │
│  │ DuelManager  │ │ Leaderboard  │ │ x402Payment  │               │
│  │ - PvP        │ │ - Rankings   │ │ - Micro-pay  │               │
│  └──────────────┘ └──────────────┘ └──────────────┘               │
│  ┌──────────────┐ ┌──────────────┐                                 │
│  │ Economics.sol│ │ Alignment.sol│                                 │
│  │ - Farming    │ │ - Counterparty│                                │
│  │ - Rewards    │ │ - Ideology   │                                 │
│  └──────────────┘ └──────────────┘                                 │
├─────────────────────────────────────────────────────────────────────┤
│  ECONOMIC MODEL                                                     │
│                                                                      │
│  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐         │
│  │ PLANTING│    │ GROWTH │    │ HARVEST │    │ FARMING │         │
│  │ Stake   │───▶│ Checkpoints │───▶│ Rewards │───▶│ Re-stake│        │
│  │ 10+ ORT │    │ 1-1000     │    │ Score*  │    │ Compound│        │
│  └─────────┘    └─────────┘    └─────────┘    └─────────┘         │
│                                                                      │
│  REWARD FORMULA:                                                    │
│  baseReward = stakeAmount * (growthScore / 100)                     │
│  federationBonus = baseReward * (1 + memberCount * 0.1)            │
│  farmingMultiplier = daysStaked * 0.01 (max 2x)                    │
│                                                                      │
├─────────────────────────────────────────────────────────────────────┤
│  COUNTERPARTY ALIGNMENT SYSTEM                                      │
│                                                                      │
│  Players align based on:                                            │
│  1. INPUT SIMILARITY - Similar seed prompts/ideas                  │
│  2. IDEOLOGY MATCH - Shared philosophical approaches                │
│  3. FEDERATION MEMBERSHIP - Collaborative play                      │
│  4. DUEL OUTCOMES - Win/loss creates rivalry/alliance              │
│                                                                      │
│  Alignment Map: players[address] → alignmentHash                   │
│  Similarity Score: 0-100% based on input vectors                   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## x402 Payment Integration

### Payment Flow for AI Agents
```
1. Agent requests /api/game/plant
2. Server returns 402 with payment manifest:
   { maxAmountRequired: "0.01", payTo: "...", asset: "USDC", network: "solana" }
3. Agent signs payment with wallet
4. Agent retries with PAYMENT-SIGNATURE header
5. Server verifies and executes action
6. Settlement on-chain
```

## SDK Design

### Python SDK (orchard-sdk-python)
```python
class OrchardAgent:
    def __init__(self, wallet: Wallet, network: str = "solana"):
        self.wallet = wallet
        self.x402 = X402Client(wallet)
        
    async def plant_seed(self, payload: str, stake: float) -> int:
        """Plant a seed and stake ORT"""
        
    async def advance_checkpoint(self, token_id: int):
        """Progress seed growth"""
        
    async def join_federation(self, federation_id: int):
        """Join collaborative federation"""
        
    async def initiate_duel(self, opponent: str, seed_id: int):
        """Challenge another player"""
        
    async def get_leaderboard(self, season: int = None) -> List[Player]:
        """Fetch rankings"""
        
    async def claim_rewards(self, token_id: int, score: int):
        """Harvest and claim rewards"""
```

### TypeScript SDK (orchard-sdk-js)
```typescript
class OrchardClient {
  constructor(wallet: WalletAdapter, network: 'solana' | 'base');
  
  plantSeed(payload: string, stake: bigint): Promise<number>;
  advanceCheckpoint(tokenId: number): Promise<void>;
  joinFederation(federationId: number): Promise<void>;
  initiateDuel(opponent: string, seedId: number): Promise<number>;
  getLeaderboard(season?: number): Promise<LeaderboardEntry[]>;
  claimRewards(tokenId: number, score: number): Promise<void>;
}
```

## Economic Mechanics

### Staking & Rewards
- **Planting**: 10+ ORT to plant a seed
- **Growth**: Seeds progress through checkpoints
- **Harvesting**: Rewards based on growth score (0-100)
- **Farming**: Compound staking over time for multiplier

### Farming Strategy
- **Time-based multiplier**: 1% per day staked (max 2x)
- **Federation bonus**: +10% per member
- **Duel winnings**: Bonus for winning duels
- **Seasonal rewards**: Higher multipliers in seasons

### Counterparty Alignment
- **Input vectors**: Hash of seed payload determines alignment
- **Similarity scoring**: Dice coefficient between inputs
- **Ideology matching**: Philosophical approach tags
- **Rivalry tracking**: Win/loss history creates rivalry curves

## Implementation Checklist

- [x] x402 Payment Contract
- [ ] Economics/Farming Contract  
- [ ] Counterparty Alignment Contract
- [ ] Python SDK
- [ ] NPM SDK
- [ ] TLA+ Specifications
- [ ] BDD Features
