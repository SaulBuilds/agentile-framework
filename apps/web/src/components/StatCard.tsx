interface StatCardProps {
  label: string;
  value: string | number;
  delta?: string;
}

export default function StatCard({ label, value, delta }: StatCardProps) {
  return (
    <div className="rounded-lg bg-white p-6 shadow-sm">
      <p className="text-sm font-medium text-gray-500">{label}</p>
      <p className="mt-2 text-3xl font-semibold text-gray-900">{value}</p>
      {delta && (
        <p
          className={`mt-1 text-sm ${
            delta.startsWith('+')
              ? 'text-green-600'
              : delta.startsWith('-')
                ? 'text-red-600'
                : 'text-gray-500'
          }`}
        >
          {delta}
        </p>
      )}
    </div>
  );
}
