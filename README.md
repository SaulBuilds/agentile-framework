# Agentile Framework

> "go ahead and make those changes... that sounds good. if you run into problems just ping me."

**Agentile** (Agent + Agile) is a lightweight, documentation-first methodology for coordinating AI agents around Behavior-Driven Development (BDD) and Test-Driven Development (TDD). It transforms your AI coding agent into a disciplined engineer that plans before it codes, tests before it ships, and documents as it goes.

## Philosophy

The best way to keep an AI agent on the rails is the same way you keep a junior engineer on the rails: **give them clear architecture, write the behaviors in natural language, and let the failing tests guide them home.**

Agentile enforces a simple loop:

```
Roadmap → Feature (Gherkin) → Test (Red) → Implementation (Green) → Refactor → Document → Next
```

## Quick Start

### Option A: New Project
1. Fork or copy this repository
2. Open the `.agentile/` folder and read `AGENT_ENTRY.md`
3. Tell your agent: *"Read .agentile/AGENT_ENTRY.md and initialize this project based on our conversation."*
4. Describe your project vision. The agent will populate the planset, create sprints, and begin the BDD/TDD cycle.

### Option B: Existing Project (Retrofit)
1. Copy the `.agentile/` folder into the root of any existing repository
2. Tell your agent: *"Read .agentile/AGENT_ENTRY.md and retrofit this project around the Agentile methodology."*
3. The agent will audit existing code, generate features/tests, and restructure planning around BDD/TDD.

## What's in `.agentile/`?

```
.agentile/
├── AGENT_ENTRY.md          # START HERE - Agent reads this first
├── MANIFEST.md             # Index of all framework files
├── CONFIG.md               # Project-level configuration
│
├── rules/                  # Behavioral guardrails for the agent
│   ├── CORE_RULES.md       # Non-negotiable development rules
│   ├── BDD_RULES.md        # Behavior-Driven Development rules
│   ├── TDD_RULES.md        # Test-Driven Development rules
│   ├── DOCUMENTATION_RULES.md
│   └── GIT_RULES.md        # Commit and branch conventions
│
├── roles/                  # Agent personas and responsibilities
│   ├── ARCHITECT.md        # System design and planning
│   ├── DEVELOPER.md        # Implementation (Red-Green-Refactor)
│   ├── QA_ENGINEER.md      # Test review and coverage
│   ├── SCRUM_MASTER.md     # Sprint management and reporting
│   └── TECH_WRITER.md      # Documentation maintenance
│
├── workflows/              # Step-by-step execution flows
│   ├── INIT_WORKFLOW.md    # Project initialization
│   ├── SPRINT_WORKFLOW.md  # Sprint planning → execution → retro
│   ├── FEATURE_WORKFLOW.md # Feature lifecycle (Gherkin → Test → Code)
│   ├── RETROFIT_WORKFLOW.md# Adopting Agentile in existing repos
│   └── REVIEW_WORKFLOW.md  # Code review and quality gates
│
├── templates/              # Reusable file templates
│   ├── FEATURE.template.md
│   ├── SPRINT.template.md
│   ├── REPORT.template.md
│   ├── ARCHITECTURE_DECISION.template.md
│   └── USER_STORY.template.md
│
├── planset/                # Project planning documents (populated during init)
│   ├── executive-summary/
│   │   └── .gitkeep
│   └── architecture/
│       └── .gitkeep
│
├── features/               # Gherkin feature files (.feature)
│   └── .gitkeep
│
├── sprints/                # Sprint tracking
│   ├── backlog/            # Unscheduled work items
│   ├── active/             # Current sprint
│   └── completed/          # Archived sprints with retro notes
│
├── reports/                # Agent-generated reports
│   └── .gitkeep
│
└── docs/                   # Living project documentation
    └── .gitkeep
```

## The Agentile Loop

Every piece of work follows this cycle:

1. **PLAN** — Agent reads the roadmap/planset and identifies the next task
2. **SPECIFY** — Agent writes a Gherkin `.feature` file describing expected behavior
3. **TEST** — Agent writes a failing test that satisfies the Gherkin specification
4. **IMPLEMENT** — Agent writes the minimum code to make the test pass
5. **REFACTOR** — Agent cleans up while keeping tests green
6. **DOCUMENT** — Agent updates docs, architecture records, and sprint reports
7. **REPORT** — Agent summarizes what was done and asks for human review
8. **NEXT** — Human approves or redirects. Repeat.

## For Humans

Your job is to be the **Product Owner** and **Staff Engineer**. You:

- Write the vision (or tell it to the agent in conversation)
- Review reports and approve direction
- Say "go ahead" or "let's change course"
- Focus on strategy, networking, and the things that matter

The agent handles the rest.

## License

MIT — Use it, fork it, make it yours.

---

*Created by Larry "Saul" Kłosowski — Cnidarian Foundation*
*Built on the principle that dynamic behavior is a feature, not a bug.*
