import { Test, TestingModule } from '@nestjs/testing';
import { ConfigService } from '@nestjs/config';
import { IndexerService } from './indexer.service';
import { PrismaService } from '../prisma/prisma.service';

describe('IndexerService', () => {
  let service: IndexerService;
  let prisma: any;

  beforeEach(async () => {
    prisma = {
      pool: { upsert: jest.fn(), findFirst: jest.fn() },
      itemReceipt: { findFirst: jest.fn(), update: jest.fn() },
      wallet: { findUnique: jest.fn() },
      claim: { findFirst: jest.fn(), update: jest.fn() },
      inventoryItem: { findUnique: jest.fn(), create: jest.fn() },
      warehouse: { findFirst: jest.fn() },
      indexerState: { findUnique: jest.fn(), upsert: jest.fn() },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        IndexerService,
        { provide: PrismaService, useValue: prisma },
        {
          provide: ConfigService,
          useValue: { get: jest.fn().mockReturnValue(undefined) },
        },
      ],
    }).compile();

    service = module.get<IndexerService>(IndexerService);
  });

  describe('handlePoolCreated', () => {
    it('should upsert a pool on PoolCreated event', async () => {
      prisma.pool.upsert.mockResolvedValue({ id: 'pool-1' });

      await service.handlePoolCreated({
        poolId: 1,
        hue: 0,
        band: 1,
        region: 'Pacific NW',
        txHash: '0xabc',
        blockNumber: 100,
      });

      expect(prisma.pool.upsert).toHaveBeenCalledWith({
        where: {
          hue_band_region_qualityTier: {
            hue: 'GREEN',
            band: 1,
            region: 'Pacific NW',
            qualityTier: 'NEW',
          },
        },
        update: { onChainPoolId: '1' },
        create: expect.objectContaining({
          hue: 'GREEN',
          band: 1,
          region: 'Pacific NW',
          onChainPoolId: '1',
        }),
      });
    });

    it('should log error for invalid hue index', async () => {
      const logSpy = jest.spyOn(service['logger'], 'error');

      await service.handlePoolCreated({
        poolId: 1,
        hue: 99,
        band: 1,
        region: 'Test',
        txHash: '0x',
        blockNumber: 1,
      });

      expect(logSpy).toHaveBeenCalledWith('Invalid hue index: 99');
      expect(prisma.pool.upsert).not.toHaveBeenCalled();
    });
  });

  describe('handleReceiptMinted', () => {
    it('should update receipt with on-chain token ID and tx hash', async () => {
      prisma.itemReceipt.findFirst.mockResolvedValue({ id: 'receipt-1' });
      prisma.itemReceipt.update.mockResolvedValue({});

      await service.handleReceiptMinted({
        tokenId: 42,
        contributor: '0xabc',
        poolId: 1,
        submissionHash: 'hash-123',
        txHash: '0xtx',
      });

      expect(prisma.itemReceipt.update).toHaveBeenCalledWith({
        where: { id: 'receipt-1' },
        data: { onChainTokenId: '42', onChainTxHash: '0xtx' },
      });
    });

    it('should skip if no receipt found', async () => {
      prisma.itemReceipt.findFirst.mockResolvedValue(null);

      await service.handleReceiptMinted({
        tokenId: 42,
        contributor: '0xabc',
        poolId: 1,
        submissionHash: 'unknown',
        txHash: '0xtx',
      });

      expect(prisma.itemReceipt.update).not.toHaveBeenCalled();
    });
  });

  describe('handleClaimMinted', () => {
    it('should activate a pending claim for the wallet owner', async () => {
      prisma.wallet.findUnique.mockResolvedValue({ userId: 'user-1' });
      prisma.pool.findFirst.mockResolvedValue({ id: 'pool-1' });
      prisma.claim.findFirst.mockResolvedValue({ id: 'claim-1' });
      prisma.claim.update.mockResolvedValue({});

      await service.handleClaimMinted({
        to: '0xuser',
        poolId: 1,
        amount: 1,
        txHash: '0xtx',
      });

      expect(prisma.claim.update).toHaveBeenCalledWith({
        where: { id: 'claim-1' },
        data: expect.objectContaining({ status: 'ACTIVE' }),
      });
    });

    it('should skip if wallet not found', async () => {
      prisma.wallet.findUnique.mockResolvedValue(null);

      await service.handleClaimMinted({
        to: '0xunknown',
        poolId: 1,
        amount: 1,
        txHash: '0xtx',
      });

      expect(prisma.claim.update).not.toHaveBeenCalled();
    });
  });

  describe('handleClaimBurned', () => {
    it('should consume an active claim for the wallet owner', async () => {
      prisma.wallet.findUnique.mockResolvedValue({ userId: 'user-1' });
      prisma.pool.findFirst.mockResolvedValue({ id: 'pool-1' });
      prisma.claim.findFirst.mockResolvedValue({ id: 'claim-1' });
      prisma.claim.update.mockResolvedValue({});

      await service.handleClaimBurned({
        from: '0xuser',
        poolId: 1,
        amount: 1,
        txHash: '0xtx',
      });

      expect(prisma.claim.update).toHaveBeenCalledWith({
        where: { id: 'claim-1' },
        data: expect.objectContaining({ status: 'CONSUMED' }),
      });
    });
  });

  describe('handleInventoryRegistered', () => {
    it('should create inventory item linked to receipt', async () => {
      prisma.itemReceipt.findFirst.mockResolvedValue({ id: 'receipt-1' });
      prisma.inventoryItem.findUnique.mockResolvedValue(null);
      prisma.pool.findFirst.mockResolvedValue({ id: 'pool-1', region: 'PDX' });
      prisma.warehouse.findFirst.mockResolvedValue({ id: 'wh-1' });
      prisma.inventoryItem.create.mockResolvedValue({ id: 'inv-1' });

      await service.handleInventoryRegistered({
        inventoryId: 1,
        receiptId: 42,
        poolId: 1,
        txHash: '0xtx',
      });

      expect(prisma.inventoryItem.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          receiptId: 'receipt-1',
          warehouseId: 'wh-1',
          poolId: 'pool-1',
          status: 'AVAILABLE',
        }),
      });
    });

    it('should skip if inventory already exists for receipt', async () => {
      prisma.itemReceipt.findFirst.mockResolvedValue({ id: 'receipt-1' });
      prisma.inventoryItem.findUnique.mockResolvedValue({ id: 'existing' });

      await service.handleInventoryRegistered({
        inventoryId: 1,
        receiptId: 42,
        poolId: 1,
        txHash: '0xtx',
      });

      expect(prisma.inventoryItem.create).not.toHaveBeenCalled();
    });
  });

  describe('getLastIndexedBlock / setLastIndexedBlock', () => {
    it('should return 0 when no state exists', async () => {
      prisma.indexerState.findUnique.mockResolvedValue(null);

      const block = await service.getLastIndexedBlock();
      expect(block).toBe(0);
    });

    it('should return stored block number', async () => {
      prisma.indexerState.findUnique.mockResolvedValue({
        key: 'lastIndexedBlock',
        value: '12345',
      });

      const block = await service.getLastIndexedBlock();
      expect(block).toBe(12345);
    });

    it('should persist block number', async () => {
      prisma.indexerState.upsert.mockResolvedValue({});

      await service.setLastIndexedBlock(99999);

      expect(prisma.indexerState.upsert).toHaveBeenCalledWith({
        where: { key: 'lastIndexedBlock' },
        update: { value: '99999' },
        create: { key: 'lastIndexedBlock', value: '99999' },
      });
    });
  });
});
