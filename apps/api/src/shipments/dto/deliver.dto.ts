import { IsString, IsNotEmpty, IsIn, IsOptional } from 'class-validator';

export class DeliverDto {
  @IsIn(['qr', 'signature', 'photo', 'pin'])
  proofMethod!: 'qr' | 'signature' | 'photo' | 'pin';

  @IsString()
  @IsNotEmpty()
  proofData!: string;

  @IsString()
  @IsOptional()
  notes?: string;
}
