'use client';

import { useReadContract, useReadContracts } from 'wagmi';
import TopBar from '@/components/TopBar';
import StatusBadge from '@/components/StatusBadge';
import { POOL_REGISTRY_ADDRESS, poolRegistryAbi } from '@/lib/contracts';

const HUE_NAMES: Record<number, string> = {
  0: 'Electronics',
  1: 'Apparel',
  2: 'Home Goods',
  3: 'Sporting',
  4: 'Collectibles',
  5: 'Tools',
  6: 'Books & Media',
  7: 'Instruments',
};

const HUE_COLORS: Record<number, string> = {
  0: 'bg-blue-500',
  1: 'bg-pink-500',
  2: 'bg-amber-500',
  3: 'bg-green-500',
  4: 'bg-purple-500',
  5: 'bg-gray-500',
  6: 'bg-teal-500',
  7: 'bg-rose-500',
};

const QUALITY_TIERS: Record<number, string> = {
  0: 'Salvage',
  1: 'Fair',
  2: 'Good',
  3: 'Excellent',
  4: 'Mint',
};

export default function PoolsPage() {
  const { data: nextPoolId, isLoading: loadingCount } = useReadContract({
    address: POOL_REGISTRY_ADDRESS,
    abi: poolRegistryAbi,
    functionName: 'nextPoolId',
  });

  const poolCount = nextPoolId ? Number(nextPoolId) : 0;

  const poolContracts = Array.from({ length: poolCount }, (_, i) => ({
    address: POOL_REGISTRY_ADDRESS,
    abi: poolRegistryAbi,
    functionName: 'getPool' as const,
    args: [BigInt(i)] as const,
  }));

  const { data: poolResults, isLoading: loadingPools } = useReadContracts({
    contracts: poolContracts,
  });

  const isLoading = loadingCount || loadingPools;

  return (
    <>
      <TopBar title="Pool Explorer" />
      <main className="p-6">
        <div className="mb-6 flex items-center justify-between">
          <p className="text-sm text-gray-500">
            {poolCount} pool{poolCount !== 1 ? 's' : ''} registered on-chain
          </p>
        </div>

        {isLoading ? (
          <div className="flex items-center justify-center py-20">
            <div className="h-8 w-8 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600" />
          </div>
        ) : poolCount === 0 ? (
          <div className="rounded-lg bg-white p-12 text-center shadow-sm">
            <p className="text-gray-500">No pools have been created yet.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {poolResults?.map((result, i) => {
              if (result.status !== 'success') return null;
              const pool = result.result;
              const hue = Number(pool.hue);
              const band = Number(pool.band);
              const qualityTier = Number(pool.qualityTier);

              return (
                <div
                  key={i}
                  className="overflow-hidden rounded-lg bg-white shadow-sm"
                >
                  {/* Color bar */}
                  <div className={`h-2 ${HUE_COLORS[hue] ?? 'bg-indigo-500'}`} />

                  <div className="p-6">
                    <div className="flex items-start justify-between">
                      <div>
                        <p className="text-xs font-medium text-gray-400">
                          Pool #{i}
                        </p>
                        <h3 className="mt-1 text-lg font-semibold text-gray-900">
                          {HUE_NAMES[hue] ?? `Hue ${hue}`}
                        </h3>
                      </div>
                      <StatusBadge
                        status={
                          pool.paused
                            ? 'QUARANTINED'
                            : pool.active
                              ? 'ACTIVE'
                              : 'PENDING'
                        }
                      />
                    </div>

                    <div className="mt-4 space-y-2">
                      <Detail label="Band" value={band.toString()} />
                      <Detail label="Region" value={pool.region || 'Global'} />
                      <Detail
                        label="Quality"
                        value={QUALITY_TIERS[qualityTier] ?? `Tier ${qualityTier}`}
                      />
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </main>
    </>
  );
}

function Detail({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex items-center justify-between text-sm">
      <span className="text-gray-500">{label}</span>
      <span className="font-medium text-gray-900">{value}</span>
    </div>
  );
}
