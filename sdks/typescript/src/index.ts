/**
 * @saulbuilds/music-box-sdk
 *
 * TypeScript SDK for the state-space-music-box HTTP API.
 *
 * ```ts
 * import { MusicBoxClient } from "@saulbuilds/music-box-sdk";
 *
 * const client = new MusicBoxClient("http://localhost:3001", "your-api-key");
 * const presets = await client.listPresets();
 * const sweep = await client.parameterSweep("demo", [1, 2, 3, 4, 5]);
 * console.log("Best seed:", sweep.ranked_seeds[0]);
 * ```
 */

export interface ApiResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
}

export interface ToolInfo {
  name: string;
  category: string;
  description: string;
  risk: string;
}

export interface PresetSummary {
  name: string;
  source: string;
}

export interface CompositionSummary {
  preset_name: string;
  seed: number;
  note_count: number;
  trajectory_frames: number;
  duration_seconds: number;
  audio_sample_count: number;
}

export interface SweepEntry {
  seed: number;
  note_count: number;
  trajectory_summary: {
    frame_count: number;
    duration_seconds: number;
    peak_abs_output: number;
    mean_abs_output: number;
  };
}

export interface SweepResult {
  sweep_id: string;
  preset_name: string;
  entries: SweepEntry[];
  ranked_seeds: number[];
}

export interface PatchResult {
  preset_name: string;
  snapshot_id: string;
  changed_fields: string[];
}

export interface SessionRecord {
  session_id: string;
  display_name: string;
  preset_name: string;
  seed: number;
  tempo_bpm: number;
  status: string;
  [key: string]: unknown;
}

export interface HarnessPlan {
  plan_id: string;
  role: string;
  prompt: string;
  proposed_actions: Array<{
    action_id: string;
    tool_name: string;
    risk_level: string;
    justification: string;
  }>;
  [key: string]: unknown;
}

export class MusicBoxClient {
  constructor(
    private baseUrl: string,
    private apiKey: string
  ) {}

