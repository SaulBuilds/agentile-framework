# Workflow: INIT -- Cold Start

> Follow this workflow when entering the project with no prior context.
> Every step has a gate. Do not skip steps.

---

## Step 1: Read AGENT_ENTRY.md

Read `.agentile/AGENT_ENTRY.md` to understand the project identity, framework structure, and routing table.

**Output:** You know what the project is, what it does, and where to find everything.

**GATE:** You can describe the project in one sentence and name its primary components. If not, re-read. DO NOT PROCEED.

---

## Step 2: Onboarding Quiz (or Skip Protocol)

If you are a new contributor, take the quiz at `onboarding/QUIZ_SPEC.md` to determine your skill tier and starting role.

If you are a returning contributor or autonomous agent with demonstrated project knowledge, follow the skip protocol at `onboarding/SKIP_PROTOCOL.md`.

**Output:** You have a skill tier (Novice/Journeyman/Expert/Master) and know your role boundaries.

**GATE:** You have either completed the quiz or executed the skip protocol. DO NOT PROCEED without a tier assignment.

---

## Step 3: Read CONFIG.md

Read `.agentile/CONFIG.md` for canonical project constants:
- Project identity and naming conventions
- Technology stack
- Workspace layout
- Build and test commands

**Output:** You know the canonical values and will use them correctly in all code and documentation.

**GATE:** You can list the core modules and their paths. You know how to run tests. DO NOT PROCEED without this knowledge.

---

## Step 4: Read CORE_RULES.md

Read `.agentile/rules/CORE_RULES.md` for the 13 non-negotiable rules.

Key rules to internalize:
- No mocks, stubs, or TODOs (Rule 2)
- Test count only increases (Rule 3)
- All code must pass linting clean (Rule 5)
- Sprint file is source of truth (Rule 9)

**Output:** You understand the operating constraints.

**GATE:** You accept and will follow all 13 rules. If any rule is unclear, read it again. DO NOT PROCEED with ambiguity about the rules.

---

## Step 5: Check sprints/CURRENT.md

Read `.agentile/sprints/CURRENT.md` to understand:
- Which sprint is active
- What work packages exist
- What is complete, in progress, or blocked

**Output:** You know the current state of the project and what work is happening.

**GATE:** You can describe the active sprint's goal and list its work packages. DO NOT PROCEED if you do not know what sprint is active.

---

## Step 6: Pick a Task

Choose a task from one of these sources (in priority order):

1. **Active sprint** -- unstarted or in-progress work packages in the current sprint's `SPRINT.md`
2. **Backlog** -- prioritized items in `sprints/backlog/`
3. **Known issues** -- bugs or regressions identified in previous sprint retros

**Output:** You have a specific task with a WP identifier.

**GATE:** Your task is recorded in a sprint file. If it is not, create the sprint entry first (see `SPRINT.md` workflow). DO NOT PROCEED with untracked work (Rule 4).

---

## Step 7: Follow the Appropriate Workflow

| Task Type | Workflow |
|-----------|----------|
| New feature | [FEATURE.md](FEATURE.md) |
| Bug fix | [FEATURE.md](FEATURE.md) (same cycle, simpler spec) |
| Sprint planning | [SPRINT.md](SPRINT.md) |
| Adopting agentile in new area | [RETROFIT.md](RETROFIT.md) |
| Quality review | [REVIEW.md](REVIEW.md) |
| Debugging | [DEBUGGING.md](DEBUGGING.md) |

**Output:** You are executing a defined workflow with clear gates.

**GATE:** You are following a documented workflow. Ad-hoc coding without a workflow is prohibited. DO NOT PROCEED without selecting a workflow.

---

## Step 8: Report Results

After completing your task:

1. Update the sprint `SPRINT.md` with the task status (complete/blocked/in-progress).
2. Update `DAILY.md` with what was done, test counts, and any blockers.
3. If the sprint is complete, write a `REPORT.md` and `RETRO.md`.

**Output:** The sprint file reflects reality. Future contributors can pick up where you left off.

**GATE:** Your work is recorded. Unrecorded work did not happen (Rule 9).

---

## Flowchart

```
START
  │
  ├── Read AGENT_ENTRY.md
  │     └── GATE: Know project identity
  │
  ├── Onboarding / Skip Protocol
  │     └── GATE: Have tier assignment
  │
  ├── Read CONFIG.md
  │     └── GATE: Know canonical values
  │
  ├── Read CORE_RULES.md
  │     └── GATE: Accept all rules
  │
  ├── Check CURRENT.md
  │     └── GATE: Know active sprint
  │
  ├── Pick task
  │     └── GATE: Task is tracked in sprint
  │
  ├── Execute workflow
  │     └── GATE: Following a documented workflow
  │
  └── Report results
        └── GATE: Sprint file updated
```
