import { Module } from '@nestjs/common';
import { SubmissionsController } from './submissions.controller';
import { SubmissionsService } from './submissions.service';
import { MediaController } from './media.controller';
import { MediaService } from './media.service';
import { LabelsController } from './labels.controller';
import { LabelsService } from './labels.service';
import { ScreeningService } from './screening.service';

@Module({
  controllers: [SubmissionsController, MediaController, LabelsController],
  providers: [SubmissionsService, MediaService, LabelsService, ScreeningService],
  exports: [SubmissionsService, ScreeningService],
})
export class SubmissionsModule {}