  private async call<T>(tool: string, params: Record<string, unknown> = {}): Promise<T> {
    const res = await fetch(`${this.baseUrl}/api/tools/${tool}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
      },
      body: JSON.stringify(params),
    });
    const json: ApiResponse<T> = await res.json();
    if (!json.success) throw new Error(json.error || "API call failed");
    return json.data as T;
  }

  // ── Health & Discovery ──────────────────────────────────────

  async health(): Promise<{ status: string; version: string }> {
    const res = await fetch(`${this.baseUrl}/api/health`);
    const json: ApiResponse<{ status: string; version: string }> = await res.json();
    return json.data!;
  }

  async tools(): Promise<ToolInfo[]> {
    const res = await fetch(`${this.baseUrl}/api/tools`);
    return res.json();
  }

  // ── Generation ──────────────────────────────────────────────

  async listPresets(): Promise<PresetSummary[]> {
    return this.call("list_presets");
  }

  async generateDemo(seed: number = 1): Promise<CompositionSummary> {
    return this.call("generate_demo", { seed });
  }

  async generateComposition(preset: string, seed: number = 1): Promise<CompositionSummary> {
    return this.call("generate_composition", { preset, seed });
  }

  // ── Creative Tools ──────────────────────────────────────────

  async parameterSweep(preset: string, seeds: number[]): Promise<SweepResult> {
    return this.call("parameter_sweep", { preset, seeds });
  }

  async presetPatch(
    preset: string,
    patches: {
      tempo_bpm?: number;
      low_note?: number;
      high_note?: number;
      root_note?: number;
      scale?: number[];
      step_beats?: number;
      duration_seconds?: number;
      peak_limit?: number;
    },
    reason: string = "sdk patch"
  ): Promise<PatchResult> {
    return this.call("preset_patch", { preset, reason, ...patches });
  }

  async listSweeps(): Promise<SweepResult[]> {
    return this.call("sweep_list");
  }

  // ── Sessions ────────────────────────────────────────────────

  async createSession(displayName: string, preset: string, seed: number = 1): Promise<SessionRecord> {
    return this.call("session_create", { display_name: displayName, preset, seed });
  }

  async listSessions(): Promise<SessionRecord[]> {
    return this.call("session_list");
  }

  async inspectSession(sessionId: string): Promise<SessionRecord> {
    return this.call("session_inspect", { session_id: sessionId });
  }

  async renderPreview(sessionId: string): Promise<unknown> {
    return this.call("session_render_preview", { session_id: sessionId });
  }

  async playSession(sessionId: string, runLabel?: string): Promise<SessionRecord> {
    return this.call("session_play", { session_id: sessionId, run_label: runLabel });
  }

  async stopSession(sessionId: string): Promise<SessionRecord> {
    return this.call("session_stop", { session_id: sessionId });
  }

  // ── Harness ─────────────────────────────────────────────────

  async harnessPlan(
    prompt: string,
    opts: { sessionId?: string; deckId?: string; adapterId?: string; role?: string } = {}
  ): Promise<HarnessPlan> {
    return this.call("harness_plan", {
      prompt,
      role: opts.role || "session_dj",
      session_id: opts.sessionId,
      deck_id: opts.deckId,
      adapter_id: opts.adapterId,
    });
  }

  async harnessExecute(planId: string, actionId: string): Promise<unknown> {
    return this.call("harness_execute", { plan_id: planId, action_id: actionId });
  }

  async harnessOutcomes(): Promise<unknown[]> {
    return this.call("harness_outcome_list");
  }

  // ── Governance ──────────────────────────────────────────────

  async requestApproval(actionScope: string, target: string, reason: string): Promise<unknown> {
    return this.call("approval_request", { action_scope: actionScope, target, reason });
  }

  async resolveApproval(approvalId: string, reason: string = "approved"): Promise<unknown> {
    return this.call("approval_resolve", { approval_id: approvalId, reason });
  }

  async createSnapshot(preset: string, reason: string): Promise<unknown> {
    return this.call("snapshot_create", { preset, reason });
  }

  // ── Audit ───────────────────────────────────────────────────

  async listRuns(): Promise<unknown[]> {
    return this.call("run_list");
  }

  async listAuditEvents(): Promise<unknown[]> {
    return this.call("audit_list");
  }

  // ── Scheduler ───────────────────────────────────────────────

  async validateJob(prompt: string, sessionId?: string): Promise<unknown> {
    return this.call("job_validate", { prompt, session_id: sessionId, retry_limit: 1 });
  }

  async listJobs(): Promise<unknown[]> {
    return this.call("job_list");
  }

  async runJob(jobId: string): Promise<unknown> {
    return this.call("job_run", { job_id: jobId });
  }

  // ── Realtime ────────────────────────────────────────────────

  async createAdapter(displayName: string, host: string, port: number): Promise<unknown> {
    return this.call("realtime_create", { display_name: displayName, host, port });
  }

  async listAdapters(): Promise<unknown[]> {
    return this.call("realtime_list");
  }

  async sendPreview(adapterId: string, sessionId: string, previewId: string): Promise<unknown> {
    return this.call("realtime_send_preview", {
      adapter_id: adapterId,
      session_id: sessionId,
      preview_id: previewId,
    });
  }

  // ── Decks ───────────────────────────────────────────────────

  async createDeck(displayName: string, sessionId: string): Promise<unknown> {
    return this.call("deck_create", { display_name: displayName, session_id: sessionId });
  }

  async listDecks(): Promise<unknown[]> {
    return this.call("deck_list");
  }

  async deckTransport(deckId: string): Promise<unknown> {
    return this.call("deck_transport", { deck_id: deckId });
  }

  // ── Evaluations ─────────────────────────────────────────────

  async listEvaluations(): Promise<unknown[]> {
    return this.call("evaluation_list");
  }

  async inspectEvaluation(evaluationId: string): Promise<unknown> {
    return this.call("evaluation_inspect", { evaluation_id: evaluationId });
  }

  // ── Datasets ────────────────────────────────────────────────

  async listDatasets(): Promise<unknown[]> {
    return this.call("dataset_list");
  }
}

export default MusicBoxClient;
