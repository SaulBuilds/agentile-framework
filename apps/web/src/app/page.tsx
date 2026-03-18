'use client';

import { useAccount, useBalance } from 'wagmi';
import { useReadContract } from 'wagmi';
import { formatEther } from 'viem';
import TopBar from '@/components/TopBar';
import StatCard from '@/components/StatCard';
import { POOL_REGISTRY_ADDRESS, poolRegistryAbi } from '@/lib/contracts';

export default function DashboardPage() {
  const { address, isConnected, chain } = useAccount();

  const { data: balanceData } = useBalance({
    address,
  });

  const { data: nextPoolId } = useReadContract({
    address: POOL_REGISTRY_ADDRESS,
    abi: poolRegistryAbi,
    functionName: 'nextPoolId',
  });

  const poolCount = nextPoolId ? Number(nextPoolId) : 0;
  const saltBalance = balanceData ? formatEther(balanceData.value) : '0';

  return (
    <>
      <TopBar title="Dashboard" />
      <main className="p-6">
        {/* Stats Grid */}
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-4">
          <StatCard
            label="Connected Wallet"
            value={
              isConnected && address
                ? `${address.slice(0, 6)}...${address.slice(-4)}`
                : 'Not connected'
            }
          />
          <StatCard
            label="Chain ID"
            value={chain?.id ?? '--'}
          />
          <StatCard
            label="SALT Balance"
            value={
              isConnected
                ? `${Number(saltBalance).toFixed(4)} SALT`
                : '--'
            }
          />
          <StatCard
            label="Pool Count"
            value={poolCount}
          />
        </div>

        {/* Getting Started */}
        <section className="mt-8">
          <div className="rounded-lg bg-white p-6 shadow-sm">
            <h2 className="text-lg font-semibold text-gray-900">Getting Started</h2>
            <p className="mt-2 text-sm text-gray-600">
              Welcome to the Gradient Barter Protocol dashboard. Connect your wallet to begin
              interacting with the warehouse-backed barter network on Citrea.
            </p>
            <div className="mt-6 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
              <StepCard
                step={1}
                title="Connect Wallet"
                description="Connect your wallet using the button in the top-right corner to interact with the Citrea chain."
              />
              <StepCard
                step={2}
                title="Submit Items"
                description="Submit items for appraisal by navigating to the Submissions page and filling out the form."
              />
              <StepCard
                step={3}
                title="Explore Pools"
                description="Browse available gradient pools to see item categories, quality tiers, and active listings."
              />
              <StepCard
                step={4}
                title="Claim Items"
                description="Use your pool claim tokens to reserve items from inventory in the Claims section."
              />
              <StepCard
                step={5}
                title="Track Shipments"
                description="Monitor inbound and outbound shipments with real-time tracking status."
              />
              <StepCard
                step={6}
                title="Resolve Disputes"
                description="If issues arise, open a dispute and track resolution through the Disputes page."
              />
            </div>
          </div>
        </section>
      </main>
    </>
  );
}

function StepCard({
  step,
  title,
  description,
}: {
  step: number;
  title: string;
  description: string;
}) {
  return (
    <div className="rounded-lg border border-gray-200 p-4">
      <div className="mb-2 flex h-8 w-8 items-center justify-center rounded-full bg-indigo-100 text-sm font-semibold text-indigo-600">
        {step}
      </div>
      <h3 className="text-sm font-semibold text-gray-900">{title}</h3>
      <p className="mt-1 text-xs text-gray-500">{description}</p>
    </div>
  );
}
