import {
  Controller,
  Post,
  Get,
  Body,
  Param,
  Query,
  UseGuards,
  Request,
} from '@nestjs/common';
import { WarehouseService } from './warehouse.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RolesGuard, Roles } from '../auth/guards/roles.guard';
import { ScanIntakeDto } from './dto/scan-intake.dto';
import { GradeSubmissionDto } from './dto/grade-submission.dto';
import { QuarantineDto, ResolveQuarantineDto } from './dto/quarantine.dto';

@Controller('warehouse')
@UseGuards(JwtAuthGuard, RolesGuard)
@Roles('OPERATOR', 'GRADER', 'ADMIN', 'SUPER_ADMIN')
export class WarehouseController {
  constructor(private readonly warehouseService: WarehouseService) {}

  @Post('intake/scan')
  async scanIntake(
    @Body() dto: ScanIntakeDto,
  ) {
    return this.warehouseService.scanIntake(dto.code);
  }

  @Get('intake/queue')
  async getIntakeQueue(
    @Query('page') page?: string,
    @Query('limit') limit?: string,
    @Query('warehouseId') warehouseId?: string,
  ) {
    return this.warehouseService.getIntakeQueue({
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
      warehouseId,
    });
  }

  @Post('intake/:submissionId/grade')
  async gradeSubmission(
    @Request() req: { user: { userId: string } },
    @Param('submissionId') submissionId: string,
    @Body() dto: GradeSubmissionDto,
  ) {
    return this.warehouseService.gradeSubmission(
      req.user.userId,
      submissionId,
      dto,
    );
  }

  @Post('inventory/:id/quarantine')
  async quarantineInventory(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Body() dto: QuarantineDto,
  ) {
    return this.warehouseService.quarantineInventory(
      req.user.userId,
      id,
      dto,
    );
  }

  @Post('inventory/:id/quarantine/resolve')
  async resolveQuarantine(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Body() dto: ResolveQuarantineDto,
  ) {
    return this.warehouseService.resolveQuarantine(
      req.user.userId,
      id,
      dto,
    );
  }
}
