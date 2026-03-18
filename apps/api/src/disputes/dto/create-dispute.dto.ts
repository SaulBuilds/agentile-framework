import { IsString, IsNotEmpty, IsIn } from 'class-validator';

export class CreateDisputeDto {
  @IsIn([
    'SUBMISSION',
    'RECEIPT',
    'INVENTORY',
    'CLAIM',
    'SHIPMENT',
    'COURIER_TASK',
  ])
  objectType!: string;

  @IsString()
  @IsNotEmpty()
  objectId!: string;

  @IsString()
  @IsNotEmpty()
  reason!: string;
}
