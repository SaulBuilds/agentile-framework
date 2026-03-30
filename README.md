# Agentile Framework

Lightweight starter infrastructure for human-agent software delivery across IDEs, harnesses, and models.

Agentile is not a single prompt. It is a repo contract:
- every agent gets routed into the same entry path
- every project creates its own root `SPIRIT.md`
- every workflow keeps hard gates around quality and traceability
- every meaningful fix or interpretation leaves behind an honest record for the next human or agent

## What This Starter Solves

AI coding agents tend to fail in repeatable ways:

| Failure Mode | What Happens | Agentile Response |
|--------------|--------------|-------------------|
| Context drift | The agent starts coding before it understands the repo | Root instruction files route into `.agentile/AGENT_ENTRY.md` |
| Generic behavior | The repo still speaks like a starter instead of the real project | `SPIRIT.md` and `README.md` must be rewritten early |
| Test amnesia | Features land without meaningful evidence | Hard gates require tests, lint, docs, and sprint traceability |
| Review blindness | Failing CI or pre-prod issues get treated like greenfield work | `REVIEW.md` and `DEBUGGING.md` handle unstable repo states |
| Lost insight | Valuable fixes die in chat history | Journals, essays, and case studies capture novel reasoning |

## Works In Any Repo State

This starter is meant for:
- brand-new projects
- retrofits of existing repos
- repos in failing CI/CD
- active debugging or incident response
- pre-prod hardening before release

The repo state changes the workflow, not the spirit.

## Universal Entry Surfaces

These files exist so different tools can discover the framework without custom setup:

| File | Typical Tools |
|------|---------------|
| `AGENTS.md` | Codex CLI, Cursor, Windsurf, Cline, Aider, Amp, Gemini CLI, and other `AGENTS.md` readers |
| `CLAUDE.md` | Claude Code |
| `.cursorrules` | Cursor legacy rules |
| `.windsurfrules` | Windsurf |
| `.clinerules` | Cline |
| `.github/copilot-instructions.md` | GitHub Copilot |

Every one of them points to [`.agentile/AGENT_ENTRY.md`](.agentile/AGENT_ENTRY.md).

## The Local Spirit

Every adopted project should have a root [`SPIRIT.md`](SPIRIT.md) that answers:
- what the project is trying to protect
- how humans and agents should collaborate here
- what kinds of shortcuts are forbidden even under pressure
- how the spirit itself can be changed safely

The framework starter ships:
- [`SPIRIT_GUIDE.md`](SPIRIT_GUIDE.md): how to draft and protect a project-specific spirit
- [`SPIRIT.md`](SPIRIT.md): the instantiated spirit for this repository
- [`.agentile/templates/SPIRIT.template.md`](.agentile/templates/SPIRIT.template.md): a short starting template
- [`.agentile/docs/SPIRIT_PROTECTION.md`](.agentile/docs/SPIRIT_PROTECTION.md): off-chain and on-chain protection patterns

If an adopted repo is still using the starter spirit or starter README verbatim, the first agent should fix that before feature work.

## Hard Gates

The framework keeps thirteen non-negotiable rules. The short version:

| Rule | Gate |
|------|------|
| Read `AGENT_ENTRY.md` first | BLOCKER |
| Plan before you code | GATE |
| No mocks, stubs, or TODOs in production code | GATE |
| Test count never decreases | BLOCKER |
| Every change traces to a sprint work package | GATE |
| Lint clean with zero warnings | GATE |
| Audits are immutable | BLOCKER |
| Docs ship with code | GATE |
| Sensitive changes require review | BLOCKER |
| Sprint files reflect reality | GATE |
| Critical logic gets formal verification or explicit justification | GATE |
| Production mock budget stays at zero | GATE |
| Project-generated governance docs carry timestamp and branch context | GATE |

Full details: [`.agentile/rules/CORE_RULES.md`](.agentile/rules/CORE_RULES.md)

## Insight Capture

Agentile treats reflection as part of delivery, not a nice-to-have.

