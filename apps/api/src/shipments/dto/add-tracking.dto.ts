import { IsString, IsNotEmpty, IsOptional, IsDateString } from 'class-validator';

export class AddTrackingDto {
  @IsString()
  @IsNotEmpty()
  eventType!: string;

  @IsString()
  @IsOptional()
  location?: string;

  @IsDateString()
  @IsOptional()
  timestamp?: string;
}
