import { IsString, IsNotEmpty, IsIn } from 'class-validator';

export class ResolveDisputeDto {
  @IsIn(['REFUND_CLAIM', 'REPLACEMENT', 'DENY', 'GOODWILL_CREDIT'])
  resolution!: string;

  @IsString()
  @IsNotEmpty()
  reason!: string;
}
