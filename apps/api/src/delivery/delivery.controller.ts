import {
  Controller,
  Post,
  Body,
  UseGuards,
  HttpCode,
} from '@nestjs/common';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { DeliveryService } from './delivery.service';
import { IsString, IsNumber, IsOptional, ValidateNested } from 'class-validator';
import { Type } from 'class-transformer';

class DimensionsDto {
  @IsNumber()
  length!: number;

  @IsNumber()
  width!: number;

  @IsNumber()
  height!: number;
}

export class GetRatesDto {
  @IsString()
  originZip!: string;

  @IsString()
  destinationZip!: string;

  @IsNumber()
  weight!: number;

  @IsOptional()
  @ValidateNested()
  @Type(() => DimensionsDto)
  dimensions?: DimensionsDto;
}

@Controller('shipping')
export class DeliveryController {
  constructor(private readonly deliveryService: DeliveryService) {}

  @Post('rates')
  @UseGuards(JwtAuthGuard)
  @HttpCode(200)
  async getRates(@Body() dto: GetRatesDto) {
    const rates = await this.deliveryService.getRatesFromAll({
      originZip: dto.originZip,
      destinationZip: dto.destinationZip,
      weight: dto.weight,
      dimensions: dto.dimensions,
    });

    return { rates };
  }
}
