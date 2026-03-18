import {
  DeliveryProvider,
  ShipmentParams,
  ShipmentResult,
  RateParams,
  Rate,
  ProviderTrackingEvent,
} from '../delivery-provider.interface';
import { PrismaService } from '../../prisma/prisma.service';
import { randomBytes } from 'crypto';

/**
 * Community courier provider.
 *
 * Instead of calling a 3P API, this creates a CourierTask record
 * that appears on the community courier task board.
 */
export class CommunityProvider implements DeliveryProvider {
  readonly name = 'community';

  constructor(private readonly prisma: PrismaService) {}

  async createShipment(params: ShipmentParams): Promise<ShipmentResult> {
    const trackingNumber = `COM-${randomBytes(6).toString('hex').toUpperCase()}`;

    // Calculate time window (default 4-hour window starting 1 hour from now)
    const timeWindowStart = new Date(Date.now() + 60 * 60 * 1000);
    const timeWindowEnd = new Date(
      timeWindowStart.getTime() + 4 * 60 * 60 * 1000,
    );

    // Base fee calculation: $9.99 base + $1/mile estimate
    const baseFee = 9.99;

    // We need a shipment to exist first to link the courier task.
    // The caller (ClaimsService.finalize) creates the shipment record,
    // so we return the result and let the caller wire up the CourierTask
    // via the shipmentId. We'll store the task creation params in metadata.
    const providerId = `community-${randomBytes(4).toString('hex')}`;

    // Store courier task params so the service layer can create the task
    // after the shipment record exists
    (this as any)._pendingTask = {
      pickupLocation: {
        street: params.originAddress.street,
        city: params.originAddress.city,
        state: params.originAddress.state,
        zip: params.originAddress.zip,
        region: params.originAddress.state,
      },
      dropoffLocation: {
        street: params.destinationAddress.street,
        city: params.destinationAddress.city,
        state: params.destinationAddress.state,
        zip: params.destinationAddress.zip,
        region: params.destinationAddress.state,
      },
      fee: baseFee,
      timeWindowStart,
      timeWindowEnd,
    };

    return {
      providerId,
      carrier: 'Community Courier',
      trackingNumber,
      estimatedDelivery: timeWindowEnd,
      cost: Math.round(baseFee * 100),
    };
  }

  /**
   * Create the CourierTask record after the shipment has been persisted.
   * Called by DeliveryService after finalize creates the shipment.
   */
  async createCourierTask(shipmentId: string): Promise<string> {
    const pending = (this as any)._pendingTask;
    if (!pending) {
      throw new Error('No pending task — call createShipment first');
    }

    const task = await this.prisma.courierTask.create({
      data: {
        shipmentId,
        pickupLocation: pending.pickupLocation,
        dropoffLocation: pending.dropoffLocation,
        fee: pending.fee,
        status: 'POSTED',
        timeWindowStart: pending.timeWindowStart,
        timeWindowEnd: pending.timeWindowEnd,
      },
    });

    (this as any)._pendingTask = null;
    return task.id;
  }

  async getTrackingEvents(
    trackingNumber: string,
  ): Promise<ProviderTrackingEvent[]> {
    // Look up courier events via the shipment's courier task
    const task = await this.prisma.courierTask.findFirst({
      where: {
        shipment: { trackingNumber },
      },
      include: {
        events: { orderBy: { createdAt: 'asc' } },
      },
    });

    if (!task) return [];

    const eventTypeMap: Record<string, string> = {
      ACCEPTED: 'accepted',
      PICKUP_SCAN: 'picked_up',
      MILESTONE: 'in_transit',
      DROPOFF_PROOF: 'delivered',
      COMPLETED: 'delivered',
      EXCEPTION: 'exception',
    };

    return task.events.map((e) => ({
      eventType: eventTypeMap[e.eventType] || e.eventType.toLowerCase(),
      timestamp: e.createdAt,
      description: `Courier event: ${e.eventType}`,
      rawData: e,
    }));
  }

  async cancelShipment(providerId: string): Promise<void> {
    // Cancel the courier task associated with this shipment
    const task = await this.prisma.courierTask.findFirst({
      where: {
        shipment: { id: providerId },
        status: { in: ['POSTED', 'ACCEPTED'] },
      },
    });

    if (task) {
      await this.prisma.courierTask.update({
        where: { id: task.id },
        data: { status: 'FAILED' },
      });
    }
  }

  async getRates(params: RateParams): Promise<Rate[]> {
    return [
      {
        provider: 'community',
        carrier: 'Community Courier',
        serviceLevel: 'community',
        cost: 999, // $9.99 base
        estimatedDays: 0, // same day
        currency: 'USD',
      },
    ];
  }
}
