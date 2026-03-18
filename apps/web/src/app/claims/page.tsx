'use client';

import { useState, useEffect } from 'react';
import TopBar from '@/components/TopBar';
import StatusBadge from '@/components/StatusBadge';
import { getClaims, type Claim } from '@/lib/api';

export default function ClaimsPage() {
  const [claims, setClaims] = useState<Claim[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchClaims = async () => {
      try {
        const { data } = await getClaims();
        setClaims(data);
      } catch {
        // API may not be running yet
      } finally {
        setLoading(false);
      }
    };
    fetchClaims();
  }, []);

  return (
    <>
      <TopBar title="Claims" />
      <main className="p-6">
        <div className="rounded-lg bg-white shadow-sm">
          <div className="border-b border-gray-200 px-6 py-4">
            <h2 className="text-lg font-semibold text-gray-900">Your Claims</h2>
            <p className="mt-1 text-sm text-gray-500">
              View and manage your pool claim reservations.
            </p>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-12">
              <div className="h-8 w-8 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600" />
            </div>
          ) : claims.length === 0 ? (
            <div className="px-6 py-12 text-center">
              <p className="text-sm text-gray-500">
                No claims yet. Browse pools to reserve items.
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-200 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    <th className="px-6 py-3">Claim ID</th>
                    <th className="px-6 py-3">Pool ID</th>
                    <th className="px-6 py-3">Receipt ID</th>
                    <th className="px-6 py-3">Status</th>
                    <th className="px-6 py-3">Created</th>
                    <th className="px-6 py-3">Actions</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {claims.map((claim) => (
                    <tr key={claim.id} className="hover:bg-gray-50">
                      <td className="px-6 py-4 text-sm font-medium text-gray-900">
                        {claim.id.slice(0, 8)}...
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        Pool #{claim.poolId}
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        #{claim.receiptId}
                      </td>
                      <td className="px-6 py-4">
                        <StatusBadge status={claim.status} />
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-500">
                        {new Date(claim.createdAt).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4">
                        {claim.status === 'AVAILABLE' && (
                          <button className="rounded-md bg-indigo-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-indigo-700">
                            Reserve
                          </button>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </main>
    </>
  );
}
