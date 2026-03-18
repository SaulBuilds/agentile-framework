import { Test, TestingModule } from '@nestjs/testing';
import { NotFoundException, ForbiddenException } from '@nestjs/common';
import { PoolsService } from './pools.service';
import { PrismaService } from '../prisma/prisma.service';

describe('PoolsService', () => {
  let service: PoolsService;
  let prisma: any;

  beforeEach(async () => {
    prisma = {
      pool: {
        findMany: jest.fn(),
        findUnique: jest.fn(),
        count: jest.fn(),
      },
      claim: { findFirst: jest.fn() },
      inventoryItem: {
        findMany: jest.fn(),
        count: jest.fn(),
      },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        PoolsService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<PoolsService>(PoolsService);
  });

  describe('findAll', () => {
    it('should return paginated pools with available counts', async () => {
      prisma.pool.findMany.mockResolvedValue([
        {
          id: 'pool-1',
          hue: 'GREEN',
          band: 1,
          region: 'PDX',
          qualityTier: 'NEW',
          status: 'ACTIVE',
          _count: { inventoryItems: 5 },
        },
      ]);
      prisma.pool.count.mockResolvedValue(1);

      const result = await service.findAll();

      expect(result.data).toHaveLength(1);
      expect(result.data[0].availableCount).toBe(5);
      expect(result.meta).toEqual({
        total: 1,
        page: 1,
        limit: 20,
        pages: 1,
      });
    });

    it('should filter pools by hue and band', async () => {
      prisma.pool.findMany.mockResolvedValue([]);
      prisma.pool.count.mockResolvedValue(0);

      await service.findAll({ hue: 'BLUE', band: 2 });

      expect(prisma.pool.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: { hue: 'BLUE', band: 2 },
        }),
      );
    });
  });

  describe('findOne', () => {
    it('should return pool with counts and rules', async () => {
      prisma.pool.findUnique.mockResolvedValue({
        id: 'pool-1',
        hue: 'GREEN',
        band: 1,
        region: 'PDX',
        qualityTier: 'NEW',
        status: 'ACTIVE',
        onChainPoolId: '1',
        rules: [],
        _count: { inventoryItems: 3, claims: 2 },
      });

      const result = await service.findOne('pool-1');

      expect(result.availableCount).toBe(3);
      expect(result.activeClaimCount).toBe(2);
    });

    it('should throw NotFoundException for unknown pool', async () => {
      prisma.pool.findUnique.mockResolvedValue(null);

      await expect(service.findOne('nope')).rejects.toThrow(
        NotFoundException,
      );
    });
  });

  describe('getInventory', () => {
    it('should return available items for claim holders', async () => {
      prisma.pool.findUnique.mockResolvedValue({ id: 'pool-1' });
      prisma.claim.findFirst.mockResolvedValue({ id: 'claim-1', status: 'ACTIVE' });
      prisma.inventoryItem.findMany.mockResolvedValue([
        {
          id: 'inv-1',
          binLocation: 'A1',
          receipt: {
            id: 'r-1',
            conditionGrade: 'good',
            finalBand: 2,
            gradingNotes: 'Clean',
            createdAt: new Date(),
            submission: {
              category: 'tools',
              conditionDescription: 'Lightly used',
              media: [{ id: 'm-1', s3Key: 'photo.jpg', mediaType: 'PHOTO' }],
            },
          },
        },
      ]);
      prisma.inventoryItem.count.mockResolvedValue(1);

      const result = await service.getInventory('user-1', 'pool-1');

      expect(result.data).toHaveLength(1);
      expect(result.data[0].conditionGrade).toBe('good');
      expect(result.data[0].photos).toHaveLength(1);
    });

    it('should throw ForbiddenException when user has no active claim', async () => {
      prisma.pool.findUnique.mockResolvedValue({ id: 'pool-1' });
      prisma.claim.findFirst.mockResolvedValue(null);

      await expect(
        service.getInventory('user-1', 'pool-1'),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should throw NotFoundException for unknown pool', async () => {
      prisma.pool.findUnique.mockResolvedValue(null);

      await expect(
        service.getInventory('user-1', 'nope'),
      ).rejects.toThrow(NotFoundException);
    });
  });
});
