import { DeliveryService } from './delivery.service';
import { LocalDevProvider } from './providers/local-dev.provider';

describe('DeliveryService', () => {
  let service: DeliveryService;

  beforeEach(() => {
    // Clear env vars to ensure only local-dev is registered
    delete process.env.SHIPENGINE_API_KEY;
    delete process.env.UBER_CLIENT_ID;
    delete process.env.UBER_CLIENT_SECRET;
    service = new DeliveryService();
  });

  describe('provider registry', () => {
    it('should register local-dev provider by default', () => {
      const provider = service.getProvider('local-dev');
      expect(provider).toBeDefined();
      expect(provider!.name).toBe('local-dev');
    });

    it('should return local-dev as default provider when no API keys set', () => {
      const provider = service.getDefaultProvider();
      expect(provider.name).toBe('local-dev');
    });

    it('should return undefined for unknown provider', () => {
      expect(service.getProvider('unknown')).toBeUndefined();
    });
  });

  describe('getProviderForMethod', () => {
    it('should return local-dev for standard when no API keys', () => {
      const provider = service.getProviderForMethod('standard');
      expect(provider.name).toBe('local-dev');
    });

    it('should return local-dev for same-day when no uber credentials', () => {
      const provider = service.getProviderForMethod('same-day');
      expect(provider.name).toBe('local-dev');
    });
  });

  describe('getRatesFromAll', () => {
    it('should return rates from all registered providers sorted by cost', async () => {
      const rates = await service.getRatesFromAll({
        originZip: '97201',
        destinationZip: '10001',
        weight: 16,
      });

      expect(rates.length).toBeGreaterThanOrEqual(2);
      expect(rates[0].cost).toBeLessThanOrEqual(rates[1].cost);
      expect(rates[0].provider).toBe('local-dev');
    });

    it('should include rate details', async () => {
      const rates = await service.getRatesFromAll({
        originZip: '97201',
        destinationZip: '10001',
        weight: 16,
      });

      const standardRate = rates.find((r) => r.serviceLevel === 'standard');
      expect(standardRate).toBeDefined();
      expect(standardRate!.cost).toBe(799);
      expect(standardRate!.carrier).toBe('DevCarrier');
      expect(standardRate!.currency).toBe('USD');
    });
  });

  describe('createShipment', () => {
    it('should create a shipment through the local-dev provider', async () => {
      const result = await service.createShipment('local-dev', {
        originAddress: {
          street: '123 Warehouse St',
          city: 'Portland',
          state: 'OR',
          zip: '97201',
          country: 'US',
        },
        destinationAddress: {
          street: '456 Main St',
          city: 'New York',
          state: 'NY',
          zip: '10001',
          country: 'US',
        },
        weight: 16,
      });

      expect(result.providerId).toMatch(/^dev-/);
      expect(result.trackingNumber).toMatch(/^DEV-/);
      expect(result.carrier).toBe('DevCarrier');
      expect(result.cost).toBe(799);
      expect(result.estimatedDelivery).toBeInstanceOf(Date);
    });

    it('should throw for unknown provider', async () => {
      await expect(
        service.createShipment('unknown-provider', {
          originAddress: { street: '', city: '', state: '', zip: '', country: 'US' },
          destinationAddress: { street: '', city: '', state: '', zip: '', country: 'US' },
          weight: 1,
        }),
      ).rejects.toThrow('Unknown delivery provider');
    });
  });

  describe('getTrackingEvents', () => {
    it('should return tracking events from provider', async () => {
      const events = await service.getTrackingEvents(
        'local-dev',
        'DEV-TEST123',
      );

      expect(events).toHaveLength(1);
      expect(events[0].eventType).toBe('created');
      expect(events[0].timestamp).toBeInstanceOf(Date);
    });

    it('should throw for unknown provider', async () => {
      await expect(
        service.getTrackingEvents('unknown', 'TRK-1'),
      ).rejects.toThrow('Unknown delivery provider');
    });
  });

  describe('cancelShipment', () => {
    it('should cancel a shipment through provider', async () => {
      // local-dev cancelShipment is a no-op, should not throw
      await expect(
        service.cancelShipment('local-dev', 'dev-1234'),
      ).resolves.toBeUndefined();
    });

    it('should throw for unknown provider', async () => {
      await expect(
        service.cancelShipment('unknown', 'id'),
      ).rejects.toThrow('Unknown delivery provider');
    });
  });
});

describe('LocalDevProvider', () => {
  let provider: LocalDevProvider;

  beforeEach(() => {
    provider = new LocalDevProvider();
  });

  it('should have name local-dev', () => {
    expect(provider.name).toBe('local-dev');
  });

  it('should create shipment with DEV tracking number', async () => {
    const result = await provider.createShipment({
      originAddress: { street: '123 A', city: 'Portland', state: 'OR', zip: '97201', country: 'US' },
      destinationAddress: { street: '456 B', city: 'NYC', state: 'NY', zip: '10001', country: 'US' },
      weight: 8,
    });

    expect(result.trackingNumber).toMatch(/^DEV-[A-F0-9]{12}$/);
    expect(result.carrier).toBe('DevCarrier');
    expect(result.cost).toBe(799);
  });

  it('should return standard and express rates', async () => {
    const rates = await provider.getRates({
      originZip: '97201',
      destinationZip: '10001',
      weight: 16,
    });

    expect(rates).toHaveLength(2);
    expect(rates[0].serviceLevel).toBe('standard');
    expect(rates[1].serviceLevel).toBe('express');
    expect(rates[0].cost).toBe(799);
    expect(rates[1].cost).toBe(1499);
  });

  it('should return a single created event for tracking', async () => {
    const events = await provider.getTrackingEvents('DEV-TEST');

    expect(events).toHaveLength(1);
    expect(events[0].eventType).toBe('created');
  });

  it('should no-op on cancel', async () => {
    await expect(provider.cancelShipment('dev-123')).resolves.toBeUndefined();
  });
});
