# Workflow: REVIEW -- Quality Gates

> This workflow defines the quality gates for feature completion, sprint completion, and release readiness.
> Each gate is a checklist. All items must pass before proceeding.

---

## Feature Review Gate

Run this checklist before marking any Work Package as `[x] COMPLETE`.

### Checklist

```
FEATURE REVIEW -- WP-<id>: <name>
Date: YYYY-MM-DD

Code Quality
[ ] All tests pass: project test command
[ ] Full workspace tests pass
[ ] Lint clean: linter with warnings-as-errors
[ ] Formatted: formatter in check mode
[ ] No TODOs, FIXMEs, or stubs in new code
[ ] No lint suppressions without documented justification

Test Quality
[ ] New behavior has at least one test
[ ] Test count increased (or stayed the same with documented reason)
[ ] Tests are deterministic (no flaky tests introduced)
[ ] Test names are descriptive

Documentation
[ ] New public items have doc comments
[ ] Module README updated (if API changed)
[ ] CHANGELOG entry added (if user-facing change)

Security (if applicable)
[ ] No hardcoded secrets or credentials
[ ] Input validation for all external inputs
[ ] Error messages do not leak internal state
[ ] Reviewed by another contributor (for security-sensitive changes)

Formal Verification (if applicable)
[ ] TLA+ spec updated (for consensus changes)
[ ] TLC model check passes
[ ] Invariant tests written (for state machine changes)
```

**GATE:** All applicable items must be checked. DO NOT PROCEED to mark the WP as complete until this checklist passes.

---

## Sprint Review Gate

Run this checklist before closing a sprint and proceeding to the retrospective.

### Checklist

```
SPRINT REVIEW -- Sprint <id>: <name>
Date: YYYY-MM-DD

Completion
[ ] All WPs are marked [x] COMPLETE or [!] BLOCKED with documented reason
[ ] Blocked WPs have been moved to the backlog for the next sprint
[ ] Sprint goal is achieved (or documented as partially achieved with reason)

Quality
[ ] Full test suite passes
[ ] Lint clean
[ ] Formatted
[ ] Frontend tests pass (if applicable)
[ ] Contract tests pass (if applicable)

Test Ratchet
[ ] Final test count: ______
[ ] Baseline test count: ______
[ ] Final >= Baseline: YES / NO
[ ] If NO: documented reason and remediation plan

Documentation
[ ] DAILY.md has entries for all work days
[ ] SPRINT.md WP statuses are current
[ ] Module READMEs updated where applicable
[ ] CHANGELOG updated for user-facing changes

Process
[ ] All commits follow conventional commit format
[ ] All commits reference a sprint WP
[ ] AI contributions have Co-Authored-By footer
```

**BLOCKER:** The sprint cannot close if:
- Test count has decreased (Rule 3)
- Any test is failing
- SPRINT.md does not reflect reality

Fix these issues before proceeding.

---

## Release Review Gate

Run this checklist before tagging a release.

### Checklist

```
RELEASE REVIEW -- v<version>
Date: YYYY-MM-DD

Test Suite
[ ] Full test suite passes
[ ] Frontend test suite passes (if applicable)
[ ] Contract test suite passes (if applicable)
[ ] E2E tests pass (if applicable)
[ ] Smoke tests pass (if applicable)

Build
[ ] Release build succeeds
[ ] No compiler warnings in release build
[ ] Frontend builds for all target platforms (if applicable)
[ ] Binary size is reasonable (no accidental debug symbols)

Security
[ ] No known security vulnerabilities in dependencies
[ ] All consensus changes have TLA+ verification
[ ] All cryptographic changes have been reviewed
[ ] No hardcoded secrets in the codebase

Documentation
[ ] README is current
[ ] CHANGELOG has entries for all changes since last release
[ ] API documentation is current
[ ] Migration guide written (if breaking changes exist)

Formal Verification
[ ] All TLA+ specs pass model checking
[ ] All invariant tests pass
[ ] Specs match the implementation

Performance
[ ] No performance regressions
[ ] Memory usage is within acceptable bounds

Deployment
[ ] Staging/testnet tested with the release candidate
[ ] Configuration is correct
[ ] Upgrade path documented (if not a fresh deploy)
```

**BLOCKER:** The release cannot proceed if:
- Any test suite fails
- Known security vulnerabilities exist
- TLA+ specs do not pass model checking
- CHANGELOG is incomplete

---

## Hotfix Review Gate

For emergency fixes that bypass the normal sprint cycle.

### Checklist

```
HOTFIX REVIEW -- <description>
Date: YYYY-MM-DD

Urgency
[ ] The issue is a production blocker or security vulnerability
[ ] The fix cannot wait for the next sprint

Fix Quality
[ ] All tests pass
[ ] Lint clean
[ ] Regression test added for the specific bug
[ ] Fix is minimal (no scope creep)

Documentation
[ ] CHANGELOG entry added
[ ] Sprint DAILY.md updated with hotfix note
[ ] Backlog item created for follow-up hardening (if needed)
```

**GATE:** Hotfixes still require passing tests and a regression test. The only thing relaxed is the sprint planning requirement. DO NOT PROCEED without a regression test -- that is how you prevent the same bug from recurring.

---

## Review Execution

### Who Reviews?

| Change Type | Reviewer |
|-------------|----------|
| Standard feature | Any contributor (human or qualified agent) |
| Security-sensitive code | Must be reviewed by a different contributor |
| Release candidate | Project maintainer |
| Hotfix | Any available contributor with relevant expertise |

### How to Record a Review

Add a review entry to the sprint `DAILY.md`:

```markdown
### Reviews
- WP-3.2 reviewed by @contributor -- approved, no issues
- WP-4.1 reviewed by @agent -- requested changes: missing error handling in edge case
```

For PRs, use the platform's review mechanism (approve/request changes/comment).
