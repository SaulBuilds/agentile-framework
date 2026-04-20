"use client";
import { useState } from "react";

export default function SettingsPage() {
  const [apiUrl, setApiUrl] = useState(process.env.NEXT_PUBLIC_API_URL || "http://localhost:3001");
  const [apiKey, setApiKey] = useState(process.env.NEXT_PUBLIC_API_KEY || "");

  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Settings</h1>
      <div className="border border-zinc-800 rounded-lg p-4 bg-zinc-950 max-w-lg">
        <h3 className="text-sm font-semibold text-white mb-3">API Configuration</h3>
        <p className="text-xs text-zinc-500 mb-4">
          Set these values in <code className="text-zinc-400">.env.local</code> or configure them here for the current session.
        </p>
        <div className="flex flex-col gap-3">
          <div>
            <label className="text-xs text-zinc-400 block mb-1">API URL</label>
            <input
              value={apiUrl}
              onChange={(e) => setApiUrl(e.target.value)}
              className="w-full bg-zinc-900 border border-zinc-700 rounded px-2 py-1 text-sm text-white"
            />
          </div>
          <div>
            <label className="text-xs text-zinc-400 block mb-1">API Key</label>
            <input
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              className="w-full bg-zinc-900 border border-zinc-700 rounded px-2 py-1 text-sm text-white"
            />
          </div>
        </div>
        <p className="text-xs text-zinc-500 mt-4">
          Start the API server: <code className="text-zinc-400">cargo run -- http --port 3001 --api-key your-key</code>
        </p>
      </div>
    </div>
  );
}
