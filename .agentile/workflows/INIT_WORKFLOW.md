# INIT_WORKFLOW.md — Project Initialization

> Run this workflow when `.agentile/planset/` is empty and you're starting a new project.

## Prerequisites
- The human has described their project vision (in conversation or a seed document)
- The `.agentile/` folder is present with framework files
- `CONFIG.md` is filled in (or will be filled during this workflow)

## Steps

### Step 1: Gather Vision (BLOCKER if missing)
Read the human's initial prompt/conversation. Extract:
- What is being built?
- Who is it for?
- What are the core features?
- What tech stack is preferred?
- What's the MVP scope?

If any of these are unclear, ASK. This is a BLOCKER.

### Step 2: Fill CONFIG.md
Based on the conversation, populate `CONFIG.md` with:
- Project name, description
- Tech stack choices
- Testing framework preferences
- CI/CD targets

Present to human for approval. (OPINION level — proceed with recommendation if no response.)

### Step 3: Create Executive Summary
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

### Step 4: Create Architecture
Activate the ARCHITECT role and produce:

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

### Step 5: Create Initial Backlog
Based on the scope, create work items in `sprints/backlog/`:
- One markdown file per epic or major feature
- Each item should reference the relevant section of SCOPE.md
- Priority: High / Medium / Low
- Rough size: S / M / L / XL

### Step 6: Plan Sprint 1
Activate the SCRUM_MASTER role:
1. Pull the highest-priority items from backlog
2. Create `sprints/active/SPRINT-1.md` using the sprint template
3. Present to human for approval (BLOCKER)

### Step 7: Initialize Project Skeleton
Based on the architecture:
1. Initialize the project (e.g., `npm init`, `cargo init`, etc.)
2. Install core dependencies
3. Set up the test runner and verify it works
4. Create the initial folder structure
5. Write a basic `README.md` for the project root
6. Commit: `chore: initialize project with Agentile framework`

### Step 8: Begin First Feature
Transition to `FEATURE_WORKFLOW.md` for the first item in Sprint 1.

## Completion Criteria
- [ ] CONFIG.md is populated
- [ ] Planset has vision, scope, milestones, and architecture docs
- [ ] Backlog has prioritized work items
- [ ] Sprint 1 is planned and approved
- [ ] Project skeleton is initialized with working test runner
- [ ] First commit is made
