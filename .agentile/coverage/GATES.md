# Coverage Gates

> Minimum test requirements by zooid role and action type.

## Per-Zooid Gates

| Zooid | Gate | Enforcement |
|-------|------|-------------|
| **DEVELOPER** | Test count must not decrease after any commit | BLOCKER |
| **DEVELOPER** | New features must include at least 1 test per public function | BLOCKER |
| **QA_ENGINEER** | Sprint test count must increase by >= 10 | GATE |
| **QA_ENGINEER** | Fuzz targets must cover all serialization boundaries | GATE |
| **FORMAL_VERIFIER** | Critical changes must have TLA+ spec with 0 violations | BLOCKER |

## Per-Action Gates

| Action | Gate | How to Verify |
|--------|------|---------------|
| **Feature PR** | All existing tests pass | Run test suite |
| **Feature PR** | At least 1 new test for the feature | Check diff for test additions |
| **Sprint Completion** | Total test count >= baseline | Compare against `BASELINE.md` |
| **Release** | Full test suite passes | Run full suite |
| **Release** | Linter clean | Run linter with warnings-as-errors |
| **Release** | 0 TODOs in production code | Grep for TODO/FIXME |

## How to Record Coverage

After each sprint, update `BASELINE.md` with:
1. New per-module test counts
2. Delta from previous baseline
3. Date and sprint reference
