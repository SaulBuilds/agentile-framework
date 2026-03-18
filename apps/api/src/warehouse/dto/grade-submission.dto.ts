import {
  IsString,
  IsNotEmpty,
  IsInt,
  IsOptional,
  IsIn,
  Min,
  Max,
} from 'class-validator';

export class GradeSubmissionDto {
  @IsIn(['ACCEPTED', 'REJECTED', 'QUARANTINED'])
  decision!: 'ACCEPTED' | 'REJECTED' | 'QUARANTINED';

  @IsInt()
  @Min(1)
  @Max(5)
  finalBand!: number;

  @IsString()
  @IsNotEmpty()
  conditionGrade!: string;

  @IsString()
  @IsOptional()
  binLocation?: string;

  @IsString()
  @IsOptional()
  gradingNotes?: string;

  @IsString()
  @IsOptional()
  reason?: string;

  @IsString()
  @IsOptional()
  warehouseId?: string;
}
