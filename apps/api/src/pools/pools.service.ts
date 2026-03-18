import {
  Injectable,
  NotFoundException,
  ForbiddenException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';

@Injectable()
export class PoolsService {
  constructor(private readonly prisma: PrismaService) {}

  async findAll(filters?: {
    hue?: string;
    band?: number;
    region?: string;
    qualityTier?: string;
    status?: string;
    page?: number;
    limit?: number;
  }) {
    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const where: any = {};
    if (filters?.hue) where.hue = filters.hue;
    if (filters?.band) where.band = filters.band;
    if (filters?.region) where.region = filters.region;
    if (filters?.qualityTier) where.qualityTier = filters.qualityTier;
    if (filters?.status) where.status = filters.status;

    const [pools, total] = await Promise.all([
      this.prisma.pool.findMany({
        where,
        include: {
          _count: {
            select: {
              inventoryItems: { where: { status: 'AVAILABLE' } },
            },
          },
        },
        orderBy: { createdAt: 'desc' },
        skip,
        take: limit,
      }),
      this.prisma.pool.count({ where }),
    ]);

    return {
      data: pools.map((pool) => ({
        id: pool.id,
        hue: pool.hue,
        band: pool.band,
        region: pool.region,
        qualityTier: pool.qualityTier,
        status: pool.status,
        availableCount: pool._count.inventoryItems,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  async findOne(poolId: string) {
    const pool = await this.prisma.pool.findUnique({
      where: { id: poolId },
      include: {
        rules: { where: { active: true } },
        _count: {
          select: {
            inventoryItems: { where: { status: 'AVAILABLE' } },
            claims: { where: { status: 'ACTIVE' } },
          },
        },
      },
    });

    if (!pool) {
      throw new NotFoundException('Pool not found');
    }

    return {
      id: pool.id,
      hue: pool.hue,
      band: pool.band,
      region: pool.region,
      qualityTier: pool.qualityTier,
      status: pool.status,
      onChainPoolId: pool.onChainPoolId,
      availableCount: pool._count.inventoryItems,
      activeClaimCount: pool._count.claims,
      rules: pool.rules,
    };
  }

  async getInventory(
    userId: string,
    poolId: string,
    filters?: { page?: number; limit?: number; sort?: string },
  ) {
    // Verify pool exists
    const pool = await this.prisma.pool.findUnique({
      where: { id: poolId },
    });

    if (!pool) {
      throw new NotFoundException('Pool not found');
    }

    // Verify user has an active claim in this pool
    const activeClaim = await this.prisma.claim.findFirst({
      where: {
        userId,
        poolId,
        status: 'ACTIVE',
      },
    });

    if (!activeClaim) {
      throw new ForbiddenException(
        'You need an active claim in this pool to browse inventory',
      );
    }

    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const orderBy: any =
      filters?.sort === 'oldest'
        ? { receipt: { createdAt: 'asc' } }
        : { receipt: { createdAt: 'desc' } };

    const where = { poolId, status: 'AVAILABLE' as const };

    const [items, total] = await Promise.all([
      this.prisma.inventoryItem.findMany({
        where,
        include: {
          receipt: {
            select: {
              id: true,
              conditionGrade: true,
              finalBand: true,
              gradingNotes: true,
              createdAt: true,
              submission: {
                select: {
                  category: true,
                  conditionDescription: true,
                  media: {
                    select: { id: true, s3Key: true, mediaType: true },
                  },
                },
              },
            },
          },
        },
        orderBy,
        skip,
        take: limit,
      }),
      this.prisma.inventoryItem.count({ where }),
    ]);

    return {
      data: items.map((item) => ({
        id: item.id,
        binLocation: item.binLocation,
        conditionGrade: item.receipt.conditionGrade,
        finalBand: item.receipt.finalBand,
        gradingNotes: item.receipt.gradingNotes,
        category: item.receipt.submission.category,
        conditionDescription: item.receipt.submission.conditionDescription,
        photos: item.receipt.submission.media,
        addedAt: item.receipt.createdAt,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }
}
