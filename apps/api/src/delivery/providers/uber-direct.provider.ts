import {
  DeliveryProvider,
  ShipmentParams,
  ShipmentResult,
  RateParams,
  Rate,
  ProviderTrackingEvent,
} from '../delivery-provider.interface';

/**
 * Uber Direct adapter for same-day metro delivery.
 *
 * In production, this calls the Uber Direct (Daas) API.
 * The API credentials are provided via UBER_CLIENT_ID / UBER_CLIENT_SECRET env vars.
 */
export class UberDirectProvider implements DeliveryProvider {
  readonly name = 'uber-direct';

  constructor(
    private readonly clientId: string,
    private readonly clientSecret: string,
  ) {}

  private accessToken: string | null = null;
  private tokenExpiry = 0;

  async createShipment(params: ShipmentParams): Promise<ShipmentResult> {
    const token = await this.getAccessToken();

    const response = await this.callApi('/v1/deliveries', 'POST', token, {
      pickup: {
        address: `${params.originAddress.street}, ${params.originAddress.city}, ${params.originAddress.state} ${params.originAddress.zip}`,
        name: params.originAddress.name || 'Gradient Barter Warehouse',
        phone_number: params.originAddress.phone || '+10000000000',
      },
      dropoff: {
        address: `${params.destinationAddress.street}, ${params.destinationAddress.city}, ${params.destinationAddress.state} ${params.destinationAddress.zip}`,
        name: params.destinationAddress.name || 'Recipient',
        phone_number: params.destinationAddress.phone || '+10000000000',
      },
      manifest_items: [
        {
          name: 'Barter item',
          quantity: 1,
          weight: params.weight,
          dimensions: params.dimensions,
        },
      ],
    });

    return {
      providerId: response.id,
      carrier: 'Uber Direct',
      trackingNumber: response.tracking_url || response.id,
      estimatedDelivery: response.dropoff?.eta
        ? new Date(response.dropoff.eta)
        : undefined,
      cost: Math.round((response.fee || 0) * 100),
    };
  }

  async getTrackingEvents(trackingNumber: string): Promise<ProviderTrackingEvent[]> {
    const token = await this.getAccessToken();
    const response = await this.callApi(
      `/v1/deliveries/${trackingNumber}`,
      'GET',
      token,
    );

    const events: ProviderTrackingEvent[] = [];

    if (response.created) {
      events.push({
        eventType: 'created',
        timestamp: new Date(response.created),
        description: 'Delivery request created',
      });
    }

    if (response.pickup?.eta) {
      events.push({
        eventType: 'pickup_scheduled',
        timestamp: new Date(response.pickup.eta),
        description: 'Pickup scheduled',
      });
    }

    const statusMap: Record<string, string> = {
      pending: 'created',
      pickup: 'picked_up',
      pickup_complete: 'picked_up',
      dropoff: 'in_transit',
      delivered: 'delivered',
      canceled: 'cancelled',
      returned: 'exception',
    };

    if (response.status) {
      events.push({
        eventType: statusMap[response.status] || response.status,
        timestamp: new Date(response.updated || Date.now()),
        description: `Status: ${response.status}`,
        rawData: response,
      });
    }

    return events;
  }

  async cancelShipment(providerId: string): Promise<void> {
    const token = await this.getAccessToken();
    await this.callApi(`/v1/deliveries/${providerId}/cancel`, 'POST', token);
  }

  async getRates(params: RateParams): Promise<Rate[]> {
    // Uber Direct doesn't have a traditional rate API;
    // estimate based on typical metro delivery pricing
    return [
      {
        provider: 'uber-direct',
        carrier: 'Uber Direct',
        serviceLevel: 'same-day',
        cost: 1999, // ~$19.99 base
        estimatedDays: 0, // same day
        currency: 'USD',
      },
    ];
  }

  private async getAccessToken(): Promise<string> {
    if (this.accessToken && Date.now() < this.tokenExpiry) {
      return this.accessToken;
    }

    const response = await fetch('https://login.uber.com/oauth/v2/token', {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({
        client_id: this.clientId,
        client_secret: this.clientSecret,
        grant_type: 'client_credentials',
        scope: 'eats.deliveries',
      }),
    });

    if (!response.ok) {
      throw new Error(`Uber auth failed: ${response.status}`);
    }

    const data = await response.json();
    this.accessToken = data.access_token;
    this.tokenExpiry = Date.now() + (data.expires_in - 60) * 1000;
    return this.accessToken!;
  }

  private async callApi(
    path: string,
    method: string,
    token: string,
    body?: any,
  ): Promise<any> {
    const baseUrl = 'https://api.uber.com';
    const response = await fetch(`${baseUrl}${path}`, {
      method,
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      const error = await response.text();
      throw new Error(`Uber Direct API error (${response.status}): ${error}`);
    }

    return response.json();
  }
}
