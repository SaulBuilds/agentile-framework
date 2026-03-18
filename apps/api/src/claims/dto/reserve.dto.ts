import { IsString, IsNotEmpty } from 'class-validator';

export class ReserveDto {
  @IsString()
  @IsNotEmpty()
  inventoryId!: string;
}
