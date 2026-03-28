# Agentile Framework

**A methodology for human-AI collaborative software engineering at scale.**

Agentile is a structured development framework that turns AI coding agents into reliable engineering partners. It was developed through empirical observation of what goes wrong when AI agents write code -- and what structures prevent those failures from recurring.

The methodology fuses agile sprint discipline with AI-agent-specific safeguards: a 13-rule operating system, role-based contributor identities (zooids), formal verification integration, and a journaling practice that serves as state transfer between agent sessions.

---

## Why Agentile Exists

AI coding agents -- Claude Code, Cursor, Copilot, Devin -- share five failure modes:

| Failure Mode | What Happens | Agentile Solution |
|-------------|-------------|-------------------|
| **Context drift** | Agent forgets prior decisions, reimplements existing modules | `CURRENT.md` + `DAILY.md` as external memory |
| **Documentation debt** | Code outpaces documentation, creating misleading gaps | Rule 7: docs ship with code, same PR |
| **Test amnesia** | Features ship without tests, or with tests that pass trivially | Rule 3: test ratchet (count never decreases) |
| **Sprint sprawl** | Work expands beyond planned scope | Work Packages with acceptance criteria + hard gates |
| **Stub proliferation** | TODOs, mocks, and placeholders become permanent | Rule 2 + Rule 11: zero mock budget, data source tracing |

These are not theoretical concerns. Each failure mode occurred multiple times during the development of a 195,000-line production system and was addressed by a specific structural intervention.

---

## The 13 Rules

Every contributor -- human or AI -- follows these non-negotiable rules:

| # | Rule | Enforcement |
|---|------|-------------|
| 0 | Read AGENT_ENTRY.md before any work | BLOCKER |
| 1 | Plan before you code (sprint files first) | GATE |
| 2 | No mocks, stubs, or TODOs in production code | GATE |
| 3 | Test count only increases (test ratchet) | BLOCKER |
| 4 | Every feature traces to a sprint work package | GATE |
| 5 | All code passes linting with zero warnings | GATE |
| 6 | Audits are dated and immutable | BLOCKER |
| 7 | Documentation updates accompany code changes | GATE |
| 8 | Security-sensitive changes require review | BLOCKER |
| 9 | The sprint file is the source of truth | GATE |
| 10 | Formal verification for critical logic | GATE |
| 11 | Mock budget = 0, data source tracing | GATE |
| 12 | All documents must have timestamps + branch context | GATE |

Rules marked **BLOCKER** stop all work. Rules marked **GATE** stop the current step.

Full details: [`.agentile/rules/CORE_RULES.md`](.agentile/rules/CORE_RULES.md)

---

## The Zooid System (Contributor Roles)

Agentile assigns contributors to **zooids** -- role-based identities with defined permissions, tools, and gates. This prevents role confusion (the same agent trying to architect, code, test, and plan simultaneously) and creates clear boundaries.

| Zooid | Purpose | Min. ELO | Key Gate |
|-------|---------|----------|----------|
| **TECH_WRITER** | Documentation | 0+ (Novice) | Verify all claims against source code |
| **DEVELOPER** | Implementation | 800+ (Journeyman) | Red-Green-Refactor TDD cycle |
| **QA_ENGINEER** | Testing & quality | 800+ (Journeyman) | Coverage never decreases |
| **SCRUM_MASTER** | Sprint management | 1200+ (Expert) | CURRENT.md always up to date |
| **ARCHITECT** | System design | 1200+ (Expert) | ADR exists before code |
| **FORMAL_VERIFIER** | TLA+ verification | 1600+ (Master) | Zero invariant violations |

Contributors earn ELO through demonstrated work (+5 to +25 for positive events, -5 to -50 for negative). Higher ELO unlocks more zooids and greater autonomy. See [`.agentile/zooids/ELO_SYSTEM.md`](.agentile/zooids/ELO_SYSTEM.md).

---

## Workflows

Six documented workflows with hard gates between every step:

