import { Test, TestingModule } from '@nestjs/testing';
import { NotFoundException, ForbiddenException } from '@nestjs/common';
import { NotificationsService } from './notifications.service';
import { PrismaService } from '../prisma/prisma.service';

describe('NotificationsService', () => {
  let service: NotificationsService;
  let prisma: any;

  beforeEach(async () => {
    prisma = {
      notification: {
        findMany: jest.fn(),
        findUnique: jest.fn(),
        count: jest.fn(),
        create: jest.fn(),
        update: jest.fn(),
      },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        NotificationsService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<NotificationsService>(NotificationsService);
  });

  describe('findAll', () => {
    it('should return paginated notifications', async () => {
      prisma.notification.findMany.mockResolvedValue([
        { id: 'n-1', eventType: 'claim_activated', content: {}, readAt: null },
      ]);
      prisma.notification.count.mockResolvedValue(1);

      const result = await service.findAll('user-1');

      expect(result.data).toHaveLength(1);
      expect(result.meta.total).toBe(1);
    });

    it('should filter unread notifications', async () => {
      prisma.notification.findMany.mockResolvedValue([]);
      prisma.notification.count.mockResolvedValue(0);

      await service.findAll('user-1', { read: false });

      expect(prisma.notification.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: { userId: 'user-1', readAt: null },
        }),
      );
    });

    it('should filter read notifications', async () => {
      prisma.notification.findMany.mockResolvedValue([]);
      prisma.notification.count.mockResolvedValue(0);

      await service.findAll('user-1', { read: true });

      expect(prisma.notification.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: { userId: 'user-1', readAt: { not: null } },
        }),
      );
    });
  });

  describe('markAsRead', () => {
    it('should mark notification as read', async () => {
      prisma.notification.findUnique.mockResolvedValue({
        id: 'n-1',
        userId: 'user-1',
        readAt: null,
      });
      prisma.notification.update.mockResolvedValue({});

      const result = await service.markAsRead('user-1', 'n-1');

      expect(result.read).toBe(true);
      expect(prisma.notification.update).toHaveBeenCalledWith({
        where: { id: 'n-1' },
        data: { readAt: expect.any(Date) },
      });
    });

    it('should throw NotFoundException for unknown notification', async () => {
      prisma.notification.findUnique.mockResolvedValue(null);

      await expect(service.markAsRead('user-1', 'nope')).rejects.toThrow(
        NotFoundException,
      );
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.notification.findUnique.mockResolvedValue({
        id: 'n-1',
        userId: 'other-user',
      });

      await expect(service.markAsRead('user-1', 'n-1')).rejects.toThrow(
        ForbiddenException,
      );
    });
  });

  describe('create', () => {
    it('should create a notification', async () => {
      prisma.notification.create.mockResolvedValue({
        id: 'n-1',
        eventType: 'shipment_delivered',
      });

      const result = await service.create(
        'user-1',
        'shipment_delivered',
        { shipmentId: 'ship-1', message: 'Your item has been delivered' },
      );

      expect(result.id).toBe('n-1');
      expect(prisma.notification.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          userId: 'user-1',
          eventType: 'shipment_delivered',
          channel: 'PUSH',
        }),
      });
    });

    it('should support custom channel', async () => {
      prisma.notification.create.mockResolvedValue({ id: 'n-2' });

      await service.create('user-1', 'claim_activated', {}, 'EMAIL');

      expect(prisma.notification.create).toHaveBeenCalledWith({
        data: expect.objectContaining({ channel: 'EMAIL' }),
      });
    });
  });
});
