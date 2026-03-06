# Agent Notes

> This is the agent's working memory. Update this file after every significant decision, blocker, or insight.
> This file persists between sessions and serves as continuity for the agent.

## Format
```
### [YYYY-MM-DD] [Topic]
[Note content]
```

---

### 2026-03-05 Framework Hardening — Gate Enforcement Overhaul

**Context**: During first use of the Agentile framework, the agent (Claude Code) skipped the INIT_WORKFLOW entirely. It read the repo, saw untracked files, committed and pushed them — but never detected the empty planset or ran initialization. The framework's guidance was too passive.

**Root cause**: `AGENT_ENTRY.md` described what to do as suggestions, not hard gates. An agent could acknowledge the framework and still proceed directly to the user's request. The rule files (`BDD_RULES.md`, `TDD_RULES.md`, etc.) similarly lacked enforcement language.

**Changes made**:
- `AGENT_ENTRY.md`: Rewritten with MANDATORY PRE-FLIGHT CHECK, HARD GATE system, and guided discovery protocol
- `rules/CORE_RULES.md`: Added Rule 0 (Initialize Before Everything) as highest-priority rule. All rules now have explicit GATE checkpoints.
- `rules/BDD_RULES.md`: Added Feature Completeness Gate, Feature Lifecycle Gate, Tagging Gate
- `rules/TDD_RULES.md`: Added gates at every RED/GREEN/REFACTOR phase transition. Added Prohibited Practices section. Added Test Completeness Gate.
- `rules/DOCUMENTATION_RULES.md`: Added gates for every documentation event. Documentation debt treated with same urgency as test debt.
- `rules/GIT_RULES.md`: Added Pre-Commit Gate, Merge Gate, Sprint Merge Gate. Added Incomplete Work Protocol. Prohibited skipped/pending tests in commits.
- `workflows/INIT_WORKFLOW.md`: Every step now has pass/fail gate with verification criteria
- `workflows/FEATURE_WORKFLOW.md`: Entry gate + phase transition gates + exit gate with full traceability chain verification
- `workflows/SPRINT_WORKFLOW.md`: Entry gate, planning gate, execution tracking gate, review gates, completion checklist
- `workflows/RETROFIT_WORKFLOW.md`: Entry gate, audit gate, planset gate, safety-net gate, backlog gate
- `workflows/REVIEW_WORKFLOW.md`: Every section is an enforceable gate with failure protocol

**New files**:
- `docs/EVALUATION.md`: 7 benchmarks for testing agent compliance (cold start detection, guided discovery, feature traceability, gate enforcement, sprint boundary respect, review completeness, retrofit detection)
- `templates/AGENT_HOOKS.template.md`: Hook file templates for Claude, Cursor, Copilot, Windsurf

**Key design decision**: Gates use "DO NOT PROCEED" language rather than "should" or "recommended." The framework now treats initialization like a hard dependency — nothing downstream works without it. This mirrors how CI/CD pipelines work: a failed gate blocks the pipeline.

**Benchmark for validation**: The failure case (Benchmark 1 in EVALUATION.md) directly mirrors what happened in this session. Future agents should be tested against all 7 benchmarks.
