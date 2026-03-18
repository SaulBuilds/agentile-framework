import { IsString, IsOptional, IsIn } from 'class-validator';

export class SubmitEvidenceDto {
  @IsIn(['PHOTO', 'VIDEO', 'TEXT', 'DOCUMENT'])
  evidenceType!: string;

  @IsString()
  @IsOptional()
  content?: string;
}
