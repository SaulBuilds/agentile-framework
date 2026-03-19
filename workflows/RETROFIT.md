# Workflow: RETROFIT -- Adopting Agentile in an Existing Codebase

> Use this workflow when applying the agentile framework to code that was written before the framework existed.
> The goal is to establish a test baseline, fill coverage gaps, and transition to normal workflow operations.

---

## When to Use This Workflow

- First time applying agentile to a repository (or a section of it).
- Bringing a new module under agentile governance.
- After a period of unstructured work that needs to be regularized.

---

## Step 1: Initialize Framework

**What to do:**
1. Verify `.agentile/` directory structure exists with all required subdirectories.
2. Verify `CONFIG.md` is current with correct canonical values.
3. Verify `AGENT_ENTRY.md` routing table is accurate.
4. Create or verify `sprints/CURRENT.md` points to an active sprint.

**Verification checklist:**
```
.agentile/
├── AGENT_ENTRY.md        exists, routing table current
├── CONFIG.md             exists, values correct
├── rules/                all rule files present
├── workflows/            all workflow files present
├── templates/            all template files present
├── sprints/
│   ├── CURRENT.md        exists, points to active sprint
│   ├── active/           exists
│   ├── backlog/          exists
│   └── completed/        exists
├── coverage/             exists
├── audits/               exists
└── formal/               exists
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
1. Record the current passing test count in `coverage/BASELINE.md`:
   ```markdown
   # Coverage Baseline

   **Date:** YYYY-MM-DD
   **Total passing tests:** N
   **Breakdown by module:**

   | Module | Tests | Passing | Failing | Ignored |
   |--------|-------|---------|---------|---------|
   | module-1 | 245 | 245 | 0 | 2 |
   | module-2 | 189 | 187 | 2 | 0 |
   | ... | ... | ... | ... | ... |
   ```
2. This baseline is the floor. The test count must never drop below this number (Rule 3).

**GATE:** `coverage/BASELINE.md` exists with dated, accurate numbers. DO NOT PROCEED without a recorded baseline.

---

## Step 4: Fix Failing Tests

**What to do:**
1. Fix all currently failing tests. For each failing test:
   - If the test is correct and the code is wrong: fix the code.
   - If the test is outdated and the code is correct: update the test.
   - If the test is flaky: make it deterministic or mark it as ignored with a documented reason and a backlog item to fix it.
2. Run the full test suite until all tests pass (or all failures are documented as ignored with backlog items).

**GATE:** Full test suite exits 0. All ignored tests have documented reasons. DO NOT PROCEED with red tests.

---

## Step 5: Wrap Untested Code

**What to do:**
1. For each module with very few tests, create a retrofit sprint WP to add tests.
2. Prioritize by risk:
   - **High risk (test immediately):** consensus, execution, cryptography, key management
   - **Medium risk (test this sprint):** API, networking, sequencing
   - **Low risk (test next sprint):** CLI, documentation tooling, scripts
3. Write tests following TDD rules (even though the code exists, write the test first to verify your understanding, then confirm it passes).

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

---

## Common Retrofit Scenarios

### Scenario: Module has zero tests

1. Start with the module's main public API (the types and functions used by other modules).
2. Write tests for each public function's happy path.
3. Add error case tests for functions that return error types.
4. Add edge case tests (empty input, max values, boundary conditions).

### Scenario: Tests exist but are outdated

1. Run the tests to see which fail.
2. Read the test to understand the original intent.
3. Compare with the current code behavior.
4. Update the test to match current behavior (if behavior change was intentional) or fix the code (if it was a regression).

### Scenario: Tests exist but are flaky

1. Identify the source of non-determinism (timing, random values, shared state, file system).
2. Eliminate the non-determinism (use fixed seeds, mock time, isolate state).
3. If non-determinism cannot be eliminated, mark as ignored with reason and create a backlog item.
