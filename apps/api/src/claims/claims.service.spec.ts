import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { ClaimsService } from './claims.service';
import { PrismaService } from '../prisma/prisma.service';
import { DeliveryService } from '../delivery/delivery.service';

describe('ClaimsService', () => {
  let service: ClaimsService;
  let prisma: any;
  let deliveryService: any;

  beforeEach(async () => {
    prisma = {
      claim: {
        findMany: jest.fn(),
        findUnique: jest.fn(),
        count: jest.fn(),
        update: jest.fn(),
      },
      inventoryItem: {
        findUnique: jest.fn(),
        update: jest.fn(),
      },
      inventoryStatusEvent: { create: jest.fn() },
      claimConsumption: {
        findUnique: jest.fn(),
        create: jest.fn(),
        delete: jest.fn(),
        update: jest.fn(),
      },
      shipment: { create: jest.fn() },
      $transaction: jest.fn((fn) => fn(prisma)),
    };

    deliveryService = {
      getProviderForMethod: jest.fn().mockReturnValue({
        name: 'local-dev',
        createShipment: jest.fn().mockResolvedValue({
          providerId: 'dev-1234',
          carrier: 'DevCarrier',
          trackingNumber: 'DEV-ABC123',
          estimatedDelivery: new Date('2026-03-10'),
          cost: 799,
        }),
      }),
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        ClaimsService,
        { provide: PrismaService, useValue: prisma },
        { provide: DeliveryService, useValue: deliveryService },
      ],
    }).compile();

    service = module.get<ClaimsService>(ClaimsService);
  });

  describe('findAll', () => {
    it('should return paginated user claims', async () => {
      prisma.claim.findMany.mockResolvedValue([
        {
          id: 'claim-1',
          poolId: 'pool-1',
          status: 'ACTIVE',
          activatedAt: new Date(),
          consumedAt: null,
          pool: { id: 'pool-1', hue: 'GREEN', band: 1, region: 'PDX', qualityTier: 'NEW' },
        },
      ]);
      prisma.claim.count.mockResolvedValue(1);

      const result = await service.findAll('user-1');

      expect(result.data).toHaveLength(1);
      expect(result.data[0].status).toBe('ACTIVE');
      expect(result.meta.total).toBe(1);
    });

    it('should filter by status', async () => {
      prisma.claim.findMany.mockResolvedValue([]);
      prisma.claim.count.mockResolvedValue(0);

      await service.findAll('user-1', { status: 'CONSUMED' });

      expect(prisma.claim.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: { userId: 'user-1', status: 'CONSUMED' },
        }),
      );
    });
  });

  describe('findOne', () => {
    it('should return claim with pool and receipt info', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        poolId: 'pool-1',
        status: 'ACTIVE',
        activatedAt: new Date(),
        consumedAt: null,
        pool: { id: 'pool-1', hue: 'GREEN', band: 1, region: 'PDX', qualityTier: 'NEW', status: 'ACTIVE' },
        receipt: { id: 'r-1', finalBand: 2, conditionGrade: 'good', submission: { category: 'tools' } },
        consumption: null,
      });
      prisma.claimConsumption.findUnique.mockResolvedValue(null);

      const result = await service.findOne('user-1', 'claim-1');

      expect(result.id).toBe('claim-1');
      expect(result.actions).toContain('reserve');
      expect(result.reservation).toBeNull();
    });

    it('should throw NotFoundException for unknown claim', async () => {
      prisma.claim.findUnique.mockResolvedValue(null);

      await expect(service.findOne('user-1', 'nope')).rejects.toThrow(
        NotFoundException,
      );
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'other-user',
      });

      await expect(service.findOne('user-1', 'claim-1')).rejects.toThrow(
        ForbiddenException,
      );
    });
  });

  describe('reserve', () => {
    it('should reserve an available inventory item', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        poolId: 'pool-1',
        status: 'ACTIVE',
      });
      prisma.claimConsumption.findUnique.mockResolvedValue(null);
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'AVAILABLE',
        poolId: 'pool-1',
      });
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.claimConsumption.create.mockResolvedValue({});

      const result = await service.reserve('user-1', 'claim-1', 'inv-1');

      expect(result.claimId).toBe('claim-1');
      expect(result.inventoryId).toBe('inv-1');
      expect(result.expiresAt).toBeDefined();
      expect(prisma.inventoryItem.update).toHaveBeenCalledWith({
        where: { id: 'inv-1' },
        data: expect.objectContaining({
          status: 'RESERVED',
          reservedById: 'user-1',
        }),
      });
    });

    it('should throw when claim is not ACTIVE', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        status: 'CONSUMED',
      });

      await expect(
        service.reserve('user-1', 'claim-1', 'inv-1'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when item is not available', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        poolId: 'pool-1',
        status: 'ACTIVE',
      });
      prisma.claimConsumption.findUnique.mockResolvedValue(null);
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'RESERVED',
        poolId: 'pool-1',
      });

      await expect(
        service.reserve('user-1', 'claim-1', 'inv-1'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when item is in different pool', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        poolId: 'pool-1',
        status: 'ACTIVE',
      });
      prisma.claimConsumption.findUnique.mockResolvedValue(null);
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'AVAILABLE',
        poolId: 'pool-2', // different pool
      });

      await expect(
        service.reserve('user-1', 'claim-1', 'inv-1'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when claim already has reservation', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        poolId: 'pool-1',
        status: 'ACTIVE',
      });
      prisma.claimConsumption.findUnique.mockResolvedValue({
        inventory: { id: 'inv-existing', status: 'RESERVED', reservedAt: new Date(), reservationExpires: new Date() },
      });

      await expect(
        service.reserve('user-1', 'claim-1', 'inv-1'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown claim', async () => {
      prisma.claim.findUnique.mockResolvedValue(null);

      await expect(
        service.reserve('user-1', 'nope', 'inv-1'),
      ).rejects.toThrow(NotFoundException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'other-user',
        status: 'ACTIVE',
      });

      await expect(
        service.reserve('user-1', 'claim-1', 'inv-1'),
      ).rejects.toThrow(ForbiddenException);
    });
  });

  describe('cancelReservation', () => {
    it('should release item and delete consumption', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        consumption: { id: 'cons-1', inventoryId: 'inv-1' },
      });
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'RESERVED',
      });
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.claimConsumption.delete.mockResolvedValue({});

      const result = await service.cancelReservation('user-1', 'claim-1');

      expect(result.status).toBe('cancelled');
      expect(prisma.inventoryItem.update).toHaveBeenCalledWith({
        where: { id: 'inv-1' },
        data: expect.objectContaining({
          status: 'AVAILABLE',
          reservedById: null,
        }),
      });
    });

    it('should throw when no reservation exists', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        consumption: null,
      });

      await expect(
        service.cancelReservation('user-1', 'claim-1'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'other-user',
      });

      await expect(
        service.cancelReservation('user-1', 'claim-1'),
      ).rejects.toThrow(ForbiddenException);
    });
  });

  describe('finalize', () => {
    const mockWarehouse = {
      id: 'wh-1',
      name: 'PDX Warehouse',
      address: { street: '100 Warehouse Blvd', city: 'Portland', state: 'OR', zip: '97201', country: 'US' },
    };

    it('should consume claim and create outbound shipment with carrier info', async () => {
      const futureDate = new Date(Date.now() + 86400000);
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        status: 'ACTIVE',
        consumption: { id: 'cons-1', inventoryId: 'inv-1' },
      });
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'RESERVED',
        reservationExpires: futureDate,
        warehouse: mockWarehouse,
        receipt: { submission: {} },
      });
      prisma.claim.update.mockResolvedValue({});
      prisma.claimConsumption.update.mockResolvedValue({});
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.shipment.create.mockResolvedValue({
        id: 'ship-1',
        carrier: 'DevCarrier',
        trackingNumber: 'DEV-ABC123',
        estimatedDelivery: new Date('2026-03-10'),
      });

      const result = await service.finalize(
        'user-1',
        'claim-1',
        'standard',
        { street: '123 Main', city: 'PDX', state: 'OR', zip: '97201', country: 'US' },
      );

      expect(result.status).toBe('finalized');
      expect(result.shipmentId).toBe('ship-1');
      expect(result.carrier).toBe('DevCarrier');
      expect(result.trackingNumber).toBe('DEV-ABC123');
      expect(deliveryService.getProviderForMethod).toHaveBeenCalledWith('standard');
      expect(prisma.claim.update).toHaveBeenCalledWith({
        where: { id: 'claim-1' },
        data: expect.objectContaining({ status: 'CONSUMED' }),
      });
      expect(prisma.shipment.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          carrier: 'DevCarrier',
          trackingNumber: 'DEV-ABC123',
        }),
      });
    });

    it('should throw when claim is already consumed', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        status: 'CONSUMED',
        consumption: { id: 'cons-1', inventoryId: 'inv-1' },
      });

      await expect(
        service.finalize('user-1', 'claim-1', 'standard'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when no reservation exists', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        status: 'ACTIVE',
        consumption: null,
      });

      await expect(
        service.finalize('user-1', 'claim-1', 'standard'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when reservation has expired', async () => {
      const pastDate = new Date(Date.now() - 86400000);
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        status: 'ACTIVE',
        consumption: { id: 'cons-1', inventoryId: 'inv-1' },
      });
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'RESERVED',
        reservationExpires: pastDate,
        warehouse: mockWarehouse,
        receipt: { submission: {} },
      });

      await expect(
        service.finalize('user-1', 'claim-1', 'standard'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown claim', async () => {
      prisma.claim.findUnique.mockResolvedValue(null);

      await expect(
        service.finalize('user-1', 'nope', 'standard'),
      ).rejects.toThrow(NotFoundException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'other-user',
        status: 'ACTIVE',
      });

      await expect(
        service.finalize('user-1', 'claim-1', 'standard'),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should still finalize when delivery provider fails', async () => {
      const futureDate = new Date(Date.now() + 86400000);
      deliveryService.getProviderForMethod.mockReturnValue({
        name: 'broken-provider',
        createShipment: jest.fn().mockRejectedValue(new Error('API down')),
      });

      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
        status: 'ACTIVE',
        consumption: { id: 'cons-1', inventoryId: 'inv-1' },
      });
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'RESERVED',
        reservationExpires: futureDate,
        warehouse: mockWarehouse,
        receipt: { submission: {} },
      });
      prisma.claim.update.mockResolvedValue({});
      prisma.claimConsumption.update.mockResolvedValue({});
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.shipment.create.mockResolvedValue({
        id: 'ship-1',
        carrier: null,
        trackingNumber: null,
        estimatedDelivery: null,
      });

      const result = await service.finalize(
        'user-1',
        'claim-1',
        'standard',
      );

      expect(result.status).toBe('finalized');
      expect(result.shipmentId).toBe('ship-1');
      // Shipment created without carrier info
      expect(prisma.shipment.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          carrier: undefined,
          trackingNumber: undefined,
        }),
      });
    });
  });
});
