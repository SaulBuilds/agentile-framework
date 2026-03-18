import { Test, TestingModule } from '@nestjs/testing';
import {
  ConflictException,
  NotFoundException,
  BadRequestException,
} from '@nestjs/common';
import { WalletService } from './wallet.service';
import { PrismaService } from '../prisma/prisma.service';

describe('WalletService', () => {
  let service: WalletService;
  let prisma: {
    wallet: {
      findUnique: jest.Mock;
      findMany: jest.Mock;
      create: jest.Mock;
      update: jest.Mock;
    };
  };

  beforeEach(async () => {
    prisma = {
      wallet: {
        findUnique: jest.fn(),
        findMany: jest.fn(),
        create: jest.fn(),
        update: jest.fn(),
      },
    };

    const module: TestingModule = await Test.createTestingModule({
      providers: [
        WalletService,
        { provide: PrismaService, useValue: prisma },
      ],
    }).compile();

    service = module.get<WalletService>(WalletService);
  });

  describe('linkWallet', () => {
    const userId = 'user-1';
    const dto = {
      address: '0x1234567890abcdef1234567890abcdef12345678',
      chainId: 1,
    };

    it('should link a new wallet and return nonce', async () => {
      prisma.wallet.findUnique.mockResolvedValue(null);
      prisma.wallet.create.mockResolvedValue({
        id: 'wallet-1',
        nonce: 'some-nonce',
      });

      const result = await service.linkWallet(userId, dto);

      expect(result.walletId).toBe('wallet-1');
      expect(result.nonce).toBeDefined();
      expect(prisma.wallet.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          userId,
          address: dto.address.toLowerCase(),
          chainId: 1,
        }),
      });
    });

    it('should throw ConflictException on duplicate wallet', async () => {
      prisma.wallet.findUnique.mockResolvedValue({ id: 'existing' });

      await expect(service.linkWallet(userId, dto)).rejects.toThrow(
        ConflictException,
      );
    });
  });

  describe('verifyWallet', () => {
    const userId = 'user-1';
    const dto = {
      address: '0x1234567890abcdef1234567890abcdef12345678',
      signature: '0xdeadbeef',
    };

    it('should throw NotFoundException for unknown wallet', async () => {
      prisma.wallet.findUnique.mockResolvedValue(null);

      await expect(service.verifyWallet(userId, dto)).rejects.toThrow(
        NotFoundException,
      );
    });

    it('should throw NotFoundException for wallet owned by another user', async () => {
      prisma.wallet.findUnique.mockResolvedValue({
        id: 'wallet-1',
        userId: 'other-user',
        verified: false,
        nonce: 'nonce',
      });

      await expect(service.verifyWallet(userId, dto)).rejects.toThrow(
        NotFoundException,
      );
    });

    it('should throw BadRequestException for already verified wallet', async () => {
      prisma.wallet.findUnique.mockResolvedValue({
        id: 'wallet-1',
        userId,
        verified: true,
        nonce: null,
      });

      await expect(service.verifyWallet(userId, dto)).rejects.toThrow(
        BadRequestException,
      );
    });

    it('should throw BadRequestException when no nonce available', async () => {
      prisma.wallet.findUnique.mockResolvedValue({
        id: 'wallet-1',
        userId,
        verified: false,
        nonce: null,
      });

      await expect(service.verifyWallet(userId, dto)).rejects.toThrow(
        BadRequestException,
      );
    });
  });

  describe('getUserWallets', () => {
    it('should return user wallets', async () => {
      const wallets = [
        {
          id: 'w-1',
          address: '0xabc',
          chainId: 1,
          verified: true,
          linkedAt: new Date(),
        },
      ];
      prisma.wallet.findMany.mockResolvedValue(wallets);

      const result = await service.getUserWallets('user-1');
      expect(result).toEqual(wallets);
      expect(prisma.wallet.findMany).toHaveBeenCalledWith({
        where: { userId: 'user-1' },
        select: {
          id: true,
          address: true,
          chainId: true,
          verified: true,
          linkedAt: true,
        },
      });
    });
  });
});
