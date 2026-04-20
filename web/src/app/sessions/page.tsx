"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function SessionsPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Sessions</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="session_list" title="Sessions" />
        <ToolPanel
          tool="session_create"
          title="Create Session"
          fields={[
            { name: "display_name", label: "Name", placeholder: "My Session" },
            { name: "preset", label: "Preset", placeholder: "demo" },
            { name: "seed", label: "Seed", type: "number", placeholder: "1" },
          ]}
        />
        <ToolPanel
          tool="session_inspect"
          title="Inspect Session"
          fields={[{ name: "session_id", label: "Session ID", placeholder: "session-..." }]}
        />
        <ToolPanel
          tool="session_render_preview"
          title="Render Preview"
          fields={[{ name: "session_id", label: "Session ID", placeholder: "session-..." }]}
        />
        <ToolPanel
          tool="session_play"
          title="Play Session"
          fields={[
            { name: "session_id", label: "Session ID", placeholder: "session-..." },
            { name: "run_label", label: "Run Label", placeholder: "run-1" },
          ]}
        />
        <ToolPanel
          tool="session_stop"
          title="Stop Session"
          fields={[{ name: "session_id", label: "Session ID", placeholder: "session-..." }]}
        />
      </div>
    </div>
  );
}
