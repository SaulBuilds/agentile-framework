import {
  DeliveryProvider,
  ShipmentParams,
  ShipmentResult,
  RateParams,
  Rate,
  ProviderTrackingEvent,
} from '../delivery-provider.interface';

/**
 * Standard carrier adapter for ShipEngine/EasyPost.
 *
 * In production, this calls the ShipEngine or EasyPost API.
 * The API key is provided via SHIPENGINE_API_KEY env var.
 */
export class StandardCarrierProvider implements DeliveryProvider {
  readonly name = 'standard-carrier';

  constructor(private readonly apiKey: string) {}

  async createShipment(params: ShipmentParams): Promise<ShipmentResult> {
    // In production: POST to ShipEngine /v1/labels
    const response = await this.callApi('/v1/labels', 'POST', {
      shipment: {
        ship_from: this.formatAddress(params.originAddress),
        ship_to: this.formatAddress(params.destinationAddress),
        packages: [
          {
            weight: { value: params.weight, unit: 'ounce' },
            dimensions: params.dimensions
              ? {
                  length: params.dimensions.length,
                  width: params.dimensions.width,
                  height: params.dimensions.height,
                  unit: 'inch',
                }
              : undefined,
          },
        ],
      },
      carrier_id: params.carrier,
      service_code: params.serviceLevel || 'usps_priority_mail',
    });

    return {
      providerId: response.label_id,
      carrier: response.carrier_code,
      trackingNumber: response.tracking_number,
      labelUrl: response.label_download?.href,
      estimatedDelivery: response.estimated_delivery
        ? new Date(response.estimated_delivery)
        : undefined,
      cost: Math.round(response.shipment_cost?.amount * 100) || 0,
    };
  }

  async getTrackingEvents(trackingNumber: string): Promise<ProviderTrackingEvent[]> {
    const response = await this.callApi(
      `/v1/tracking?tracking_number=${trackingNumber}`,
      'GET',
    );

    return (response.events || []).map((event: any) => ({
      eventType: this.mapEventType(event.status_code),
      timestamp: new Date(event.occurred_at),
      location: event.city_locality
        ? `${event.city_locality}, ${event.state_province}`
        : undefined,
      description: event.description,
      rawData: event,
    }));
  }

  async cancelShipment(providerId: string): Promise<void> {
    await this.callApi(`/v1/labels/${providerId}/void`, 'PUT');
  }

  async getRates(params: RateParams): Promise<Rate[]> {
    const response = await this.callApi('/v1/rates/estimate', 'POST', {
      from_postal_code: params.originZip,
      to_postal_code: params.destinationZip,
      weight: { value: params.weight, unit: 'ounce' },
      dimensions: params.dimensions
        ? {
            length: params.dimensions.length,
            width: params.dimensions.width,
            height: params.dimensions.height,
            unit: 'inch',
          }
        : undefined,
    });

    return (response || []).map((rate: any) => ({
      provider: 'standard-carrier',
      carrier: rate.carrier_code,
      serviceLevel: rate.service_code,
      cost: Math.round((rate.shipping_amount?.amount || 0) * 100),
      estimatedDays: rate.delivery_days || 5,
      currency: 'USD',
    }));
  }

  private formatAddress(addr: any) {
    return {
      name: addr.name || 'Gradient Barter',
      address_line1: addr.street,
      city_locality: addr.city,
      state_province: addr.state,
      postal_code: addr.zip,
      country_code: addr.country || 'US',
    };
  }

  private mapEventType(statusCode: string): string {
    const map: Record<string, string> = {
      AC: 'accepted',
      IT: 'in_transit',
      DE: 'delivered',
      EX: 'exception',
      AT: 'delivery_attempt',
      NY: 'not_yet_in_system',
    };
    return map[statusCode] || statusCode.toLowerCase();
  }

  private async callApi(path: string, method: string, body?: any): Promise<any> {
    const baseUrl = 'https://api.shipengine.com';
    const response = await fetch(`${baseUrl}${path}`, {
      method,
      headers: {
        'API-Key': this.apiKey,
        'Content-Type': 'application/json',
      },
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`ShipEngine API error (${response.status}): ${error}`);
    }

    return response.json();
  }
}
