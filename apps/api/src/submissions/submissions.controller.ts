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
import { SubmissionsService } from './submissions.service';
import { CreateSubmissionDto } from './dto/create-submission.dto';
import { SubmitItemDto } from './dto/submit-item.dto';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';

@Controller('submissions')
@UseGuards(JwtAuthGuard)
export class SubmissionsController {
  constructor(private readonly submissionsService: SubmissionsService) {}

  @Post()
  async create(
    @Request() req: { user: { userId: string } },
    @Body() dto: CreateSubmissionDto,
  ) {
    return this.submissionsService.create(req.user.userId, dto);
  }

  @Get()
  async findAll(
    @Request() req: { user: { userId: string } },
    @Query('status') status?: string,
    @Query('page') page?: string,
    @Query('limit') limit?: string,
  ) {
    return this.submissionsService.findAll(req.user.userId, {
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
    return this.submissionsService.findOne(req.user.userId, id);
  }

  @Post(':id/submit')
  async submit(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Body() dto: SubmitItemDto,
  ) {
    return this.submissionsService.submit(req.user.userId, id, dto);
  }
}
