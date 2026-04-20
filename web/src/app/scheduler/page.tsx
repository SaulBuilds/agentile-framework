"use client";
import ListPanel from "@/components/ListPanel";
import ToolPanel from "@/components/ToolPanel";

export default function SchedulerPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Scheduler</h1>
      <div className="flex flex-col gap-4">
        <ListPanel tool="job_list" title="Scheduled Jobs" />
        <ToolPanel
          tool="job_validate"
          title="Validate Job"
          fields={[
            { name: "prompt", label: "Prompt", placeholder: "render and evaluate" },
            { name: "session_id", label: "Session ID", placeholder: "session-..." },
            { name: "retry_limit", label: "Retry Limit", type: "number", defaultValue: "1" },
          ]}
        />
        <ToolPanel
          tool="job_run"
          title="Run Job"
          fields={[{ name: "job_id", label: "Job ID", placeholder: "job-..." }]}
        />
      </div>
    </div>
  );
}
