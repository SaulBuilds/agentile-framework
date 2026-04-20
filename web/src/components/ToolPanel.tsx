"use client";
import { useState } from "react";
import { callTool, type ApiResponse } from "@/lib/api";

interface Props {
  tool: string;
  title: string;
  fields: { name: string; label: string; type?: string; placeholder?: string; defaultValue?: string }[];
}

export default function ToolPanel({ tool, title, fields }: Props) {
  const [values, setValues] = useState<Record<string, string>>(() =>
    Object.fromEntries(fields.map((f) => [f.name, f.defaultValue || ""]))
  );
  const [result, setResult] = useState<ApiResponse | null>(null);
  const [loading, setLoading] = useState(false);

  const run = async () => {
    setLoading(true);
    setResult(null);
    const params: Record<string, unknown> = {};
    for (const f of fields) {
      const v = values[f.name];
      if (!v) continue;
      if (f.type === "number") params[f.name] = Number(v);
      else if (f.type === "json") {
        try { params[f.name] = JSON.parse(v); } catch { params[f.name] = v; }
      } else params[f.name] = v;
    }
    const res = await callTool(tool, params);
    setResult(res);
    setLoading(false);
  };

  return (
    <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-950">
      <h3 className="text-sm font-semibold text-white mb-3">{title}</h3>
      <div className="flex flex-wrap gap-2 mb-3">
        {fields.map((f) => (
          <input
            key={f.name}
            placeholder={f.placeholder || f.label}
            value={values[f.name]}
            onChange={(e) => setValues({ ...values, [f.name]: e.target.value })}
            className="bg-zinc-900 border border-zinc-700 rounded px-2 py-1 text-sm text-white placeholder-zinc-500 flex-1 min-w-[140px]"
          />
        ))}
      </div>
      <button
        onClick={run}
        disabled={loading}
        className="bg-blue-600 hover:bg-blue-700 disabled:bg-zinc-700 text-white text-sm px-3 py-1.5 rounded"
      >
        {loading ? "Running..." : `Run ${tool}`}
      </button>
      {result && (
        <pre className="mt-3 bg-zinc-900 border border-zinc-800 rounded p-3 text-xs text-zinc-300 overflow-auto max-h-96">
          {JSON.stringify(result, null, 2)}
        </pre>
      )}
    </div>
  );
}
