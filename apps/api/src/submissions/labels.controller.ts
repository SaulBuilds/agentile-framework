import {
  Controller,
  Get,
  Param,
  UseGuards,
  Request,
  Res,
} from '@nestjs/common';
import { Response } from 'express';
import { LabelsService } from './labels.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';

@Controller('submissions')
@UseGuards(JwtAuthGuard)
export class LabelsController {
  constructor(private readonly labelsService: LabelsService) {}

  @Get(':id/label')
  async getLabel(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
  ) {
    return this.labelsService.getLabel(req.user.userId, id);
  }

  @Get(':id/label/pdf')
  async getLabelPdf(
    @Request() req: { user: { userId: string } },
    @Param('id') id: string,
    @Res() res: Response,
  ) {
    const pdf = await this.labelsService.getLabelPdf(req.user.userId, id);

    res.set({
      'Content-Type': 'application/pdf',
      'Content-Disposition': `attachment; filename="label-${id}.pdf"`,
      'Content-Length': pdf.length,
    });

    res.end(pdf);
  }
}
