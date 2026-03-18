const statusColors: Record<string, string> = {
  ACTIVE: 'bg-green-100 text-green-800',
  AVAILABLE: 'bg-green-100 text-green-800',
  OPEN: 'bg-green-100 text-green-800',

  PENDING: 'bg-yellow-100 text-yellow-800',
  DRAFT: 'bg-yellow-100 text-yellow-800',
  POSTED: 'bg-yellow-100 text-yellow-800',

  RESERVED: 'bg-blue-100 text-blue-800',
  ACCEPTED: 'bg-blue-100 text-blue-800',
  IN_TRANSIT: 'bg-blue-100 text-blue-800',

  DELIVERED: 'bg-purple-100 text-purple-800',
  COMPLETED: 'bg-purple-100 text-purple-800',
  RESOLVED: 'bg-purple-100 text-purple-800',

  REJECTED: 'bg-red-100 text-red-800',
  FAILED: 'bg-red-100 text-red-800',
  DENIED: 'bg-red-100 text-red-800',
  EXPIRED: 'bg-red-100 text-red-800',

  QUARANTINED: 'bg-orange-100 text-orange-800',
};

interface StatusBadgeProps {
  status: string;
}

export default function StatusBadge({ status }: StatusBadgeProps) {
  const normalized = status.toUpperCase().replace(/ /g, '_');
  const color = statusColors[normalized] ?? 'bg-gray-100 text-gray-800';

  return (
    <span
      className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${color}`}
    >
      {status}
    </span>
  );
}
