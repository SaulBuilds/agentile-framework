import {
  Injectable,
  Logger,
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { NotificationsService } from '../notifications/notifications.service';
import { createHash, randomUUID } from 'crypto';
import { extname } from 'path';
import { writeFileSync, mkdirSync, existsSync } from 'fs';

const DEFAULT_SLA_HOURS = 72;

const ALLOWED_EVIDENCE_MIMES = [
  'image/jpeg',
  'image/png',
  'image/heic',
  'video/mp4',
  'application/pdf',
];

const MAX_EVIDENCE_SIZE = 10 * 1024 * 1024; // 10MB

@Injectable()
export class DisputesService {
  private readonly logger = new Logger(DisputesService.name);

  constructor(
    private readonly prisma: PrismaService,
    private readonly notificationsService: NotificationsService,
  ) {}

  // ── Create Dispute ──────────────────────────────────────────

  async create(userId: string, dto: { objectType: string; objectId: string; reason: string }) {
    // Validate the referenced object exists and user has standing
    await this.validateObjectOwnership(userId, dto.objectType, dto.objectId);

    // Check for duplicate open disputes
    const existing = await this.prisma.dispute.findFirst({
      where: {
        objectType: dto.objectType as any,
        objectId: dto.objectId,
        status: { in: ['OPEN', 'UNDER_REVIEW'] },
      },
    });

    if (existing) {
      throw new BadRequestException(
        'An active dispute already exists for this object',
      );
    }

    const slaDeadline = new Date(
      Date.now() + DEFAULT_SLA_HOURS * 60 * 60 * 1000,
    );

    const dispute = await this.prisma.dispute.create({
      data: {
        openedById: userId,
        objectType: dto.objectType as any,
        objectId: dto.objectId,
        reason: dto.reason,
        status: 'OPEN',
        slaDeadline,
      },
    });

    await this.notificationsService.create(userId, 'dispute_opened', {
      disputeId: dispute.id,
      objectType: dto.objectType,
      objectId: dto.objectId,
      slaDeadline: slaDeadline.toISOString(),
    });

    return { disputeId: dispute.id, slaDeadline };
  }

  // ── Submit Evidence ─────────────────────────────────────────

  async submitEvidence(
    userId: string,
    disputeId: string,
    dto: { evidenceType: string; content?: string },
    file?: { originalname: string; mimetype: string; size: number; buffer: Buffer },
    isAdmin = false,
  ) {
    const dispute = await this.prisma.dispute.findUnique({
      where: { id: disputeId },
    });

    if (!dispute) {
      throw new NotFoundException('Dispute not found');
    }

    if (!isAdmin && dispute.openedById !== userId) {
      throw new ForbiddenException('Not your dispute');
    }

    if (!['OPEN', 'UNDER_REVIEW'].includes(dispute.status)) {
      throw new BadRequestException(
        'Cannot submit evidence to a resolved dispute',
      );
    }

    let s3Key: string | undefined;
    let cidHash: string | undefined;

    if (dto.evidenceType === 'TEXT') {
      if (!dto.content) {
        throw new BadRequestException(
          'Text evidence requires content field',
        );
      }
      cidHash = createHash('sha256').update(dto.content).digest('hex');
    } else {
      if (!file) {
        throw new BadRequestException(
          `${dto.evidenceType} evidence requires a file upload`,
        );
      }

      if (!ALLOWED_EVIDENCE_MIMES.includes(file.mimetype)) {
        throw new BadRequestException(
          `Unsupported file type: ${file.mimetype}`,
        );
      }

      if (file.size > MAX_EVIDENCE_SIZE) {
        throw new BadRequestException(
          `File too large (max ${MAX_EVIDENCE_SIZE / 1024 / 1024}MB)`,
        );
      }

      const ext = extname(file.originalname);
      s3Key = `disputes/${disputeId}/${randomUUID()}${ext}`;
      cidHash = createHash('sha256').update(file.buffer).digest('hex');

      // Local storage for dev
      const dir = `uploads/disputes/${disputeId}`;
      if (!existsSync(dir)) {
        mkdirSync(dir, { recursive: true });
      }
      writeFileSync(`uploads/${s3Key}`, file.buffer);
    }

    const evidence = await this.prisma.disputeEvidence.create({
      data: {
        disputeId,
        submittedById: userId,
        evidenceType: dto.evidenceType as any,
        s3Key,
        content: dto.evidenceType === 'TEXT' ? dto.content : undefined,
        cidHash,
      },
    });

    return evidence;
  }

  // ── List Disputes ───────────────────────────────────────────

  async findAll(
    userId: string,
    filters?: { status?: string; page?: number; limit?: number },
  ) {
    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const where: any = { openedById: userId };
    if (filters?.status) where.status = filters.status;

    const [disputes, total] = await Promise.all([
      this.prisma.dispute.findMany({
        where,
        include: {
          _count: { select: { evidence: true } },
        },
        orderBy: { createdAt: 'desc' },
        skip,
        take: limit,
      }),
      this.prisma.dispute.count({ where }),
    ]);

    return {
      data: disputes.map((d) => ({
        id: d.id,
        objectType: d.objectType,
        objectId: d.objectId,
        reason: d.reason,
        status: d.status,
        resolution: d.resolution,
        slaDeadline: d.slaDeadline,
        resolvedAt: d.resolvedAt,
        createdAt: d.createdAt,
        evidenceCount: (d as any)._count?.evidence ?? 0,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  // ── Dispute Detail ──────────────────────────────────────────

  async findOne(userId: string, disputeId: string) {
    const dispute = await this.prisma.dispute.findUnique({
      where: { id: disputeId },
      include: {
        evidence: {
          orderBy: { submittedAt: 'asc' },
        },
        openedBy: {
          select: { id: true, displayName: true, email: true },
        },
        resolvedBy: {
          select: { id: true, displayName: true },
        },
      },
    });

    if (!dispute) {
      throw new NotFoundException('Dispute not found');
    }

    if (dispute.openedById !== userId) {
      throw new ForbiddenException('Not your dispute');
    }

    return {
      id: dispute.id,
      objectType: dispute.objectType,
      objectId: dispute.objectId,
      reason: dispute.reason,
      status: dispute.status,
      resolution: dispute.resolution,
      slaDeadline: dispute.slaDeadline,
      resolvedAt: dispute.resolvedAt,
      createdAt: dispute.createdAt,
      openedBy: dispute.openedBy,
      resolvedBy: dispute.resolvedBy,
      evidence: dispute.evidence,
    };
  }

  // ── Admin Queue ─────────────────────────────────────────────

  async getAdminQueue(filters?: { page?: number; limit?: number }) {
    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const where = {
      status: { in: ['OPEN', 'UNDER_REVIEW'] as any[] },
    };

    const [disputes, total] = await Promise.all([
      this.prisma.dispute.findMany({
        where,
        include: {
          openedBy: {
            select: { id: true, displayName: true, email: true },
          },
          _count: { select: { evidence: true } },
        },
        orderBy: { slaDeadline: 'asc' },
        skip,
        take: limit,
      }),
      this.prisma.dispute.count({ where }),
    ]);

    const now = Date.now();

    return {
      data: disputes.map((d) => ({
        id: d.id,
        objectType: d.objectType,
        objectId: d.objectId,
        reason: d.reason,
        status: d.status,
        slaDeadline: d.slaDeadline,
        slaRemainingMs: d.slaDeadline.getTime() - now,
        overdue: d.slaDeadline.getTime() < now,
        createdAt: d.createdAt,
        openedBy: d.openedBy,
        evidenceCount: (d as any)._count?.evidence ?? 0,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  // ── Resolve Dispute ─────────────────────────────────────────

  async resolve(
    adminId: string,
    disputeId: string,
    dto: { resolution: string; reason: string },
  ) {
    const dispute = await this.prisma.dispute.findUnique({
      where: { id: disputeId },
    });

    if (!dispute) {
      throw new NotFoundException('Dispute not found');
    }

    if (!['OPEN', 'UNDER_REVIEW'].includes(dispute.status)) {
      throw new BadRequestException(
        `Cannot resolve dispute in ${dispute.status} status`,
      );
    }

    const newStatus = dto.resolution === 'DENY' ? 'DENIED' : 'RESOLVED';

    const result = await this.prisma.$transaction(async (tx) => {
      // Update dispute
      const updated = await tx.dispute.update({
        where: { id: disputeId },
        data: {
          status: newStatus as any,
          resolution: dto.resolution as any,
          resolvedAt: new Date(),
          resolvedById: adminId,
        },
      });

      // Log operator action
      await tx.operatorAction.create({
        data: {
          operatorId: adminId,
          actionType: 'DISPUTE_RESOLVE',
          targetType: 'DISPUTE',
          targetId: disputeId,
          reason: dto.reason,
          metadata: { resolution: dto.resolution },
        },
      });

      // Apply resolution effects
      let newClaim = null;
      if (['REFUND_CLAIM', 'REPLACEMENT', 'GOODWILL_CREDIT'].includes(dto.resolution)) {
        const poolAndReceipt = await this.findPoolAndReceiptForObject(
          tx,
          dispute.objectType,
          dispute.objectId,
        );

        if (poolAndReceipt) {
          newClaim = await tx.claim.create({
            data: {
              userId: dispute.openedById,
              poolId: poolAndReceipt.poolId,
              receiptId: poolAndReceipt.receiptId,
              status: 'ACTIVE',
              activatedAt: new Date(),
            },
          });
        }
      }

      return { updated, newClaim };
    });

    // Notify user
    await this.notificationsService.create(
      dispute.openedById,
      'dispute_resolved',
      {
        disputeId,
        resolution: dto.resolution,
        reason: dto.reason,
        newClaimId: result.newClaim?.id,
      },
    );

    return {
      disputeId,
      status: newStatus,
      resolution: dto.resolution,
      newClaimId: result.newClaim?.id,
    };
  }

  // ── Private Helpers ─────────────────────────────────────────

  private async validateObjectOwnership(
    userId: string,
    objectType: string,
    objectId: string,
  ) {
    switch (objectType) {
      case 'SUBMISSION': {
        const sub = await this.prisma.itemSubmission.findUnique({
          where: { id: objectId },
        });
        if (!sub) throw new NotFoundException('Submission not found');
        if (sub.userId !== userId)
          throw new ForbiddenException('Not your submission');
        break;
      }
      case 'RECEIPT': {
        const receipt = await this.prisma.itemReceipt.findUnique({
          where: { id: objectId },
          include: { submission: { select: { userId: true } } },
        });
        if (!receipt) throw new NotFoundException('Receipt not found');
        if (receipt.submission.userId !== userId)
          throw new ForbiddenException('Not your receipt');
        break;
      }
      case 'INVENTORY': {
        const item = await this.prisma.inventoryItem.findUnique({
          where: { id: objectId },
        });
        if (!item) throw new NotFoundException('Inventory item not found');
        if (item.reservedById !== userId)
          throw new ForbiddenException('Not your inventory item');
        break;
      }
      case 'CLAIM': {
        const claim = await this.prisma.claim.findUnique({
          where: { id: objectId },
        });
        if (!claim) throw new NotFoundException('Claim not found');
        if (claim.userId !== userId)
          throw new ForbiddenException('Not your claim');
        break;
      }
      case 'SHIPMENT': {
        const shipment = await this.prisma.shipment.findUnique({
          where: { id: objectId },
          include: {
            submission: { select: { userId: true } },
            inventory: { select: { reservedById: true } },
          },
        });
        if (!shipment) throw new NotFoundException('Shipment not found');
        const isOwner =
          shipment.submission?.userId === userId ||
          shipment.inventory?.reservedById === userId;
        if (!isOwner)
          throw new ForbiddenException('Not your shipment');
        break;
      }
      case 'COURIER_TASK': {
        const task = await this.prisma.courierTask.findUnique({
          where: { id: objectId },
          include: {
            shipment: {
              include: {
                submission: { select: { userId: true } },
                inventory: { select: { reservedById: true } },
              },
            },
          },
        });
        if (!task) throw new NotFoundException('Courier task not found');
        const taskOwner =
          task.shipment?.submission?.userId === userId ||
          task.shipment?.inventory?.reservedById === userId;
        if (!taskOwner)
          throw new ForbiddenException('Not your courier task');
        break;
      }
      default:
        throw new BadRequestException(`Invalid object type: ${objectType}`);
    }
  }

  private async findPoolAndReceiptForObject(
    tx: any,
    objectType: string,
    objectId: string,
  ): Promise<{ poolId: string; receiptId: string } | null> {
    switch (objectType) {
      case 'CLAIM': {
        const claim = await tx.claim.findUnique({
          where: { id: objectId },
        });
        if (claim) return { poolId: claim.poolId, receiptId: claim.receiptId };
        break;
      }
      case 'RECEIPT': {
        const receipt = await tx.itemReceipt.findUnique({
          where: { id: objectId },
        });
        if (receipt) return { poolId: receipt.poolId, receiptId: receipt.id };
        break;
      }
      case 'INVENTORY': {
        const item = await tx.inventoryItem.findUnique({
          where: { id: objectId },
          include: { receipt: true },
        });
        if (item) return { poolId: item.poolId, receiptId: item.receiptId };
        break;
      }
      case 'SHIPMENT': {
        const shipment = await tx.shipment.findUnique({
          where: { id: objectId },
          include: {
            inventory: { include: { receipt: true } },
          },
        });
        if (shipment?.inventory) {
          return {
            poolId: shipment.inventory.poolId,
            receiptId: shipment.inventory.receiptId,
          };
        }
        break;
      }
      case 'SUBMISSION': {
        const receipt = await tx.itemReceipt.findFirst({
          where: { submissionId: objectId },
        });
        if (receipt) return { poolId: receipt.poolId, receiptId: receipt.id };
        break;
      }
      case 'COURIER_TASK': {
        const task = await tx.courierTask.findUnique({
          where: { id: objectId },
          include: {
            shipment: {
              include: { inventory: { include: { receipt: true } } },
            },
          },
        });
        if (task?.shipment?.inventory) {
          return {
            poolId: task.shipment.inventory.poolId,
            receiptId: task.shipment.inventory.receiptId,
          };
        }
        break;
      }
    }
    return null;
  }
}
