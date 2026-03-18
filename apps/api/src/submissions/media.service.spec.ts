import { Test, TestingModule } from '@nestjs/testing';
import {
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { MediaService } from './media.service';
import { PrismaService } from '../prisma/prisma.service';

// Mock fs to avoid actual file writes
jest.mock('fs', () => ({
  existsSync: jest.fn().mockReturnValue(true),
  mkdirSync: jest.fn(),
  writeFileSync: jest.fn(),
}));

describe('MediaService', () => {
  let service: MediaService;
  let prisma: {
    itemSubmission: { findUnique: jest.Mock };
    itemMedia: { create: jest.Mock };
  };

  beforeEach(async () => {
    prisma = {
      itemSubmission: { findUnique: jest.fn() },
      itemMedia: { create: jest.fn() },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        MediaService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<MediaService>(MediaService);
  });

  const mockFile = (name = 'photo.jpg', size = 1024) => ({
    originalname: name,
    mimetype: 'image/jpeg',
    size,
    buffer: Buffer.alloc(size),
  });

  describe('uploadMedia', () => {
    it('should upload files for a draft submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: [],
      });
      prisma.itemMedia.create.mockResolvedValue({ id: 'media-1' });

      const result = await service.uploadMedia('user-1', 'sub-1', [mockFile()]);
      expect(result.mediaIds).toHaveLength(1);
    });

    it('should throw NotFoundException for unknown submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue(null);

      await expect(
        service.uploadMedia('user-1', 'nope', [mockFile()]),
      ).rejects.toThrow(NotFoundException);
    });

    it('should throw ForbiddenException for non-owner', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'other-user',
        status: 'DRAFT',
        media: [],
      });

      await expect(
        service.uploadMedia('user-1', 'sub-1', [mockFile()]),
      ).rejects.toThrow(ForbiddenException);
    });

    it('should reject uploads after submission', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'SUBMITTED',
        media: [],
      });

      await expect(
        service.uploadMedia('user-1', 'sub-1', [mockFile()]),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject when exceeding max media count', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: Array(9).fill({ id: 'existing' }),
      });

      await expect(
        service.uploadMedia('user-1', 'sub-1', [mockFile(), mockFile()]),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject invalid file types', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: [],
      });

      const badFile = {
        originalname: 'script.js',
        mimetype: 'application/javascript',
        size: 100,
        buffer: Buffer.alloc(100),
      };

      await expect(
        service.uploadMedia('user-1', 'sub-1', [badFile]),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject files exceeding size limit', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: [],
      });

      const bigFile = mockFile('big.jpg', 11 * 1024 * 1024); // 11MB

      await expect(
        service.uploadMedia('user-1', 'sub-1', [bigFile]),
      ).rejects.toThrow(BadRequestException);
    });

    it('should reject empty file array', async () => {
      prisma.itemSubmission.findUnique.mockResolvedValue({
        id: 'sub-1',
        userId: 'user-1',
        status: 'DRAFT',
        media: [],
      });

      await expect(
        service.uploadMedia('user-1', 'sub-1', []),
      ).rejects.toThrow(BadRequestException);
    });
  });
});
