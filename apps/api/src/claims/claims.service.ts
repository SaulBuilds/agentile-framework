import {
  Injectable,
  Logger,
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { DeliveryService } from '../delivery/delivery.service';

const RESERVATION_TIMEOUT_MS = 24 * 60 * 60 * 1000; // 24 hours

@Injectable()
export class ClaimsService {
  private readonly logger = new Logger(ClaimsService.name);

  constructor(
    private readonly prisma: PrismaService,
    private readonly deliveryService: DeliveryService,
  ) {}

  async findAll(
    userId: string,
    filters?: { status?: string; poolId?: string; page?: number; limit?: number },
  ) {
    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const where: any = { userId };
    if (filters?.status) where.status = filters.status;
    if (filters?.poolId) where.poolId = filters.poolId;

    const [claims, total] = await Promise.all([
      this.prisma.claim.findMany({
        where,
        include: {
          pool: {
            select: { id: true, hue: true, band: true, region: true, qualityTier: true },
          },
        },
        orderBy: { id: 'desc' },
        skip,
        take: limit,
      }),
      this.prisma.claim.count({ where }),
    ]);

    return {
      data: claims.map((claim) => ({
        id: claim.id,
        poolId: claim.poolId,
        status: claim.status,
        activatedAt: claim.activatedAt,
        consumedAt: claim.consumedAt,
        pool: claim.pool,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  async findOne(userId: string, claimId: string) {
    const claim = await this.prisma.claim.findUnique({
      where: { id: claimId },
      include: {
        pool: {
          select: { id: true, hue: true, band: true, region: true, qualityTier: true, status: true },
        },
        receipt: {
          select: {
            id: true,
            finalBand: true,
            conditionGrade: true,
            submission: { select: { category: true } },
          },
        },
        consumption: true,
      },
    });

    if (!claim) {
      throw new NotFoundException('Claim not found');
    }

    if (claim.userId !== userId) {
      throw new ForbiddenException('Not your claim');
    }

    // Determine available actions based on status
    const actions: string[] = [];
    if (claim.status === 'ACTIVE') {
      actions.push('reserve');
    }

    // Check if there's an active reservation
    const reservation = await this.getActiveReservation(claimId);

    return {
      id: claim.id,
      poolId: claim.poolId,
      status: claim.status,
      activatedAt: claim.activatedAt,
      consumedAt: claim.consumedAt,
      pool: claim.pool,
      receipt: claim.receipt,
      consumption: claim.consumption,
      reservation,
      actions,
    };
  }

  async reserve(userId: string, claimId: string, inventoryId: string) {
    const claim = await this.prisma.claim.findUnique({
      where: { id: claimId },
    });

    if (!claim) {
      throw new NotFoundException('Claim not found');
    }

    if (claim.userId !== userId) {
      throw new ForbiddenException('Not your claim');
    }

    if (claim.status !== 'ACTIVE') {
      throw new BadRequestException(
        `Cannot reserve with claim in ${claim.status} status`,
      );
    }

    // Check if claim already has an active reservation
    const existingReservation = await this.getActiveReservation(claimId);
    if (existingReservation) {
      throw new BadRequestException('Claim already has an active reservation');
    }

    // Check inventory item
    const item = await this.prisma.inventoryItem.findUnique({
      where: { id: inventoryId },
    });

    if (!item) {
      throw new NotFoundException('Inventory item not found');
    }

    if (item.status !== 'AVAILABLE') {
      throw new BadRequestException(
        `Item is not available (current status: ${item.status})`,
      );
    }

    if (item.poolId !== claim.poolId) {
      throw new BadRequestException(
        'Item is not in the same pool as your claim',
      );
    }

    const expiresAt = new Date(Date.now() + RESERVATION_TIMEOUT_MS);

    // Atomically reserve the item
    await this.prisma.$transaction(async (tx) => {
      await tx.inventoryItem.update({
        where: { id: inventoryId },
        data: {
          status: 'RESERVED',
          reservedById: userId,
          reservedAt: new Date(),
          reservationExpires: expiresAt,
        },
      });

      await tx.inventoryStatusEvent.create({
        data: {
          inventoryId,
          previousStatus: 'AVAILABLE',
          newStatus: 'RESERVED',
          actorId: userId,
        },
      });

      // Create claim consumption record to link claim ↔ inventory
      await tx.claimConsumption.create({
        data: {
          claimId,
          inventoryId,
        },
      });
    });

    return {
      claimId,
      inventoryId,
      expiresAt: expiresAt.toISOString(),
    };
  }

  async cancelReservation(userId: string, claimId: string) {
    const claim = await this.prisma.claim.findUnique({
      where: { id: claimId },
      include: { consumption: true },
    });

    if (!claim) {
      throw new NotFoundException('Claim not found');
    }

    if (claim.userId !== userId) {
      throw new ForbiddenException('Not your claim');
    }

    if (!claim.consumption) {
      throw new BadRequestException('No active reservation to cancel');
    }

    const item = await this.prisma.inventoryItem.findUnique({
      where: { id: claim.consumption.inventoryId },
    });

    if (!item || item.status !== 'RESERVED') {
      throw new BadRequestException('No active reservation to cancel');
    }

    // Release both claim and inventory
    await this.prisma.$transaction(async (tx) => {
      await tx.inventoryItem.update({
        where: { id: item.id },
        data: {
          status: 'AVAILABLE',
          reservedById: null,
          reservedAt: null,
          reservationExpires: null,
        },
      });

      await tx.inventoryStatusEvent.create({
        data: {
          inventoryId: item.id,
          previousStatus: 'RESERVED',
          newStatus: 'AVAILABLE',
          actorId: userId,
        },
      });

      // Delete the consumption record
      await tx.claimConsumption.delete({
        where: { id: claim.consumption!.id },
      });
    });

    return { status: 'cancelled', claimId };
  }

  async finalize(
    userId: string,
    claimId: string,
    deliveryMethod: string,
    shippingAddress?: any,
  ) {
    const claim = await this.prisma.claim.findUnique({
      where: { id: claimId },
      include: { consumption: true },
    });

    if (!claim) {
      throw new NotFoundException('Claim not found');
    }

    if (claim.userId !== userId) {
      throw new ForbiddenException('Not your claim');
    }

    if (claim.status !== 'ACTIVE') {
      throw new BadRequestException(
        `Cannot finalize claim in ${claim.status} status`,
      );
    }

    if (!claim.consumption) {
      throw new BadRequestException(
        'No reservation to finalize — reserve an item first',
      );
    }

    const item = await this.prisma.inventoryItem.findUnique({
      where: { id: claim.consumption.inventoryId },
      include: {
        warehouse: true,
        receipt: { include: { submission: true } },
      },
    });

    if (!item || item.status !== 'RESERVED') {
      throw new BadRequestException('Reservation is no longer active');
    }

    // Check if reservation expired
    if (item.reservationExpires && item.reservationExpires < new Date()) {
      throw new BadRequestException('Reservation has expired');
    }

    // Call delivery provider to create a carrier shipment
    let carrierResult: {
      carrier: string;
      trackingNumber: string;
      estimatedDelivery?: Date;
      cost: number;
    } | null = null;

    try {
      const provider =
        this.deliveryService.getProviderForMethod(deliveryMethod);
      const warehouseAddr = (item.warehouse.address as any) || {};
      const destAddr = shippingAddress || {};

      const shipmentResult = await provider.createShipment({
        originAddress: {
          street: warehouseAddr.street || '',
          city: warehouseAddr.city || '',
          state: warehouseAddr.state || '',
          zip: warehouseAddr.zip || '',
          country: warehouseAddr.country || 'US',
          name: item.warehouse.name,
        },
        destinationAddress: {
          street: destAddr.street || '',
          city: destAddr.city || '',
          state: destAddr.state || '',
          zip: destAddr.zip || '',
          country: destAddr.country || 'US',
        },
        weight: 16, // default 1 lb until item weight is tracked
      });

      carrierResult = {
        carrier: shipmentResult.carrier,
        trackingNumber: shipmentResult.trackingNumber,
        estimatedDelivery: shipmentResult.estimatedDelivery,
        cost: shipmentResult.cost,
      };
    } catch (error) {
      this.logger.warn(
        `Delivery provider failed, creating shipment without carrier: ${(error as Error).message}`,
      );
    }

    // Atomically consume claim, update inventory, create shipment
    const result = await this.prisma.$transaction(async (tx) => {
      // Consume the claim
      await tx.claim.update({
        where: { id: claimId },
        data: {
          status: 'CONSUMED',
          consumedAt: new Date(),
        },
      });

      // Update consumption timestamp
      await tx.claimConsumption.update({
        where: { id: claim.consumption!.id },
        data: { consumedAt: new Date() },
      });

      // Move inventory to OUTBOUND
      await tx.inventoryItem.update({
        where: { id: item.id },
        data: { status: 'OUTBOUND' },
      });

      await tx.inventoryStatusEvent.create({
        data: {
          inventoryId: item.id,
          previousStatus: 'RESERVED',
          newStatus: 'OUTBOUND',
          actorId: userId,
        },
      });

      // Create outbound shipment with carrier details
      const shipment = await tx.shipment.create({
        data: {
          inventoryId: item.id,
          direction: 'OUTBOUND',
          status: 'CREATED',
          carrier: carrierResult?.carrier,
          trackingNumber: carrierResult?.trackingNumber,
          estimatedDelivery: carrierResult?.estimatedDelivery,
        },
      });

      return shipment;
    });

    return {
      status: 'finalized',
      claimId,
      shipmentId: result.id,
      deliveryMethod,
      carrier: result.carrier,
      trackingNumber: result.trackingNumber,
      estimatedDelivery: result.estimatedDelivery,
    };
  }

  private async getActiveReservation(claimId: string) {
    const consumption = await this.prisma.claimConsumption.findUnique({
      where: { claimId },
      include: {
        inventory: {
          select: {
            id: true,
            status: true,
            reservedAt: true,
            reservationExpires: true,
            binLocation: true,
          },
        },
      },
    });

    if (!consumption) return null;

    // Only return if the inventory is in RESERVED status
    if (consumption.inventory.status === 'RESERVED') {
      return {
        inventoryId: consumption.inventory.id,
        reservedAt: consumption.inventory.reservedAt,
        expiresAt: consumption.inventory.reservationExpires,
      };
    }

    return null;
  }
}
