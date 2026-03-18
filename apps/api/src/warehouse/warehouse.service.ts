import {
  Injectable,
  NotFoundException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { GradeSubmissionDto } from './dto/grade-submission.dto';
import { QuarantineDto, ResolveQuarantineDto } from './dto/quarantine.dto';

@Injectable()
export class WarehouseService {
  constructor(private readonly prisma: PrismaService) {}

  async scanIntake(code: string) {
    // Try finding by QR label human-readable code first
    const label = await this.prisma.qrLabel.findFirst({
      where: { humanReadableCode: code },
      include: {
        submission: {
          include: { media: true, user: { select: { id: true, displayName: true, email: true } } },
        },
      },
    });

    if (label) {
      return {
        submission: label.submission,
        qrLabel: {
          humanReadableCode: label.humanReadableCode,
          qrData: label.qrData,
        },
      };
    }

    // Try finding by submission ID directly
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: code },
      include: {
        media: true,
        qrLabel: true,
        user: { select: { id: true, displayName: true, email: true } },
      },
    });

    if (!submission) {
      throw new NotFoundException('No submission found for this code');
    }

    return {
      submission,
      qrLabel: submission.qrLabel
        ? {
            humanReadableCode: submission.qrLabel.humanReadableCode,
            qrData: submission.qrLabel.qrData,
          }
        : null,
    };
  }

  async getIntakeQueue(filters?: { warehouseId?: string; page?: number; limit?: number }) {
    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const where = {
      status: { in: ['SUBMITTED', 'RECEIVED'] as any[] },
    };

    const [data, total] = await Promise.all([
      this.prisma.itemSubmission.findMany({
        where,
        include: {
          media: true,
          qrLabel: true,
          user: { select: { id: true, displayName: true } },
        },
        orderBy: { createdAt: 'asc' as const },
        skip,
        take: limit,
      }),
      this.prisma.itemSubmission.count({ where }),
    ]);

    return {
      data,
      meta: {
        total,
        page,
        limit,
        pages: Math.ceil(total / limit),
      },
    };
  }

  async gradeSubmission(
    operatorId: string,
    submissionId: string,
    dto: GradeSubmissionDto,
  ) {
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: submissionId },
      include: { receipt: true },
    });

    if (!submission) {
      throw new NotFoundException('Submission not found');
    }

    if (!['SUBMITTED', 'RECEIVED'].includes(submission.status)) {
      throw new BadRequestException(
        `Cannot grade submission in ${submission.status} status`,
      );
    }

    if (submission.receipt) {
      throw new BadRequestException('Submission already graded');
    }

    if (dto.decision === 'ACCEPTED') {
      return this.acceptSubmission(operatorId, submission, dto);
    }

    if (dto.decision === 'REJECTED') {
      return this.rejectSubmission(operatorId, submission, dto);
    }

    return this.quarantineSubmission(operatorId, submission, dto);
  }

  private async acceptSubmission(
    operatorId: string,
    submission: any,
    dto: GradeSubmissionDto,
  ) {
    // Find or determine pool
    const pool = submission.targetPoolId
      ? await this.prisma.pool.findUnique({ where: { id: submission.targetPoolId } })
      : await this.prisma.pool.findFirst({
          where: { band: dto.finalBand, status: 'ACTIVE' },
        });

    if (!pool) {
      throw new BadRequestException('No active pool found for the given band');
    }

    // Find warehouse
    const warehouse = dto.warehouseId
      ? await this.prisma.warehouse.findUnique({ where: { id: dto.warehouseId } })
      : await this.prisma.warehouse.findFirst({
          where: { region: pool.region, status: 'ACTIVE_WH' },
        });

    if (!warehouse) {
      throw new BadRequestException('No active warehouse found');
    }

    // Create receipt, inventory item, claim, and update submission in a transaction
    const result = await this.prisma.$transaction(async (tx) => {
      // Create ItemReceipt
      const receipt = await tx.itemReceipt.create({
        data: {
          submissionId: submission.id,
          poolId: pool.id,
          finalBand: dto.finalBand,
          conditionGrade: dto.conditionGrade,
          graderId: operatorId,
          gradingNotes: dto.gradingNotes,
        },
      });

      // Create InventoryItem
      const inventoryItem = await tx.inventoryItem.create({
        data: {
          receiptId: receipt.id,
          warehouseId: warehouse.id,
          poolId: pool.id,
          binLocation: dto.binLocation,
          status: 'AVAILABLE',
        },
      });

      // Record initial inventory status event
      await tx.inventoryStatusEvent.create({
        data: {
          inventoryId: inventoryItem.id,
          previousStatus: 'AVAILABLE',
          newStatus: 'AVAILABLE',
          actorId: operatorId,
        },
      });

      // Create active claim for the contributor
      const claim = await tx.claim.create({
        data: {
          userId: submission.userId,
          poolId: pool.id,
          receiptId: receipt.id,
          status: 'ACTIVE',
          activatedAt: new Date(),
        },
      });

      // Update submission status
      await tx.itemSubmission.update({
        where: { id: submission.id },
        data: { status: 'ACCEPTED' },
      });

      // Log operator action
      await tx.operatorAction.create({
        data: {
          operatorId,
          actionType: 'GRADE_ACCEPT',
          targetType: 'SUBMISSION',
          targetId: submission.id,
          reason: dto.gradingNotes || 'Item accepted',
          metadata: {
            finalBand: dto.finalBand,
            conditionGrade: dto.conditionGrade,
            binLocation: dto.binLocation,
          },
        },
      });

      return { receipt, inventoryItem, claim };
    });

    return {
      status: 'accepted',
      receiptId: result.receipt.id,
      inventoryId: result.inventoryItem.id,
      claimId: result.claim.id,
      binLocation: dto.binLocation,
    };
  }

  private async rejectSubmission(
    operatorId: string,
    submission: any,
    dto: GradeSubmissionDto,
  ) {
    if (!dto.reason) {
      throw new BadRequestException('Rejection reason is required');
    }

    await this.prisma.$transaction(async (tx) => {
      await tx.itemSubmission.update({
        where: { id: submission.id },
        data: { status: 'REJECTED' },
      });

      await tx.operatorAction.create({
        data: {
          operatorId,
          actionType: 'GRADE_REJECT',
          targetType: 'SUBMISSION',
          targetId: submission.id,
          reason: dto.reason!,
          metadata: {
            conditionGrade: dto.conditionGrade,
            gradingNotes: dto.gradingNotes,
          },
        },
      });
    });

    return {
      status: 'rejected',
      reason: dto.reason,
    };
  }

  private async quarantineSubmission(
    operatorId: string,
    submission: any,
    dto: GradeSubmissionDto,
  ) {
    if (!dto.reason) {
      throw new BadRequestException('Quarantine reason is required');
    }

    await this.prisma.$transaction(async (tx) => {
      await tx.itemSubmission.update({
        where: { id: submission.id },
        data: { status: 'QUARANTINED' },
      });

      await tx.operatorAction.create({
        data: {
          operatorId,
          actionType: 'GRADE_QUARANTINE',
          targetType: 'SUBMISSION',
          targetId: submission.id,
          reason: dto.reason!,
          metadata: {
            conditionGrade: dto.conditionGrade,
            gradingNotes: dto.gradingNotes,
          },
        },
      });
    });

    return {
      status: 'quarantined',
      reason: dto.reason,
    };
  }

  async quarantineInventory(
    operatorId: string,
    inventoryId: string,
    dto: QuarantineDto,
  ) {
    const item = await this.prisma.inventoryItem.findUnique({
      where: { id: inventoryId },
    });

    if (!item) {
      throw new NotFoundException('Inventory item not found');
    }

    if (item.status === 'QUARANTINED') {
      throw new BadRequestException('Item is already quarantined');
    }

    if (!['AVAILABLE', 'RESERVED'].includes(item.status)) {
      throw new BadRequestException(
        `Cannot quarantine item in ${item.status} status`,
      );
    }

    await this.prisma.$transaction(async (tx) => {
      const previousStatus = item.status;

      await tx.inventoryItem.update({
        where: { id: inventoryId },
        data: { status: 'QUARANTINED' },
      });

      await tx.inventoryStatusEvent.create({
        data: {
          inventoryId,
          previousStatus,
          newStatus: 'QUARANTINED',
          actorId: operatorId,
        },
      });

      await tx.operatorAction.create({
        data: {
          operatorId,
          actionType: 'QUARANTINE',
          targetType: 'INVENTORY',
          targetId: inventoryId,
          reason: dto.reason,
        },
      });
    });

    return { status: 'quarantined', inventoryId, reason: dto.reason };
  }

  async resolveQuarantine(
    operatorId: string,
    inventoryId: string,
    dto: ResolveQuarantineDto,
  ) {
    const item = await this.prisma.inventoryItem.findUnique({
      where: { id: inventoryId },
      include: { receipt: true },
    });

    if (!item) {
      throw new NotFoundException('Inventory item not found');
    }

    if (item.status !== 'QUARANTINED') {
      throw new BadRequestException('Item is not quarantined');
    }

    if (dto.resolution === 'RELEASE') {
      await this.prisma.$transaction(async (tx) => {
        await tx.inventoryItem.update({
          where: { id: inventoryId },
          data: {
            status: 'AVAILABLE',
            binLocation: dto.binLocation || item.binLocation,
          },
        });

        await tx.inventoryStatusEvent.create({
          data: {
            inventoryId,
            previousStatus: 'QUARANTINED',
            newStatus: 'AVAILABLE',
            actorId: operatorId,
          },
        });

        await tx.operatorAction.create({
          data: {
            operatorId,
            actionType: 'QUARANTINE_RELEASE',
            targetType: 'INVENTORY',
            targetId: inventoryId,
            reason: dto.notes || 'Released from quarantine',
          },
        });
      });

      return { status: 'released', inventoryId };
    }

    // REJECT — mark inventory as returned and update submission
    await this.prisma.$transaction(async (tx) => {
      await tx.inventoryItem.update({
        where: { id: inventoryId },
        data: { status: 'RETURNED' },
      });

      await tx.inventoryStatusEvent.create({
        data: {
          inventoryId,
          previousStatus: 'QUARANTINED',
          newStatus: 'RETURNED',
          actorId: operatorId,
        },
      });

      // Update the submission status to REJECTED
      if (item.receipt) {
        await tx.itemSubmission.update({
          where: { id: item.receipt.submissionId },
          data: { status: 'REJECTED' },
        });
      }

      await tx.operatorAction.create({
        data: {
          operatorId,
          actionType: 'QUARANTINE_REJECT',
          targetType: 'INVENTORY',
          targetId: inventoryId,
          reason: dto.notes || 'Rejected from quarantine',
        },
      });
    });

    return { status: 'rejected', inventoryId };
  }
}
