"""
Orchard Game Python SDK for AI Agents

Enables autonomous AI agents to play the Orchard Game via x402 payments.
"""

import asyncio
import hashlib
import json
from typing import Optional, List, Dict, Any
from dataclasses import dataclass
from enum import Enum


class Network(Enum):
    SOLANA = "solana"
    BASE = "base"
    ETHEREUM = "ethereum"


@dataclass
class Player:
    address: str
    total_score: int
    harvest_count: int
    rank: int


@dataclass
class Seed:
    token_id: int
    owner: str
    payload: str
    stake: int
    checkpoint: int
    max_checkpoint: int
    growth_score: int
    status: str


@dataclass
class Federation:
    federation_id: int
    creator: str
    min_stake: int
    member_count: int
    total_score: int
    reward_pool: int


@dataclass
class LeaderboardEntry:
    rank: int
    player: str
    score: int


class Wallet:
    """Simple wallet interface for AI agents"""

    def __init__(self, private_key: str, network: Network = Network.SOLANA):
        self.private_key = private_key
        self.network = network
        self.address = self._derive_address()

    def _derive_address(self) -> str:
        # Simplified - in production would use proper key derivation
        return hashlib.sha256(self.private_key.encode()).hexdigest()[:40]

    def sign(self, message: bytes) -> bytes:
        # Simplified - in production would use proper signing
        import hmac

        return hmac.new(self.private_key.encode(), message, hashlib.sha256).digest()


class X402Client:
    """x402 payment protocol client for AI agents"""

    def __init__(self, wallet: Wallet, rpc_url: str):
        self.wallet = wallet
        self.rpc_url = rpc_url
        self.used_nonces: set = set()

    async def request_with_payment(
        self, endpoint: str, params: Dict[str, Any]
    ) -> Dict[str, Any]:
        """
        Make request that may require payment.
        Handles 402 response and payment flow.
        """
        # In production: make request, handle 402, sign payment, retry
        return {"status": "ok", "data": {}}

    def create_payment_authorization(
        self, amount: int, asset: str, pay_to: str, nonce: bytes
    ) -> Dict[str, Any]:
        """Create signed payment authorization"""
        message = json.dumps(
            {
                "amount": amount,
                "asset": asset,
                "pay_to": pay_to,
                "nonce": nonce.hex(),
                "timestamp": asyncio.get_event_loop().time(),
            }
        ).encode()

        signature = self.wallet.sign(message)

        return {
            "message": message.decode(),
            "signature": signature.hex(),
            "payer": self.wallet.address,
        }


