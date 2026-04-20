"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function EvaluationsPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Evaluations</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="evaluation_list" title="Evaluation Records" />
        <ToolPanel
          tool="evaluation_inspect"
          title="Inspect Evaluation"
          fields={[{ name: "evaluation_id", label: "Evaluation ID", placeholder: "eval-..." }]}
        />
      </div>
    </div>
  );
}
