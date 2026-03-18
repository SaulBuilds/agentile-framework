'use client';

import { useState, useEffect } from 'react';
import TopBar from '@/components/TopBar';
import StatusBadge from '@/components/StatusBadge';
import {
  getSubmissions,
  createSubmission,
  type Submission,
} from '@/lib/api';

const CATEGORIES = [
  'Electronics',
  'Apparel',
  'Home Goods',
  'Sporting',
  'Collectibles',
  'Tools',
  'Books & Media',
  'Instruments',
];

export default function SubmissionsPage() {
  const [submissions, setSubmissions] = useState<Submission[]>([]);
  const [loading, setLoading] = useState(true);
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState('');

  // Form state
  const [category, setCategory] = useState('');
  const [conditionDescription, setConditionDescription] = useState('');
  const [estimatedBand, setEstimatedBand] = useState('');

  const fetchSubmissions = async () => {
    try {
      const { data } = await getSubmissions();
      setSubmissions(data);
    } catch {
      // API may not be running yet
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSubmissions();
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSubmitting(true);

    try {
      await createSubmission({
        category,
        conditionDescription,
        estimatedBand: Number(estimatedBand),
      });
      setCategory('');
      setConditionDescription('');
      setEstimatedBand('');
      fetchSubmissions();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create submission');
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <>
      <TopBar title="Submissions" />
      <main className="p-6">
        {/* Create Submission Form */}
        <div className="rounded-lg bg-white p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-gray-900">
            Submit an Item
          </h2>
          <p className="mt-1 text-sm text-gray-500">
            Submit an item for appraisal and inclusion in a gradient pool.
          </p>

          <form onSubmit={handleSubmit} className="mt-6 space-y-4">
            <div className="grid grid-cols-1 gap-4 sm:grid-cols-3">
              <div>
                <label
                  htmlFor="category"
                  className="block text-sm font-medium text-gray-700"
                >
                  Category
                </label>
                <select
                  id="category"
                  value={category}
                  onChange={(e) => setCategory(e.target.value)}
                  required
                  className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:outline-none"
                >
                  <option value="">Select a category</option>
                  {CATEGORIES.map((cat) => (
                    <option key={cat} value={cat}>
                      {cat}
                    </option>
                  ))}
                </select>
              </div>

              <div>
                <label
                  htmlFor="estimatedBand"
                  className="block text-sm font-medium text-gray-700"
                >
                  Estimated Band
                </label>
                <input
                  id="estimatedBand"
                  type="number"
                  min="0"
                  max="10"
                  value={estimatedBand}
                  onChange={(e) => setEstimatedBand(e.target.value)}
                  required
                  placeholder="0-10"
                  className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 text-sm shadow-sm focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 focus:outline-none"
                />
              </div>

              <div>
                <label
                  htmlFor="conditionDescription"
                  className="block text-sm font-medium text-gray-700"
                >
                  Condition Description
                </label>
                <input
                  id="conditionDescription"
                  type="text"
                  value={conditionDescription}
                  onChange={(e) => setConditionDescription(e.target.value)}
                  required
                  placeholder="Describe item condition"
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
              className="inline-flex items-center rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:outline-none disabled:opacity-50"
            >
              {submitting ? 'Submitting...' : 'Submit Item'}
            </button>
          </form>
        </div>

        {/* Submissions List */}
        <div className="mt-8 rounded-lg bg-white shadow-sm">
          <div className="border-b border-gray-200 px-6 py-4">
            <h2 className="text-lg font-semibold text-gray-900">
              Your Submissions
            </h2>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-12">
              <div className="h-8 w-8 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600" />
            </div>
          ) : submissions.length === 0 ? (
            <div className="px-6 py-12 text-center">
              <p className="text-sm text-gray-500">
                No submissions yet. Submit your first item above.
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead>
                  <tr className="border-b border-gray-200 text-left text-xs font-medium uppercase tracking-wider text-gray-500">
                    <th className="px-6 py-3">Category</th>
                    <th className="px-6 py-3">Condition</th>
                    <th className="px-6 py-3">Band</th>
                    <th className="px-6 py-3">Status</th>
                    <th className="px-6 py-3">Created</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-gray-100">
                  {submissions.map((sub) => (
                    <tr
                      key={sub.id}
                      className="hover:bg-gray-50"
                    >
                      <td className="px-6 py-4 text-sm font-medium text-gray-900">
                        {sub.category}
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        {sub.conditionDescription}
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-600">
                        {sub.estimatedBand}
                      </td>
                      <td className="px-6 py-4">
                        <StatusBadge status={sub.status} />
                      </td>
                      <td className="px-6 py-4 text-sm text-gray-500">
                        {new Date(sub.createdAt).toLocaleDateString()}
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
