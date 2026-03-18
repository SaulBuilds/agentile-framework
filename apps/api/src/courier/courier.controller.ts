import {
  Controller,
  Get,
  Post,
  Param,
  Body,
  Query,
  UseGuards,
  Request,
} from '@nestjs/common';
import { CourierService } from './courier.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RolesGuard, Roles } from '../auth/guards/roles.guard';
import {
  PickupDto,
  MilestoneDto,
  DeliverDto,
  EmergencyReportDto,
} from './dto/courier.dto';

@Controller('courier')
@UseGuards(JwtAuthGuard, RolesGuard)
@Roles('COURIER')
export class CourierController {
  constructor(private readonly courierService: CourierService) {}

  @Get('tasks')
  async listTasks(
    @Request() req: { user: { userId: string } },
    @Query('region') region?: string,
    @Query('status') status?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.courierService.getAvailableTasks(req.user.userId, {
      region,
      status,
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Post('tasks/:id/accept')
  async acceptTask(
    @Request() req: { user: { userId: string } },
    @Param('id') taskId: string,
  ) {
    return this.courierService.acceptTask(req.user.userId, taskId);
  }

  @Post('tasks/:id/pickup')
  async confirmPickup(
    @Request() req: { user: { userId: string } },
    @Param('id') taskId: string,
    @Body() dto: PickupDto,
  ) {
    return this.courierService.confirmPickup(req.user.userId, taskId, {
      qrCode: dto.qrCode,
      pin: dto.pin,
      photoProof: dto.photoProof,
    });
  }

  @Post('tasks/:id/milestone')
  async reportMilestone(
    @Request() req: { user: { userId: string } },
    @Param('id') taskId: string,
    @Body() dto: MilestoneDto,
  ) {
    return this.courierService.reportMilestone(
      req.user.userId,
      taskId,
      dto.location,
      dto.notes,
    );
  }

  @Post('tasks/:id/deliver')
  async confirmDelivery(
    @Request() req: { user: { userId: string } },
    @Param('id') taskId: string,
    @Body() dto: DeliverDto,
  ) {
    return this.courierService.confirmDelivery(
      req.user.userId,
      taskId,
      dto.proofMethod,
      dto.proofData,
    );
  }

  @Get('earnings')
  async getEarnings(
    @Request() req: { user: { userId: string } },
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.courierService.getEarnings(req.user.userId, {
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Post('tasks/:id/emergency')
  async reportEmergency(
    @Request() req: { user: { userId: string } },
    @Param('id') taskId: string,
    @Body() dto: EmergencyReportDto,
  ) {
    return this.courierService.reportEmergency(req.user.userId, taskId, {
      type: dto.type,
      description: dto.description,
      location: dto.location,
    });
  }
}
