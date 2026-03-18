'use client';

import { ConnectButton } from '@rainbow-me/rainbowkit';

interface TopBarProps {
  title: string;
}

export default function TopBar({ title }: TopBarProps) {
  return (
    <header className="flex h-16 items-center justify-between border-b border-gray-200 bg-white px-6">
      <h1 className="text-xl font-semibold text-gray-900">{title}</h1>
      <ConnectButton
        showBalance={{ smallScreen: false, largeScreen: true }}
        chainStatus="icon"
        accountStatus="address"
      />
    </header>
  );
}
