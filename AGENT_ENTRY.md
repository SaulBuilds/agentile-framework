# Agent Entry Point

> **Read this file first.** Every contributor -- human or AI -- starts here.

## Who Are You?

| If you are... | Go to... |
|---------------|----------|
| **A new contributor (human or AI)** | [Onboarding Quiz](#onboarding) below |
| **A qualified engineer (no AI assist)** | [Skip Protocol](onboarding/SKIP_PROTOCOL.md) |
| **An autonomous agent entering cold** | [Cold Start](#cold-start) below |
| **Returning to an active sprint** | [sprints/CURRENT.md](sprints/CURRENT.md) |

---

## Cold Start

If you have no prior context about this project:

1. **Read** [CONFIG.md](CONFIG.md) -- canonical project constants
2. **Read** [rules/CORE_RULES.md](rules/CORE_RULES.md) -- non-negotiable operating rules
3. **Check** [sprints/CURRENT.md](sprints/CURRENT.md) -- what work is active right now
4. **Check** [sprints/backlog/](sprints/backlog/) -- what needs to be done
5. **Pick a task** from the current sprint or backlog
6. **Follow** the [FEATURE workflow](workflows/FEATURE.md) for implementation

**GATE: Do NOT write code until you have read CONFIG.md and CORE_RULES.md.**

---

## Onboarding

New contributors take a 5-question adaptive quiz to determine:
- Your **skill tier** (Novice, Journeyman, Expert, Master)
- Your **starting zooid** (role assignment)
- Your **ELO starting score**

See [onboarding/QUIZ_SPEC.md](onboarding/QUIZ_SPEC.md) for the full quiz.

After the quiz, you will be directed to the appropriate workflow:
- **Novice (0-800)**: Start with documentation tasks and guided feature work
- **Journeyman (800-1200)**: Standard feature workflow with review gates
- **Expert (1200-1600)**: Feature workflow with relaxed gates, can review others
- **Master (1600+)**: Can propose architecture changes, mentor novices

---

## Project Overview

{{PROJECT_DESCRIPTION}}

- **Token**: {{TOKEN_NAME}} ({{TOKEN_TOTAL_SUPPLY}} supply, {{TOKEN_DECIMALS}} decimals)
- **Chain ID**: {{CHAIN_ID_PRIMARY}}
- **VM**: {{VM_NAME}} -- {{VM_DESCRIPTION}}
- **Consensus**: {{CONSENSUS_NAME}} with {{CONSENSUS_PARAMS}}
- **Test suite**: {{TEST_SUITE_SUMMARY}}

For full project details, see the workspace root `README.md` or [CONFIG.md](CONFIG.md).

---

## Framework Structure

```
.agentile/
├── AGENT_ENTRY.md          # You are here
├── CONFIG.md               # Canonical project constants
├── MANIFEST.md             # Index of all framework files
├── rules/                  # Non-negotiable operating rules
├── zooids/                 # Contributor identities + ELO system
├── onboarding/             # Skill assessment quiz
├── workflows/              # Step-by-step execution procedures
├── templates/              # Copyable document templates
├── docs/                   # Living documentation + canonical essays
├── formal/                 # TLA+ specs index + verification workflow
├── coverage/               # Test coverage tracking + gates
├── sprints/                # active/, backlog/, completed/
├── audits/                 # Dated audit reports
└── reports/                # Generated sprint reports
```

---

## The Golden Rules

1. **Plan before you code** -- check CURRENT.md, follow the workflow
2. **No stubs, no TODOs** -- every line is production-ready
3. **Test count only goes up** -- never decrease the test count
4. **Audits are immutable** -- dated directories, never edited after creation
5. **The sprint file is the source of truth** -- not your memory, not a chat log

For the full rule set, see [rules/CORE_RULES.md](rules/CORE_RULES.md).
