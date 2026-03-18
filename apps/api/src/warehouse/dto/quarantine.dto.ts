import { IsString, IsNotEmpty, IsIn, IsOptional } from 'class-validator';

export class QuarantineDto {
  @IsString()
  @IsNotEmpty()
  reason!: string;
}

export class ResolveQuarantineDto {
  @IsIn(['RELEASE', 'REJECT'])
  resolution!: 'RELEASE' | 'REJECT';

  @IsString()
  @IsOptional()
  notes?: string;

  @IsString()
  @IsOptional()
  binLocation?: string;
}
