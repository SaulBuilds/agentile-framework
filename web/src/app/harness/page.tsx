"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function HarnessPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Agent Harness</h1>
      <div className="flex flex-col gap-4">
        <ToolPanel
          tool="harness_plan"
          title="Create Plan"
          fields={[
            { name: "prompt", label: "Prompt", placeholder: "set tempo to 140 and render a preview" },
            { name: "session_id", label: "Session ID", placeholder: "session-..." },
            { name: "deck_id", label: "Deck ID (optional)", placeholder: "deck-..." },
            { name: "adapter_id", label: "Adapter ID (optional)", placeholder: "realtime-adapter-..." },
          ]}
        />
        <ToolPanel
          tool="harness_execute"
          title="Execute Action"
          fields={[
            { name: "plan_id", label: "Plan ID", placeholder: "harness-plan-..." },
            { name: "action_id", label: "Action ID", placeholder: "harness-action-..." },
          ]}
        />
        <ListPanel tool="harness_outcome_list" title="Execution Outcomes" />
      </div>
    </div>
  );
}
