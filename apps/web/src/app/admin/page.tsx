'use client';

import { useState, useEffect } from 'react';
import TopBar from '@/components/TopBar';
import StatusBadge from '@/components/StatusBadge';
import { getAdminDisputeQueue, type Dispute } from '@/lib/api';

export default function AdminPage() {
  const [disputes, setDisputes] = useState<Dispute[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchQueue = async () => {
    try {
      const { data } = await getAdminDisputeQueue();
      // Sort by SLA deadline, soonest first
      const sorted = data.sort(
        (a, b) =>
          new Date(a.slaDeadline).getTime() - new Date(b.slaDeadline).getTime(),
      );
      setDisputes(sorted);
    } catch {
      // API may not be running or user lacks ADMIN role
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchQueue();
  }, []);

  const getTimeRemaining = (deadline: string) => {
    const diff = new Date(deadline).getTime() - Date.now();
    if (diff < 0) return 'Overdue';
    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    if (hours > 24) {
      const days = Math.floor(hours / 24);
      return `${days}d ${hours % 24}h`;
    }
    return `${hours}h ${minutes}m`;
  };

  const getUrgencyColor = (deadline: string) => {
    const diff = new Date(deadline).getTime() - Date.now();
    if (diff < 0) return 'text-red-600 bg-red-50';
    if (diff < 4 * 60 * 60 * 1000) return 'text-orange-600 bg-orange-50';
    if (diff < 24 * 60 * 60 * 1000) return 'text-yellow-600 bg-yellow-50';
    return 'text-green-600 bg-green-50';
  };

  return (
    <>
      <TopBar title="Admin - Dispute Queue" />
      <main className="p-6">
        <div className="mb-6">
          <p className="text-sm text-gray-500">
            Manage disputes sorted by SLA urgency. Resolve disputes before their deadlines.
          </p>
        </div>

        <div className="rounded-lg bg-white shadow-sm">
          <div className="border-b border-gray-200 px-6 py-4">
            <h2 className="text-lg font-semibold text-gray-900">
              Dispute Queue
            </h2>
            <p className="mt-1 text-sm text-gray-500">
              {disputes.length} dispute{disputes.length !== 1 ? 's' : ''} requiring attention
            </p>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-12">
              <div className="h-8 w-8 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600" />
            </div>
          ) : disputes.length === 0 ? (
            <div className="px-6 py-12 text-center">
              <p className="text-sm text-gray-500">
                No disputes in the queue. All clear.
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-200 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    <th className="px-6 py-3">SLA</th>
                    <th className="px-6 py-3">ID</th>
                    <th className="px-6 py-3">Type</th>
                    <th className="px-6 py-3">Object ID</th>
                    <th className="px-6 py-3">Reason</th>
                    <th className="px-6 py-3">Status</th>
                    <th className="px-6 py-3">Actions</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {disputes.map((dispute) => (
                    <tr key={dispute.id} className="hover:bg-gray-50">
                      <td className="px-6 py-4">
                        <span
                          className={`inline-flex items-center rounded-full px-2.5 py-1 text-xs font-semibold ${getUrgencyColor(dispute.slaDeadline)}`}
                        >
                          {getTimeRemaining(dispute.slaDeadline)}
                        </span>
                      </td>
                      <td className="px-6 py-4 text-sm font-medium text-gray-900">
                        {dispute.id.slice(0, 8)}...
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        {dispute.objectType}
                      </td>
                      <td className="px-6 py-4 text-sm font-mono text-gray-600">
                        {dispute.objectId.slice(0, 8)}...
                      </td>
                      <td className="max-w-xs truncate px-6 py-4 text-sm text-gray-600">
                        {dispute.reason}
                      </td>
                      <td className="px-6 py-4">
                        <StatusBadge status={dispute.status} />
                      </td>
                      <td className="px-6 py-4">
                        <div className="flex gap-2">
                          <button className="rounded-md bg-green-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-green-700">
                            Approve
                          </button>
                          <button className="rounded-md bg-red-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-red-700">
                            Deny
                          </button>
                        </div>
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
