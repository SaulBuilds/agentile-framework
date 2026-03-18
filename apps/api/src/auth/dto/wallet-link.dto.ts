import { IsInt, IsString, Matches, Min } from 'class-validator';

export class WalletLinkDto {
  @IsString()
  @Matches(/^0x[a-fA-F0-9]{40}$/, { message: 'Invalid EVM address' })
  address!: string;

  @IsInt()
  @Min(1)
  chainId!: number;
}

export class WalletVerifyDto {
  @IsString()
  @Matches(/^0x[a-fA-F0-9]{40}$/, { message: 'Invalid EVM address' })
  address!: string;

  @IsString()
  signature!: string;
}
