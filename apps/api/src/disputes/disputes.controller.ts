import {
  Controller,
  Get,
  Post,
  Param,
  Body,
  Query,
  UseGuards,
  Request,
  BadRequestException,
} from '@nestjs/common';
import { DisputesService } from './disputes.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { CreateDisputeDto } from './dto/create-dispute.dto';
import { SubmitEvidenceDto } from './dto/submit-evidence.dto';

interface UploadedFile {
  originalname: string;
  mimetype: string;
  size: number;
  buffer: Buffer;
}

@Controller('disputes')
@UseGuards(JwtAuthGuard)
export class DisputesController {
  constructor(private readonly disputesService: DisputesService) {}

  @Post()
  async create(
    @Request() req: { user: { userId: string } },
    @Body() dto: CreateDisputeDto,
  ) {
    return this.disputesService.create(req.user.userId, dto);
  }

  @Post(':id/evidence')
  async submitEvidence(
    @Request() req: { user: { userId: string; role: string }; files?: UploadedFile[] },
    @Param('id') id: string,
    @Body() dto: SubmitEvidenceDto,
  ) {
    const file = req.files?.[0];
    if (dto.evidenceType !== 'TEXT' && !file) {
      throw new BadRequestException(
        `${dto.evidenceType} evidence requires a file upload`,
      );
    }

    const isAdmin = ['ADMIN', 'SUPER_ADMIN'].includes(req.user.role);
    return this.disputesService.submitEvidence(
      req.user.userId,
      id,
      dto,
      file,
      isAdmin,
    );
  }

  @Get()
  async findAll(
    @Request() req: { user: { userId: string } },
    @Query('status') status?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.disputesService.findAll(req.user.userId, {
      status,
      page: page ? parseInt(page, 10) : undefined,
      limit: limit ? parseInt(limit, 10) : undefined,
    });
  }

  @Get(':id')
  async findOne(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
  ) {
    return this.disputesService.findOne(req.user.userId, id);
  }
}
