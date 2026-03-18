import { Injectable, Logger, Optional, Inject } from '@nestjs/common';
import {
  DeliveryProvider,
  ShipmentParams,
  ShipmentResult,
  RateParams,
  Rate,
  ProviderTrackingEvent,
} from './delivery-provider.interface';
import { LocalDevProvider } from './providers/local-dev.provider';
import { StandardCarrierProvider } from './providers/standard-carrier.provider';
import { UberDirectProvider } from './providers/uber-direct.provider';
import { CommunityProvider } from './providers/community.provider';
import { PrismaService } from '../prisma/prisma.service';

@Injectable()
export class DeliveryService {
  private readonly logger = new Logger(DeliveryService.name);
  private readonly providers = new Map<string, DeliveryProvider>();

  constructor(@Optional() @Inject(PrismaService) private readonly prisma?: PrismaService) {
    this.registerProviders();
  }

  private registerProviders() {
    // Always register local-dev for testing
    this.providers.set('local-dev', new LocalDevProvider());

    // Register community courier provider if Prisma is available
    if (this.prisma) {
      this.providers.set('community', new CommunityProvider(this.prisma));
      this.logger.log('Registered community provider');
    }

    // Register standard carrier if API key is available
    const shipEngineKey = process.env.SHIPENGINE_API_KEY;
    if (shipEngineKey) {
      this.providers.set(
        'standard-carrier',
        new StandardCarrierProvider(shipEngineKey),
      );
      this.logger.log('Registered standard-carrier provider');
    }

    // Register Uber Direct if credentials are available
    const uberClientId = process.env.UBER_CLIENT_ID;
    const uberClientSecret = process.env.UBER_CLIENT_SECRET;
    if (uberClientId && uberClientSecret) {
      this.providers.set(
        'uber-direct',
        new UberDirectProvider(uberClientId, uberClientSecret),
      );
      this.logger.log('Registered uber-direct provider');
    }

    this.logger.log(
      `Delivery providers registered: ${[...this.providers.keys()].join(', ')}`,
    );
  }

  getProvider(name: string): DeliveryProvider | undefined {
    return this.providers.get(name);
  }

  getDefaultProvider(): DeliveryProvider {
    // Prefer standard-carrier → uber-direct → local-dev
    return (
      this.providers.get('standard-carrier') ??
      this.providers.get('uber-direct') ??
      this.providers.get('local-dev')!
    );
  }

  getProviderForMethod(deliveryMethod: string): DeliveryProvider {
    if (deliveryMethod === 'community' && this.providers.has('community')) {
      return this.providers.get('community')!;
    }
    if (deliveryMethod === 'same-day' && this.providers.has('uber-direct')) {
      return this.providers.get('uber-direct')!;
    }
    if (
      deliveryMethod === 'standard' &&
      this.providers.has('standard-carrier')
    ) {
      return this.providers.get('standard-carrier')!;
    }
    return this.getDefaultProvider();
  }

  async getRatesFromAll(params: RateParams): Promise<Rate[]> {
    const allRates: Rate[] = [];

    const promises = [...this.providers.entries()].map(
      async ([name, provider]) => {
        try {
          const rates = await provider.getRates(params);
          return rates;
        } catch (error) {
          this.logger.warn(
            `Failed to get rates from ${name}: ${(error as Error).message}`,
          );
          return [];
        }
      },
    );

    const results = await Promise.all(promises);
    for (const rates of results) {
      allRates.push(...rates);
    }

    return allRates.sort((a, b) => a.cost - b.cost);
  }

  async createShipment(
    providerName: string,
    params: ShipmentParams,
  ): Promise<ShipmentResult> {
    const provider = this.providers.get(providerName);
    if (!provider) {
      throw new Error(`Unknown delivery provider: ${providerName}`);
    }
    return provider.createShipment(params);
  }

  async getTrackingEvents(
    providerName: string,
    trackingNumber: string,
  ): Promise<ProviderTrackingEvent[]> {
    const provider = this.providers.get(providerName);
    if (!provider) {
      throw new Error(`Unknown delivery provider: ${providerName}`);
    }
    return provider.getTrackingEvents(trackingNumber);
  }

  async cancelShipment(
    providerName: string,
    providerId: string,
  ): Promise<void> {
    const provider = this.providers.get(providerName);
    if (!provider) {
      throw new Error(`Unknown delivery provider: ${providerName}`);
    }
    return provider.cancelShipment(providerId);
  }
}