| Workflow | When to Use |
|----------|-------------|
| [**INIT**](.agentile/workflows/INIT.md) | Cold start -- entering the project with no context |
| [**SPRINT**](.agentile/workflows/SPRINT.md) | Full sprint lifecycle: Plan, Execute, Review, Retro, Archive |
| [**FEATURE**](.agentile/workflows/FEATURE.md) | Implementing any feature: Specify, Red, Green, Refactor, Verify, Document, Report |
| [**RETROFIT**](.agentile/workflows/RETROFIT.md) | Adopting Agentile in an existing codebase |
| [**REVIEW**](.agentile/workflows/REVIEW.md) | Quality gates for features, sprints, releases, and hotfixes |
| [**DEBUGGING**](.agentile/workflows/DEBUGGING.md) | Systematic debugging: Observe, Hypothesize, Verify, Isolate, Fix, Regress |

---

## Philosophy

### The Wittgenstein Insight

The framework's most important philosophical contribution is the recognition that **AI agents follow rules by pattern matching on surface features, not by understanding semantic intent**.

An agent told "no mocks" will avoid functions named `mock_*` while still returning hardcoded data structures that are functionally identical to mocks. This is not a failure of the rule. It is a demonstration of Wittgenstein's rule-following paradox: rules are not self-interpreting, and genuine rule-following requires community correction.

The solution is threefold:
1. **Mechanical enforcement** -- compile-time gates that do not require interpretation
2. **Integration tests as community** -- tests that use real data sources serve as external checks
3. **Human feedback as correction** -- the journal system institutionalizes community correction at sprint boundaries

Full essay: [`.agentile/docs/essays/THE_RULE_FOLLOWERS_PARADOX.md`](.agentile/docs/essays/THE_RULE_FOLLOWERS_PARADOX.md)

### The Compound Interest of Testing

Test investment compounds. The first test in a category costs 10 minutes (design the pattern, set up helpers). The 50th costs 30 seconds (copy, change input, change assertion). Each test you write makes every future test cheaper and more valuable.

Different test categories prevent different failures: unit tests catch regressions, adversarial tests document attack surfaces, property tests cover input spaces, integration tests verify system interactions, and formal specifications prevent design flaws.

Full essay: [`.agentile/docs/essays/THE_COMPOUND_INTEREST_OF_TESTING.md`](.agentile/docs/essays/THE_COMPOUND_INTEREST_OF_TESTING.md)

### The Handover Problem

AI agents have finite context windows. When a session ends, everything learned disappears. The Agentile journal system is not documentation for humans -- it is **state transfer between AI agents**. Each journal entry compresses hours of debugging into a map that the next agent can navigate in seconds.

The protocol: state the metrics, document the traps, leave the map, be honest about what is broken, give the timeline.

Full essay: [`.agentile/docs/essays/THE_HANDOVER.md`](.agentile/docs/essays/THE_HANDOVER.md)

### Mock Persistence

Temporary code becomes permanent through six mechanisms: context loss, "it works" inertia, missing definition of done, mock drift, flag rot, and coupling amplification. Knight Capital lost $440 million because a 9-year-old test program reactivated in production.

The Agentile response: zero mock budget, data source tracing in acceptance criteria, mock registries that block release builds, and formal specs as mock detectors.

Full case study: [`.agentile/docs/case_studies/MOCK_PERSISTENCE.md`](.agentile/docs/case_studies/MOCK_PERSISTENCE.md)

---

## Quick Start

### 1. Copy the framework into your project

```bash
# Clone this repo
git clone https://github.com/saulbuilds/agentile-framework.git

# Copy .agentile/ into your project
cp -r agentile-framework/.agentile your-project/.agentile
```

### 2. Configure for your project

Edit `.agentile/CONFIG.md` with your project's canonical values:
- Project name, token/product name
- Technology stack
- Module layout
- Build and test commands

Edit `.agentile/AGENT_ENTRY.md` with your project overview.

### 3. Customize the onboarding quiz

Edit `.agentile/onboarding/QUIZ_SPEC.md` to include questions relevant to your project's domain. The framework provides the quiz structure; you provide the questions.

### 4. Create your first sprint

```bash
mkdir -p .agentile/sprints/active/sprint-1-bootstrap
cp .agentile/templates/SPRINT.template.md .agentile/sprints/active/sprint-1-bootstrap/SPRINT.md
```

Edit the sprint file with your first work packages. Update `.agentile/sprints/CURRENT.md` to reference it.

### 5. Record your test baseline

Run your test suite and record the count in `.agentile/coverage/BASELINE.md`. This is the floor -- it can only go up.

### 6. Start building

Follow the [FEATURE workflow](.agentile/workflows/FEATURE.md): Specify, Red, Green, Refactor, Verify, Document, Report.

