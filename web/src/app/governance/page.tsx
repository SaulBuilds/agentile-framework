"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function GovernancePage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Governance</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="dataset_list" title="Registered Datasets" />
        <ToolPanel
          tool="approval_request"
          title="Request Approval"
          fields={[
            { name: "action_scope", label: "Action Scope", placeholder: "jobs.schedule" },
            { name: "target", label: "Target", placeholder: "nightly-sweep" },
            { name: "reason", label: "Reason", placeholder: "schedule nightly evaluation" },
          ]}
        />
        <ToolPanel
          tool="approval_resolve"
          title="Resolve Approval"
          fields={[
            { name: "approval_id", label: "Approval ID", placeholder: "approval-..." },
            { name: "reason", label: "Reason", placeholder: "approved for nightly use" },
          ]}
        />
        <ToolPanel
          tool="snapshot_create"
          title="Create Preset Snapshot"
          fields={[
            { name: "preset", label: "Preset", placeholder: "demo" },
            { name: "reason", label: "Reason", placeholder: "before parameter exploration" },
          ]}
        />
      </div>
    </div>
  );
}
