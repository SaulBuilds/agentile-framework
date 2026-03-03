# TDD_RULES.md — Test-Driven Development Rules

## The Red-Green-Refactor Cycle

This is the heartbeat of Agentile development. Every feature goes through this exact cycle.

### Phase 1: RED (Write a Failing Test)
1. Read the Gherkin feature file
2. Write a test that implements ONE scenario from the feature
3. Run the test
4. **CONFIRM it fails** — If it passes without implementation, your test is wrong
5. Commit the failing test with message: `test(red): [feature-name] - [scenario-name]`

### Phase 2: GREEN (Make It Pass)
1. Write the MINIMUM code to make the failing test pass
2. No optimization. No elegance. Just make it work.
3. Run the test
4. **CONFIRM it passes**
5. Run the FULL test suite — nothing else should have broken
6. Commit with message: `feat(green): [feature-name] - [scenario-name]`

### Phase 3: REFACTOR (Clean Up)
1. Look at the implementation code. Is it clean? Readable? DRY?
2. Look at the test code. Is it clear? Maintainable?
3. Refactor as needed
4. Run the FULL test suite after EVERY change
5. **CONFIRM everything still passes**
6. Commit with message: `refactor: [what was improved]`

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

## Test Quality Rules

1. **One assertion per test** (when possible). Multiple assertions only for tightly coupled outcomes.
2. **Descriptive test names**: `should return 401 when credentials are invalid` not `test1`
3. **No test interdependency**: Tests must run in any order
4. **No production state**: Tests set up and tear down their own state
5. **Fast tests**: Unit tests should complete in milliseconds. Mock external dependencies.
6. **Test the behavior, not the implementation**: Don't test private methods directly.

## Coverage Requirements

Per `CONFIG.md` settings (default: 90%):
- All new code must meet the coverage target
- Coverage drops are treated as test failures
- The agent reports coverage after each feature completion

## When Tests Fail Unexpectedly

If a previously passing test fails:
1. **STOP** current work
2. Identify the breaking change
3. Fix the regression BEFORE continuing
4. Document the incident in `docs/AGENT_NOTES.md`

## Test Naming Convention

```
[unit|integration|e2e]/[module]/[feature].[test|spec].[ext]
```

Examples:
- `unit/auth/login.test.ts`
- `integration/api/user-endpoints.spec.ts`
- `features/auth/user-login.steps.ts`

## The Testing Pyramid

Prioritize in this order:
1. **Unit tests** — Fast, isolated, numerous (base of pyramid)
2. **Integration tests** — Verify components work together
3. **BDD/Feature tests** — Validate user-facing behavior
4. **E2E tests** — Sparingly, for critical user journeys only
