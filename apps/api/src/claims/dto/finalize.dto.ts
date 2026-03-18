import { IsString, IsNotEmpty, IsOptional, IsObject } from 'class-validator';

export class FinalizeDto {
  @IsString()
  @IsNotEmpty()
  deliveryMethod!: string;

  @IsObject()
  @IsOptional()
  shippingAddress?: {
    street: string;
    city: string;
    state: string;
    zip: string;
    country: string;
  };
}