If you are retrofitting an existing codebase, follow the [RETROFIT workflow](.agentile/workflows/RETROFIT.md) instead.

---

## Directory Structure

```
.agentile/
├── AGENT_ENTRY.md              # Start here -- every session
├── CONFIG.md                   # Project constants (single source of truth)
├── MANIFEST.md                 # Index of all framework files
│
├── rules/                      # Non-negotiable operating rules
│   ├── CORE_RULES.md           # 13 rules (0-12)
│   ├── TDD_RULES.md            # Red-Green-Refactor with gates
│   ├── BDD_RULES.md            # Gherkin feature specs
│   ├── DOCUMENTATION_RULES.md  # Doc governance
│   ├── GIT_RULES.md            # Conventional commits, branches, PRs
│   └── FORMAL_VERIFICATION_RULES.md  # TLA+ requirements
│
├── zooids/                     # Contributor role identities
│   ├── ARCHITECT.md            # System design (Expert+)
│   ├── DEVELOPER.md            # Implementation (Journeyman+)
│   ├── QA_ENGINEER.md          # Testing (Journeyman+)
│   ├── SCRUM_MASTER.md         # Sprint management (Expert+)
│   ├── TECH_WRITER.md          # Documentation (Novice+)
│   ├── FORMAL_VERIFIER.md      # TLA+ verification (Master only)
│   ├── ELO_SYSTEM.md           # Scoring algorithm + tiers
│   └── REGISTRY.md             # Contributor scores (append-only)
│
├── onboarding/                 # New contributor assessment
│   ├── QUIZ_SPEC.md            # 5-question adaptive quiz
│   └── SKIP_PROTOCOL.md        # Fast track for experienced engineers
│
├── workflows/                  # Step-by-step procedures
│   ├── INIT.md                 # Cold start
│   ├── SPRINT.md               # Sprint lifecycle
│   ├── FEATURE.md              # Feature implementation loop
│   ├── RETROFIT.md             # Adopting agentile in existing code
│   ├── REVIEW.md               # Quality gates
│   └── DEBUGGING.md            # Systematic debugging (OHVIRF)
│
├── templates/                  # Copyable document templates
│   ├── SPRINT.template.md
│   ├── FEATURE.template.md
│   ├── ADR.template.md
│   ├── REPORT.template.md
│   ├── MODULE_README.template.md
│   └── PR.template.md
│
├── docs/                       # Methodology documentation
│   ├── AGENTILE_METHODOLOGY.md # Framework specification
│   ├── JOURNAL_RULES.md        # Sprint journaling rules
│   ├── essays/                 # Methodology insights
│   │   ├── THE_RULE_FOLLOWERS_PARADOX.md
│   │   ├── THE_COMPOUND_INTEREST_OF_TESTING.md
│   │   ├── THE_HANDOVER.md
│   │   └── THE_SECOND_PAIR_OF_EYES.md
│   ├── case_studies/
│   │   └── MOCK_PERSISTENCE.md
│   └── journals/               # Sprint journals (populated per project)
│
├── formal/                     # Formal verification
│   ├── VERIFICATION_WORKFLOW.md
│   ├── specs/                  # TLA+ specifications
│   └── models/                 # TLC model configurations
│
├── coverage/                   # Test tracking
│   ├── BASELINE.md             # Current test counts
│   └── GATES.md                # Minimum coverage requirements
│
├── sprints/                    # Sprint management
│   ├── CURRENT.md              # Active sprint dashboard
│   ├── active/                 # Current sprint directories
│   ├── backlog/                # Prioritized backlog items
│   └── completed/              # Archived sprint directories
│
└── audits/                     # Immutable audit reports
    └── README.md               # Audit directory instructions
```

---

## Origin

Agentile was developed by Larry Klosowski ([@saulbuilds](https://github.com/saulbuilds)) during the construction of a 195,000-line Layer-1 blockchain using AI coding assistants over a four-month period. The methodology emerged from 35+ sprints, 1,100+ story points, and the repeated observation that **the limiting factor in AI-assisted software engineering is not the capability of the AI but the discipline of the workflow.**

The essays in this repository were written during that development process and capture the insights that shaped the framework's evolution -- from the Wittgenstein paradox that explains why AI agents mock despite explicit rules, to the compound interest mathematics of test investment, to the handover protocol that enables continuity across agent sessions.

---

## License

MIT License. See [LICENSE](LICENSE).
