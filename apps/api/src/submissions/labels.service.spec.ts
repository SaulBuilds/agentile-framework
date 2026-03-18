import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { LabelsService } from './labels.service';
import { PrismaService } from '../prisma/prisma.service';

describe('LabelsService', () => {
  let service: LabelsService;
  let prisma: {
    itemSubmission: { findUnique: jest.Mock };
    qrLabel: { create: jest.Mock };
  };

  beforeEach(async () => {
    prisma = {
      itemSubmission: { findUnique: jest.fn() },
      qrLabel: { create: jest.fn() },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        LabelsService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<LabelsService>(LabelsService);
  });

  describe('getLabel', () => {
    it('should generate label for submitted item', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'SUBMITTED',
        category: 'power tools',
        estimatedBand: 2,
        qrLabel: null,
      });
      prisma.qrLabel.create.mockResolvedValue({});

      const result = await service.getLabel('user-1', 'sub-1');

      expect(result.qrData).toBeDefined();
      expect(result.humanReadableCode).toMatch(/^GBP-[A-F0-9]{4}-[A-F0-9]{4}$/);
      expect(result.labelPdfUrl).toContain('sub-1');
      expect(result.packingInstructions).toBeDefined();
      expect(result.packingInstructions.steps.length).toBeGreaterThan(0);
    });

    it('should return existing label if already generated', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'SUBMITTED',
        category: 'hand tools',
        qrLabel: {
          qrData: '{"existing":"data"}',
          humanReadableCode: 'GBP-AAAA-BBBB',
        },
      });

      const result = await service.getLabel('user-1', 'sub-1');

      expect(result.qrData).toBe('{"existing":"data"}');
      expect(result.humanReadableCode).toBe('GBP-AAAA-BBBB');
      expect(prisma.qrLabel.create).not.toHaveBeenCalled();
    });

    it('should throw BadRequestException for draft submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        qrLabel: null,
      });

      await expect(service.getLabel('user-1', 'sub-1')).rejects.toThrow(
        BadRequestException,
      );
    });

    it('should throw NotFoundException for unknown submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue(null);

      await expect(service.getLabel('user-1', 'nope')).rejects.toThrow(
        NotFoundException,
      );
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'other-user',
        status: 'SUBMITTED',
      });

      await expect(service.getLabel('user-1', 'sub-1')).rejects.toThrow(
        ForbiddenException,
      );
    });
  });

  describe('getPackingInstructions', () => {
    it('should return power tools instructions', () => {
      const result = service.getPackingInstructions('Power Tools');
      expect(result.category).toBe('Power Tools');
      expect(result.steps).toContain('Remove battery if detachable');
    });

    it('should return consumer electronics instructions', () => {
      const result = service.getPackingInstructions('Consumer Electronics');
      expect(result.category).toBe('Consumer Electronics');
      expect(result.warnings).toContain(
        'Do NOT ship devices with swollen batteries',
      );
    });

    it('should return gaming console instructions', () => {
      const result = service.getPackingInstructions('Gaming Consoles');
      expect(result.category).toBe('Gaming Consoles & Accessories');
    });

    it('should return small appliances instructions', () => {
      const result = service.getPackingInstructions('small appliances');
      expect(result.category).toBe('Small Appliances');
    });

    it('should return default instructions for unknown category', () => {
      const result = service.getPackingInstructions('unknown widgets');
      expect(result.category).toBe('General Item');
      expect(result.steps.length).toBeGreaterThan(0);
    });
  });

  describe('getLabelPdf', () => {
    it('should return PDF buffer for labeled submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        category: 'tools',
        estimatedBand: 2,
        qrLabel: {
          humanReadableCode: 'GBP-1234-ABCD',
          qrData: '{"test":"data"}',
        },
      });

      const buffer = await service.getLabelPdf('user-1', 'sub-1');
      expect(Buffer.isBuffer(buffer)).toBe(true);
      expect(buffer.length).toBeGreaterThan(0);
      // Real PDF starts with %PDF magic bytes
      expect(buffer.slice(0, 5).toString('ascii')).toBe('%PDF-');
    });

    it('should throw BadRequestException when label not generated', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        qrLabel: null,
      });

      await expect(service.getLabelPdf('user-1', 'sub-1')).rejects.toThrow(
        BadRequestException,
      );
    });
  });
});
