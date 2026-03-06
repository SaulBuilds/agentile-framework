# REVIEW_WORKFLOW.md — Code Review and Quality Gates

> Run this workflow before any feature is considered "done" and before sprint completion.
> **Every section is an enforceable gate. A feature or sprint does not pass review until ALL gates clear.**

---

## Feature-Level Review

**Entry Gate: The feature must have completed all phases of FEATURE_WORKFLOW.md (SPECIFY → RED → GREEN → REFACTOR → VERIFY → DOCUMENT) before entering review.**

### 1. Traceability Check
- [ ] Feature has a corresponding planset item (can name which SCOPE.md entry it serves)
- [ ] Feature has a `.feature` file in Gherkin with at least one happy-path and one error scenario
- [ ] All Gherkin scenarios have step definitions or corresponding test cases
- [ ] All tests trace to a Gherkin scenario (no orphaned tests)

**GATE: If any traceability link is broken, the feature fails review. Fix the broken link before proceeding. An untraceable feature is an undocumented feature.**

### 2. Test Quality
- [ ] All tests pass (zero failures)
- [ ] Coverage meets or exceeds CONFIG.md target for this feature
- [ ] Edge cases are tested (null, empty, invalid, boundary)
- [ ] Error handling is tested (what happens when things go wrong)
- [ ] No test dependencies (each test runs independently — can run in any order)

**GATE: If any test fails, coverage is below target, or edge cases are missing — the feature fails review. Write additional tests or fix failures before proceeding.**

### 3. Code Quality
- [ ] Code follows established patterns from architecture docs
- [ ] No commented-out code (delete it or make it active)
- [ ] No TODO/FIXME without a corresponding backlog item (create the item or remove the comment)
- [ ] Functions are small and single-purpose
- [ ] Names are clear and descriptive
- [ ] No hardcoded values that should be configuration

**GATE: If code quality issues are found, fix them. Run the full test suite after each fix to verify no regressions. The refactor must be clean.**

### 4. Documentation
- [ ] Sprint tracker in `sprints/active/` shows the feature as DONE with coverage %
- [ ] `AGENT_NOTES.md` has an entry for decisions made during this feature
- [ ] CHANGELOG.md reflects the change (if user-facing)
- [ ] Any new/changed public APIs are documented

**GATE: Missing documentation fails the review. Update docs before marking the feature as review-complete.**

---

## Sprint-Level Review

**Entry Gate: All features in the sprint must have individually passed Feature-Level Review above, or be explicitly deferred with documented reason.**

### 5. Full Suite Regression
- Run the complete test suite
- Run the coverage report
- Check for flaky tests (run twice if suspicious)

**GATE: Full test suite must pass with zero failures. If flaky tests are found, fix them — flaky tests erode confidence and must be resolved, not ignored.**

### 6. Integration Verification
- Do all features work together?
- Any conflicts between features built in this sprint?
- Test cross-feature interactions explicitly if features touch shared state

**GATE: If integration issues are found, write integration tests that expose the issue, then fix the code. Follow the RED → GREEN cycle even for integration fixes.**

### 7. Documentation Audit
- README.md is current and accurate
- Setup instructions work from scratch (mentally walk through them — would a new developer succeed?)
- API docs match implementation
- Architecture docs reflect current state (not stale from before the sprint)

**GATE: Stale or inaccurate docs fail the audit. Update them. If architecture has evolved during the sprint, update SYSTEM_DESIGN.md.**

### 8. Generate Report
Save to `reports/SPRINT-N-REPORT.md` using the report template.
Present to human for sign-off.

**GATE: Report must exist and be presented to the user. Sprint is not complete until the user has reviewed the report (explicitly or by not objecting).**

---

## Review Failure Protocol

When a gate fails:
1. **Identify** which gate failed and why
2. **Fix** the issue (write tests, update docs, fix code)
3. **Re-run** the gate check from the beginning of that section
4. **Do not skip ahead** — a failed gate means everything after it is suspect

A feature that fails review goes back to the appropriate phase of FEATURE_WORKFLOW.md. It does not get marked as DONE.

---

## Review Completion Checklist

**For a feature to pass review, ALL must be true:**
- [ ] Full traceability chain: Planset → Feature → Test → Code
- [ ] All tests pass, coverage meets target
- [ ] Code is clean, follows patterns, no dead code
- [ ] Documentation is complete and current
- [ ] Sprint tracker reflects reality

**For a sprint to pass review, ALL must be true:**
- [ ] Every feature individually passed review (or is explicitly deferred)
- [ ] Full regression suite passes
- [ ] No integration conflicts
- [ ] All docs are current
- [ ] Sprint report is generated and presented
