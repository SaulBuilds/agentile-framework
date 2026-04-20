"use client";
import { useEffect, useState } from "react";
import { getHealth, getTools, type ApiResponse, type ToolInfo } from "@/lib/api";

export default function Dashboard() {
  const [health, setHealth] = useState<ApiResponse | null>(null);
  const [tools, setTools] = useState<ToolInfo[]>([]);

  useEffect(() => {
    getHealth().then(setHealth).catch(() => setHealth({ success: false, error: "Cannot reach API" }));
    getTools().then(setTools).catch(() => {});
  }, []);

  const categories = [...new Set(tools.map((t) => t.category))];

  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Dashboard</h1>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-950">
          <div className="text-xs text-zinc-500 mb-1">API Status</div>
          <div className={`text-lg font-bold ${health?.success ? "text-green-400" : "text-red-400"}`}>
            {health?.success ? "Connected" : health?.error || "Checking..."}
          </div>
          {health?.success ? (
            <div className="text-xs text-zinc-500 mt-1">
              v{String((health.data as Record<string, string>)?.version ?? "")}
            </div>
          ) : null}
        </div>
        <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-950">
          <div className="text-xs text-zinc-500 mb-1">Available Tools</div>
          <div className="text-lg font-bold text-white">{tools.length}</div>
        </div>
        <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-950">
          <div className="text-xs text-zinc-500 mb-1">Categories</div>
          <div className="text-lg font-bold text-white">{categories.length}</div>
        </div>
      </div>

      <h2 className="text-lg font-semibold mb-3">Tools by Category</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {categories.map((cat) => (
          <div key={cat} className="border border-zinc-800 rounded-lg p-4 bg-zinc-950">
            <h3 className="text-sm font-semibold text-white capitalize mb-2">{cat}</h3>
            <div className="flex flex-col gap-1">
              {tools.filter((t) => t.category === cat).map((t) => (
                <div key={t.name} className="flex items-center justify-between text-xs">
                  <span className="text-zinc-300 font-mono">{t.name}</span>
                  <span className={`px-1.5 py-0.5 rounded text-[10px] ${
                    t.risk === "low" ? "bg-green-900 text-green-300" :
                    t.risk === "medium" ? "bg-yellow-900 text-yellow-300" :
                    "bg-red-900 text-red-300"
                  }`}>{t.risk}</span>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
