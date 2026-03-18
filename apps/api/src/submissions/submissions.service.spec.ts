import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { SubmissionsService } from './submissions.service';
import { ScreeningService } from './screening.service';
import { PrismaService } from '../prisma/prisma.service';

describe('SubmissionsService', () => {
  let service: SubmissionsService;
  let prisma: {
    itemSubmission: {
      create: jest.Mock;
      findUnique: jest.Mock;
      findMany: jest.Mock;
      count: jest.Mock;
      update: jest.Mock;
    };
  };
  let screening: { screen: jest.Mock };

  beforeEach(async () => {
    prisma = {
      itemSubmission: {
        create: jest.fn(),
        findUnique: jest.fn(),
        findMany: jest.fn(),
        count: jest.fn(),
        update: jest.fn(),
      },
    };
    screening = { screen: jest.fn() };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        SubmissionsService,
        { provide: PrismaService, useValue: prisma },
        { provide: ScreeningService, useValue: screening },
      ],
    }).compile();

    service = module.get<SubmissionsService>(SubmissionsService);
  });

  describe('create', () => {
    it('should create a draft submission', async () => {
      prisma.itemSubmission.create.mockResolvedValue({
        id: 'sub-1',
        status: 'DRAFT',
      });

      const result = await service.create('user-1', {
        category: 'power tools',
        estimatedBand: 2,
        conditionDescription: 'Lightly used DeWalt drill',
      });

      expect(result).toEqual({ submissionId: 'sub-1', status: 'DRAFT' });
      expect(prisma.itemSubmission.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          userId: 'user-1',
          category: 'power tools',
          estimatedBand: 2,
          status: 'DRAFT',
        }),
      });
    });
  });

  describe('findOne', () => {
    it('should return submission for owner', async () => {
      const sub = { id: 'sub-1', userId: 'user-1', media: [], qrLabel: null };
      prisma.itemSubmission.findUnique.mockResolvedValue(sub);

      const result = await service.findOne('user-1', 'sub-1');
      expect(result).toEqual(sub);
    });

    it('should throw NotFoundException for unknown submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue(null);

      await expect(service.findOne('user-1', 'nope')).rejects.toThrow(
        NotFoundException,
      );
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'other-user',
      });

      await expect(service.findOne('user-1', 'sub-1')).rejects.toThrow(
        ForbiddenException,
      );
    });
  });

  describe('findAll', () => {
    it('should return paginated submissions', async () => {
      prisma.itemSubmission.findMany.mockResolvedValue([
        { id: 'sub-1', category: 'tools', status: 'DRAFT' },
      ]);
      prisma.itemSubmission.count.mockResolvedValue(1);

      const result = await service.findAll('user-1', { page: 1, limit: 20 });

      expect(result.data).toHaveLength(1);
      expect(result.meta).toEqual({
        total: 1,
        page: 1,
        limit: 20,
        pages: 1,
      });
    });

    it('should filter by status', async () => {
      prisma.itemSubmission.findMany.mockResolvedValue([]);
      prisma.itemSubmission.count.mockResolvedValue(0);

      await service.findAll('user-1', { status: 'SUBMITTED' });

      expect(prisma.itemSubmission.findMany).toHaveBeenCalledWith(
        expect.objectContaining({
          where: { userId: 'user-1', status: 'SUBMITTED' },
        }),
      );
    });
  });

  describe('submit', () => {
    const validDeclarations = {
      declareNotStolen: true,
      declareNotRecalled: true,
      declareNoHazardous: true,
      declareMeetsPackaging: true,
      declareRightToContribute: true,
    };

    it('should submit with valid declarations and passing screening', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        category: 'power tools',
        media: [{ id: 'm1' }, { id: 'm2' }],
      });
      screening.screen.mockResolvedValue({ blocked: false, requiresReview: false });
      prisma.itemSubmission.update.mockResolvedValue({});

      const result = await service.submit('user-1', 'sub-1', validDeclarations);

      expect(result.status).toBe('submitted');
      expect(prisma.itemSubmission.update).toHaveBeenCalledWith({
        where: { id: 'sub-1' },
        data: { status: 'SUBMITTED' },
      });
    });

    it('should block submission for excluded category', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        category: 'firearms',
        media: [{ id: 'm1' }, { id: 'm2' }],
      });
      screening.screen.mockResolvedValue({
        blocked: true,
        requiresReview: false,
        reason: 'Firearms not allowed',
      });
      prisma.itemSubmission.update.mockResolvedValue({});

      const result = await service.submit('user-1', 'sub-1', validDeclarations);

      expect(result.status).toBe('blocked');
      expect(result.blockReason).toBe('Firearms not allowed');
    });

    it('should quarantine items requiring manual review', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        category: 'ambiguous item',
        media: [{ id: 'm1' }, { id: 'm2' }],
      });
      screening.screen.mockResolvedValue({
        blocked: false,
        requiresReview: true,
        reason: 'Ambiguous category requires operator review',
      });
      prisma.itemSubmission.update.mockResolvedValue({});

      const result = await service.submit('user-1', 'sub-1', validDeclarations);

      expect(result.status).toBe('quarantined');
      expect(result.reviewRequired).toBe(true);
    });

    it('should reject when declarations are incomplete', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: [{ id: 'm1' }, { id: 'm2' }],
      });

      await expect(
        service.submit('user-1', 'sub-1', {
          ...validDeclarations,
          declareNotStolen: false,
        }),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject when fewer than 2 photos', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: [{ id: 'm1' }], // only 1 photo
      });

      await expect(
        service.submit('user-1', 'sub-1', validDeclarations),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject already-submitted items', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'SUBMITTED',
        media: [],
      });

      await expect(
        service.submit('user-1', 'sub-1', validDeclarations),
      ).rejects.toThrow(BadRequestException);
    });

    it('should throw NotFoundException for unknown submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue(null);

      await expect(
        service.submit('user-1', 'nope', validDeclarations),
      ).rejects.toThrow(NotFoundException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'other-user',
        status: 'DRAFT',
      });

      await expect(
        service.submit('user-1', 'sub-1', validDeclarations),
      ).rejects.toThrow(ForbiddenException);
    });
  });
});
