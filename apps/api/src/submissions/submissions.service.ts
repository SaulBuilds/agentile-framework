import {
  Injectable,
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { CreateSubmissionDto } from './dto/create-submission.dto';
import { SubmitItemDto } from './dto/submit-item.dto';
import { ScreeningService } from './screening.service';

@Injectable()
export class SubmissionsService {
  constructor(
    private readonly prisma: PrismaService,
    private readonly screening: ScreeningService,
  ) {}

  async create(userId: string, dto: CreateSubmissionDto) {
    const submission = await this.prisma.itemSubmission.create({
      data: {
        userId,
        category: dto.category,
        estimatedBand: dto.estimatedBand,
        conditionDescription: dto.conditionDescription,
        status: 'DRAFT',
      },
    });

    return { submissionId: submission.id, status: submission.status };
  }

  async findOne(userId: string, submissionId: string) {
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: submissionId },
      include: {
        media: { select: { id: true, mediaType: true, s3Key: true, uploadedAt: true } },
        qrLabel: true,
      },
    });

    if (!submission) {
      throw new NotFoundException('Submission not found');
    }

    if (submission.userId !== userId) {
      throw new ForbiddenException('Not your submission');
    }

    return submission;
  }

  async findAll(userId: string, query: { status?: string; page?: number; limit?: number }) {
    const page = query.page ?? 1;
    const limit = query.limit ?? 20;
    const skip = (page - 1) * limit;

    const where: Record<string, unknown> = { userId };
    if (query.status) {
      where.status = query.status;
    }

    const [submissions, total] = await Promise.all([
      this.prisma.itemSubmission.findMany({
        where,
        skip,
        take: limit,
        orderBy: { createdAt: 'desc' },
        select: {
          id: true,
          category: true,
          estimatedBand: true,
          status: true,
          createdAt: true,
          updatedAt: true,
        },
      }),
      this.prisma.itemSubmission.count({ where }),
    ]);

    return {
      data: submissions,
      meta: {
        total,
        page,
        limit,
        pages: Math.ceil(total / limit),
      },
    };
  }

  async submit(userId: string, submissionId: string, dto: SubmitItemDto) {
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: submissionId },
      include: { media: true },
    });

    if (!submission) {
      throw new NotFoundException('Submission not found');
    }

    if (submission.userId !== userId) {
      throw new ForbiddenException('Not your submission');
    }

    if (submission.status !== 'DRAFT') {
      throw new BadRequestException('Submission already submitted');
    }

    // Validate declarations
    if (
      !dto.declareNotStolen ||
      !dto.declareNotRecalled ||
      !dto.declareNoHazardous ||
      !dto.declareMeetsPackaging ||
      !dto.declareRightToContribute
    ) {
      throw new BadRequestException('All declarations must be accepted');
    }

    // Validate media count (min 2)
    if (submission.media.length < 2) {
      throw new BadRequestException('At least 2 photos required');
    }

    // Run restricted item screening
    const screeningResult = await this.screening.screen(submission.category);

    if (screeningResult.blocked) {
      await this.prisma.itemSubmission.update({
        where: { id: submissionId },
        data: { status: 'REJECTED' },
      });

      return {
        status: 'blocked',
        blockReason: screeningResult.reason,
      };
    }

    const newStatus = screeningResult.requiresReview ? 'QUARANTINED' : 'SUBMITTED';

    await this.prisma.itemSubmission.update({
      where: { id: submissionId },
      data: { status: newStatus },
    });

    return {
      status: newStatus.toLowerCase(),
      reviewRequired: screeningResult.requiresReview,
      reviewReason: screeningResult.reason,
    };
  }
}
