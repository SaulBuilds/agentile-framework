import {
  IsString,
  IsOptional,
  IsNumber,
  IsNotEmpty,
  ValidateNested,
} from 'class-validator';
import { Type } from 'class-transformer';

class LocationDto {
  @IsNumber()
  lat!: number;

  @IsNumber()
  lng!: number;
}

export class PickupDto {
  @IsString()
  @IsOptional()
  qrCode?: string;

  @IsString()
  @IsOptional()
  pin?: string;

  @IsString()
  @IsOptional()
  photoProof?: string;
}

export class MilestoneDto {
  @ValidateNested()
  @Type(() => LocationDto)
  location!: LocationDto;

  @IsString()
  @IsOptional()
  notes?: string;
}

export class DeliverDto {
  @IsString()
  @IsNotEmpty()
  proofMethod!: string;

  @IsString()
  @IsNotEmpty()
  proofData!: string;
}

export class EmergencyReportDto {
  @IsString()
  @IsNotEmpty()
  type!: string;

  @IsString()
  @IsNotEmpty()
  description!: string;

  @IsOptional()
  @ValidateNested()
  @Type(() => LocationDto)
  location?: LocationDto;
}
