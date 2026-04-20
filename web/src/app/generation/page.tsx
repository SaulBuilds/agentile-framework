"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function GenerationPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Generation</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="list_presets" title="Available Presets" />
        <ToolPanel
          tool="generate_demo"
          title="Generate Demo Composition"
          fields={[{ name: "seed", label: "Seed", type: "number", placeholder: "42" }]}
        />
        <ToolPanel
          tool="parameter_sweep"
          title="Parameter Sweep"
          fields={[
            { name: "preset", label: "Preset", placeholder: "demo" },
            { name: "seeds", label: "Seeds (JSON array)", type: "json", placeholder: "[1,2,3,4,5,6,7,8,9,10]" },
          ]}
        />
        <ToolPanel
          tool="preset_patch"
          title="Patch Preset"
          fields={[
            { name: "preset", label: "Preset", placeholder: "demo" },
            { name: "tempo_bpm", label: "Tempo BPM", type: "number", placeholder: "120" },
            { name: "low_note", label: "Low Note", type: "number", placeholder: "36" },
            { name: "high_note", label: "High Note", type: "number", placeholder: "96" },
            { name: "reason", label: "Reason", placeholder: "creative exploration" },
          ]}
        />
      </div>
    </div>
  );
}
