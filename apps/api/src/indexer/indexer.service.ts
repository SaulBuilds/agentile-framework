import { Injectable, Logger, OnModuleInit } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { PrismaService } from '../prisma/prisma.service';

@Injectable()
export class IndexerService implements OnModuleInit {
  private readonly logger = new Logger(IndexerService.name);

  constructor(
    private readonly prisma: PrismaService,
    private readonly config: ConfigService,
  ) {}

  onModuleInit() {
    const rpcUrl = this.config.get<string>('RPC_URL');
    if (rpcUrl) {
      this.logger.log(`Indexer configured with RPC: ${rpcUrl}`);
    } else {
      this.logger.warn(
        'RPC_URL not configured — indexer running in scaffold mode',
      );
    }
  }

  async handlePoolCreated(event: {
    poolId: number;
    hue: number;
    band: number;
    region: string;
    txHash: string;
    blockNumber: number;
  }) {
    this.logger.log(
      `PoolCreated: poolId=${event.poolId} hue=${event.hue} band=${event.band}`,
    );

    const hueMap = ['GREEN', 'BLUE', 'AMBER', 'VIOLET', 'TEAL', 'RED'] as const;
    const hue = hueMap[event.hue];
    if (!hue) {
      this.logger.error(`Invalid hue index: ${event.hue}`);
      return;
    }

    await this.prisma.pool.upsert({
      where: {
        hue_band_region_qualityTier: {
          hue,
          band: event.band,
          region: event.region,
          qualityTier: 'NEW',
        },
      },
      update: {
        onChainPoolId: String(event.poolId),
      },
      create: {
        hue,
        band: event.band,
        region: event.region,
        qualityTier: 'NEW',
        onChainPoolId: String(event.poolId),
      },
    });
  }

  async handleReceiptMinted(event: {
    tokenId: number;
    contributor: string;
    poolId: number;
    submissionHash: string;
    txHash: string;
  }) {
    this.logger.log(
      `ReceiptMinted: tokenId=${event.tokenId} contributor=${event.contributor}`,
    );

    // Find the receipt by matching the submission hash
    const receipt = await this.prisma.itemReceipt.findFirst({
      where: {
        submission: { submissionHash: event.submissionHash },
      },
    });

    if (!receipt) {
      this.logger.warn(
        `No receipt found for submissionHash=${event.submissionHash}`,
      );
      return;
    }

    await this.prisma.itemReceipt.update({
      where: { id: receipt.id },
      data: {
        onChainTokenId: String(event.tokenId),
        onChainTxHash: event.txHash,
      },
    });
  }

  async handleClaimMinted(event: {
    to: string;
    poolId: number;
    amount: number;
    txHash: string;
  }) {
    this.logger.log(
      `ClaimMinted: to=${event.to} poolId=${event.poolId} amount=${event.amount}`,
    );

    // Find user by wallet address
    const wallet = await this.prisma.wallet.findUnique({
      where: { address: event.to },
    });

    if (!wallet) {
      this.logger.warn(`No wallet found for address=${event.to}`);
      return;
    }

    // Find the pool by on-chain ID
    const pool = await this.prisma.pool.findFirst({
      where: { onChainPoolId: String(event.poolId) },
    });

    if (!pool) {
      this.logger.warn(`No pool found for onChainPoolId=${event.poolId}`);
      return;
    }

    // Find the most recent pending claim for this user+pool and activate it
    const claim = await this.prisma.claim.findFirst({
      where: {
        userId: wallet.userId,
        poolId: pool.id,
        status: 'PENDING',
      },
      orderBy: { id: 'desc' },
    });

    if (claim) {
      await this.prisma.claim.update({
        where: { id: claim.id },
        data: {
          status: 'ACTIVE',
          onChainTokenId: String(event.poolId),
          activatedAt: new Date(),
        },
      });
    }
  }

  async handleClaimBurned(event: {
    from: string;
    poolId: number;
    amount: number;
    txHash: string;
  }) {
    this.logger.log(
      `ClaimBurned: from=${event.from} poolId=${event.poolId} amount=${event.amount}`,
    );

    const wallet = await this.prisma.wallet.findUnique({
      where: { address: event.from },
    });

    if (!wallet) {
      this.logger.warn(`No wallet found for address=${event.from}`);
      return;
    }

    const pool = await this.prisma.pool.findFirst({
      where: { onChainPoolId: String(event.poolId) },
    });

    if (!pool) {
      this.logger.warn(`No pool found for onChainPoolId=${event.poolId}`);
      return;
    }

    // Find the most recent active claim for this user+pool and consume it
    const claim = await this.prisma.claim.findFirst({
      where: {
        userId: wallet.userId,
        poolId: pool.id,
        status: 'ACTIVE',
      },
      orderBy: { activatedAt: 'desc' },
    });

    if (claim) {
      await this.prisma.claim.update({
        where: { id: claim.id },
        data: {
          status: 'CONSUMED',
          consumedAt: new Date(),
        },
      });
    }
  }

  async handleInventoryRegistered(event: {
    inventoryId: number;
    receiptId: number;
    poolId: number;
    txHash: string;
  }) {
    this.logger.log(
      `InventoryRegistered: invId=${event.inventoryId} receiptId=${event.receiptId}`,
    );

    // Find the receipt by on-chain token ID
    const receipt = await this.prisma.itemReceipt.findFirst({
      where: { onChainTokenId: String(event.receiptId) },
    });

    if (!receipt) {
      this.logger.warn(
        `No receipt found for onChainTokenId=${event.receiptId}`,
      );
      return;
    }

    // Check if inventory item already exists for this receipt
    const existing = await this.prisma.inventoryItem.findUnique({
      where: { receiptId: receipt.id },
    });

    if (existing) {
      this.logger.warn(
        `InventoryItem already exists for receipt=${receipt.id}`,
      );
      return;
    }

    // Find warehouse for the pool's region
    const pool = await this.prisma.pool.findFirst({
      where: { onChainPoolId: String(event.poolId) },
    });

    if (!pool) {
      this.logger.warn(`No pool found for onChainPoolId=${event.poolId}`);
      return;
    }

    const warehouse = await this.prisma.warehouse.findFirst({
      where: { region: pool.region, status: 'ACTIVE_WH' },
    });

    if (!warehouse) {
      this.logger.warn(`No active warehouse found for region=${pool.region}`);
      return;
    }

    await this.prisma.inventoryItem.create({
      data: {
        receiptId: receipt.id,
        warehouseId: warehouse.id,
        poolId: pool.id,
        status: 'AVAILABLE',
      },
    });
  }

  async getLastIndexedBlock(): Promise<number> {
    const state = await this.prisma.indexerState.findUnique({
      where: { key: 'lastIndexedBlock' },
    });
    return state ? parseInt(state.value, 10) : 0;
  }

  async setLastIndexedBlock(blockNumber: number): Promise<void> {
    await this.prisma.indexerState.upsert({
      where: { key: 'lastIndexedBlock' },
      update: { value: String(blockNumber) },
      create: { key: 'lastIndexedBlock', value: String(blockNumber) },
    });
  }
}
