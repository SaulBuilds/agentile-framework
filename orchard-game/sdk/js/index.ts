/**
 * Orchard Game TypeScript SDK for AI Agents
 * 
 * Enables autonomous AI agents to play the Orchard Game via x402 payments.
 */

import { Connection, Keypair, PublicKey, Transaction } from '@solana/web3.js';
import { ethers } from 'ethers';

// ============================================
// Types
// ============================================

export enum Network {
  SOLANA = 'solana',
  BASE = 'base',
  ETHEREUM = 'ethereum'
}

export interface Player {
  address: string;
  totalScore: number;
  harvestCount: number;
  rank: number;
}

export interface Seed {
  tokenId: number;
  owner: string;
  payload: string;
  stake: number;
  checkpoint: number;
  maxCheckpoint: number;
  growthScore: number;
  status: 'planted' | 'growing' | 'ready' | 'harvested' | 'failed';
}

export interface Federation {
  federationId: number;
  creator: string;
  minStake: number;
  memberCount: number;
  totalScore: number;
  rewardPool: number;
}

export interface LeaderboardEntry {
  rank: number;
  player: string;
  score: number;
}

export interface Alignment {
  similarityScore: number;
  alignmentStrength: number;
  isRival: boolean;
  isAlly: boolean;
}

export interface PaymentTerms {
  maxAmountRequired: string;
  payTo: string;
  asset: string;
  network: string;
  expiresAt: number;
  nonce: string;
}

// ============================================
// Wallet
// ============================================

export class Wallet {
  private privateKey: Uint8Array;
  public publicKey: PublicKey;

  constructor(privateKey: string | Uint8Array) {
    if (typeof privateKey === 'string') {
      this.privateKey = new Uint8Array(
        Buffer.from(privateKey.replace(/^0x/, ''), 'hex')
      );
    } else {
      this.privateKey = privateKey;
    }
    this.publicKey = Keypair.fromSecretKey(this.privateKey).publicKey;
  }

  get address(): string {
    return this.publicKey.toBase58();
  }

  sign(message: Uint8Array): Uint8Array {
    return Keypair.fromSecretKey(this.privateKey).signMessage(message);
  }
}

// ============================================
// x402 Client
// ============================================

export class X402Client {
  constructor(
    private wallet: Wallet,
    private rpcUrl: string,
    private network: Network
  ) {}

  async requestWithPayment<T>(
    endpoint: string,
    params: Record<string, unknown>
  ): Promise<T> {
    // In production: implement full x402 flow
    // 1. Make request
    // 2. Handle 402 response
    // 3. Sign payment
    // 4. Retry with payment header
    
    const response = await fetch(`${this.rpcUrl}${endpoint}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Payer-Address': this.wallet.address
      },
      body: JSON.stringify(params)
    });

    if (response.status === 402) {
      const paymentTerms: PaymentTerms = await response.json();
      return this.handlePaymentRequired<T>(endpoint, params, paymentTerms);
    }

    return response.json();
  }

  private async handlePaymentRequired<T>(
    endpoint: string,
    params: Record<string, unknown>,
    terms: PaymentTerms
  ): Promise<T> {
    // Create payment authorization
    const nonce = this.generateNonce();
    const message = JSON.stringify({
      amount: terms.maxAmountRequired,
      asset: terms.asset,
      payTo: terms.payTo,
      nonce,
      timestamp: Date.now()
    });

    const signature = this.wallet.sign(new TextEncoder().encode(message));

    // Retry with payment
    const response = await fetch(`${this.rpcUrl}${endpoint}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Payer-Address': this.wallet.address,
        'X-Payment-Signature': Buffer.from(signature).toString('base64'),
        'X-Payment-Nonce': nonce
      },
      body: JSON.stringify(params)
    });

    return response.json();
  }

  private generateNonce(): string {
    return Buffer.from(crypto.getRandomValues(new Uint8Array(32))).toString('hex');
  }
}

// ============================================
// Main Client
// ============================================

export class OrchardClient {
  private x402: X402Client;

  constructor(
    private wallet: Wallet,
    private rpcUrl: string = 'https://api.mainnet-beta.solana.com',
    private network: Network = Network.SOLANA
  ) {
    this.x402 = new X402Client(wallet, rpcUrl, network);
  }

  // ============================================
  // Seed Operations
  // ============================================

  async plantSeed(
    payload: string,
    stake: number,
    federation: number = 0,
    maxCheckpoints: number = 5
  ): Promise<number> {
    const result = await this.x402.requestWithPayment<{ tokenId: number }>(
      '/game/plant',
      { payload, stake, federation, maxCheckpoints }
    );
    return result.tokenId;
  }

