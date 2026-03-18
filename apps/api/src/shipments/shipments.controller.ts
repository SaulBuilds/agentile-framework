import {
  Controller,
  Get,
  Post,
  Patch,
  Param,
  Body,
  Query,
  UseGuards,
  Request,
} from '@nestjs/common';
import { ShipmentsService } from './shipments.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RolesGuard, Roles } from '../auth/guards/roles.guard';
import { AddTrackingDto } from './dto/add-tracking.dto';
import { DeliverDto } from './dto/deliver.dto';

@Controller('shipments')
@UseGuards(JwtAuthGuard)
export class ShipmentsController {
  constructor(private readonly shipmentsService: ShipmentsService) {}

  @Get()
  async listShipments(
    @Request() req: { user: { userId: string } },
    @Query('direction') direction?: string,
    @Query('status') status?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.shipmentsService.findAll(req.user.userId, {
      direction,
      status,
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Get(':id')
  async getShipment(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
  ) {
    return this.shipmentsService.findOne(req.user.userId, id);
  }

  @Get(':id/tracking')
  async getTracking(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
  ) {
    return this.shipmentsService.getTracking(req.user.userId, id);
  }

  @Post(':id/tracking')
  @UseGuards(RolesGuard)
  @Roles('OPERATOR', 'ADMIN', 'SUPER_ADMIN')
  async addTrackingEvent(
    @Param('id') id: string,
    @Body() dto: AddTrackingDto,
  ) {
    return this.shipmentsService.addTrackingEvent(
      id,
      dto.eventType,
      dto.location,
      dto.timestamp,
    );
  }

  @Post(':id/deliver')
  async confirmDelivery(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Body() dto: DeliverDto,
  ) {
    return this.shipmentsService.confirmDelivery(
      req.user.userId,
      id,
      dto.proofMethod,
      dto.proofData,
      dto.notes,
    );
  }
}
