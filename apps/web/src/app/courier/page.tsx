'use client';

import { useState, useEffect } from 'react';
import TopBar from '@/components/TopBar';
import StatusBadge from '@/components/StatusBadge';
import { getCourierTasks, acceptCourierTask, type CourierTask } from '@/lib/api';

export default function CourierPage() {
  const [tasks, setTasks] = useState<CourierTask[]>([]);
  const [loading, setLoading] = useState(true);
  const [acceptingId, setAcceptingId] = useState<string | null>(null);

  const fetchTasks = async () => {
    try {
      const { data } = await getCourierTasks();
      setTasks(data);
    } catch {
      // API may not be running yet
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTasks();
  }, []);

  const handleAccept = async (taskId: string) => {
    setAcceptingId(taskId);
    try {
      await acceptCourierTask(taskId);
      fetchTasks();
    } catch {
      // handle error
    } finally {
      setAcceptingId(null);
    }
  };

  return (
    <>
      <TopBar title="Courier Tasks" />
      <main className="p-6">
        <div className="mb-6">
          <p className="text-sm text-gray-500">
            Available courier pickup and delivery tasks. Accept a task to begin fulfillment.
          </p>
        </div>

        {loading ? (
          <div className="flex items-center justify-center py-20">
            <div className="h-8 w-8 animate-spin rounded-full border-4 border-indigo-200 border-t-indigo-600" />
          </div>
        ) : tasks.length === 0 ? (
          <div className="rounded-lg bg-white p-12 text-center shadow-sm">
            <p className="text-sm text-gray-500">
              No courier tasks available at the moment.
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            {tasks.map((task) => (
              <div
                key={task.id}
                className="rounded-lg bg-white p-6 shadow-sm"
              >
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-2">
                    <svg
                      className="h-5 w-5 text-gray-400"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                      strokeWidth={1.5}
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        d="M15 10.5a3 3 0 11-6 0 3 3 0 016 0z"
                      />
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        d="M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1115 0z"
                      />
                    </svg>
                    <span className="text-sm font-medium text-gray-900">
                      {task.pickupCity}
                    </span>
                  </div>
                  <StatusBadge status={task.status} />
                </div>

                <div className="mt-3 flex items-center gap-2 text-sm text-gray-500">
                  <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
                  </svg>
                  <span className="font-medium text-gray-900">
                    {task.dropoffCity}
                  </span>
                </div>

                <div className="mt-4 flex items-center justify-between border-t border-gray-100 pt-4">
                  <div>
                    <p className="text-xs text-gray-500">Fee</p>
                    <p className="text-sm font-semibold text-gray-900">
                      {task.fee} SALT
                    </p>
                  </div>
                  <div className="text-right">
                    <p className="text-xs text-gray-500">Time Window</p>
                    <p className="text-sm text-gray-700">{task.timeWindow}</p>
                  </div>
                </div>

                {task.status === 'AVAILABLE' && (
                  <button
                    onClick={() => handleAccept(task.id)}
                    disabled={acceptingId === task.id}
                    className="mt-4 w-full rounded-md bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:opacity-50"
                  >
                    {acceptingId === task.id ? 'Accepting...' : 'Accept Task'}
                  </button>
                )}
              </div>
            ))}
          </div>
        )}
      </main>
    </>
  );
}
