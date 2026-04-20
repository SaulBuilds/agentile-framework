"use client";
import { useEffect, useState } from "react";
import { callTool, type ApiResponse } from "@/lib/api";

interface Props {
  tool: string;
  title: string;
  renderItem?: (item: Record<string, unknown>, i: number) => React.ReactNode;
}

export default function ListPanel({ tool, title, renderItem }: Props) {
  const [result, setResult] = useState<ApiResponse | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    callTool(tool).then((r) => { setResult(r); setLoading(false); });
  }, [tool]);

  const items = Array.isArray(result?.data) ? result.data : [];

  return (
    <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-950">
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-sm font-semibold text-white">{title}</h3>
        <button
          onClick={() => { setLoading(true); callTool(tool).then((r) => { setResult(r); setLoading(false); }); }}
          className="text-xs text-zinc-400 hover:text-white"
        >
          refresh
        </button>
      </div>
      {loading && <p className="text-xs text-zinc-500">Loading...</p>}
      {result?.error && <p className="text-xs text-red-400">{result.error}</p>}
      {items.length === 0 && !loading && <p className="text-xs text-zinc-500">No items.</p>}
      <div className="flex flex-col gap-2">
        {items.map((item: Record<string, unknown>, i: number) =>
          renderItem ? (
            <div key={i}>{renderItem(item, i)}</div>
          ) : (
            <pre key={i} className="bg-zinc-900 border border-zinc-800 rounded p-2 text-xs text-zinc-300 overflow-auto">
              {JSON.stringify(item, null, 2)}
            </pre>
          )
        )}
      </div>
    </div>
  );
}
