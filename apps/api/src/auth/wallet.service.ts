import {
  Injectable,
  ConflictException,
  NotFoundException,
  BadRequestException,
} from '@nestjs/common';
import { randomBytes } from 'crypto';
import { PrismaService } from '../prisma/prisma.service';
import { WalletLinkDto, WalletVerifyDto } from './dto/wallet-link.dto';

@Injectable()
export class WalletService {
  constructor(private readonly prisma: PrismaService) {}

  async linkWallet(userId: string, dto: WalletLinkDto) {
    // Check for duplicate wallet
    const existing = await this.prisma.wallet.findUnique({
      where: { address: dto.address.toLowerCase() },
    });
    if (existing) {
      throw new ConflictException('Wallet already linked');
    }

    const nonce = randomBytes(32).toString('hex');

    const wallet = await this.prisma.wallet.create({
      data: {
        userId,
        address: dto.address.toLowerCase(),
        chainId: dto.chainId,
        nonce,
      },
    });

    return { walletId: wallet.id, nonce };
  }

  async verifyWallet(userId: string, dto: WalletVerifyDto) {
    const wallet = await this.prisma.wallet.findUnique({
      where: { address: dto.address.toLowerCase() },
    });

    if (!wallet || wallet.userId !== userId) {
      throw new NotFoundException('Wallet not found');
    }

    if (wallet.verified) {
      throw new BadRequestException('Wallet already verified');
    }

    if (!wallet.nonce) {
      throw new BadRequestException('No nonce available — re-link wallet');
    }

    // Verify EIP-191 signature using viem
    const isValid = await this.verifySignature(
      dto.address.toLowerCase(),
      wallet.nonce,
      dto.signature,
    );

    if (!isValid) {
      throw new BadRequestException('Invalid signature');
    }

    await this.prisma.wallet.update({
      where: { id: wallet.id },
      data: { verified: true, nonce: null },
    });

    return { verified: true };
  }

  private async verifySignature(
    address: string,
    nonce: string,
    signature: string,
  ): Promise<boolean> {
    try {
      const { verifyMessage } = await import('viem');
      const message = `Sign this message to verify wallet ownership.\n\nNonce: ${nonce}`;

      return await verifyMessage({
        address: address as `0x${string}`,
        message,
        signature: signature as `0x${string}`,
      });
    } catch {
      return false;
    }
  }

  async getUserWallets(userId: string) {
    return this.prisma.wallet.findMany({
      where: { userId },
      select: {
        id: true,
        address: true,
        chainId: true,
        verified: true,
        linkedAt: true,
      },
    });
  }
}
