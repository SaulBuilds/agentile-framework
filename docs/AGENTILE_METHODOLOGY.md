# Agentile Methodology -- Formal Specification

**Version**: 1.0
**Status**: Stable

---

## 1. Abstract

Agentile is a software engineering methodology for projects where one or more AI coding agents collaborate with human engineers over sustained, multi-month timelines. It addresses the structural failures that occur when AI agents produce code without persistent memory, consistent documentation, or enforced quality gates.

The methodology comprises:
- A **directory specification** (`.agentile/`) that serves as the agent's external memory
- A **zooid system** of contributor identities with ELO-based competence scoring
- **Hard gates** that block phase transitions without evidence
- **Workflow definitions** for project initialization, sprint execution, feature implementation, legacy adoption, and quality review
- **Coverage tracking** via a test ratchet that enforces monotonically increasing test counts

---

## 2. Problem Statement

AI coding agents exhibit five failure modes when used on sustained projects. These are not theoretical; each has been observed repeatedly in real-world projects.

### 2.1 Context Drift

The agent loses track of decisions made in prior sessions. It reimplements existing modules, contradicts established architecture, or uses deprecated patterns. Root cause: agents have no persistent memory between conversation turns or sessions.

### 2.2 Documentation Debt

The agent produces code faster than documentation. Within weeks, the gap between what the system does and what is written down becomes a source of confusion for both the human and subsequent agent sessions. Root cause: documentation is not a reward signal in any AI coding benchmark.

### 2.3 Test Amnesia

The agent writes features without tests, writes tests that assert trivially true conditions, or writes tests that pass by hardcoding expected values. Root cause: test correctness is harder to verify than test existence; agents optimize for the latter.

### 2.4 Sprint Sprawl

Without structured planning, work expands to fill the available context window. A planned bug fix becomes a refactor becomes an architecture change. The human loses track of scope and the agent has no mechanism to self-limit. Root cause: agents are reward-driven toward producing more output, not toward staying within a plan.

### 2.5 Stub Proliferation

Under pressure to show progress, the agent produces TODO comments, mock implementations, and placeholder functions. These persist indefinitely because no gate prevents their creation, and they are invisible in test output. Root cause: stubs are syntactically valid and do not fail tests.

---

## 3. Framework Architecture

The `.agentile/` directory is the canonical home for all framework artifacts.

```
.agentile/
├── AGENT_ENTRY.md              # Entry point -- every contributor starts here
├── CONFIG.md                   # Canonical project constants
├── MANIFEST.md                 # Index of all framework files
│
├── rules/                      # Non-negotiable operating rules
│   ├── CORE_RULES.md           # 10 rules (0-9), each BLOCKER or GATE
│   ├── TDD_RULES.md            # Test-driven development cycle
│   ├── BDD_RULES.md            # Behavior-driven development (feature specs)
│   ├── DOCUMENTATION_RULES.md  # Documentation governance
│   ├── GIT_RULES.md            # Git conventions, PR templates
│   └── FORMAL_VERIFICATION_RULES.md  # TLA+ requirements
│
├── zooids/                     # Contributor identities
│   ├── ARCHITECT.md            # System design identity
│   ├── DEVELOPER.md            # Implementation identity
│   ├── QA_ENGINEER.md          # Quality assurance identity
│   ├── SCRUM_MASTER.md         # Sprint management identity
│   ├── TECH_WRITER.md          # Documentation identity
│   ├── FORMAL_VERIFIER.md      # Formal verification identity
│   ├── ELO_SYSTEM.md           # Scoring algorithm and tier definitions
│   └── REGISTRY.md             # Contributor scores (append-only)
│
├── onboarding/                 # Skill assessment
│   ├── QUIZ_SPEC.md            # 5-question adaptive quiz
│   └── SKIP_PROTOCOL.md        # Bypass for qualified engineers
│
├── workflows/                  # Step-by-step execution procedures
│   ├── INIT.md                 # Cold start -- first-time project setup
│   ├── SPRINT.md               # Sprint lifecycle (plan -> execute -> review)
│   ├── FEATURE.md              # Feature implementation loop
│   ├── RETROFIT.md             # Adopting agentile in existing codebases
│   └── REVIEW.md               # Quality gates for completion
│
├── templates/                  # Copyable document templates
│   ├── SPRINT.template.md
│   ├── FEATURE.template.md
│   ├── ADR.template.md
│   ├── REPORT.template.md
│   ├── CRATE_README.template.md
│   └── PR.template.md
│
├── docs/                       # Living documentation
│   ├── MODULE_INDEX.md         # All module READMEs indexed
│   └── AGENTILE_METHODOLOGY.md # This document
│
├── formal/                     # Formal verification
│   ├── SPEC_INDEX.md           # TLA+ spec inventory
│   └── VERIFICATION_WORKFLOW.md # When and how to write specs
│
├── coverage/                   # Test coverage tracking
│   ├── BASELINE.md             # Current test counts per module
│   └── GATES.md                # Minimum coverage requirements
│
├── sprints/                    # Sprint management
│   ├── CURRENT.md              # Active sprint status
│   ├── SPRINT_INDEX.md         # Complete history
│   ├── active/                 # Current sprint work
│   ├── backlog/                # Prioritized backlog
│   └── completed/              # Archived sprint directories
│
├── audits/                     # Immutable, dated audit records
└── reports/                    # Generated sprint reports
```

### 3.1 Entry Point Contract

`AGENT_ENTRY.md` is the first file any contributor reads. It routes contributors based on their status:

