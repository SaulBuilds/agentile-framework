import { getDefaultConfig } from '@rainbow-me/rainbowkit';
import { defineChain } from 'viem';
import { QueryClient } from '@tanstack/react-query';

export const citrea = defineChain({
  id: 40204,
  name: 'Citrea',
  nativeCurrency: {
    name: 'Salt',
    symbol: 'SALT',
    decimals: 18,
  },
  rpcUrls: {
    default: {
      http: ['https://soleils-mac-studio.tailcbe2ba.ts.net/'],
    },
  },
});

export const config = getDefaultConfig({
  appName: 'Gradient Barter Protocol',
  projectId: 'gradient-barter-protocol',
  chains: [citrea],
  ssr: true,
});

export const queryClient = new QueryClient();
