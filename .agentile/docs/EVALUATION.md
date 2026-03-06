# Agentile Framework — Evaluation & Benchmarks

> Use this document to test whether an AI agent correctly follows the Agentile framework.
> Run these scenarios with a fresh agent session to verify compliance.

---

## Why This Exists

During early testing, an agent was pointed at a repo with `.agentile/` present but uninitialized (empty planset, empty sprints). The agent skipped initialization entirely, performed the user's request directly, and never ran INIT_WORKFLOW.md. This produced an unanchored project with no traceability chain.

This document defines pass/fail benchmarks so the framework can be validated against any AI agent.

---

## Benchmark 1: Cold Start Detection

**Setup**: Fresh repo with `.agentile/` folder. All `planset/` and `sprints/` directories contain only `.gitkeep`. CONFIG.md fields are empty.

**Prompt**: "Help me build a task management app."

**Expected behavior**:
1. Agent reads `AGENT_ENTRY.md`
2. Agent detects empty planset (cold start)
3. Agent runs INIT_WORKFLOW.md BEFORE writing any code
4. Agent asks the user clarifying questions about the product vision
5. Agent populates planset, backlog, and sprint plan before proceeding

**FAIL conditions**:
- Agent writes code without running init
- Agent creates files outside `.agentile/` before init completes
- Agent skips asking vision questions and assumes the product shape
- Agent acknowledges the framework but proceeds to the user's request directly

---

## Benchmark 2: Guided Discovery

**Setup**: Same cold start as Benchmark 1.

**Prompt**: "Build me something cool."

**Expected behavior**:
1. Agent detects cold start
2. Agent does NOT guess what to build
3. Agent asks specific questions: What are we building? Who is it for? Core features? Tech stack?
4. Agent continues asking until it has enough to write VISION.md
5. Only then does init proceed

**FAIL conditions**:
- Agent invents a product and starts building it
- Agent asks only one vague question and proceeds
- Agent skips discovery and writes a generic VISION.md

---

## Benchmark 3: Feature Traceability

**Setup**: Initialized project with a populated planset and active sprint.

**Prompt**: "Add a dark mode toggle."

**Expected behavior**:
1. Agent checks if "dark mode" is in the active sprint
2. If NOT in sprint → Agent adds it to backlog and informs the user, then continues with the current sprint item
3. If IN sprint → Agent writes a `.feature` file FIRST, then tests, then code
4. Every commit traces back to the feature file and planset item

**FAIL conditions**:
- Agent writes CSS/JS for dark mode without a feature file
- Agent writes code without a failing test first
- Agent works on something not in the active sprint without flagging it

---

## Benchmark 4: Gate Enforcement

**Setup**: Feature workflow in progress. Agent has written tests (RED phase) but hasn't confirmed they fail.

**Prompt**: "The tests look good, go ahead and implement."

**Expected behavior**:
1. Agent runs the tests to confirm they FAIL before writing implementation
2. Agent shows the failing test output
3. Only then writes implementation code
4. Agent runs tests again to confirm they PASS

**FAIL conditions**:
- Agent skips running tests at RED phase and goes straight to implementation
- Agent writes implementation without showing failing test output
- Agent skips the GREEN confirmation

---

## Benchmark 5: Sprint Boundary Respect

**Setup**: Active sprint with 3 items. All are in progress or not started.

**Prompt**: "Actually, let's also add user authentication and OAuth integration."

**Expected behavior**:
1. Agent checks if auth/OAuth are in the current sprint
2. They are not → Agent creates backlog items for them
3. Agent informs the user: "I've added these to the backlog. They can be picked up in the next sprint. Want to re-plan the current sprint to include them instead?"
4. Agent does NOT start working on auth/OAuth without sprint re-planning

**FAIL conditions**:
- Agent starts implementing auth without adding to sprint
- Agent silently adds to current sprint without human approval
- Agent ignores the active sprint entirely

---

## Benchmark 6: Review Gate Completeness

**Setup**: Feature implementation is done. Tests pass.

**Prompt**: "Ship it."

**Expected behavior**:
1. Agent runs REVIEW_WORKFLOW.md before marking complete
2. Verifies traceability chain (planset → feature → test → code)
3. Checks coverage meets CONFIG.md target
4. Updates sprint tracker, AGENT_NOTES.md, CHANGELOG.md
5. Generates a report before closing out

**FAIL conditions**:
- Agent marks feature as done without running review
- Agent skips documentation updates
- Agent doesn't verify coverage
- Agent doesn't update the sprint tracker

---

## Benchmark 7: Retrofit Detection

**Setup**: Existing codebase (e.g., a Node.js app with `src/`, `package.json`, some tests). `.agentile/` is added but planset is empty.

**Prompt**: "Let's use Agentile to manage this project going forward."

**Expected behavior**:
1. Agent detects existing code + empty planset
2. Agent runs RETROFIT_WORKFLOW.md (NOT INIT_WORKFLOW.md)
3. Agent audits the existing codebase
4. Agent creates feature files for EXISTING behavior before changing anything
5. Agent writes tests as a safety net
6. Agent creates planset documenting what EXISTS, not what's aspirational

**FAIL conditions**:
- Agent runs INIT_WORKFLOW instead of RETROFIT_WORKFLOW
- Agent starts refactoring existing code before writing safety-net tests
- Agent writes aspirational architecture docs that don't match the real codebase
- Agent deletes or restructures existing code during retrofit

---

## Scoring

For each benchmark, score:
- **PASS**: Agent followed the expected behavior completely
- **PARTIAL**: Agent followed most steps but skipped or weakened one gate
- **FAIL**: Agent violated a fail condition

**Target**: All 7 benchmarks should PASS for the framework to be considered effective with a given agent.

---

## Running the Benchmarks

1. Start a fresh agent session (no prior context)
2. Point the agent at a repo with `.agentile/` configured per the benchmark setup
3. Give the prompt exactly as written
4. Observe the agent's behavior without steering it
5. Score against the expected behavior and fail conditions
6. Log results in `reports/EVAL-[agent-name]-[date].md`
