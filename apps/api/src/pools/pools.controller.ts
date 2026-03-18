import {
  Controller,
  Get,
  Param,
  Query,
  UseGuards,
  Request,
} from '@nestjs/common';
import { PoolsService } from './pools.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';

@Controller('pools')
export class PoolsController {
  constructor(private readonly poolsService: PoolsService) {}

  @Get()
  async listPools(
    @Query('hue') hue?: string,
    @Query('band') band?: string,
    @Query('region') region?: string,
    @Query('qualityTier') qualityTier?: string,
    @Query('status') status?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.poolsService.findAll({
      hue,
      band: band ? parseInt(band, 10) : undefined,
      region,
      qualityTier,
      status,
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Get(':id')
  async getPool(@Param('id') id: string) {
    return this.poolsService.findOne(id);
  }

  @Get(':id/inventory')
  @UseGuards(JwtAuthGuard)
  async getPoolInventory(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
    @Query('sort') sort?: string,
  ) {
    return this.poolsService.getInventory(req.user.userId, id, {
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
      sort,
    });
  }
}
