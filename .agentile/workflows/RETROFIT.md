# Workflow: RETROFIT -- Adopting Agentile in an Existing Codebase

> Use this workflow when applying the agentile framework to code that was written before the framework existed.
> The goal is to establish a test baseline, fill coverage gaps, and transition to normal workflow operations.

---

## When to Use This Workflow

- First time applying agentile to your repository (or a section of it).
- Bringing a new module under agentile governance.
- After a period of unstructured work that needs to be regularized.

---

## Step 1: Initialize Framework

**What to do:**
1. Copy the `.agentile/` directory from the agentile-framework repository into your project root.
2. Fill in `CONFIG.md` with your project's canonical values.
3. Update `AGENT_ENTRY.md` with your project overview.
4. Create `sprints/CURRENT.md` pointing to your first sprint.

**Verification checklist:**
```
.agentile/
├── AGENT_ENTRY.md        ✓ exists, routing table current
├── CONFIG.md             ✓ exists, values correct
├── rules/                ✓ all rule files present
├── workflows/            ✓ all workflow files present
├── templates/            ✓ all template files present
├── sprints/
│   ├── CURRENT.md        ✓ exists, points to active sprint
│   ├── active/           ✓ exists
│   ├── backlog/          ✓ exists
│   └── completed/        ✓ exists
├── coverage/             ✓ exists
├── audits/               ✓ exists
└── formal/               ✓ exists
```

**GATE:** All framework files exist and are populated. DO NOT PROCEED with missing framework infrastructure.

---

## Step 2: Audit Existing Tests

**What to do:**
1. Run the full test suite and record results.
2. Count total tests, passing, failing, and ignored.
3. Identify untested modules (modules with zero or very few tests).
4. Identify flaky tests (tests that pass/fail inconsistently).

**Output:** A test audit summary documenting:
- Total test count per module
- Failing tests (with failure reasons)
- Ignored tests (with ignore reasons)
- Modules with very few tests (coverage gaps)

**GATE:** You have a complete inventory of the test landscape. Every failing test is documented with its failure reason. DO NOT PROCEED without knowing the current state.

---

## Step 3: Create Coverage Baseline

**What to do:**
1. Record the current passing test count in `coverage/BASELINE.md`.
2. This baseline is the floor. The test count must never drop below this number (Rule 3).

**GATE:** `coverage/BASELINE.md` exists with dated, accurate numbers. DO NOT PROCEED without a recorded baseline.

---

## Step 4: Fix Failing Tests

**What to do:**
1. Fix all currently failing tests. For each:
   - If the test is correct and the code is wrong: fix the code.
   - If the test is outdated and the code is correct: update the test.
   - If the test is flaky: make it deterministic or mark it as ignored with a documented reason and a backlog item to fix it.
2. Run tests until all pass (or all failures are documented as ignored with backlog items).

**GATE:** All tests pass. All ignored tests have documented reasons. DO NOT PROCEED with red tests.

---

## Step 5: Wrap Untested Code

**What to do:**
1. For each module with very few tests, create a retrofit sprint WP to add tests.
2. Prioritize by risk:
   - **High risk (test immediately):** Security-critical, financial logic, consensus
   - **Medium risk (test this sprint):** API, networking, data processing
   - **Low risk (test next sprint):** CLI, documentation tooling, scripts
3. Write tests following TDD rules.

**Output:** Sprint WPs for test gap coverage, prioritized by risk.

**GATE:** High-risk modules have tests covering their core public API. DO NOT PROCEED to normal feature work while high-risk code is untested.

---

## Step 6: Transition to Normal Workflow

**What to do:**
1. Mark the retrofit sprint as complete.
2. Verify the test ratchet: final count > baseline count.
3. Update `sprints/CURRENT.md` to point to a normal feature sprint.
4. From this point forward, follow the standard SPRINT and FEATURE workflows.

**GATE:** Retrofit sprint is archived. Test count has increased. Normal workflow is in effect. The codebase is under agentile governance.

---

## Flowchart

```
Initialize Framework
    │
    └── GATE: All framework files exist
        │
        Audit Existing Tests
        │
        └── GATE: Complete test inventory
            │
            Create Coverage Baseline
            │
            └── GATE: Baseline recorded
                │
                Fix Failing Tests
                │
                └── GATE: All tests pass
                    │
                    Wrap Untested Code
                    │
                    └── GATE: High-risk code covered
                        │
                        Transition to Normal Workflow
                        │
                        └── GATE: Retrofit complete, normal sprint active
```
