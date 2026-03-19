# Agentile Framework

> **First step for every contributor (human or AI): read [`.agentile/AGENT_ENTRY.md`](.agentile/AGENT_ENTRY.md).** This is the decision tree that routes you to the right workflow, assigns your zooid identity, and ensures you follow the project's quality gates. No code should be written before reading this file.

A software engineering methodology for AI-assisted development. Provides persistent structure, quality gates, and contributor identity management for projects where human engineers collaborate with AI coding agents.

## IDE Auto-Discovery

The framework automatically injects instructions into every major AI coding tool. After installation, create these files at your repo root (templates provided):

| File | Tool(s) Covered |
|------|----------------|
| `AGENTS.md` | Cursor, Copilot, Windsurf, Codex CLI, Gemini CLI, Amp, Junie, Zed, Aider, Cline, 20+ more |
| `CLAUDE.md` | Claude Code |
| `.cursorrules` | Cursor (legacy) |
| `.windsurfrules` | Windsurf |
| `.clinerules` | Cline |
| `.github/copilot-instructions.md` | GitHub Copilot |

Each file contains the same core message: **"Read `.agentile/AGENT_ENTRY.md` before doing anything."** This ensures no agent can enter the repo without hitting the framework.

## What is Agentile?

Agentile solves the five failure modes of AI-assisted development:

1. **Context Drift** -- Agents lose track of prior decisions across sessions
2. **Documentation Debt** -- Code outpaces documentation within weeks
3. **Test Amnesia** -- Features ship without meaningful test coverage
4. **Sprint Sprawl** -- Work expands without scope boundaries
5. **Stub Proliferation** -- TODOs and placeholders persist indefinitely

The framework provides:
- A `.agentile/` directory that serves as the agent's external memory
- A zooid system of contributor identities with ELO-based competence scoring
- Hard gates that block phase transitions without evidence
- Workflow definitions for sprint execution and feature implementation
- A test ratchet that enforces monotonically increasing test counts

## Installation

1. Copy the `.agentile/` directory structure to your project root:
   ```bash
   cp -r agentile-framework-bare/ your-project/.agentile/
   ```

2. Customize `CONFIG.md` with your project's values (see [SETUP.md](SETUP.md) for all placeholders)

3. Write your onboarding quiz questions in `onboarding/QUIZ_SPEC.md`

4. Run the INIT workflow: follow `workflows/INIT.md` step by step

5. Create your first sprint using `templates/SPRINT.template.md`

## Directory Structure

```
.agentile/
├── AGENT_ENTRY.md          # Entry point -- every contributor starts here
├── CONFIG.md               # Canonical project constants (CUSTOMIZE THIS)
├── MANIFEST.md             # Index of all framework files
├── rules/                  # 6 rule files (project-agnostic, use as-is)
├── zooids/                 # 6 contributor identities + ELO system + registry
├── onboarding/             # Quiz spec (CUSTOMIZE) + skip protocol
├── workflows/              # 5 workflow definitions (project-agnostic)
├── templates/              # 6 document templates (project-agnostic)
├── docs/                   # Module index + methodology spec
├── formal/                 # TLA+ spec index + verification workflow
├── coverage/               # Test baseline + coverage gates
└── sprints/                # CURRENT.md + index + active/backlog/completed
```

## Quick Start

1. **New contributor?** Read `AGENT_ENTRY.md`
2. **Setting up?** Read [SETUP.md](SETUP.md) and customize `CONFIG.md`
3. **Existing codebase?** Follow `workflows/RETROFIT.md`
4. **New project?** Follow `workflows/INIT.md`

## Files to Customize

| File | What to Change |
|------|---------------|
| `CONFIG.md` | All `{{PLACEHOLDER}}` values with your project's constants |
| `AGENT_ENTRY.md` | `{{PROJECT_DESCRIPTION}}` and related placeholders |
| `onboarding/QUIZ_SPEC.md` | All `{{QUIZ_*}}` placeholders with project-specific questions |

All other files are project-agnostic and can be used as-is.

## License

MIT License. See [LICENSE](LICENSE).
