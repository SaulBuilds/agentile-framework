# INIT_WORKFLOW.md — Project Initialization

> Run this workflow when `.agentile/planset/` is empty and you're starting a new project.
> **This is a HARD GATE workflow. Every step has a completion gate. Do not skip steps. Do not proceed past a gate until it passes.**

---

## Prerequisites
- The `.agentile/` folder is present with framework files
- The agent has read `AGENT_ENTRY.md`, `MANIFEST.md`, and all `rules/` files

---

## Step 1: Gather Vision

**GATE TYPE: BLOCKER — Cannot proceed without human input.**

Read the human's initial prompt/conversation. Extract:
- What is being built?
- Who is it for?
- What are the core features?
- What tech stack is preferred?
- What's the MVP scope?

### If any of these are unclear, ASK:

Do not guess. Do not infer. Ask the user directly. Use questions like:
- "What are we building? Give me a one-paragraph description."
- "Who are the target users?"
- "What are the 3-5 must-have features for the first version?"
- "What tech stack do you want? (language, framework, database, hosting)"
- "What does 'done' look like for v1?"

If the user gives short or vague answers, ask follow-up questions that help them think through the product. Example follow-ups:
- "You mentioned [X] — does that mean [A] or [B]?"
- "For the [feature], should it support [edge case]?"
- "Do you have a preference between [option A] and [option B], or should I recommend one?"

**GATE: Do not proceed to Step 2 until you can answer all five vision questions. Write the answers down in your response before moving on.**

---

## Step 2: Fill CONFIG.md

Based on the conversation, populate `CONFIG.md` with:
- Project name, description
- Tech stack choices
- Testing framework preferences
- CI/CD targets

Present to human for approval. (OPINION level — proceed with recommendation if no response.)

**GATE: CONFIG.md must have all fields populated. Verify by reading the file back. Empty fields = gate fails.**

---

## Step 3: Create Executive Summary

Write the following documents in `planset/executive-summary/`:

**VISION.md**
- Project purpose and goals
- Target users/audience
- Key differentiators
- Success criteria

**SCOPE.md**
- In-scope features (prioritized)
- Explicitly out-of-scope items
- MVP vs future phases

**MILESTONES.md**
- Phase 1 (MVP) deliverables
- Phase 2+ aspirational features
- Rough sprint mapping

**GATE: All three files must exist and contain substantive content (not just headers). Verify each file has real content before proceeding.**

---

## Step 4: Create Architecture

Activate the ARCHITECT role and produce in `planset/architecture/`:

**SYSTEM_DESIGN.md**
- High-level component diagram (described in text/ASCII)
- Component responsibilities
- Data flow between components

**TECH_STACK.md**
- Chosen technologies with rationale (ADR for each major choice)
- Dependencies list
- Development tooling

**DATA_MODEL.md** (if applicable)
- Core entities and relationships
- Database schema overview

**API_DESIGN.md** (if applicable)
- Core endpoints/routes
- Request/response contracts

**GATE: SYSTEM_DESIGN.md and TECH_STACK.md are mandatory. DATA_MODEL.md and API_DESIGN.md are required if the project has a database or API. Verify each required file exists and contains a complete design, not placeholders.**

---

## Step 5: Create Initial Backlog

Based on the scope, create work items in `sprints/backlog/`:
- One markdown file per epic or major feature
- Each item should reference the relevant section of SCOPE.md
- Priority: High / Medium / Low
- Rough size: S / M / L / XL

**GATE: At least 3 backlog items must exist. Each must have a priority and size. Each must reference a scope item. Verify before proceeding.**

---

## Step 6: Plan Sprint 1

Activate the SCRUM_MASTER role:
1. Pull the highest-priority items from backlog
2. Create `sprints/active/SPRINT-1.md` using the sprint template
3. Present to human for approval (BLOCKER)

**GATE: `sprints/active/SPRINT-1.md` must exist and be populated with at least 2 items. Human must approve before execution begins.**

---

## Step 7: Initialize Project Skeleton

Based on the architecture:
1. Initialize the project (e.g., `npm init`, `cargo init`, etc.)
2. Install core dependencies
3. Set up the test runner and verify it works
4. Create the initial folder structure
5. Write a basic `README.md` for the project root (or update the existing one)
6. Commit: `chore: initialize project with Agentile framework`

**GATE: The test runner must execute successfully (even with zero tests). The project must build/run without errors. Verify both before proceeding.**

---

## Step 8: Begin First Feature

Transition to `FEATURE_WORKFLOW.md` for the first item in Sprint 1.

---

## Completion Checklist

**Every item must be checked. If any item fails, go back and fix it.**

- [ ] CONFIG.md is fully populated (no empty fields)
- [ ] `planset/executive-summary/VISION.md` exists with real content
- [ ] `planset/executive-summary/SCOPE.md` exists with real content
- [ ] `planset/executive-summary/MILESTONES.md` exists with real content
- [ ] `planset/architecture/SYSTEM_DESIGN.md` exists with real content
- [ ] `planset/architecture/TECH_STACK.md` exists with real content
- [ ] `sprints/backlog/` has at least 3 prioritized items
- [ ] `sprints/active/SPRINT-1.md` exists and is approved
- [ ] Project skeleton is initialized with working test runner
- [ ] Initial commit is made

**Only when ALL items pass is initialization complete. Until then, the project is in cold-start and no feature work should begin.**
