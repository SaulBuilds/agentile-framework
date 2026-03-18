import {
  Controller,
  Post,
  Param,
  UseGuards,
  Request,
  BadRequestException,
} from '@nestjs/common';
import { MediaService, UploadedFile } from './media.service';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';

@Controller('submissions')
@UseGuards(JwtAuthGuard)
export class MediaController {
  constructor(private readonly mediaService: MediaService) {}

  /**
   * Upload media files for a submission.
   * In production, this would use @UseInterceptors(FilesInterceptor(...))
   * with multer. For the scaffold, we accept raw request parsing.
   *
   * The actual file upload is handled via signed URLs in production:
   * 1. Client calls this endpoint to get signed upload URLs
   * 2. Client uploads directly to S3
   * 3. Client confirms upload with media IDs
   *
   * For local dev, files are written to ./uploads/
   */
  @Post(':id/media')
  async uploadMedia(
    @Request() req: { user: { userId: string }; files?: UploadedFile[] },
    @Param('id') id: string,
  ) {
    const files = req.files;
    if (!files || files.length === 0) {
      throw new BadRequestException('No files provided');
    }

    return this.mediaService.uploadMedia(req.user.userId, id, files);
  }
}
