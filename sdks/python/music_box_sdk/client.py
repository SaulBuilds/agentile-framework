"""Typed HTTP client for the state-space-music-box API."""

from __future__ import annotations

from typing import Any

import httpx


class MusicBoxError(Exception):
    """Raised when the API returns success=false."""


class MusicBoxClient:
    """Client for the state-space-music-box HTTP API.

    Args:
        base_url: API server URL (e.g. "http://localhost:3001").
        api_key: Bearer token for authentication.
        timeout: Request timeout in seconds.
    """

    def __init__(self, base_url: str, api_key: str, timeout: float = 30.0):
        self._base = base_url.rstrip("/")
        self._headers = {
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
        }
        self._client = httpx.Client(timeout=timeout)

    def _call(self, tool: str, params: dict[str, Any] | None = None) -> Any:
        resp = self._client.post(
            f"{self._base}/api/tools/{tool}",
            headers=self._headers,
            json=params or {},
        )
        data = resp.json()
        if not data.get("success"):
            raise MusicBoxError(data.get("error", "unknown error"))
        return data.get("data")

    # ── Health & Discovery ──────────────────────────────────────

    def health(self) -> dict:
        """Check API health."""
        return self._client.get(f"{self._base}/api/health").json().get("data", {})

    def tools(self) -> list[dict]:
        """List all available tools."""
        return self._client.get(f"{self._base}/api/tools").json()

    # ── Generation ──────────────────────────────────────────────

    def list_presets(self) -> list[dict]:
        """List available presets."""
        return self._call("list_presets")

    def generate_demo(self, seed: int = 1) -> dict:
        """Generate a demo composition."""
        return self._call("generate_demo", {"seed": seed})

    def generate_composition(self, preset: str, seed: int = 1) -> dict:
        """Generate a composition from a named preset."""
        return self._call("generate_composition", {"preset": preset, "seed": seed})

    # ── Creative Tools ──────────────────────────────────────────

    def parameter_sweep(self, preset: str, seeds: list[int]) -> dict:
        """Run compositions across multiple seeds and rank by dynamics."""
        return self._call("parameter_sweep", {"preset": preset, "seeds": seeds})

    def preset_patch(self, preset: str, reason: str = "sdk patch", **patches: Any) -> dict:
        """Patch preset parameters with automatic snapshot."""
        return self._call("preset_patch", {"preset": preset, "reason": reason, **patches})

    def list_sweeps(self) -> list[dict]:
        """List stored sweep results."""
        return self._call("sweep_list")

    # ── Sessions ────────────────────────────────────────────────

    def create_session(self, display_name: str, preset: str, seed: int = 1) -> dict:
        """Create a new session."""
        return self._call("session_create", {"display_name": display_name, "preset": preset, "seed": seed})

    def list_sessions(self) -> list[dict]:
        """List all sessions."""
        return self._call("session_list")

    def inspect_session(self, session_id: str) -> dict:
        """Inspect a session by ID."""
        return self._call("session_inspect", {"session_id": session_id})

    def render_preview(self, session_id: str) -> dict:
        """Render a MIDI/WAV preview from session state."""
        return self._call("session_render_preview", {"session_id": session_id})

    def play_session(self, session_id: str, run_label: str | None = None) -> dict:
        """Start session transport."""
        return self._call("session_play", {"session_id": session_id, "run_label": run_label})

    def stop_session(self, session_id: str) -> dict:
        """Stop session transport."""
        return self._call("session_stop", {"session_id": session_id})

    # ── Harness ─────────────────────────────────────────────────

    def harness_plan(self, prompt: str, session_id: str | None = None, **opts: Any) -> dict:
        """Create a constrained agent plan."""
        return self._call("harness_plan", {"prompt": prompt, "session_id": session_id, "role": "session_dj", **opts})

    def harness_execute(self, plan_id: str, action_id: str) -> dict:
        """Execute one action from a harness plan."""
        return self._call("harness_execute", {"plan_id": plan_id, "action_id": action_id})

    def harness_outcomes(self) -> list[dict]:
        """List execution outcomes."""
        return self._call("harness_outcome_list")

    # ── Governance ──────────────────────────────────────────────

    def request_approval(self, action_scope: str, target: str, reason: str) -> dict:
        """Create an approval request."""
        return self._call("approval_request", {"action_scope": action_scope, "target": target, "reason": reason})

    def resolve_approval(self, approval_id: str, reason: str = "approved") -> dict:
        """Resolve a pending approval."""
        return self._call("approval_resolve", {"approval_id": approval_id, "reason": reason})

    def create_snapshot(self, preset: str, reason: str) -> dict:
        """Create a preset snapshot for rollback."""
        return self._call("snapshot_create", {"preset": preset, "reason": reason})

    def list_datasets(self) -> list[dict]:
        """List registered datasets."""
        return self._call("dataset_list")

    # ── Audit ───────────────────────────────────────────────────

    def list_runs(self) -> list[dict]:
        """List run manifests."""
        return self._call("run_list")

    def list_audit_events(self) -> list[dict]:
        """List audit events."""
        return self._call("audit_list")

    # ── Scheduler ───────────────────────────────────────────────

    def validate_job(self, prompt: str, session_id: str | None = None) -> dict:
        """Validate a scheduled job config."""
        return self._call("job_validate", {"prompt": prompt, "session_id": session_id, "retry_limit": 1})

    def list_jobs(self) -> list[dict]:
        """List scheduled jobs."""
        return self._call("job_list")

    def run_job(self, job_id: str) -> dict:
        """Execute a job locally."""
        return self._call("job_run", {"job_id": job_id})

    # ── Realtime ────────────────────────────────────────────────

    def create_adapter(self, display_name: str, host: str, port: int) -> dict:
        """Create an OSC adapter."""
        return self._call("realtime_create", {"display_name": display_name, "host": host, "port": port})

    def list_adapters(self) -> list[dict]:
        """List realtime adapters."""
        return self._call("realtime_list")

    # ── Decks ───────────────────────────────────────────────────

    def create_deck(self, display_name: str, session_id: str) -> dict:
        """Create a deck bound to a session."""
        return self._call("deck_create", {"display_name": display_name, "session_id": session_id})

    def list_decks(self) -> list[dict]:
        """List all decks."""
        return self._call("deck_list")

    def deck_transport(self, deck_id: str) -> dict:
        """Inspect deck transport state."""
        return self._call("deck_transport", {"deck_id": deck_id})

    # ── Evaluations ─────────────────────────────────────────────

    def list_evaluations(self) -> list[dict]:
        """List evaluation records."""
        return self._call("evaluation_list")

    def inspect_evaluation(self, evaluation_id: str) -> dict:
        """Inspect an evaluation by ID."""
        return self._call("evaluation_inspect", {"evaluation_id": evaluation_id})