| Artifact | When to Write It |
|----------|------------------|
| Sprint `JOURNAL.md` | At sprint close, and whenever a sprint changes meaningfully |
| `.agentile/docs/journals/*.md` | After reviews, debugging loops, CI failures, or handoffs worth preserving |
| `.agentile/docs/essays/*.md` | When a lesson generalizes beyond one fix |
| `.agentile/docs/case_studies/*.md` | When a concrete failure and remedy can teach future work |

The trigger logic lives in:
- [`.agentile/docs/JOURNAL_RULES.md`](.agentile/docs/JOURNAL_RULES.md)
- [`.agentile/docs/INSIGHT_CAPTURE.md`](.agentile/docs/INSIGHT_CAPTURE.md)

## Quick Start

### 1. Install the starter

Use this repo as a template, or copy:
- `.agentile/`
- `AGENTS.md`
- `CLAUDE.md`
- `.cursorrules`
- `.windsurfrules`
- `.clinerules`
- `.github/copilot-instructions.md`
- `SPIRIT_GUIDE.md`

### 2. Fill in the project contract

Update:
- [`.agentile/CONFIG.md`](.agentile/CONFIG.md)
- create or rewrite [`SPIRIT.md`](SPIRIT.md) using [`SPIRIT_GUIDE.md`](SPIRIT_GUIDE.md)
- `README.md` so it describes the actual project rather than this starter
- `CHANGELOG.md` if the adopted repo does not already have one

### 3. Choose the right workflow

| Situation | Workflow |
|-----------|----------|
| Cold start in a healthy repo | [`.agentile/workflows/INIT.md`](.agentile/workflows/INIT.md) |
| Retrofitting an existing repo | [`.agentile/workflows/RETROFIT.md`](.agentile/workflows/RETROFIT.md) |
| Failing CI, review fallout, or pre-prod hardening | [`.agentile/workflows/REVIEW.md`](.agentile/workflows/REVIEW.md) then [`.agentile/workflows/DEBUGGING.md`](.agentile/workflows/DEBUGGING.md) |
| Normal planned delivery | [`.agentile/workflows/SPRINT.md`](.agentile/workflows/SPRINT.md) and [`.agentile/workflows/FEATURE.md`](.agentile/workflows/FEATURE.md) |

### 4. Create the first sprint

```bash
mkdir -p .agentile/sprints/active/sprint-1-bootstrap
cp .agentile/templates/SPRINT.template.md .agentile/sprints/active/sprint-1-bootstrap/SPRINT.md
cp .agentile/templates/DAILY.template.md .agentile/sprints/active/sprint-1-bootstrap/DAILY.md
cp .agentile/templates/JOURNAL.template.md .agentile/sprints/active/sprint-1-bootstrap/JOURNAL.md
cp .agentile/templates/REPORT.template.md .agentile/sprints/active/sprint-1-bootstrap/REPORT.md
```

Then update [`.agentile/sprints/CURRENT.md`](.agentile/sprints/CURRENT.md).

### 5. Record the baseline and start

Run your test suite, record the baseline in [`.agentile/coverage/BASELINE.md`](.agentile/coverage/BASELINE.md), and begin work through the relevant workflow.

## Structure

```text
.
├── AGENTS.md
├── CLAUDE.md
├── SPIRIT.md
├── SPIRIT_GUIDE.md
├── CHANGELOG.md
├── .github/copilot-instructions.md
└── .agentile/
    ├── AGENT_ENTRY.md
    ├── CONFIG.md
    ├── MANIFEST.md
    ├── rules/
    ├── workflows/
    ├── templates/
    ├── docs/
    ├── coverage/
    ├── audits/
    ├── formal/
    └── sprints/
```

## What To Customize

Use as-is:
- most rules
- most workflows
- most templates

Customize early:
- `SPIRIT.md`
- `README.md`
- `.agentile/CONFIG.md`
- `.agentile/AGENT_ENTRY.md`
- `.agentile/onboarding/QUIZ_SPEC.md`

## Origin

Agentile grew out of repeated failures and recoveries in AI-assisted software work. The framework keeps the hard gates, but the starter here is intentionally lighter, more portable, and more explicit about spirit, review adaptation, and insight capture than the earlier branches it evolved from.

## License

MIT. See [LICENSE](LICENSE).
