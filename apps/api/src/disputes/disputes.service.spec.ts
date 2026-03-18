import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { DisputesService } from './disputes.service';
import { PrismaService } from '../prisma/prisma.service';
import { NotificationsService } from '../notifications/notifications.service';

describe('DisputesService', () => {
  let service: DisputesService;
  let prisma: any;
  let notificationsService: any;

  beforeEach(async () => {
    prisma = {
      dispute: {
        findUnique: jest.fn(),
        findMany: jest.fn(),
        findFirst: jest.fn(),
        count: jest.fn(),
        create: jest.fn(),
        update: jest.fn(),
      },
      disputeEvidence: { create: jest.fn() },
      claim: { findUnique: jest.fn(), create: jest.fn() },
      itemSubmission: { findUnique: jest.fn() },
      itemReceipt: { findUnique: jest.fn(), findFirst: jest.fn() },
      inventoryItem: { findUnique: jest.fn() },
      shipment: { findUnique: jest.fn() },
      courierTask: { findUnique: jest.fn() },
      operatorAction: { create: jest.fn() },
      $transaction: jest.fn((fn) => fn(prisma)),
    };

    notificationsService = {
      create: jest.fn().mockResolvedValue({}),
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        DisputesService,
        { provide: PrismaService, useValue: prisma },
        { provide: NotificationsService, useValue: notificationsService },
      ],
    }).compile();

    service = module.get<DisputesService>(DisputesService);
  });

  describe('create', () => {
    it('should create a dispute with SLA deadline', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
      });
      prisma.dispute.findFirst.mockResolvedValue(null);
      prisma.dispute.create.mockResolvedValue({
        id: 'dispute-1',
        slaDeadline: new Date(),
      });

      const result = await service.create('user-1', {
        objectType: 'CLAIM',
        objectId: 'claim-1',
        reason: 'Grading was unfair',
      });

      expect(result.disputeId).toBe('dispute-1');
      expect(result.slaDeadline).toBeDefined();
      expect(notificationsService.create).toHaveBeenCalledWith(
        'user-1',
        'dispute_opened',
        expect.objectContaining({ disputeId: 'dispute-1' }),
      );
    });

    it('should reject duplicate open disputes', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'user-1',
      });
      prisma.dispute.findFirst.mockResolvedValue({ id: 'existing-dispute' });

      await expect(
        service.create('user-1', {
          objectType: 'CLAIM',
          objectId: 'claim-1',
          reason: 'Duplicate',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when referenced object not found', async () => {
      prisma.claim.findUnique.mockResolvedValue(null);

      await expect(
        service.create('user-1', {
          objectType: 'CLAIM',
          objectId: 'nope',
          reason: 'Bad',
        }),
      ).rejects.toThrow(NotFoundException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        userId: 'other-user',
      });

      await expect(
        service.create('user-1', {
          objectType: 'CLAIM',
          objectId: 'claim-1',
          reason: 'Not mine',
        }),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should validate submission ownership', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
      });
      prisma.dispute.findFirst.mockResolvedValue(null);
      prisma.dispute.create.mockResolvedValue({
        id: 'dispute-2',
        slaDeadline: new Date(),
      });

      const result = await service.create('user-1', {
        objectType: 'SUBMISSION',
        objectId: 'sub-1',
        reason: 'Issue with submission',
      });

      expect(result.disputeId).toBe('dispute-2');
    });
  });

  describe('submitEvidence', () => {
    it('should store text evidence with CID hash', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'user-1',
        status: 'OPEN',
      });
      prisma.disputeEvidence.create.mockResolvedValue({
        id: 'ev-1',
        evidenceType: 'TEXT',
        content: 'The item was damaged',
        cidHash: expect.any(String),
      });

      const result = await service.submitEvidence(
        'user-1',
        'dispute-1',
        { evidenceType: 'TEXT', content: 'The item was damaged' },
      );

      expect(prisma.disputeEvidence.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          disputeId: 'dispute-1',
          evidenceType: 'TEXT',
          content: 'The item was damaged',
          cidHash: expect.any(String),
        }),
      });
    });

    it('should reject text evidence without content', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'user-1',
        status: 'OPEN',
      });

      await expect(
        service.submitEvidence('user-1', 'dispute-1', {
          evidenceType: 'TEXT',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject evidence on resolved dispute', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'user-1',
        status: 'RESOLVED',
      });

      await expect(
        service.submitEvidence('user-1', 'dispute-1', {
          evidenceType: 'TEXT',
          content: 'Late evidence',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject evidence from non-owner (non-admin)', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'other-user',
        status: 'OPEN',
      });

      await expect(
        service.submitEvidence('user-1', 'dispute-1', {
          evidenceType: 'TEXT',
          content: 'Not mine',
        }),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should allow admin to submit evidence to any dispute', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'other-user',
        status: 'OPEN',
      });
      prisma.disputeEvidence.create.mockResolvedValue({
        id: 'ev-1',
        evidenceType: 'TEXT',
      });

      await expect(
        service.submitEvidence(
          'admin-1',
          'dispute-1',
          { evidenceType: 'TEXT', content: 'Admin note' },
          undefined,
          true,
        ),
      ).resolves.toBeDefined();
    });

    it('should throw NotFoundException for unknown dispute', async () => {
      prisma.dispute.findUnique.mockResolvedValue(null);

      await expect(
        service.submitEvidence('user-1', 'nope', {
          evidenceType: 'TEXT',
          content: 'x',
        }),
      ).rejects.toThrow(NotFoundException);
    });
  });

  describe('findAll', () => {
    it('should return paginated user disputes', async () => {
      prisma.dispute.findMany.mockResolvedValue([
        {
          id: 'dispute-1',
          objectType: 'CLAIM',
          objectId: 'claim-1',
          reason: 'Unfair',
          status: 'OPEN',
          resolution: null,
          slaDeadline: new Date(),
          resolvedAt: null,
          createdAt: new Date(),
          _count: { evidence: 2 },
        },
      ]);
      prisma.dispute.count.mockResolvedValue(1);

      const result = await service.findAll('user-1');

      expect(result.data).toHaveLength(1);
      expect(result.data[0].evidenceCount).toBe(2);
      expect(result.meta.total).toBe(1);
    });

    it('should filter by status', async () => {
      prisma.dispute.findMany.mockResolvedValue([]);
      prisma.dispute.count.mockResolvedValue(0);

      await service.findAll('user-1', { status: 'RESOLVED' });

      expect(prisma.dispute.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: { openedById: 'user-1', status: 'RESOLVED' },
        }),
      );
    });
  });

  describe('findOne', () => {
    it('should return dispute detail with evidence', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'user-1',
        objectType: 'CLAIM',
        objectId: 'claim-1',
        reason: 'Bad grading',
        status: 'OPEN',
        resolution: null,
        slaDeadline: new Date(),
        resolvedAt: null,
        createdAt: new Date(),
        evidence: [{ id: 'ev-1', evidenceType: 'TEXT', content: 'Details' }],
        openedBy: { id: 'user-1', displayName: 'Test User', email: 'test@test.com' },
        resolvedBy: null,
      });

      const result = await service.findOne('user-1', 'dispute-1');

      expect(result.id).toBe('dispute-1');
      expect(result.evidence).toHaveLength(1);
      expect(result.openedBy.displayName).toBe('Test User');
    });

    it('should throw for non-owner', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        id: 'dispute-1',
        openedById: 'other-user',
      });

      await expect(
        service.findOne('user-1', 'dispute-1'),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should throw NotFoundException', async () => {
      prisma.dispute.findUnique.mockResolvedValue(null);

      await expect(
        service.findOne('user-1', 'nope'),
      ).rejects.toThrow(NotFoundException);
    });
  });

  describe('getAdminQueue', () => {
    it('should return disputes sorted by SLA urgency', async () => {
      const urgentDeadline = new Date(Date.now() + 3600000); // 1h
      const laterDeadline = new Date(Date.now() + 86400000); // 24h

      prisma.dispute.findMany.mockResolvedValue([
        {
          id: 'dispute-urgent',
          objectType: 'CLAIM',
          objectId: 'c-1',
          reason: 'urgent',
          status: 'OPEN',
          slaDeadline: urgentDeadline,
          createdAt: new Date(),
          openedBy: { id: 'u-1', displayName: 'User', email: 'u@test.com' },
          _count: { evidence: 1 },
        },
        {
          id: 'dispute-later',
          objectType: 'SHIPMENT',
          objectId: 's-1',
          reason: 'later',
          status: 'UNDER_REVIEW',
          slaDeadline: laterDeadline,
          createdAt: new Date(),
          openedBy: { id: 'u-2', displayName: 'User2', email: 'u2@test.com' },
          _count: { evidence: 0 },
        },
      ]);
      prisma.dispute.count.mockResolvedValue(2);

      const result = await service.getAdminQueue();

      expect(result.data).toHaveLength(2);
      expect(result.data[0].id).toBe('dispute-urgent');
      expect(result.data[0].slaRemainingMs).toBeLessThan(
        result.data[1].slaRemainingMs,
      );
    });
  });

  describe('resolve', () => {
    const mockDispute = {
      id: 'dispute-1',
      openedById: 'user-1',
      objectType: 'CLAIM',
      objectId: 'claim-1',
      status: 'OPEN',
    };

    it('should resolve with REFUND_CLAIM and create new claim', async () => {
      prisma.dispute.findUnique.mockResolvedValue(mockDispute);
      prisma.dispute.update.mockResolvedValue({
        ...mockDispute,
        status: 'RESOLVED',
      });
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        poolId: 'pool-1',
        receiptId: 'receipt-1',
      });
      prisma.claim.create.mockResolvedValue({
        id: 'new-claim-1',
        status: 'ACTIVE',
      });
      prisma.operatorAction.create.mockResolvedValue({});

      const result = await service.resolve('admin-1', 'dispute-1', {
        resolution: 'REFUND_CLAIM',
        reason: 'Grading error confirmed',
      });

      expect(result.status).toBe('RESOLVED');
      expect(result.newClaimId).toBe('new-claim-1');
      expect(prisma.claim.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          userId: 'user-1',
          poolId: 'pool-1',
          receiptId: 'receipt-1',
          status: 'ACTIVE',
        }),
      });
      expect(prisma.operatorAction.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          actionType: 'DISPUTE_RESOLVE',
          targetType: 'DISPUTE',
        }),
      });
      expect(notificationsService.create).toHaveBeenCalledWith(
        'user-1',
        'dispute_resolved',
        expect.objectContaining({
          resolution: 'REFUND_CLAIM',
          newClaimId: 'new-claim-1',
        }),
      );
    });

    it('should resolve with DENY without creating claim', async () => {
      prisma.dispute.findUnique.mockResolvedValue(mockDispute);
      prisma.dispute.update.mockResolvedValue({
        ...mockDispute,
        status: 'DENIED',
      });
      prisma.operatorAction.create.mockResolvedValue({});

      const result = await service.resolve('admin-1', 'dispute-1', {
        resolution: 'DENY',
        reason: 'Insufficient evidence',
      });

      expect(result.status).toBe('DENIED');
      expect(result.newClaimId).toBeUndefined();
      expect(prisma.claim.create).not.toHaveBeenCalled();
    });

    it('should resolve with GOODWILL_CREDIT and create claim', async () => {
      prisma.dispute.findUnique.mockResolvedValue(mockDispute);
      prisma.dispute.update.mockResolvedValue({
        ...mockDispute,
        status: 'RESOLVED',
      });
      prisma.claim.findUnique.mockResolvedValue({
        id: 'claim-1',
        poolId: 'pool-1',
        receiptId: 'receipt-1',
      });
      prisma.claim.create.mockResolvedValue({
        id: 'goodwill-claim',
        status: 'ACTIVE',
      });
      prisma.operatorAction.create.mockResolvedValue({});

      const result = await service.resolve('admin-1', 'dispute-1', {
        resolution: 'GOODWILL_CREDIT',
        reason: 'Customer satisfaction',
      });

      expect(result.status).toBe('RESOLVED');
      expect(result.newClaimId).toBe('goodwill-claim');
    });

    it('should throw when dispute already resolved', async () => {
      prisma.dispute.findUnique.mockResolvedValue({
        ...mockDispute,
        status: 'RESOLVED',
      });

      await expect(
        service.resolve('admin-1', 'dispute-1', {
          resolution: 'DENY',
          reason: 'x',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException', async () => {
      prisma.dispute.findUnique.mockResolvedValue(null);

      await expect(
        service.resolve('admin-1', 'nope', {
          resolution: 'DENY',
          reason: 'x',
        }),
      ).rejects.toThrow(NotFoundException);
    });
  });
});
