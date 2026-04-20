"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function RealtimePage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Realtime Adapters</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="realtime_list" title="Adapters" />
        <ToolPanel
          tool="realtime_create"
          title="Create Adapter"
          fields={[
            { name: "display_name", label: "Name", placeholder: "Loopback" },
            { name: "host", label: "Host", placeholder: "127.0.0.1" },
            { name: "port", label: "Port", type: "number", placeholder: "9000" },
            { name: "base_path", label: "Base Path", placeholder: "/agentic_dj" },
          ]}
        />
        <ToolPanel
          tool="realtime_send_preview"
          title="Send Preview to Adapter"
          fields={[
            { name: "adapter_id", label: "Adapter ID", placeholder: "realtime-adapter-..." },
            { name: "session_id", label: "Session ID", placeholder: "session-..." },
            { name: "preview_id", label: "Preview ID", placeholder: "preview-..." },
          ]}
        />
      </div>
    </div>
  );
}
