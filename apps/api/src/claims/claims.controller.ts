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
import { ClaimsService } from './claims.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { ReserveDto } from './dto/reserve.dto';
import { FinalizeDto } from './dto/finalize.dto';

@Controller('claims')
@UseGuards(JwtAuthGuard)
export class ClaimsController {
  constructor(private readonly claimsService: ClaimsService) {}

  @Get()
  async listClaims(
    @Request() req: { user: { userId: string } },
    @Query('status') status?: string,
    @Query('poolId') poolId?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.claimsService.findAll(req.user.userId, {
      status,
      poolId,
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Get(':id')
  async getClaim(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
  ) {
    return this.claimsService.findOne(req.user.userId, id);
  }

  @Post(':claimId/reserve')
  async reserve(
    @Request() req: { user: { userId: string } },
    @Param('claimId') claimId: string,
    @Body() dto: ReserveDto,
  ) {
    return this.claimsService.reserve(
      req.user.userId,
      claimId,
      dto.inventoryId,
    );
  }

  @Post(':claimId/reserve/cancel')
  async cancelReservation(
    @Request() req: { user: { userId: string } },
    @Param('claimId') claimId: string,
  ) {
    return this.claimsService.cancelReservation(req.user.userId, claimId);
  }

  @Post(':claimId/reserve/finalize')
  async finalize(
    @Request() req: { user: { userId: string } },
    @Param('claimId') claimId: string,
    @Body() dto: FinalizeDto,
  ) {
    return this.claimsService.finalize(
      req.user.userId,
      claimId,
      dto.deliveryMethod,
      dto.shippingAddress,
    );
  }
}
