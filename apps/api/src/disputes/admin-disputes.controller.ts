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
import { DisputesService } from './disputes.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RolesGuard, Roles } from '../auth/guards/roles.guard';
import { ResolveDisputeDto } from './dto/resolve-dispute.dto';

@Controller('admin/disputes')
@UseGuards(JwtAuthGuard, RolesGuard)
@Roles('ADMIN', 'SUPER_ADMIN')
export class AdminDisputesController {
  constructor(private readonly disputesService: DisputesService) {}

  @Get('queue')
  async getQueue(
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.disputesService.getAdminQueue({
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Post(':id/resolve')
  async resolve(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Body() dto: ResolveDisputeDto,
  ) {
    return this.disputesService.resolve(req.user.userId, id, dto);
  }
}
