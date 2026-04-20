"use client";
import ListPanel from "@/components/ListPanel";

export default function AuditPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Audit Trail</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="run_list" title="Run Manifests" />
        <ListPanel tool="audit_list" title="Audit Events" />
        <ListPanel tool="sweep_list" title="Parameter Sweeps" />
      </div>
    </div>
  );
}
