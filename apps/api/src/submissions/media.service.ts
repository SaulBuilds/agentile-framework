import {
  Injectable,
  BadRequestException,
  NotFoundException,
  ForbiddenException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { randomUUID } from 'crypto';
import * as path from 'path';
import * as fs from 'fs';

const ALLOWED_MIME_TYPES = ['image/jpeg', 'image/png', 'image/heic'];
const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB
const MAX_MEDIA_PER_SUBMISSION = 10;

export interface UploadedFile {
  originalname: string;
  mimetype: string;
  size: number;
  buffer: Buffer;
}

@Injectable()
export class MediaService {
  private readonly uploadDir: string;

  constructor(private readonly prisma: PrismaService) {
    this.uploadDir = path.resolve(process.cwd(), 'uploads');
    if (!fs.existsSync(this.uploadDir)) {
      fs.mkdirSync(this.uploadDir, { recursive: true });
    }
  }

  async uploadMedia(userId: string, submissionId: string, files: UploadedFile[]) {
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: submissionId },
      include: { media: true },
    });

    if (!submission) {
      throw new NotFoundException('Submission not found');
    }

    if (submission.userId !== userId) {
      throw new ForbiddenException('Not your submission');
    }

    if (submission.status !== 'DRAFT') {
      throw new BadRequestException('Cannot upload media after submission');
    }

    if (!files || files.length === 0) {
      throw new BadRequestException('At least one file required');
    }

    const totalAfterUpload = submission.media.length + files.length;
    if (totalAfterUpload > MAX_MEDIA_PER_SUBMISSION) {
      throw new BadRequestException(
        `Maximum ${MAX_MEDIA_PER_SUBMISSION} photos allowed. Currently have ${submission.media.length}.`,
      );
    }

    // Validate each file
    for (const file of files) {
      if (!ALLOWED_MIME_TYPES.includes(file.mimetype)) {
        throw new BadRequestException(
          `Invalid file type: ${file.mimetype}. Allowed: JPEG, PNG, HEIC`,
        );
      }
      if (file.size > MAX_FILE_SIZE) {
        throw new BadRequestException(
          `File ${file.originalname} exceeds 10MB limit`,
        );
      }
    }

    // Save files and create media records
    const mediaIds: string[] = [];

    for (const file of files) {
      const ext = path.extname(file.originalname) || '.jpg';
      const s3Key = `submissions/${submissionId}/${randomUUID()}${ext}`;
      const localPath = path.join(this.uploadDir, s3Key);

      // Create directory structure
      fs.mkdirSync(path.dirname(localPath), { recursive: true });
      fs.writeFileSync(localPath, file.buffer);

      const mediaType = file.mimetype === 'video/mp4' ? 'VIDEO' : 'PHOTO';

      const media = await this.prisma.itemMedia.create({
        data: {
          submissionId,
          mediaType: mediaType as 'PHOTO' | 'VIDEO',
          s3Key,
        },
      });

      mediaIds.push(media.id);
    }

    return { mediaIds };
  }
}
