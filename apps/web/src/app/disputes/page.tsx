'use client';

import { useState, useEffect } from 'react';
import TopBar from '@/components/TopBar';
import StatusBadge from '@/components/StatusBadge';
import { createDispute, getDisputes, type Dispute } from '@/lib/api';

const OBJECT_TYPES = [
  'SUBMISSION',
  'CLAIM',
  'SHIPMENT',
  'RESERVATION',
  'INVENTORY',
];

export default function DisputesPage() {
  const [disputes, setDisputes] = useState<Dispute[]>([]);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState('');

  // Form state
  const [objectType, setObjectType] = useState('');
  const [objectId, setObjectId] = useState('');
  const [reason, setReason] = useState('');

  const fetchDisputes = async () => {
    try {
      const { data } = await getDisputes();
      setDisputes(data);
    } catch {
      // API may not be running yet
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDisputes();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSubmitting(true);

    try {
      await createDispute({ objectType, objectId, reason });
      setObjectType('');
      setObjectId('');
      setReason('');
      fetchDisputes();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create dispute');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <>
      <TopBar title="Disputes" />
      <main className="p-6">
        {/* Open Dispute Form */}
        <div className="rounded-lg bg-white p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-gray-900">
            Open a Dispute
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Report an issue with a submission, claim, shipment, or reservation.
          </p>

          <form onSubmit={handleSubmit} className="mt-6 space-y-4">
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div>
                <label
                  htmlFor="objectType"
                  className="block text-sm font-medium text-gray-700"
                >
                  Object Type
                </label>
                <select
                  id="objectType"
                  value={objectType}
                  onChange={(e) => setObjectType(e.target.value)}
                  required
                  className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:outline-none"
                >
                  <option value="">Select type</option>
                  {OBJECT_TYPES.map((type) => (
                    <option key={type} value={type}>
                      {type}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label
                  htmlFor="objectId"
                  className="block text-sm font-medium text-gray-700"
                >
                  Object ID
                </label>
                <input
                  id="objectId"
                  type="text"
                  value={objectId}
                  onChange={(e) => setObjectId(e.target.value)}
                  required
                  placeholder="Enter the object ID"
                  className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:outline-none"
                />
              </div>

              <div>
                <label
                  htmlFor="reason"
                  className="block text-sm font-medium text-gray-700"
                >
                  Reason
                </label>
                <input
                  id="reason"
                  type="text"
                  value={reason}
                  onChange={(e) => setReason(e.target.value)}
                  required
                  placeholder="Describe the issue"
                  className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:outline-none"
                />
              </div>
            </div>

            {error && (
              <p className="text-sm text-red-600">{error}</p>
            )}

            <button
              type="submit"
              disabled={submitting}
              className="inline-flex items-center rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
            >
              {submitting ? 'Submitting...' : 'Open Dispute'}
            </button>
          </form>
        </div>

        {/* Disputes List */}
        <div className="mt-8 rounded-lg bg-white shadow-sm">
          <div className="border-b border-gray-200 px-6 py-4">
            <h2 className="text-lg font-semibold text-gray-900">
              Your Disputes
            </h2>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-12">
              <div className="h-8 w-8 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600" />
            </div>
          ) : disputes.length === 0 ? (
            <div className="px-6 py-12 text-center">
              <p className="text-sm text-gray-500">
                No disputes filed.
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-200 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    <th className="px-6 py-3">ID</th>
                    <th className="px-6 py-3">Type</th>
                    <th className="px-6 py-3">Object ID</th>
                    <th className="px-6 py-3">Reason</th>
                    <th className="px-6 py-3">Status</th>
                    <th className="px-6 py-3">Created</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {disputes.map((dispute) => (
                    <tr key={dispute.id} className="hover:bg-gray-50">
                      <td className="px-6 py-4 text-sm font-medium text-gray-900">
                        {dispute.id.slice(0, 8)}...
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        {dispute.objectType}
                      </td>
                      <td className="px-6 py-4 text-sm font-mono text-gray-600">
                        {dispute.objectId.slice(0, 8)}...
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        {dispute.reason}
                      </td>
                      <td className="px-6 py-4">
                        <StatusBadge status={dispute.status} />
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-500">
                        {new Date(dispute.createdAt).toLocaleDateString()}
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