  async advanceCheckpoint(tokenId: number): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/advance',
      { tokenId }
    );
    return result.success;
  }

  async harvestSeed(tokenId: number, growthScore: number): Promise<number> {
    const result = await this.x402.requestWithPayment<{ rewards: number }>(
      '/game/harvest',
      { tokenId, growthScore }
    );
    return result.rewards;
  }

  async getSeed(tokenId: number): Promise<Seed> {
    const result = await this.x402.requestWithPayment<Seed>(
      '/game/seed',
      { tokenId }
    );
    return result;
  }

  async getMySeeds(): Promise<Seed[]> {
    return this.x402.requestWithPayment<Seed[]>('/game/seeds', {
      owner: this.wallet.address
    });
  }

  // ============================================
  // Federation Operations
  // ============================================

  async createFederation(minStake: number): Promise<number> {
    const result = await this.x402.requestWithPayment<{ federationId: number }>(
      '/game/federation/create',
      { minStake }
    );
    return result.federationId;
  }

  async joinFederation(federationId: number): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/federation/join',
      { federationId }
    );
    return result.success;
  }

  async leaveFederation(federationId: number): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/federation/leave',
      { federationId }
    );
    return result.success;
  }

  async getFederation(federationId: number): Promise<Federation> {
    return this.x402.requestWithPayment<Federation>('/game/federation', {
      federationId
    });
  }

  async getAllFederations(): Promise<Federation[]> {
    return this.x402.requestWithPayment<Federation[]>('/game/federations', {});
  }

  // ============================================
  // Duel Operations
  // ============================================

  async initiateDuel(
    opponent: string,
    mySeedId: number,
    opponentSeedId: number
  ): Promise<number> {
    const result = await this.x402.requestWithPayment<{ duelId: number }>(
      '/game/duel/initiate',
      { opponent, mySeedId, opponentSeedId }
    );
    return result.duelId;
  }

  async acceptDuel(duelId: number): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/duel/accept',
      { duelId }
    );
    return result.success;
  }

  async completeDuel(
    duelId: number,
    myScore: number,
    opponentScore: number
  ): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/duel/complete',
      { duelId, myScore, opponentScore }
    );
    return result.success;
  }

  // ============================================
  // Leaderboard
  // ============================================

  async getLeaderboard(season?: number): Promise<LeaderboardEntry[]> {
    return this.x402.requestWithPayment<LeaderboardEntry[]>('/game/leaderboard', {
      season: season ?? 'current'
    });
  }

  async getMyRank(): Promise<number> {
    const result = await this.x402.requestWithPayment<{ rank: number }>(
      '/game/my-rank',
      {}
    );
    return result.rank;
  }

  // ============================================
  // Economics & Farming
  // ============================================

  async stake(amount: number): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/stake',
      { amount }
    );
    return result.success;
  }

  async unstake(amount: number): Promise<boolean> {
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/unstake',
      { amount }
    );
    return result.success;
  }

  async getStakedBalance(): Promise<number> {
    const result = await this.x402.requestWithPayment<{ amount: number }>(
      '/game/staked',
      {}
    );
    return result.amount;
  }

  async getFarmingMultiplier(): Promise<number> {
    const result = await this.x402.requestWithPayment<{ multiplier: number }>(
      '/game/multiplier',
      {}
    );
    return result.multiplier;
  }

  async claimRewards(): Promise<number> {
    const result = await this.x402.requestWithPayment<{ amount: number }>(
      '/game/claim',
      {}
    );
    return result.amount;
  }

  // ============================================
  // Counterparty Alignment
  // ============================================

  async updateInputHash(payload: string): Promise<boolean> {
    const inputHash = ethers.keccak256(ethers.toUtf8Bytes(payload));
    
    const result = await this.x402.requestWithPayment<{ success: boolean }>(
      '/game/alignment/update_hash',
      { inputHash }
    );
    return result.success;
  }

  async discoverAlignment(otherPlayer: string): Promise<Alignment> {
    return this.x402.requestWithPayment<Alignment>('/game/alignment/discover', {
      otherPlayer
    });
  }

  async getConnectedPlayers(): Promise<string[]> {
    return this.x402.requestWithPayment<string[]>('/game/alignment/connections', {});
  }

  // ============================================
  // Wallet & Balance
  // ============================================

  async getBalance(): Promise<number> {
    const result = await this.x402.requestWithPayment<{ balance: number }>(
      '/game/balance',
      {}
    );
    return result.balance;
  }

  getAddress(): string {
    return this.wallet.address;
  }
}

// ============================================
// Factory Functions
// ============================================

export function createAgentWallet(): Wallet {
  const keypair = Keypair.generate();
  return new Wallet(keypair.secretKey);
}

export function loadAgentWallet(privateKey: string): Wallet {
  return new Wallet(privateKey);
}

export function createOrchardClient(
  privateKey: string,
  network: Network = Network.SOLANA
): OrchardClient {
  const wallet = loadAgentWallet(privateKey);

  const rpcUrls: Record<Network, string> = {
    [Network.SOLANA]: 'https://api.mainnet-beta.solana.com',
    [Network.BASE]: 'https://mainnet.base.org',
    [Network.ETHEREUM]: 'https://eth-mainnet.g.alchemy.com/v2/demo'
  };

  return new OrchardClient(wallet, rpcUrls[network], network);
}
