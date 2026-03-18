import {
  Injectable,
  Logger,
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { DeliveryService } from '../delivery/delivery.service';
import { NotificationsService } from '../notifications/notifications.service';
import { createHash } from 'crypto';

@Injectable()
export class ShipmentsService {
  private readonly logger = new Logger(ShipmentsService.name);

  constructor(
    private readonly prisma: PrismaService,
    private readonly deliveryService: DeliveryService,
    private readonly notificationsService: NotificationsService,
  ) {}

  async findAll(
    userId: string,
    filters?: {
      direction?: string;
      status?: string;
      page?: number;
      limit?: number;
    },
  ) {
    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    // User can see shipments for their submissions (inbound) or their reservations (outbound)
    const where: any = {
      OR: [
        { submission: { userId } },
        { inventory: { reservedById: userId } },
      ],
    };

    if (filters?.direction) where.direction = filters.direction;
    if (filters?.status) where.status = filters.status;

    const [shipments, total] = await Promise.all([
      this.prisma.shipment.findMany({
        where,
        include: {
          trackingEvents: {
            orderBy: { timestamp: 'desc' },
            take: 1,
          },
        },
        orderBy: { createdAt: 'desc' },
        skip,
        take: limit,
      }),
      this.prisma.shipment.count({ where }),
    ]);

    return {
      data: shipments.map((s) => ({
        id: s.id,
        direction: s.direction,
        carrier: s.carrier,
        trackingNumber: s.trackingNumber,
        status: s.status,
        estimatedDelivery: s.estimatedDelivery,
        deliveredAt: s.deliveredAt,
        createdAt: s.createdAt,
        lastEvent: s.trackingEvents[0] || null,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  async findOne(userId: string, shipmentId: string) {
    const shipment = await this.prisma.shipment.findUnique({
      where: { id: shipmentId },
      include: {
        submission: { select: { userId: true, id: true, category: true } },
        inventory: {
          select: {
            id: true,
            reservedById: true,
            binLocation: true,
            receipt: {
              select: {
                conditionGrade: true,
                finalBand: true,
                submission: { select: { category: true } },
              },
            },
          },
        },
        trackingEvents: { orderBy: { timestamp: 'asc' } },
      },
    });

    if (!shipment) {
      throw new NotFoundException('Shipment not found');
    }

    // Check ownership: user must be the submitter or the person who reserved
    const isOwner =
      shipment.submission?.userId === userId ||
      shipment.inventory?.reservedById === userId;

    if (!isOwner) {
      throw new ForbiddenException('Not your shipment');
    }

    return {
      id: shipment.id,
      direction: shipment.direction,
      carrier: shipment.carrier,
      trackingNumber: shipment.trackingNumber,
      status: shipment.status,
      estimatedDelivery: shipment.estimatedDelivery,
      deliveredAt: shipment.deliveredAt,
      proofHash: shipment.proofHash,
      createdAt: shipment.createdAt,
      submission: shipment.submission,
      inventory: shipment.inventory,
      timeline: shipment.trackingEvents,
    };
  }

  async getTracking(userId: string, shipmentId: string) {
    // Verify ownership first
    await this.findOne(userId, shipmentId);

    const events = await this.prisma.trackingEvent.findMany({
      where: { shipmentId },
      orderBy: { timestamp: 'asc' },
    });

    return { shipmentId, events };
  }

  async addTrackingEvent(
    shipmentId: string,
    eventType: string,
    location?: string,
    timestamp?: string,
  ) {
    const shipment = await this.prisma.shipment.findUnique({
      where: { id: shipmentId },
    });

    if (!shipment) {
      throw new NotFoundException('Shipment not found');
    }

    if (shipment.status === 'DELIVERED') {
      throw new BadRequestException('Shipment already delivered');
    }

    const event = await this.prisma.trackingEvent.create({
      data: {
        shipmentId,
        eventType,
        location,
        timestamp: timestamp ? new Date(timestamp) : new Date(),
      },
    });

    // Auto-update shipment status based on event type
    const statusMap: Record<string, string> = {
      picked_up: 'PICKED_UP',
      in_transit: 'IN_TRANSIT',
      out_for_delivery: 'IN_TRANSIT',
      exception: 'EXCEPTION',
    };

    const newStatus = statusMap[eventType.toLowerCase()];
    if (newStatus) {
      await this.prisma.shipment.update({
        where: { id: shipmentId },
        data: { status: newStatus as any },
      });

      // Notify user on delivery exceptions
      if (newStatus === 'EXCEPTION') {
        await this.notifyDeliveryException(shipment, eventType, location);
      }
    }

    return event;
  }

  async syncProviderTracking(shipmentId: string) {
    const shipment = await this.prisma.shipment.findUnique({
      where: { id: shipmentId },
    });

    if (!shipment || !shipment.carrier || !shipment.trackingNumber) {
      return;
    }

    // Map carrier name to provider name
    const providerMap: Record<string, string> = {
      DevCarrier: 'local-dev',
      'Uber Direct': 'uber-direct',
    };
    const providerName =
      providerMap[shipment.carrier] || 'standard-carrier';

    try {
      const events = await this.deliveryService.getTrackingEvents(
        providerName,
        shipment.trackingNumber,
      );

      for (const event of events) {
        // Check if event already exists to avoid duplicates
        const existing = await this.prisma.trackingEvent.findFirst({
          where: {
            shipmentId,
            eventType: event.eventType,
            timestamp: event.timestamp,
          },
        });

        if (!existing) {
          await this.addTrackingEvent(
            shipmentId,
            event.eventType,
            event.location,
            event.timestamp.toISOString(),
          );
        }
      }
    } catch (error) {
      this.logger.warn(
        `Failed to sync tracking for shipment ${shipmentId}: ${(error as Error).message}`,
      );
    }
  }

  private async notifyDeliveryException(
    shipment: any,
    eventType: string,
    location?: string,
  ) {
    // Find the user who owns this shipment
    let userId: string | null = null;

    if (shipment.submissionId) {
      const submission = await this.prisma.itemSubmission.findUnique({
        where: { id: shipment.submissionId },
        select: { userId: true },
      });
      userId = submission?.userId ?? null;
    } else if (shipment.inventoryId) {
      const inventory = await this.prisma.inventoryItem.findUnique({
        where: { id: shipment.inventoryId },
        select: { reservedById: true },
      });
      userId = inventory?.reservedById ?? null;
    }

    if (userId) {
      await this.notificationsService.create(
        userId,
        'delivery_exception',
        {
          shipmentId: shipment.id,
          trackingNumber: shipment.trackingNumber,
          eventType,
          location,
          message: `Delivery exception on shipment ${shipment.trackingNumber || shipment.id}`,
        },
      );
    }
  }

  async confirmDelivery(
    userId: string,
    shipmentId: string,
    proofMethod: string,
    proofData: string,
    notes?: string,
  ) {
    const shipment = await this.prisma.shipment.findUnique({
      where: { id: shipmentId },
      include: { inventory: true },
    });

    if (!shipment) {
      throw new NotFoundException('Shipment not found');
    }

    if (shipment.status === 'DELIVERED') {
      throw new BadRequestException('Shipment already delivered');
    }

    // Generate proof hash
    const proofHash = createHash('sha256')
      .update(`${proofMethod}:${proofData}:${shipmentId}`)
      .digest('hex');

    await this.prisma.$transaction(async (tx) => {
      // Update shipment
      await tx.shipment.update({
        where: { id: shipmentId },
        data: {
          status: 'DELIVERED',
          deliveredAt: new Date(),
          proofHash,
        },
      });

      // Add delivery tracking event
      await tx.trackingEvent.create({
        data: {
          shipmentId,
          eventType: 'delivered',
          timestamp: new Date(),
          rawData: { proofMethod, notes },
        },
      });

      // Update inventory to DELIVERED if outbound shipment
      if (shipment.inventoryId && shipment.direction === 'OUTBOUND') {
        await tx.inventoryItem.update({
          where: { id: shipment.inventoryId },
          data: { status: 'DELIVERED' },
        });

        await tx.inventoryStatusEvent.create({
          data: {
            inventoryId: shipment.inventoryId,
            previousStatus: 'OUTBOUND',
            newStatus: 'DELIVERED',
            actorId: userId,
            evidenceHash: proofHash,
          },
        });
      }
    });

    return {
      status: 'delivered',
      shipmentId,
      proofHash,
      deliveredAt: new Date().toISOString(),
    };
  }
}
