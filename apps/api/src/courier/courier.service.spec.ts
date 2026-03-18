import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { CourierService } from './courier.service';
import { PrismaService } from '../prisma/prisma.service';
import { NotificationsService } from '../notifications/notifications.service';

describe('CourierService', () => {
  let service: CourierService;
  let prisma: any;
  let notificationsService: any;

  const approvedCourier = {
    id: 'courier-1',
    role: 'COURIER',
    kycStatus: 'APPROVED',
  };

  beforeEach(async () => {
    prisma = {
      user: { findUnique: jest.fn(), findMany: jest.fn() },
      courierTask: {
        findUnique: jest.fn(),
        findMany: jest.fn(),
        findFirst: jest.fn(),
        count: jest.fn(),
        create: jest.fn(),
        update: jest.fn(),
        aggregate: jest.fn(),
      },
      courierEvent: { create: jest.fn() },
      shipment: { findUnique: jest.fn(), update: jest.fn() },
      inventoryItem: { update: jest.fn() },
      inventoryStatusEvent: { create: jest.fn() },
      $transaction: jest.fn((fn) => fn(prisma)),
    };

    notificationsService = {
      create: jest.fn().mockResolvedValue({}),
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        CourierService,
        { provide: PrismaService, useValue: prisma },
        { provide: NotificationsService, useValue: notificationsService },
      ],
    }).compile();

    service = module.get<CourierService>(CourierService);
  });

  describe('getAvailableTasks', () => {
    it('should return paginated posted tasks with redacted addresses', async () => {
      prisma.user.findUnique.mockResolvedValue(approvedCourier);
      prisma.courierTask.findMany.mockResolvedValue([
        {
          id: 'task-1',
          shipmentId: 'ship-1',
          pickupLocation: {
            street: '123 Warehouse Blvd',
            city: 'Portland',
            state: 'OR',
            region: 'OR',
          },
          dropoffLocation: {
            street: '456 Main St',
            city: 'Portland',
            state: 'OR',
            region: 'OR',
          },
          fee: 9.99,
          tip: null,
          status: 'POSTED',
          timeWindowStart: new Date(),
          timeWindowEnd: new Date(),
          createdAt: new Date(),
          shipment: { id: 'ship-1', direction: 'OUTBOUND', status: 'CREATED' },
        },
      ]);
      prisma.courierTask.count.mockResolvedValue(1);

      const result = await service.getAvailableTasks('courier-1');

      expect(result.data).toHaveLength(1);
      expect(result.data[0].pickupLocation).not.toHaveProperty('street');
      expect(result.data[0].pickupLocation.city).toBe('Portland');
      expect(result.meta.total).toBe(1);
    });

    it('should reject unapproved couriers', async () => {
      prisma.user.findUnique.mockResolvedValue({
        ...approvedCourier,
        kycStatus: 'NONE',
      });

      await expect(
        service.getAvailableTasks('courier-1'),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should reject non-courier users', async () => {
      prisma.user.findUnique.mockResolvedValue({
        ...approvedCourier,
        role: 'CONTRIBUTOR',
      });

      await expect(
        service.getAvailableTasks('courier-1'),
      ).rejects.toThrow(ForbiddenException);
    });
  });

  describe('acceptTask', () => {
    it('should accept a posted task and lock it to courier', async () => {
      prisma.user.findUnique.mockResolvedValue(approvedCourier);
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        status: 'POSTED',
      });
      prisma.courierTask.update.mockResolvedValue({});
      prisma.courierEvent.create.mockResolvedValue({});

      const result = await service.acceptTask('courier-1', 'task-1');

      expect(result.status).toBe('ACCEPTED');
      expect(prisma.courierTask.update).toHaveBeenCalledWith({
        where: { id: 'task-1' },
        data: { courierId: 'courier-1', status: 'ACCEPTED' },
      });
    });

    it('should throw when task is not POSTED', async () => {
      prisma.user.findUnique.mockResolvedValue(approvedCourier);
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        status: 'ACCEPTED',
      });

      await expect(
        service.acceptTask('courier-1', 'task-1'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown task', async () => {
      prisma.user.findUnique.mockResolvedValue(approvedCourier);
      prisma.courierTask.findUnique.mockResolvedValue(null);

      await expect(
        service.acceptTask('courier-1', 'nope'),
      ).rejects.toThrow(NotFoundException);
    });
  });

  describe('confirmPickup', () => {
    it('should verify pickup with QR code and generate proof hash', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'ACCEPTED',
        pickupLocation: { city: 'Portland' },
      });
      prisma.courierTask.update.mockResolvedValue({});
      prisma.courierEvent.create.mockResolvedValue({});

      const result = await service.confirmPickup('courier-1', 'task-1', {
        qrCode: 'QR-SCAN-DATA',
      });

      expect(result.status).toBe('PICKUP_VERIFIED');
      expect(result.proofHash).toBeDefined();
      expect(result.proofHash.length).toBe(64);
    });

    it('should verify pickup with PIN', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'ACCEPTED',
        pickupLocation: {},
      });
      prisma.courierTask.update.mockResolvedValue({});
      prisma.courierEvent.create.mockResolvedValue({});

      const result = await service.confirmPickup('courier-1', 'task-1', {
        pin: '1234',
      });

      expect(result.status).toBe('PICKUP_VERIFIED');
    });

    it('should throw without QR or PIN', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'ACCEPTED',
      });

      await expect(
        service.confirmPickup('courier-1', 'task-1', {}),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when task not in ACCEPTED status', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'POSTED',
      });

      await expect(
        service.confirmPickup('courier-1', 'task-1', { qrCode: 'data' }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'other-courier',
        status: 'ACCEPTED',
      });

      await expect(
        service.confirmPickup('courier-1', 'task-1', { qrCode: 'data' }),
      ).rejects.toThrow(ForbiddenException);
    });
  });

  describe('reportMilestone', () => {
    it('should record milestone and transition to IN_TRANSIT', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'PICKUP_VERIFIED',
      });
      prisma.courierTask.update.mockResolvedValue({});
      prisma.courierEvent.create.mockResolvedValue({});

      const result = await service.reportMilestone(
        'courier-1',
        'task-1',
        { lat: 45.5, lng: -122.6 },
        'Passing through downtown',
      );

      expect(result.status).toBe('IN_TRANSIT_COURIER');
      expect(prisma.courierTask.update).toHaveBeenCalledWith({
        where: { id: 'task-1' },
        data: { status: 'IN_TRANSIT_COURIER' },
      });
    });

    it('should allow multiple milestones while IN_TRANSIT', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'IN_TRANSIT_COURIER',
      });
      prisma.courierEvent.create.mockResolvedValue({});

      const result = await service.reportMilestone(
        'courier-1',
        'task-1',
        { lat: 45.51, lng: -122.61 },
      );

      expect(result.status).toBe('IN_TRANSIT_COURIER');
      // Should not update status since already IN_TRANSIT_COURIER
      expect(prisma.courierTask.update).not.toHaveBeenCalled();
    });

    it('should throw for invalid status', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'ACCEPTED',
      });

      await expect(
        service.reportMilestone('courier-1', 'task-1', { lat: 0, lng: 0 }),
      ).rejects.toThrow(BadRequestException);
    });
  });

  describe('confirmDelivery', () => {
    it('should confirm delivery with proof hash and update shipment', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        shipmentId: 'ship-1',
        status: 'IN_TRANSIT_COURIER',
        dropoffLocation: { city: 'Portland' },
      });
      prisma.courierTask.update.mockResolvedValue({});
      prisma.courierEvent.create.mockResolvedValue({});
      prisma.shipment.findUnique.mockResolvedValue({
        id: 'ship-1',
        inventoryId: 'inv-1',
        direction: 'OUTBOUND',
      });
      prisma.shipment.update.mockResolvedValue({});
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});

      const result = await service.confirmDelivery(
        'courier-1',
        'task-1',
        'photo',
        'base64-photo-data',
      );

      expect(result.status).toBe('DELIVERED_COURIER');
      expect(result.proofHash.length).toBe(64);
      expect(prisma.shipment.update).toHaveBeenCalledWith({
        where: { id: 'ship-1' },
        data: expect.objectContaining({
          status: 'DELIVERED',
          proofHash: result.proofHash,
        }),
      });
      expect(prisma.inventoryItem.update).toHaveBeenCalledWith({
        where: { id: 'inv-1' },
        data: { status: 'DELIVERED' },
      });
    });

    it('should reject invalid proof method', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'IN_TRANSIT_COURIER',
      });

      await expect(
        service.confirmDelivery('courier-1', 'task-1', 'telepathy', 'data'),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw for wrong status', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'ACCEPTED',
      });

      await expect(
        service.confirmDelivery('courier-1', 'task-1', 'qr', 'data'),
      ).rejects.toThrow(BadRequestException);
    });
  });

  describe('completeTask', () => {
    it('should complete a delivered task and notify courier', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'DELIVERED_COURIER',
        fee: 9.99,
        tip: 2.0,
      });
      prisma.courierTask.update.mockResolvedValue({});
      prisma.courierEvent.create.mockResolvedValue({});

      const result = await service.completeTask('task-1');

      expect(result.status).toBe('COMPLETED');
      expect(notificationsService.create).toHaveBeenCalledWith(
        'courier-1',
        'payout_released',
        expect.objectContaining({
          fee: 9.99,
          tip: 2.0,
          total: 11.99,
        }),
      );
    });

    it('should throw when task not delivered yet', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        status: 'IN_TRANSIT_COURIER',
      });

      await expect(service.completeTask('task-1')).rejects.toThrow(
        BadRequestException,
      );
    });
  });

  describe('getEarnings', () => {
    it('should return earnings with summary', async () => {
      prisma.user.findUnique.mockResolvedValue(approvedCourier);
      prisma.courierTask.findMany.mockResolvedValue([
        { id: 'task-1', fee: 9.99, tip: 2.0, status: 'COMPLETED', createdAt: new Date() },
        { id: 'task-2', fee: 12.5, tip: null, status: 'DELIVERED_COURIER', createdAt: new Date() },
      ]);
      prisma.courierTask.count.mockResolvedValue(2);
      prisma.courierTask.aggregate
        .mockResolvedValueOnce({
          _sum: { fee: 9.99, tip: 2.0 },
          _count: 1,
        })
        .mockResolvedValueOnce({
          _sum: { fee: 12.5, tip: null },
          _count: 1,
        });

      const result = await service.getEarnings('courier-1');

      expect(result.data).toHaveLength(2);
      expect(result.data[0].status).toBe('paid');
      expect(result.data[1].status).toBe('pending');
      expect(result.summary.totalEarned).toBe(11.99);
      expect(result.summary.totalPending).toBe(12.5);
      expect(result.summary.completedTasks).toBe(1);
    });
  });

  describe('reportEmergency', () => {
    it('should create exception event and notify admins', async () => {
      prisma.courierTask.findUnique.mockResolvedValue({
        id: 'task-1',
        courierId: 'courier-1',
        status: 'IN_TRANSIT_COURIER',
      });
      prisma.courierEvent.create.mockResolvedValue({});
      prisma.user.findMany.mockResolvedValue([
        { id: 'admin-1' },
        { id: 'admin-2' },
      ]);

      const result = await service.reportEmergency('courier-1', 'task-1', {
        type: 'unsafe_location',
        description: 'Address appears abandoned',
        location: { lat: 45.5, lng: -122.6 },
      });

      expect(result.reported).toBe(true);
      expect(notificationsService.create).toHaveBeenCalledTimes(2);
      expect(notificationsService.create).toHaveBeenCalledWith(
        'admin-1',
        'courier_emergency',
        expect.objectContaining({
          reportType: 'unsafe_location',
        }),
      );
    });
  });
});