| Contributor Type | Route |
|-----------------|-------|
| New (human or AI) | Onboarding quiz -> zooid assignment -> workflow |
| Qualified engineer (no AI) | Skip protocol -> direct to sprint |
| Autonomous agent (cold start) | CONFIG.md -> CORE_RULES.md -> CURRENT.md -> pick task |
| Returning contributor | CURRENT.md -> resume active sprint |

**Hard gate**: No code may be written until AGENT_ENTRY.md and CONFIG.md have been read.

---

## 4. Zooid System

A zooid is a contributor identity with defined responsibilities, permissions, and quality expectations. The term is borrowed from colonial marine biology: a zooid is an individual organism that functions as part of a colonial organism.

### 4.1 Zooid Definitions

| Zooid | Responsibility | Minimum Tier |
|-------|---------------|-------------|
| **ARCHITECT** | System design, ADRs, dependency decisions, API contracts | Expert (1200+) |
| **DEVELOPER** | Feature implementation, bug fixes, refactoring | Journeyman (800+) |
| **QA_ENGINEER** | Test writing, coverage analysis, regression investigation | Journeyman (800+) |
| **SCRUM_MASTER** | Sprint planning, daily logs, retrospectives, gate enforcement | Expert (1200+) |
| **TECH_WRITER** | Documentation, READMEs, guides, changelogs | Novice (0+) |
| **FORMAL_VERIFIER** | TLA+ specifications, model checking, invariant design | Master (1600+) |

A contributor may hold qualifications for multiple zooids but operates as exactly one zooid at a time within a sprint.

### 4.2 ELO Scoring

The ELO system tracks contributor competence through demonstrated work, not credentials or seniority.

**Tiers**:

| Tier | ELO Range | Autonomy |
|------|-----------|----------|
| Novice | 0-799 | Guided work only. All PRs require Expert+ review. |
| Journeyman | 800-1199 | Standard workflow with mandatory review gates. |
| Expert | 1200-1599 | Relaxed gates. Can review others. Architecture input. |
| Master | 1600+ | Full autonomy. Can propose architecture. Can mentor. |

**Properties**:
- Points are applied immediately (no batching)
- ELO does not decay over time
- No contributor starts above Journeyman (1000 max from onboarding quiz)
- Expert and Master must be earned through contributions
- REGISTRY.md is append-only

---

## 5. Hard Gates

A hard gate is a precondition that must be satisfied before the agent or contributor may proceed. Gates are classified as BLOCKER (halts all work) or GATE (halts the current step).

### 5.1 Gate Inventory

| Gate | Type | Precondition | Evidence Required |
|------|------|-------------|------------------|
| **Session start** | BLOCKER | AGENT_ENTRY.md and CONFIG.md read | First action in session log |
| **Sprint planning** | GATE | Sprint file has goal, 2+ items with priority and size | Human approval |
| **Feature start** | GATE | WBS with acceptance criteria exists | File committed |
| **Code commit** | GATE | Linter clean, all tests pass | CI green |
| **Test ratchet** | BLOCKER | Test count >= previous sprint's count | Recorded in DAILY.md |
| **Sprint close** | GATE | Definition of Done fully checked | All checkboxes marked |
| **No stubs** | GATE | Grep for TODO/FIXME/STUB returns 0 hits in new code | Grep output |
| **Security review** | BLOCKER | Security-sensitive changes reviewed by Expert+ | PR approval metadata |
| **Documentation sync** | GATE | Code changes to public APIs include doc updates in same PR | PR diff |

---

## 6. Workflow Definitions

### 6.1 INIT -- First-Time Project Setup

Bootstrap a new project with the agentile directory structure. See `workflows/INIT.md`.

### 6.2 SPRINT -- Sprint Lifecycle

Execute a time-boxed unit of work with planning, execution, and review. See `workflows/SPRINT.md`.

### 6.3 FEATURE -- Feature Implementation Loop

Implement a single feature or work package within a sprint. See `workflows/FEATURE.md`.

### 6.4 RETROFIT -- Adopting Agentile in Existing Codebases

Add the agentile framework to a project that already has code. See `workflows/RETROFIT.md`.

### 6.5 REVIEW -- Quality Gates for Completion

Verify that a sprint, feature, or project milestone meets the quality bar. See `workflows/REVIEW.md`.

---

## 7. IDE Compatibility

Agentile is IDE-agnostic. The `.agentile/` directory is a set of markdown files that any tool can read and write.

| IDE / Tool | Integration Pattern |
|------------|-------------------|
| **Claude Code** | Reads `.agentile/AGENT_ENTRY.md` via `CLAUDE.md` instructions. Full workflow support. |
| **Cursor** | `.cursorrules` file references `.agentile/rules/CORE_RULES.md`. Agent follows workflows. |
| **GitHub Copilot** | `.github/copilot-instructions.md` references `.agentile/`. |
| **Windsurf** | `.windsurfrules` references `.agentile/`. |
| **Manual (no IDE)** | Developer reads markdown files directly. Uses templates for new artifacts. |

The framework does not depend on any IDE-specific feature. It depends only on the ability to read and write files in a git repository.

---

## 8. Coverage Tracking

### 8.1 The Test Ratchet

The test ratchet is the methodology's primary quality metric:

> The total number of passing automated tests must be recorded at the start and end of every sprint and must never decrease without a documented explanation committed to DAILY.md.

The ratchet creates a monotonically increasing quality floor. Deleting a test requires replacing it with an equivalent or better test in the same commit.

### 8.2 Coverage Gates

`coverage/GATES.md` defines minimum acceptable coverage per module category. Customize these thresholds for your project.

---

*Specification based on the Agentile methodology developed empirically across 35+ sprints of production software engineering.*
*Framework: Agentile -- a methodology for AI-assisted software engineering*
