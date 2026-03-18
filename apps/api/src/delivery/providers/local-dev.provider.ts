import { randomBytes } from 'crypto';
import {
  DeliveryProvider,
  ShipmentParams,
  ShipmentResult,
  RateParams,
  Rate,
  ProviderTrackingEvent,
} from '../delivery-provider.interface';

export class LocalDevProvider implements DeliveryProvider {
  readonly name = 'local-dev';

  async createShipment(params: ShipmentParams): Promise<ShipmentResult> {
    const trackingNumber = `DEV-${randomBytes(6).toString('hex').toUpperCase()}`;
    const estimatedDelivery = new Date();
    estimatedDelivery.setDate(estimatedDelivery.getDate() + 3);

    return {
      providerId: `dev-${randomBytes(4).toString('hex')}`,
      carrier: 'DevCarrier',
      trackingNumber,
      estimatedDelivery,
      cost: 799, // $7.99
    };
  }

  async getTrackingEvents(trackingNumber: string): Promise<ProviderTrackingEvent[]> {
    return [
      {
        eventType: 'created',
        timestamp: new Date(),
        description: 'Shipment created (dev mode)',
      },
    ];
  }

  async cancelShipment(providerId: string): Promise<void> {
    // No-op in dev mode
  }

  async getRates(params: RateParams): Promise<Rate[]> {
    return [
      {
        provider: 'local-dev',
        carrier: 'DevCarrier',
        serviceLevel: 'standard',
        cost: 799,
        estimatedDays: 3,
        currency: 'USD',
      },
      {
        provider: 'local-dev',
        carrier: 'DevCarrier',
        serviceLevel: 'express',
        cost: 1499,
        estimatedDays: 1,
        currency: 'USD',
      },
    ];
  }
}
