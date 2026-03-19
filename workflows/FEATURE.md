# Workflow: FEATURE -- Feature Implementation Loop

> The standard workflow for implementing any feature, bug fix, or enhancement.
> Each step has a hard gate. DO NOT PROCEED to the next step until the gate is satisfied.

---

## Overview

```
SPECIFY ──gate──> RED ──gate──> GREEN ──gate──> REFACTOR ──gate──> VERIFY ──gate──> DOCUMENT ──gate──> REPORT
```

---

## Step 1: SPECIFY

**What to do:**
- Identify the behavior to implement from the sprint WP.
- For new user-facing features: write a Gherkin `.feature` file (see `rules/BDD_RULES.md`).
- For bug fixes and internal work: write a plain-language description of the expected behavior in the sprint `DAILY.md`.

**Outputs:**
- Clear description of what the code should do.
- Acceptance criteria (from sprint WP or feature file).
- For user-facing features: `.feature` file in `features/<domain>/`.

**GATE:** You can state in one sentence what the code will do and how you will know it works. If you cannot, your spec is incomplete. DO NOT PROCEED without a clear specification.

---

## Step 2: RED -- Write a Failing Test

**What to do:**
- Write a test that exercises the behavior described in Step 1.
- Run the test. It must fail.
- The failure must be for the right reason (missing behavior, wrong output) -- not a compile error or missing import.

**Rules:**
- One test per behavior. Do not write a mega-test covering everything.
- Name tests descriptively: `test_checkpoint_quorum_requires_67_votes`.
- Place tests in the appropriate location (see `rules/TDD_RULES.md`).

**Outputs:**
- At least one new failing test.

**GATE:** The test exists and fails for the right reason. DO NOT PROCEED to implementation until you have a red test.

---

## Step 3: GREEN -- Implement Minimum Code

**What to do:**
- Write the minimum production code that makes the failing test pass.
- Do not add extra functionality beyond what the test requires.
- Run the full module test suite.

**Rules:**
- All tests in the module must pass -- not just the new one.
- If making the test pass requires changes to other modules, run their tests too.
- No mocks, stubs, or TODOs (Rule 2).

**Outputs:**
- Production code that makes the test pass.
- All module tests green.

**GATE:** Module test suite exits 0. All tests pass including the new one. DO NOT PROCEED while any test is red.

---

## Step 4: REFACTOR -- Clean Up

**What to do:**
- Review the code from Step 3 for duplication, unclear naming, excessive complexity.
- Refactor freely: rename, extract functions, simplify logic, improve error messages.
- Run the full module test suite again.

**Rules:**
- Do not add new behavior. If you find new behavior needed, go back to Step 2 (RED).
- Do not remove tests. If a test is redundant, replace it with an equivalent or better test.

**Outputs:**
- Cleaner code that passes all tests.

**GATE:** Module test suite exits 0 after refactoring. The code is cleaner than before. DO NOT PROCEED with messy code.

---

## Step 5: VERIFY -- Full Verification

**What to do:**
- Run the full workspace test suite
- Run the linter with warnings-as-errors
- Run the formatter in check mode
- Run frontend tests (if applicable)
- Run contract tests (if applicable)
- Verify test count has increased (or at minimum stayed the same).

**Outputs:**
- All checks pass.
- Test count recorded.

**GATE:** All checks exit 0. Test count >= previous count. DO NOT PROCEED with any failures.

---

## Step 6: DOCUMENT -- Update Documentation

**What to do:**
- If the change affects a public API: update the module README.
- If the change is user-facing: add a CHANGELOG entry.
- If the change affects architecture: update relevant docs or create an ADR.
- Add documentation comments to all new public items.

**Outputs:**
- Documentation is current with the code.

**GATE:** All new public items have doc comments. README and CHANGELOG are updated where required (see `rules/DOCUMENTATION_RULES.md`). DO NOT PROCEED without documentation.

---

## Step 7: REPORT -- Record the Work

**What to do:**
- Commit the work following `rules/GIT_RULES.md` conventions.
- Update the sprint `SPRINT.md` WP status to `[x] COMPLETE`.
- Update `DAILY.md` with what was done and the current test count.
- Record the commit hash in the sprint file.

**Outputs:**
- Clean commit with conventional message.
- Sprint file reflects the completed work.

**GATE:** The commit is pushed (or ready to push). The sprint file is updated. The work is traceable from commit to WP to sprint goal. DO NOT PROCEED to the next feature without recording this one.

---

## Iteration

If the feature requires multiple behaviors:

```
SPECIFY ──> RED ──> GREEN ──> REFACTOR ──> (repeat for next behavior)
                                              │
                                              └── when all behaviors done:
                                                    VERIFY ──> DOCUMENT ──> REPORT
```

You may batch the VERIFY, DOCUMENT, and REPORT steps across multiple RED-GREEN-REFACTOR cycles within the same feature, as long as you run verification before committing.

---

## Quick Reference

| Step | Action | Gate |
|------|--------|------|
| SPECIFY | Define behavior + acceptance criteria | Can state what and how to verify |
| RED | Write failing test | Test fails for right reason |
| GREEN | Minimum code to pass | All module tests pass |
| REFACTOR | Clean up code | All module tests still pass |
| VERIFY | Full workspace verification | All checks exit 0, test count up |
| DOCUMENT | Update docs | Public items documented, README current |
| REPORT | Commit + update sprint file | Sprint file reflects reality |
