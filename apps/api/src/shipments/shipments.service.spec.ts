import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { ShipmentsService } from './shipments.service';
import { PrismaService } from '../prisma/prisma.service';
import { DeliveryService } from '../delivery/delivery.service';
import { NotificationsService } from '../notifications/notifications.service';

describe('ShipmentsService', () => {
  let service: ShipmentsService;
  let prisma: any;
  let deliveryService: any;
  let notificationsService: any;

  beforeEach(async () => {
    prisma = {
      shipment: {
        findMany: jest.fn(),
        findUnique: jest.fn(),
        count: jest.fn(),
        update: jest.fn(),
      },
      trackingEvent: {
        findMany: jest.fn(),
        findFirst: jest.fn(),
        create: jest.fn(),
      },
      inventoryItem: { findUnique: jest.fn(), update: jest.fn() },
      inventoryStatusEvent: { create: jest.fn() },
      itemSubmission: { findUnique: jest.fn() },
      $transaction: jest.fn((fn) => fn(prisma)),
    };

    deliveryService = {
      getTrackingEvents: jest.fn(),
    };

    notificationsService = {
      create: jest.fn().mockResolvedValue({}),
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        ShipmentsService,
        { provide: PrismaService, useValue: prisma },
        { provide: DeliveryService, useValue: deliveryService },
        { provide: NotificationsService, useValue: notificationsService },
      ],
    }).compile();

    service = module.get<ShipmentsService>(ShipmentsService);
  });

  describe('findAll', () => {
    it('should return paginated user shipments', async () => {
      prisma.shipment.findMany.mockResolvedValue([
        {
          id: 'ship-1',
          direction: 'OUTBOUND',
          carrier: 'UPS',
          trackingNumber: '1Z999',
          status: 'IN_TRANSIT',
          estimatedDelivery: null,
          deliveredAt: null,
          createdAt: new Date(),
          trackingEvents: [{ id: 'ev-1', eventType: 'in_transit' }],
        },
      ]);
      prisma.shipment.count.mockResolvedValue(1);

      const result = await service.findAll('user-1');

      expect(result.data).toHaveLength(1);
      expect(result.data[0].lastEvent).toBeDefined();
      expect(result.meta.total).toBe(1);
    });

    it('should filter by direction', async () => {
      prisma.shipment.findMany.mockResolvedValue([]);
      prisma.shipment.count.mockResolvedValue(0);

      await service.findAll('user-1', { direction: 'INBOUND' });

      expect(prisma.shipment.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: expect.objectContaining({ direction: 'INBOUND' }),
        }),
      );
    });
  });

  describe('findOne', () => {
    it('should return shipment with timeline for submission owner', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        direction: 'INBOUND',
        carrier: null,
        trackingNumber: null,
        status: 'CREATED',
        estimatedDelivery: null,
        deliveredAt: null,
        proofHash: null,
        createdAt: new Date(),
        submission: { userId: 'user-1', id: 'sub-1', category: 'tools' },
        inventory: null,
        trackingEvents: [],
      });

      const result = await service.findOne('user-1', 'ship-1');

      expect(result.id).toBe('ship-1');
      expect(result.timeline).toEqual([]);
    });

    it('should return shipment for reservation holder', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-2',
        direction: 'OUTBOUND',
        status: 'CREATED',
        submission: null,
        inventory: { id: 'inv-1', reservedById: 'user-1', binLocation: 'A1', receipt: null },
        trackingEvents: [],
        carrier: null,
        trackingNumber: null,
        estimatedDelivery: null,
        deliveredAt: null,
        proofHash: null,
        createdAt: new Date(),
      });

      const result = await service.findOne('user-1', 'ship-2');
      expect(result.id).toBe('ship-2');
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        submission: { userId: 'other-user' },
        inventory: null,
        trackingEvents: [],
      });

      await expect(service.findOne('user-1', 'ship-1')).rejects.toThrow(
        ForbiddenException,
      );
    });

    it('should throw NotFoundException for unknown shipment', async () => {
      prisma.shipment.findUnique.mockResolvedValue(null);

      await expect(service.findOne('user-1', 'nope')).rejects.toThrow(
        NotFoundException,
      );
    });
  });

  describe('addTrackingEvent', () => {
    it('should add a tracking event and update shipment status', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'CREATED',
      });
      prisma.trackingEvent.create.mockResolvedValue({
        id: 'ev-1',
        eventType: 'picked_up',
      });
      prisma.shipment.update.mockResolvedValue({});

      const result = await service.addTrackingEvent(
        'ship-1',
        'picked_up',
        'Portland, OR',
      );

      expect(result.eventType).toBe('picked_up');
      expect(prisma.shipment.update).toHaveBeenCalledWith({
        where: { id: 'ship-1' },
        data: { status: 'PICKED_UP' },
      });
    });

    it('should not update status for unknown event types', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'IN_TRANSIT',
      });
      prisma.trackingEvent.create.mockResolvedValue({
        id: 'ev-1',
        eventType: 'customs_cleared',
      });

      await service.addTrackingEvent('ship-1', 'customs_cleared');

      expect(prisma.shipment.update).not.toHaveBeenCalled();
    });

    it('should throw when shipment already delivered', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'DELIVERED',
      });

      await expect(
        service.addTrackingEvent('ship-1', 'picked_up'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown shipment', async () => {
      prisma.shipment.findUnique.mockResolvedValue(null);

      await expect(
        service.addTrackingEvent('nope', 'picked_up'),
      ).rejects.toThrow(NotFoundException);
    });
  });

  describe('confirmDelivery', () => {
    it('should confirm delivery with proof and update statuses', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'IN_TRANSIT',
        direction: 'OUTBOUND',
        inventoryId: 'inv-1',
        inventory: { id: 'inv-1' },
      });
      prisma.shipment.update.mockResolvedValue({});
      prisma.trackingEvent.create.mockResolvedValue({});
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});

      const result = await service.confirmDelivery(
        'user-1',
        'ship-1',
        'photo',
        'base64-proof-data',
        'Left at door',
      );

      expect(result.status).toBe('delivered');
      expect(result.proofHash).toBeDefined();
      expect(result.proofHash.length).toBe(64); // SHA-256 hex
      expect(prisma.inventoryItem.update).toHaveBeenCalledWith({
        where: { id: 'inv-1' },
        data: { status: 'DELIVERED' },
      });
    });

    it('should not update inventory for inbound shipments', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'IN_TRANSIT',
        direction: 'INBOUND',
        inventoryId: null,
        inventory: null,
      });
      prisma.shipment.update.mockResolvedValue({});
      prisma.trackingEvent.create.mockResolvedValue({});

      const result = await service.confirmDelivery(
        'user-1',
        'ship-1',
        'signature',
        'sig-data',
      );

      expect(result.status).toBe('delivered');
      expect(prisma.inventoryItem.update).not.toHaveBeenCalled();
    });

    it('should throw when already delivered', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'DELIVERED',
      });

      await expect(
        service.confirmDelivery('user-1', 'ship-1', 'photo', 'data'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown shipment', async () => {
      prisma.shipment.findUnique.mockResolvedValue(null);

      await expect(
        service.confirmDelivery('user-1', 'nope', 'photo', 'data'),
      ).rejects.toThrow(NotFoundException);
    });
  });

  describe('exception handling', () => {
    it('should notify user on delivery exception event', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        status: 'IN_TRANSIT',
        submissionId: 'sub-1',
        trackingNumber: 'TRK-123',
      });
      prisma.trackingEvent.create.mockResolvedValue({
        id: 'ev-1',
        eventType: 'exception',
      });
      prisma.shipment.update.mockResolvedValue({});
      prisma.itemSubmission.findUnique.mockResolvedValue({
        userId: 'user-1',
      });

      await service.addTrackingEvent('ship-1', 'exception', 'Memphis, TN');

      expect(prisma.shipment.update).toHaveBeenCalledWith({
        where: { id: 'ship-1' },
        data: { status: 'EXCEPTION' },
      });
      expect(notificationsService.create).toHaveBeenCalledWith(
        'user-1',
        'delivery_exception',
        expect.objectContaining({
          shipmentId: 'ship-1',
          trackingNumber: 'TRK-123',
        }),
      );
    });

    it('should notify outbound shipment owner on exception', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-2',
        status: 'IN_TRANSIT',
        submissionId: null,
        inventoryId: 'inv-1',
        trackingNumber: 'TRK-456',
      });
      prisma.trackingEvent.create.mockResolvedValue({
        id: 'ev-2',
        eventType: 'exception',
      });
      prisma.shipment.update.mockResolvedValue({});
      prisma.inventoryItem.findUnique.mockResolvedValue({
        reservedById: 'user-2',
      });

      await service.addTrackingEvent('ship-2', 'exception');

      expect(notificationsService.create).toHaveBeenCalledWith(
        'user-2',
        'delivery_exception',
        expect.objectContaining({ shipmentId: 'ship-2' }),
      );
    });
  });

  describe('syncProviderTracking', () => {
    it('should sync events from delivery provider', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        carrier: 'DevCarrier',
        trackingNumber: 'DEV-ABC123',
        status: 'CREATED',
      });

      deliveryService.getTrackingEvents.mockResolvedValue([
        {
          eventType: 'created',
          timestamp: new Date('2026-03-01'),
          description: 'Shipment created',
        },
      ]);

      // No existing event
      prisma.trackingEvent.findFirst.mockResolvedValue(null);
      prisma.trackingEvent.create.mockResolvedValue({});

      await service.syncProviderTracking('ship-1');

      expect(deliveryService.getTrackingEvents).toHaveBeenCalledWith(
        'local-dev',
        'DEV-ABC123',
      );
    });

    it('should skip shipments without carrier', async () => {
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        carrier: null,
        trackingNumber: null,
      });

      await service.syncProviderTracking('ship-1');

      expect(deliveryService.getTrackingEvents).not.toHaveBeenCalled();
    });
  });
});
