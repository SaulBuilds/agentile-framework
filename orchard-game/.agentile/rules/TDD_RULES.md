# TDD Rules -- Red-Green-Refactor with Gates

> Test-Driven Development is mandatory for all new production code.
> Each phase has a hard gate. DO NOT PROCEED to the next phase until the gate is satisfied.

---

## The Cycle

```
RED ──gate──> GREEN ──gate──> REFACTOR ──gate──> COMMIT
 │                                                  │
 └──────────── next feature ────────────────────────┘
```

---

## Phase 1: RED -- Write a Failing Test First

**What to do:**
1. Identify the behavior you are about to implement.
2. Write a test that exercises that behavior.
3. Run the test. It **must fail**.

**Rules:**
- The test must be specific. It tests one behavior, not the entire module.
- The test must fail for the right reason (missing function, wrong return value) -- not because of a syntax error or missing import.
- Name the test descriptively: `test_transfer_rejects_insufficient_balance`, not `test1`.

**GATE:** The test must exist and must fail. If the test passes before you write the implementation, either the behavior already exists (check for duplication) or the test is wrong. DO NOT PROCEED until you have a legitimate red test.

---

## Phase 2: GREEN -- Minimum Code to Pass

**What to do:**
1. Write the minimum production code that makes the failing test pass.
2. Run the full test suite for the affected module.
3. All tests -- including the new one -- must pass.

**Rules:**
- Write only enough code to satisfy the test. Do not add extra functionality, optimizations, or "while I'm here" changes.
- If the implementation requires touching multiple files, that is fine -- but all changes must be motivated by making the test pass.
- If the implementation reveals that the test was wrong, fix the test in this phase.

**GATE:** All tests in the affected module pass. DO NOT PROCEED to refactor while any test is red.

---

## Phase 3: REFACTOR -- Clean Up Without Changing Behavior

**What to do:**
1. Review the code you just wrote for duplication, unclear naming, excessive complexity, or missed abstractions.
2. Refactor freely -- rename, extract functions, simplify logic.
3. Run the full test suite again. All tests must still pass.

**Rules:**
- Do not add new behavior during refactor. If you discover a new behavior that needs to exist, start a new RED phase.
- Do not delete or skip tests during refactor. If a test is now redundant, replace it with a better test that covers the same behavior.
- Refactoring is not optional. Skipping it leads to code that works but nobody can maintain.

**GATE:** All tests pass after refactoring. The refactored code is cleaner than what you started with. DO NOT PROCEED to commit while any test is red.

---

## Phase 4: COMMIT -- Record the Work

**What to do:**
1. Run the full test suite.
2. Run the linter with warnings-as-errors.
3. Run the formatter.
4. Commit with a conventional commit message referencing the sprint WP.

**GATE:** All checks pass. The commit message follows Git conventions (see `GIT_RULES.md`). DO NOT PROCEED with a commit that has failing tests or lint warnings.

---

## Coverage Ratchet

At sprint boundaries, the total passing test count is recorded.

| Checkpoint | Action |
|------------|--------|
| Sprint start | Record baseline in `coverage/BASELINE.md` |
| Sprint end | Record final count in sprint `REPORT.md` |
| Comparison | Final count >= baseline count |

**BLOCKER:** If the test count has decreased at sprint end, the sprint cannot close. Investigate and restore the missing tests.

---

## When to Skip TDD

TDD is **mandatory** for:
- All new production logic
- All new API endpoints or commands
- All new service functions

TDD is **optional** (but recommended) for:
- Pure documentation changes
- Configuration file changes
- CI/CD pipeline changes
- Cosmetic UI changes with no logic

Even when TDD is optional, the test ratchet still applies -- you must not reduce the passing test count.

---

## Anti-Patterns

| Anti-Pattern | Why It Is Wrong | What to Do Instead |
|--------------|-----------------|---------------------|
| Writing tests after the code | Tests become confirmations, not specifications | Write the test first (RED) |
| Testing implementation details | Tests break on refactor | Test behavior and public API |
| Large integration tests only | Slow feedback, hard to pinpoint failures | Write small unit tests + targeted integration tests |
| Ignoring tests without justification | Hidden test debt | Fix the test or delete it (and replace with equivalent coverage) |
| Commenting out failing tests | Silently reduces coverage | Fix the code or fix the test -- never comment out |
