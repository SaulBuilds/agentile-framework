import { IsString, IsNotEmpty } from 'class-validator';

export class ScanIntakeDto {
  @IsString()
  @IsNotEmpty()
  code!: string;
}
