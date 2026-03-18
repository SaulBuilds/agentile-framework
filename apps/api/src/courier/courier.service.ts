import {
  Injectable,
  Logger,
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { NotificationsService } from '../notifications/notifications.service';
import { createHash } from 'crypto';

@Injectable()
export class CourierService {
  private readonly logger = new Logger(CourierService.name);

  constructor(
    private readonly prisma: PrismaService,
    private readonly notificationsService: NotificationsService,
  ) {}

  // ── Task Board ──────────────────────────────────────────────

  async getAvailableTasks(
    courierId: string,
    filters?: { region?: string; status?: string; page?: number; limit?: number },
  ) {
    // Verify courier is approved
    await this.requireApprovedCourier(courierId);

    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const where: any = {
      status: filters?.status || 'POSTED',
    };

    if (filters?.region) {
      where.pickupLocation = { path: ['region'], equals: filters.region };
    }

    const [tasks, total] = await Promise.all([
      this.prisma.courierTask.findMany({
        where,
        orderBy: { createdAt: 'desc' },
        skip,
        take: limit,
        include: {
          shipment: {
            select: { id: true, direction: true, status: true },
          },
        },
      }),
      this.prisma.courierTask.count({ where }),
    ]);

    return {
      data: tasks.map((t) => ({
        id: t.id,
        shipmentId: t.shipmentId,
        // Redact precise addresses for unaccepted tasks
        pickupLocation: this.redactAddress(t.pickupLocation as any),
        dropoffLocation: this.redactAddress(t.dropoffLocation as any),
        fee: Number(t.fee),
        tip: t.tip ? Number(t.tip) : null,
        status: t.status,
        timeWindowStart: t.timeWindowStart,
        timeWindowEnd: t.timeWindowEnd,
        createdAt: t.createdAt,
      })),
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  async acceptTask(courierId: string, taskId: string) {
    await this.requireApprovedCourier(courierId);

    const task = await this.prisma.courierTask.findUnique({
      where: { id: taskId },
    });

    if (!task) {
      throw new NotFoundException('Courier task not found');
    }

    if (task.status !== 'POSTED') {
      throw new BadRequestException(
        `Cannot accept task in ${task.status} status`,
      );
    }

    await this.prisma.$transaction(async (tx) => {
      await tx.courierTask.update({
        where: { id: taskId },
        data: {
          courierId,
          status: 'ACCEPTED',
        },
      });

      await tx.courierEvent.create({
        data: {
          taskId,
          eventType: 'ACCEPTED',
        },
      });
    });

    return {
      taskId,
      status: 'ACCEPTED',
      message: 'Task accepted — proceed to pickup location',
    };
  }

  async confirmPickup(
    courierId: string,
    taskId: string,
    verification: { qrCode?: string; pin?: string; photoProof?: string },
  ) {
    const task = await this.getOwnedTask(courierId, taskId);

    if (task.status !== 'ACCEPTED') {
      throw new BadRequestException(
        `Cannot pickup in ${task.status} status — must accept first`,
      );
    }

    if (!verification.qrCode && !verification.pin) {
      throw new BadRequestException(
        'Pickup requires QR code scan or PIN verification',
      );
    }

    const proofData = verification.qrCode || verification.pin!;
    const proofHash = createHash('sha256')
      .update(`pickup:${proofData}:${taskId}`)
      .digest('hex');

    await this.prisma.$transaction(async (tx) => {
      await tx.courierTask.update({
        where: { id: taskId },
        data: { status: 'PICKUP_VERIFIED' },
      });

      await tx.courierEvent.create({
        data: {
          taskId,
          eventType: 'PICKUP_SCAN',
          proofHash,
          location: (task.pickupLocation as any) || undefined,
        },
      });
    });

    return {
      taskId,
      status: 'PICKUP_VERIFIED',
      proofHash,
    };
  }

  async reportMilestone(
    courierId: string,
    taskId: string,
    location: { lat: number; lng: number },
    notes?: string,
  ) {
    const task = await this.getOwnedTask(courierId, taskId);

    if (!['PICKUP_VERIFIED', 'IN_TRANSIT_COURIER'].includes(task.status)) {
      throw new BadRequestException(
        `Cannot report milestone in ${task.status} status`,
      );
    }

    await this.prisma.$transaction(async (tx) => {
      // Move to IN_TRANSIT_COURIER if not already
      if (task.status === 'PICKUP_VERIFIED') {
        await tx.courierTask.update({
          where: { id: taskId },
          data: { status: 'IN_TRANSIT_COURIER' },
        });
      }

      await tx.courierEvent.create({
        data: {
          taskId,
          eventType: 'MILESTONE',
          location: { ...location, notes },
        },
      });
    });

    return {
      taskId,
      status: 'IN_TRANSIT_COURIER',
    };
  }

  async confirmDelivery(
    courierId: string,
    taskId: string,
    proofMethod: string,
    proofData: string,
  ) {
    const task = await this.getOwnedTask(courierId, taskId);

    if (!['PICKUP_VERIFIED', 'IN_TRANSIT_COURIER'].includes(task.status)) {
      throw new BadRequestException(
        `Cannot deliver in ${task.status} status`,
      );
    }

    const validMethods = ['qr', 'signature', 'photo', 'pin'];
    if (!validMethods.includes(proofMethod)) {
      throw new BadRequestException(
        `Invalid proof method. Must be one of: ${validMethods.join(', ')}`,
      );
    }

    const proofHash = createHash('sha256')
      .update(`${proofMethod}:${proofData}:${taskId}`)
      .digest('hex');

    await this.prisma.$transaction(async (tx) => {
      await tx.courierTask.update({
        where: { id: taskId },
        data: { status: 'DELIVERED_COURIER' },
      });

      await tx.courierEvent.create({
        data: {
          taskId,
          eventType: 'DROPOFF_PROOF',
          proofHash,
          location: (task.dropoffLocation as any) || undefined,
        },
      });

      // Also update the shipment status
      await tx.shipment.update({
        where: { id: task.shipmentId },
        data: {
          status: 'DELIVERED',
          deliveredAt: new Date(),
          proofHash,
        },
      });

      // Update inventory to DELIVERED if outbound
      const shipment = await tx.shipment.findUnique({
        where: { id: task.shipmentId },
      });
      if (shipment?.inventoryId && shipment.direction === 'OUTBOUND') {
        await tx.inventoryItem.update({
          where: { id: shipment.inventoryId },
          data: { status: 'DELIVERED' },
        });

        await tx.inventoryStatusEvent.create({
          data: {
            inventoryId: shipment.inventoryId,
            previousStatus: 'OUTBOUND',
            newStatus: 'DELIVERED',
            actorId: courierId,
            evidenceHash: proofHash,
          },
        });
      }
    });

    return {
      taskId,
      status: 'DELIVERED_COURIER',
      proofHash,
    };
  }

  async completeTask(taskId: string) {
    const task = await this.prisma.courierTask.findUnique({
      where: { id: taskId },
    });

    if (!task) {
      throw new NotFoundException('Courier task not found');
    }

    if (task.status !== 'DELIVERED_COURIER') {
      throw new BadRequestException(
        `Cannot complete task in ${task.status} status`,
      );
    }

    await this.prisma.$transaction(async (tx) => {
      await tx.courierTask.update({
        where: { id: taskId },
        data: { status: 'COMPLETED' },
      });

      await tx.courierEvent.create({
        data: {
          taskId,
          eventType: 'COMPLETED',
        },
      });
    });

    // Notify courier of payout
    if (task.courierId) {
      await this.notificationsService.create(
        task.courierId,
        'payout_released',
        {
          taskId: task.id,
          fee: Number(task.fee),
          tip: task.tip ? Number(task.tip) : 0,
          total: Number(task.fee) + (task.tip ? Number(task.tip) : 0),
        },
      );
    }

    return { taskId, status: 'COMPLETED' };
  }

  // ── Earnings ────────────────────────────────────────────────

  async getEarnings(
    courierId: string,
    filters?: { page?: number; limit?: number },
  ) {
    await this.requireApprovedCourier(courierId);

    const page = filters?.page ?? 1;
    const limit = filters?.limit ?? 20;
    const skip = (page - 1) * limit;

    const [tasks, total] = await Promise.all([
      this.prisma.courierTask.findMany({
        where: {
          courierId,
          status: { in: ['COMPLETED', 'DELIVERED_COURIER'] },
        },
        orderBy: { createdAt: 'desc' },
        skip,
        take: limit,
        select: {
          id: true,
          fee: true,
          tip: true,
          status: true,
          createdAt: true,
        },
      }),
      this.prisma.courierTask.count({
        where: {
          courierId,
          status: { in: ['COMPLETED', 'DELIVERED_COURIER'] },
        },
      }),
    ]);

    // Calculate totals from all completed tasks
    const totals = await this.prisma.courierTask.aggregate({
      where: { courierId, status: 'COMPLETED' },
      _sum: { fee: true, tip: true },
      _count: true,
    });

    const pending = await this.prisma.courierTask.aggregate({
      where: { courierId, status: 'DELIVERED_COURIER' },
      _sum: { fee: true, tip: true },
      _count: true,
    });

    return {
      data: tasks.map((t) => ({
        taskId: t.id,
        fee: Number(t.fee),
        tip: t.tip ? Number(t.tip) : 0,
        total: Number(t.fee) + (t.tip ? Number(t.tip) : 0),
        status: t.status === 'COMPLETED' ? 'paid' : 'pending',
        completedAt: t.createdAt,
      })),
      summary: {
        totalEarned: Number(totals._sum.fee || 0) + Number(totals._sum.tip || 0),
        totalPending: Number(pending._sum.fee || 0) + Number(pending._sum.tip || 0),
        completedTasks: totals._count,
        pendingTasks: pending._count,
      },
      meta: { total, page, limit, pages: Math.ceil(total / limit) },
    };
  }

  // ── Safety ──────────────────────────────────────────────────

  async reportEmergency(
    courierId: string,
    taskId: string,
    report: { type: string; description: string; location?: { lat: number; lng: number } },
  ) {
    const task = await this.getOwnedTask(courierId, taskId);

    await this.prisma.courierEvent.create({
      data: {
        taskId,
        eventType: 'EXCEPTION',
        location: report.location || undefined,
      },
    });

    // Notify admins
    const admins = await this.prisma.user.findMany({
      where: { role: { in: ['ADMIN', 'SUPER_ADMIN', 'OPERATOR'] } },
      select: { id: true },
    });

    for (const admin of admins) {
      await this.notificationsService.create(
        admin.id,
        'courier_emergency',
        {
          taskId,
          courierId,
          reportType: report.type,
          description: report.description,
          location: report.location,
        },
      );
    }

    return { taskId, reported: true };
  }

  // ── Helpers ─────────────────────────────────────────────────

  private async requireApprovedCourier(userId: string) {
    const user = await this.prisma.user.findUnique({
      where: { id: userId },
      select: { role: true, kycStatus: true },
    });

    if (!user) {
      throw new NotFoundException('User not found');
    }

    if (user.role !== 'COURIER') {
      throw new ForbiddenException('User is not a courier');
    }

    if (user.kycStatus !== 'APPROVED') {
      throw new ForbiddenException(
        'Courier not approved — complete verification first',
      );
    }
  }

  private async getOwnedTask(courierId: string, taskId: string) {
    const task = await this.prisma.courierTask.findUnique({
      where: { id: taskId },
    });

    if (!task) {
      throw new NotFoundException('Courier task not found');
    }

    if (task.courierId !== courierId) {
      throw new ForbiddenException('Not your task');
    }

    return task;
  }

  private redactAddress(location: any): any {
    if (!location) return location;
    // Only show city/region, hide precise street until task is accepted
    return {
      city: location.city,
      region: location.region,
      state: location.state,
    };
  }
}
