import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  BadRequestException,
} from '@nestjs/common';
import { WarehouseService } from './warehouse.service';
import { PrismaService } from '../prisma/prisma.service';

describe('WarehouseService', () => {
  let service: WarehouseService;
  let prisma: any;

  beforeEach(async () => {
    prisma = {
      qrLabel: { findFirst: jest.fn() },
      itemSubmission: {
        findUnique: jest.fn(),
        findMany: jest.fn(),
        count: jest.fn(),
        update: jest.fn(),
      },
      itemReceipt: { create: jest.fn() },
      inventoryItem: {
        findUnique: jest.fn(),
        create: jest.fn(),
        update: jest.fn(),
      },
      inventoryStatusEvent: { create: jest.fn() },
      claim: { create: jest.fn() },
      pool: { findUnique: jest.fn(), findFirst: jest.fn() },
      warehouse: { findUnique: jest.fn(), findFirst: jest.fn() },
      operatorAction: { create: jest.fn() },
      $transaction: jest.fn((fn) => fn(prisma)),
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        WarehouseService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<WarehouseService>(WarehouseService);
  });

  describe('scanIntake', () => {
    it('should find submission by QR label code', async () => {
      const mockSubmission = {
        id: 'sub-1',
        category: 'tools',
        status: 'SUBMITTED',
        media: [],
        user: { id: 'user-1', displayName: 'Test' },
      };
      prisma.qrLabel.findFirst.mockResolvedValue({
        humanReadableCode: 'GBP-AAAA-BBBB',
        qrData: '{"test":"data"}',
        submission: mockSubmission,
      });

      const result = await service.scanIntake('GBP-AAAA-BBBB');

      expect(result.submission).toEqual(mockSubmission);
      expect(result.qrLabel!.humanReadableCode).toBe('GBP-AAAA-BBBB');
    });

    it('should find submission by ID when QR code not found', async () => {
      prisma.qrLabel.findFirst.mockResolvedValue(null);
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        category: 'tools',
        status: 'SUBMITTED',
        media: [],
        qrLabel: null,
        user: { id: 'user-1', displayName: 'Test' },
      });

      const result = await service.scanIntake('sub-1');

      expect(result.submission.id).toBe('sub-1');
      expect(result.qrLabel).toBeNull();
    });

    it('should throw NotFoundException when no submission found', async () => {
      prisma.qrLabel.findFirst.mockResolvedValue(null);
      prisma.itemSubmission.findUnique.mockResolvedValue(null);

      await expect(service.scanIntake('invalid')).rejects.toThrow(
        NotFoundException,
      );
    });
  });

  describe('getIntakeQueue', () => {
    it('should return paginated queue of pending submissions', async () => {
      prisma.itemSubmission.findMany.mockResolvedValue([
        { id: 'sub-1', status: 'SUBMITTED' },
        { id: 'sub-2', status: 'RECEIVED' },
      ]);
      prisma.itemSubmission.count.mockResolvedValue(2);

      const result = await service.getIntakeQueue({ page: 1, limit: 20 });

      expect(result.data).toHaveLength(2);
      expect(result.meta).toEqual({
        total: 2,
        page: 1,
        limit: 20,
        pages: 1,
      });
    });

    it('should use default pagination when not specified', async () => {
      prisma.itemSubmission.findMany.mockResolvedValue([]);
      prisma.itemSubmission.count.mockResolvedValue(0);

      const result = await service.getIntakeQueue();

      expect(result.meta.page).toBe(1);
      expect(result.meta.limit).toBe(20);
    });
  });

  describe('gradeSubmission', () => {
    const baseSubmission = {
      id: 'sub-1',
      userId: 'user-1',
      category: 'power tools',
      targetPoolId: null,
      receipt: null,
    };

    describe('accept', () => {
      it('should accept a submitted item and create receipt, inventory, claim', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
        });
        prisma.pool.findFirst.mockResolvedValue({ id: 'pool-1', region: 'PDX' });
        prisma.warehouse.findFirst.mockResolvedValue({ id: 'wh-1' });
        prisma.itemReceipt.create.mockResolvedValue({ id: 'receipt-1' });
        prisma.inventoryItem.create.mockResolvedValue({ id: 'inv-1' });
        prisma.claim.create.mockResolvedValue({ id: 'claim-1' });
        prisma.inventoryStatusEvent.create.mockResolvedValue({});
        prisma.itemSubmission.update.mockResolvedValue({});
        prisma.operatorAction.create.mockResolvedValue({});

        const result = await service.gradeSubmission('op-1', 'sub-1', {
          decision: 'ACCEPTED',
          finalBand: 2,
          conditionGrade: 'good',
          binLocation: 'PDX-BLU-B2-R01-S05',
          gradingNotes: 'Clean item',
        });

        expect(result.status).toBe('accepted');
        expect((result as any).receiptId).toBe('receipt-1');
        expect((result as any).inventoryId).toBe('inv-1');
        expect((result as any).claimId).toBe('claim-1');
        expect(prisma.itemReceipt.create).toHaveBeenCalledWith({
          data: expect.objectContaining({
            submissionId: 'sub-1',
            finalBand: 2,
            conditionGrade: 'good',
            graderId: 'op-1',
          }),
        });
      });

      it('should use targetPoolId if present', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
          targetPoolId: 'pool-existing',
        });
        prisma.pool.findUnique.mockResolvedValue({ id: 'pool-existing', region: 'PDX' });
        prisma.warehouse.findFirst.mockResolvedValue({ id: 'wh-1' });
        prisma.itemReceipt.create.mockResolvedValue({ id: 'r-1' });
        prisma.inventoryItem.create.mockResolvedValue({ id: 'i-1' });
        prisma.claim.create.mockResolvedValue({ id: 'c-1' });
        prisma.inventoryStatusEvent.create.mockResolvedValue({});
        prisma.itemSubmission.update.mockResolvedValue({});
        prisma.operatorAction.create.mockResolvedValue({});

        const result = await service.gradeSubmission('op-1', 'sub-1', {
          decision: 'ACCEPTED',
          finalBand: 2,
          conditionGrade: 'good',
        });

        expect(result.status).toBe('accepted');
        expect(prisma.pool.findUnique).toHaveBeenCalledWith({
          where: { id: 'pool-existing' },
        });
      });

      it('should throw when no pool found', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
        });
        prisma.pool.findFirst.mockResolvedValue(null);

        await expect(
          service.gradeSubmission('op-1', 'sub-1', {
            decision: 'ACCEPTED',
            finalBand: 2,
            conditionGrade: 'good',
          }),
        ).rejects.toThrow(BadRequestException);
      });
    });

    describe('reject', () => {
      it('should reject a submission with reason', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
        });
        prisma.itemSubmission.update.mockResolvedValue({});
        prisma.operatorAction.create.mockResolvedValue({});

        const result = await service.gradeSubmission('op-1', 'sub-1', {
          decision: 'REJECTED',
          finalBand: 2,
          conditionGrade: 'poor',
          reason: 'Item does not match description',
        });

        expect(result.status).toBe('rejected');
        expect((result as any).reason).toBe('Item does not match description');
        expect(prisma.itemSubmission.update).toHaveBeenCalledWith({
          where: { id: 'sub-1' },
          data: { status: 'REJECTED' },
        });
      });

      it('should require reason for rejection', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
        });

        await expect(
          service.gradeSubmission('op-1', 'sub-1', {
            decision: 'REJECTED',
            finalBand: 2,
            conditionGrade: 'poor',
          }),
        ).rejects.toThrow(BadRequestException);
      });
    });

    describe('quarantine', () => {
      it('should quarantine a submission with reason', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
        });
        prisma.itemSubmission.update.mockResolvedValue({});
        prisma.operatorAction.create.mockResolvedValue({});

        const result = await service.gradeSubmission('op-1', 'sub-1', {
          decision: 'QUARANTINED',
          finalBand: 2,
          conditionGrade: 'suspect',
          reason: 'Possible counterfeit',
        });

        expect(result.status).toBe('quarantined');
        expect(prisma.itemSubmission.update).toHaveBeenCalledWith({
          where: { id: 'sub-1' },
          data: { status: 'QUARANTINED' },
        });
      });

      it('should require reason for quarantine', async () => {
        prisma.itemSubmission.findUnique.mockResolvedValue({
          ...baseSubmission,
          status: 'SUBMITTED',
        });

        await expect(
          service.gradeSubmission('op-1', 'sub-1', {
            decision: 'QUARANTINED',
            finalBand: 2,
            conditionGrade: 'suspect',
          }),
        ).rejects.toThrow(BadRequestException);
      });
    });

    it('should throw NotFoundException for unknown submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue(null);

      await expect(
        service.gradeSubmission('op-1', 'nope', {
          decision: 'ACCEPTED',
          finalBand: 2,
          conditionGrade: 'good',
        }),
      ).rejects.toThrow(NotFoundException);
    });

    it('should reject grading of already graded submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        ...baseSubmission,
        status: 'SUBMITTED',
        receipt: { id: 'existing-receipt' },
      });

      await expect(
        service.gradeSubmission('op-1', 'sub-1', {
          decision: 'ACCEPTED',
          finalBand: 2,
          conditionGrade: 'good',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject grading of non-submitted status', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        ...baseSubmission,
        status: 'DRAFT',
        receipt: null,
      });

      await expect(
        service.gradeSubmission('op-1', 'sub-1', {
          decision: 'ACCEPTED',
          finalBand: 2,
          conditionGrade: 'good',
        }),
      ).rejects.toThrow(BadRequestException);
    });
  });

  describe('quarantineInventory', () => {
    it('should quarantine an available inventory item', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'AVAILABLE',
      });
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.operatorAction.create.mockResolvedValue({});

      const result = await service.quarantineInventory('op-1', 'inv-1', {
        reason: 'Safety concern',
      });

      expect(result.status).toBe('quarantined');
      expect(result.reason).toBe('Safety concern');
    });

    it('should throw when item already quarantined', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'QUARANTINED',
      });

      await expect(
        service.quarantineInventory('op-1', 'inv-1', {
          reason: 'Safety concern',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw when item is delivered', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'DELIVERED',
      });

      await expect(
        service.quarantineInventory('op-1', 'inv-1', {
          reason: 'Too late',
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown item', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue(null);

      await expect(
        service.quarantineInventory('op-1', 'nope', { reason: 'test' }),
      ).rejects.toThrow(NotFoundException);
    });
  });

  describe('resolveQuarantine', () => {
    it('should release quarantined item back to available', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'QUARANTINED',
        binLocation: 'old-bin',
        receipt: null,
      });
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.operatorAction.create.mockResolvedValue({});

      const result = await service.resolveQuarantine('op-1', 'inv-1', {
        resolution: 'RELEASE',
        binLocation: 'new-bin',
      });

      expect(result.status).toBe('released');
      expect(prisma.inventoryItem.update).toHaveBeenCalledWith({
        where: { id: 'inv-1' },
        data: { status: 'AVAILABLE', binLocation: 'new-bin' },
      });
    });

    it('should reject quarantined item', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'QUARANTINED',
        receipt: { submissionId: 'sub-1' },
      });
      prisma.inventoryItem.update.mockResolvedValue({});
      prisma.inventoryStatusEvent.create.mockResolvedValue({});
      prisma.itemSubmission.update.mockResolvedValue({});
      prisma.operatorAction.create.mockResolvedValue({});

      const result = await service.resolveQuarantine('op-1', 'inv-1', {
        resolution: 'REJECT',
        notes: 'Confirmed counterfeit',
      });

      expect(result.status).toBe('rejected');
      expect(prisma.itemSubmission.update).toHaveBeenCalledWith({
        where: { id: 'sub-1' },
        data: { status: 'REJECTED' },
      });
    });

    it('should throw when item is not quarantined', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue({
        id: 'inv-1',
        status: 'AVAILABLE',
      });

      await expect(
        service.resolveQuarantine('op-1', 'inv-1', { resolution: 'RELEASE' }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown item', async () => {
      prisma.inventoryItem.findUnique.mockResolvedValue(null);

      await expect(
        service.resolveQuarantine('op-1', 'nope', { resolution: 'RELEASE' }),
      ).rejects.toThrow(NotFoundException);
    });
  });
});
