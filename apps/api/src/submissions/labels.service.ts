import {
  Injectable,
  NotFoundException,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { PrismaService } from '../prisma/prisma.service';
import { randomBytes } from 'crypto';
import * as PDFDocument from 'pdfkit';

export interface PackingInstruction {
  category: string;
  steps: string[];
  warnings: string[];
  materials: string[];
}

const PACKING_INSTRUCTIONS: Record<string, PackingInstruction> = {
  'power tools': {
    category: 'Power Tools',
    steps: [
      'Remove battery if detachable',
      'Wrap tool in bubble wrap or protective foam',
      'Secure moving parts with tape or zip ties',
      'Place in sturdy box with padding on all sides',
      'Seal box with packing tape',
    ],
    warnings: [
      'Do NOT ship with damaged lithium batteries',
      'Ensure trigger lock is engaged if applicable',
    ],
    materials: ['Bubble wrap', 'Packing tape', 'Sturdy box', 'Zip ties'],
  },
  'hand tools': {
    category: 'Hand Tools',
    steps: [
      'Wrap sharp edges with cardboard or blade guards',
      'Bundle small tools together with rubber bands',
      'Place in padded envelope or small box',
      'Fill empty space with packing paper',
    ],
    warnings: ['Cover all sharp edges to prevent injury during handling'],
    materials: ['Cardboard guards', 'Rubber bands', 'Packing paper'],
  },
  'consumer electronics': {
    category: 'Consumer Electronics',
    steps: [
      'Power off and remove batteries if possible',
      'Wrap device in anti-static bag or bubble wrap',
      'Include cables/accessories in separate bag',
      'Place in box with minimum 2 inches of padding on each side',
      'Mark box as FRAGILE',
    ],
    warnings: [
      'Do NOT ship devices with swollen batteries',
      'Remove SIM cards and personal data before shipping',
    ],
    materials: ['Anti-static bag', 'Bubble wrap', 'Packing peanuts', 'Fragile stickers'],
  },
  'gaming consoles': {
    category: 'Gaming Consoles & Accessories',
    steps: [
      'Use original box if available',
      'Wrap console in bubble wrap',
      'Include all controllers and cables in separate bags',
      'Fill empty space with packing material',
    ],
    warnings: ['Ensure disc drive is empty'],
    materials: ['Bubble wrap', 'Ziplock bags for cables', 'Packing paper'],
  },
  'small appliances': {
    category: 'Small Appliances',
    steps: [
      'Clean item thoroughly',
      'Wrap in protective material',
      'Secure cord with twist tie',
      'Place in box with adequate padding',
    ],
    warnings: ['Empty all reservoirs (water, filters, etc.)'],
    materials: ['Bubble wrap', 'Twist ties', 'Packing paper'],
  },
  'default': {
    category: 'General Item',
    steps: [
      'Clean item before packing',
      'Wrap in protective material (bubble wrap or paper)',
      'Place in sturdy box or padded envelope',
      'Fill void with packing material',
      'Seal securely with packing tape',
      'Attach shipping label to outside',
    ],
    warnings: [],
    materials: ['Bubble wrap or packing paper', 'Box or padded envelope', 'Packing tape'],
  },
};

@Injectable()
export class LabelsService {
  constructor(private readonly prisma: PrismaService) {}

  async getLabel(userId: string, submissionId: string) {
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: submissionId },
      include: { qrLabel: true },
    });

    if (!submission) {
      throw new NotFoundException('Submission not found');
    }

    if (submission.userId !== userId) {
      throw new ForbiddenException('Not your submission');
    }

    if (submission.status === 'DRAFT') {
      throw new BadRequestException('Submit item before generating label');
    }

    if (submission.qrLabel) {
      return {
        qrData: submission.qrLabel.qrData,
        humanReadableCode: submission.qrLabel.humanReadableCode,
        labelPdfUrl: `/api/v1/submissions/${submissionId}/label/pdf`,
        packingInstructions: this.getPackingInstructions(submission.category),
      };
    }

    const humanReadableCode = this.generateHumanReadableCode();
    const qrData = JSON.stringify({
      submissionId,
      code: humanReadableCode,
      category: submission.category,
      band: submission.estimatedBand,
    });

    await this.prisma.qrLabel.create({
      data: {
        submissionId,
        qrData,
        humanReadableCode,
      },
    });

    return {
      qrData,
      humanReadableCode,
      labelPdfUrl: `/api/v1/submissions/${submissionId}/label/pdf`,
      packingInstructions: this.getPackingInstructions(submission.category),
    };
  }

  async getLabelPdf(userId: string, submissionId: string): Promise<Buffer> {
    const submission = await this.prisma.itemSubmission.findUnique({
      where: { id: submissionId },
      include: { qrLabel: true },
    });

    if (!submission) {
      throw new NotFoundException('Submission not found');
    }

    if (submission.userId !== userId) {
      throw new ForbiddenException('Not your submission');
    }

    if (!submission.qrLabel) {
      throw new BadRequestException('Label not yet generated — call GET /label first');
    }

    return this.generateLabelPdf(
      submission.qrLabel.humanReadableCode,
      submission.qrLabel.qrData,
      submission.category,
      submission.estimatedBand,
    );
  }

  getPackingInstructions(category: string): PackingInstruction {
    const normalized = category.toLowerCase().trim();

    for (const [key, instructions] of Object.entries(PACKING_INSTRUCTIONS)) {
      if (key === 'default') continue;
      if (normalized.includes(key)) {
        return instructions;
      }
    }

    return PACKING_INSTRUCTIONS['default'];
  }

  private generateHumanReadableCode(): string {
    const prefix = 'GBP';
    const random = randomBytes(4).toString('hex').toUpperCase();
    return `${prefix}-${random.slice(0, 4)}-${random.slice(4, 8)}`;
  }

  private generateLabelPdf(
    code: string,
    qrData: string,
    category: string,
    band: number,
  ): Promise<Buffer> {
    return new Promise((resolve, reject) => {
      const doc = new PDFDocument({ size: [288, 432], margin: 20 });
      const chunks: Buffer[] = [];

      doc.on('data', (chunk: Buffer) => chunks.push(chunk));
      doc.on('end', () => resolve(Buffer.concat(chunks)));
      doc.on('error', reject);

      // Header
      doc.fontSize(14).font('Helvetica-Bold')
        .text('GRADIENT BARTER PROTOCOL', { align: 'center' });
      doc.fontSize(10).font('Helvetica')
        .text('SHIPPING LABEL', { align: 'center' });
      doc.moveDown(0.5);

      // Divider
      doc.moveTo(20, doc.y).lineTo(268, doc.y).stroke();
      doc.moveDown(0.5);

      // Code
      doc.fontSize(20).font('Helvetica-Bold')
        .text(code, { align: 'center' });
      doc.moveDown(0.5);

      // Item details
      doc.fontSize(10).font('Helvetica-Bold').text('Category: ', { continued: true });
      doc.font('Helvetica').text(category);
      doc.font('Helvetica-Bold').text('Band: ', { continued: true });
      doc.font('Helvetica').text(String(band));
      doc.moveDown(0.5);

      // Divider
      doc.moveTo(20, doc.y).lineTo(268, doc.y).stroke();
      doc.moveDown(0.5);

      // QR data (text representation)
      doc.fontSize(8).font('Helvetica-Bold').text('QR Data:');
      doc.font('Courier').text(qrData, { width: 248 });
      doc.moveDown(0.5);

      // Divider
      doc.moveTo(20, doc.y).lineTo(268, doc.y).stroke();
      doc.moveDown(0.5);

      // Ship to
      doc.fontSize(10).font('Helvetica-Bold').text('Ship to:');
      doc.font('Helvetica').text('Gradient Barter Warehouse');
      doc.text('[Address configured per region]');
      doc.moveDown(1);

      // Footer
      doc.fontSize(8).font('Helvetica')
        .text('Handle with care. Do not bend.', { align: 'center' });

      doc.end();
    });
  }
}
