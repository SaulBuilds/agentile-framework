# TDD_RULES.md — Test-Driven Development Rules

> **Every phase transition in the Red-Green-Refactor cycle is an enforceable gate. Do not advance without proof that the current gate passes.**

---

## The Red-Green-Refactor Cycle

This is the heartbeat of Agentile development. Every feature goes through this exact cycle. No exceptions.

### Phase 1: RED (Write a Failing Test)

1. Read the Gherkin feature file
2. Write a test that implements ONE scenario from the feature
3. Run the test
4. **CONFIRM it fails** — If it passes without implementation, your test is wrong
5. Commit the failing test with message: `test(red): [feature-name] - [scenario-name]`

**GATE: You MUST run the test and show that it fails. Paste or reference the failing output. If a test passes without implementation, the test is wrong — fix it before proceeding. Do NOT move to GREEN until you have proven RED.**

### Phase 2: GREEN (Make It Pass)

1. Write the MINIMUM code to make the failing test pass
2. No optimization. No elegance. Just make it work.
3. Run the test
4. **CONFIRM it passes**
5. Run the FULL test suite — nothing else should have broken
6. Commit with message: `feat(green): [feature-name] - [scenario-name]`

**GATE: Both the new test AND the full suite must pass. Show passing output. If ANY test fails (including existing tests), you have a regression — fix it before committing. Zero regressions allowed at GREEN.**

### Phase 3: REFACTOR (Clean Up)

1. Look at the implementation code. Is it clean? Readable? DRY?
2. Look at the test code. Is it clear? Maintainable?
3. Refactor as needed
4. Run the FULL test suite after EVERY change
5. **CONFIRM everything still passes**
6. Commit with message: `refactor: [what was improved]`

**GATE: Full suite must pass after every refactor change. If you break a test during refactor, undo the refactor and try a different approach. A refactor that breaks tests is not a refactor — it's a regression.**

---

## Test Organization

Mirror the feature file structure:
```
tests/
├── features/          # BDD step definitions
│   ├── auth/
│   │   ├── user-login.steps.ts
│   │   └── user-registration.steps.ts
│   └── dashboard/
│       └── data-display.steps.ts
├── unit/              # Unit tests
│   ├── services/
│   └── utils/
├── integration/       # Integration tests
│   └── api/
└── e2e/               # End-to-end tests (if applicable)
```

---

## Test Quality Rules

1. **One assertion per test** (when possible). Multiple assertions only for tightly coupled outcomes.
2. **Descriptive test names**: `should return 401 when credentials are invalid` not `test1`
3. **No test interdependency**: Tests must run in any order
4. **No production state**: Tests set up and tear down their own state
5. **Fast tests**: Unit tests should complete in milliseconds. Mock external dependencies.
6. **Test the behavior, not the implementation**: Don't test private methods directly.

**GATE: Before a feature passes review, verify every test against these six rules. Tests that violate any rule must be rewritten.**

---

## Test Completeness Gate

**Before a feature is considered tested, ALL must be true:**

- [ ] Every Gherkin scenario has a corresponding test
- [ ] Happy-path scenarios have tests
- [ ] Error/edge-case scenarios have tests
- [ ] Boundary conditions are tested (null, empty, max, min, off-by-one)
- [ ] Tests run independently (no shared mutable state)
- [ ] Test names clearly describe the behavior being verified

**GATE: Count the Gherkin scenarios. Count the tests. If tests < scenarios, you're not done. If edge cases from the feature file aren't tested, you're not done.**

---

## Coverage Requirements

Per `CONFIG.md` settings (default: 90%):
- All new code must meet the coverage target
- Coverage drops are treated as test failures
- The agent reports coverage after each feature completion

**GATE: Run the coverage tool after GREEN phase. If coverage is below the CONFIG.md target, write additional tests before proceeding to REFACTOR. Coverage below target blocks phase advancement.**

---

## When Tests Fail Unexpectedly

If a previously passing test fails:
1. **STOP** current work immediately
2. Identify the breaking change
3. Fix the regression BEFORE continuing with anything else
4. Document the incident in `docs/AGENT_NOTES.md`

**GATE: A regression is a P0 blocker. Everything stops until it's fixed. Do not continue feature work, do not commit, do not move on. Fix the regression first.**

---

## Test Naming Convention

```
[unit|integration|e2e]/[module]/[feature].[test|spec].[ext]
```

Examples:
- `unit/auth/login.test.ts`
- `integration/api/user-endpoints.spec.ts`
- `features/auth/user-login.steps.ts`

---

## The Testing Pyramid

Prioritize in this order:
1. **Unit tests** — Fast, isolated, numerous (base of pyramid)
2. **Integration tests** — Verify components work together
3. **BDD/Feature tests** — Validate user-facing behavior
4. **E2E tests** — Sparingly, for critical user journeys only

---

## Prohibited Practices

The following are hard failures in any review:

- Writing implementation code before a failing test exists → **UNDO AND START OVER**
- Committing tests that are skipped, pending, or commented out → **REMOVE OR COMPLETE THEM**
- Tests that pass by accident (testing nothing, always-true assertions) → **FIX OR DELETE**
- Tests with hardcoded timeouts or sleep-based synchronization → **USE PROPER ASYNC PATTERNS**
- Tests that depend on network, filesystem, or external state without mocking → **ISOLATE THEM**
