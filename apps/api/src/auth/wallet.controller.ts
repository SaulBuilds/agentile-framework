import { Controller, Post, Get, Body, UseGuards, Request } from '@nestjs/common';
import { WalletService } from './wallet.service';
import { WalletLinkDto, WalletVerifyDto } from './dto/wallet-link.dto';
import { JwtAuthGuard } from './guards/jwt-auth.guard';

@Controller('auth/wallet')
@UseGuards(JwtAuthGuard)
export class WalletController {
  constructor(private readonly walletService: WalletService) {}

  @Post('link')
  async linkWallet(
    @Request() req: { user: { userId: string } },
    @Body() dto: WalletLinkDto,
  ) {
    return this.walletService.linkWallet(req.user.userId, dto);
  }

  @Post('verify')
  async verifyWallet(
    @Request() req: { user: { userId: string } },
    @Body() dto: WalletVerifyDto,
  ) {
    return this.walletService.verifyWallet(req.user.userId, dto);
  }

  @Get()
  async listWallets(@Request() req: { user: { userId: string } }) {
    return this.walletService.getUserWallets(req.user.userId);
  }
}