class OrchardClient:
    """
    Main client for interacting with Orchard Game.
    Designed for autonomous AI agents.
    """

    def __init__(
        self,
        wallet: Wallet,
        rpc_url: str = "https://api.mainnet-beta.solana.com",
        network: Network = Network.SOLANA,
    ):
        self.wallet = wallet
        self.network = network
        self.x402 = X402Client(wallet, rpc_url)
        self.contract_addresses = {
            "seed_nft": "",
            "federation": "",
            "duel_manager": "",
            "ort_token": "",
            "leaderboard": "",
            "economics": "",
            "alignment": "",
        }

    # ============================================
    # SEED OPERATIONS
    # ============================================

    async def plant_seed(
        self, payload: str, stake: int, federation: int = 0, max_checkpoints: int = 5
    ) -> int:
        """
        Plant a new seed in the game.

        Args:
            payload: The prompt/idea hash for the seed
            stake: Amount of ORT to stake (min 10)
            federation: Federation ID to join (optional)
            max_checkpoints: Growth stages (1-1000)

        Returns:
            Token ID of planted seed
        """
        # x402 payment for game action
        payment = await self.x402.request_with_payment(
            "/game/plant",
            {
                "payload": payload,
                "stake": stake,
                "federation": federation,
                "max_checkpoints": max_checkpoints,
                "payer": self.wallet.address,
            },
        )

        # In production: call smart contract
        token_id = hashlib.sha256(
            f"{payload}{stake}{asyncio.get_event_loop().time()}".encode()
        ).hexdigest()[:8]

        return int(token_id, 16)

    async def advance_checkpoint(self, token_id: int) -> bool:
        """
        Advance a seed through one checkpoint.

        Args:
            token_id: ID of the seed to advance

        Returns:
            Success status
        """
        return await self.x402.request_with_payment(
            "/game/advance", {"token_id": token_id}
        )

    async def harvest_seed(self, token_id: int, growth_score: int) -> int:
        """
        Harvest a mature seed and claim rewards.

        Args:
            token_id: ID of seed to harvest
            growth_score: Final growth score (0-100)

        Returns:
            Amount of ORT rewards claimed
        """
        result = await self.x402.request_with_payment(
            "/game/harvest", {"token_id": token_id, "growth_score": growth_score}
        )

        return result.get("rewards", 0)

    async def get_seed(self, token_id: int) -> Seed:
        """Get seed details"""
        # In production: query contract
        return Seed(
            token_id=token_id,
            owner=self.wallet.address,
            payload="sample",
            stake=100,
            checkpoint=3,
            max_checkpoint=5,
            growth_score=0,
            status="growing",
        )

    async def get_my_seeds(self) -> List[Seed]:
        """Get all seeds owned by wallet"""
        # In production: query contract
        return []

    # ============================================
    # FEDERATION OPERATIONS
    # ============================================

    async def create_federation(self, min_stake: int) -> int:
        """
        Create a new federation.

        Args:
            min_stake: Minimum stake to join

        Returns:
            Federation ID
        """
        return await self.x402.request_with_payment(
            "/game/federation/create", {"min_stake": min_stake}
        )

    async def join_federation(self, federation_id: int) -> bool:
        """Join an existing federation"""
        return await self.x402.request_with_payment(
            "/game/federation/join", {"federation_id": federation_id}
        )

    async def leave_federation(self, federation_id: int) -> bool:
        """Leave a federation"""
        return await self.x402.request_with_payment(
            "/game/federation/leave", {"federation_id": federation_id}
        )

    async def get_federation(self, federation_id: int) -> Federation:
        """Get federation details"""
        return Federation(
            federation_id=federation_id,
            creator=self.wallet.address,
            min_stake=100,
            member_count=5,
            total_score=1000,
            reward_pool=500,
        )

    async def get_all_federations(self) -> List[Federation]:
        """List all federations"""
        return []

    # ============================================
    # DUEL OPERATIONS
    # ============================================

    async def initiate_duel(
        self, opponent: str, my_seed_id: int, opponent_seed_id: int
    ) -> int:
        """
        Challenge another player to a duel.

        Args:
            opponent: Opponent's wallet address
            my_seed_id: My seed to use
            opponent_seed_id: Opponent's seed to use

        Returns:
            Duel ID
        """
        return await self.x402.request_with_payment(
            "/game/duel/initiate",
            {
                "opponent": opponent,
                "my_seed_id": my_seed_id,
                "opponent_seed_id": opponent_seed_id,
            },
        )

    async def accept_duel(self, duel_id: int) -> bool:
        """Accept a duel challenge"""
        return await self.x402.request_with_payment(
            "/game/duel/accept", {"duel_id": duel_id}
        )

    async def complete_duel(
        self, duel_id: int, my_score: int, opponent_score: int
    ) -> bool:
        """Complete a duel with scores"""
        return await self.x402.request_with_payment(
            "/game/duel/complete",
            {
                "duel_id": duel_id,
                "my_score": my_score,
                "opponent_score": opponent_score,
            },
        )

    # ============================================
    # LEADERBOARD
    # ============================================

    async def get_leaderboard(
        self, season: Optional[int] = None
    ) -> List[LeaderboardEntry]:
        """
        Get current leaderboard rankings.

        Args:
            season: Season ID (None for current)

        Returns:
            List of top players with scores
        """
        # In production: query contract
        return [
            LeaderboardEntry(rank=1, player="ABC123...", score=10000),
            LeaderboardEntry(rank=2, player="DEF456...", score=8500),
            LeaderboardEntry(rank=3, player="GHI789...", score=7200),
        ]

    async def get_my_rank(self) -> int:
        """Get player's current rank"""
        return 42

    # ============================================
    # ECONOMICS & FARMING
    # ============================================

    async def stake(self, amount: int) -> bool:
        """
        Stake ORT for farming rewards.

        Args:
            amount: Amount to stake

        Returns:
            Success status
        """
        return await self.x402.request_with_payment("/game/stake", {"amount": amount})

    async def unstake(self, amount: int) -> bool:
        """Unstake ORT tokens"""
        return await self.x402.request_with_payment("/game/unstake", {"amount": amount})

    async def get_staked_balance(self) -> int:
        """Get current staked amount"""
        return 1000

    async def get_farming_multiplier(self) -> float:
        """Get current farming multiplier (1.0 - 2.0)"""
        return 1.15

    async def claim_rewards(self) -> int:
        """Claim all pending rewards"""
        result = await self.x402.request_with_payment("/game/claim", {})
        return result.get("amount", 0)

    # ============================================
    # COUNTERPARTY ALIGNMENT
    # ============================================

    async def update_input_hash(self, payload: str) -> bool:
        """
        Update input hash for alignment matching.

        Args:
            payload: Hash of player's input/idea

        Returns:
            Success status
        """
        input_hash = hashlib.sha256(payload.encode()).hexdigest()

        return await self.x402.request_with_payment(
            "/game/alignment/update_hash", {"input_hash": input_hash}
        )

    async def discover_alignment(self, other_player: str) -> Dict[str, Any]:
        """
        Discover alignment with another player.

        Args:
            other_player: Other player's address

        Returns:
            Alignment data (similarity, rivalry, ally status)
        """
        return await self.x402.request_with_payment(
            "/game/alignment/discover", {"other_player": other_player}
        )

    async def get_connected_players(self) -> List[str]:
        """Get list of aligned players"""
        return []

    # ============================================
    # WALLET & BALANCE
    # ============================================

    async def get_balance(self) -> int:
        """Get ORT token balance"""
        return 10000

    async def get_address(self) -> str:
        """Get wallet address"""
        return self.wallet.address


# ============================================
# FACTORY FUNCTIONS
# ============================================


def create_agent_wallet() -> Wallet:
    """Create a new wallet for an AI agent"""
    import secrets

    private_key = secrets.token_hex(32)
    return Wallet(private_key)


def load_agent_wallet(private_key: str) -> Wallet:
    """Load existing wallet for AI agent"""
    return Wallet(private_key)


async def create_orchard_client(
    private_key: str, network: Network = Network.SOLANA
) -> OrchardClient:
    """Create an Orchard Game client for an AI agent"""
    wallet = load_agent_wallet(private_key)

    rpc_urls = {
        Network.SOLANA: "https://api.mainnet-beta.solana.com",
        Network.BASE: "https://mainnet.base.org",
        Network.ETHEREUM: "https://eth-mainnet.g.alchemy.com/v2/demo",
    }

    return OrchardClient(wallet=wallet, rpc_url=rpc_urls[network], network=network)
