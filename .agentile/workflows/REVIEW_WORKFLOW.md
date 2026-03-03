# REVIEW_WORKFLOW.md — Code Review and Quality Gates

> Run this workflow before any feature is considered "done" and before sprint completion.

## Feature-Level Review

### 1. Traceability Check
- [ ] Feature has a corresponding planset item
- [ ] Feature has a `.feature` file in Gherkin
- [ ] All Gherkin scenarios have step definitions
- [ ] All tests trace to a Gherkin scenario

### 2. Test Quality
- [ ] All tests pass
- [ ] Coverage meets or exceeds CONFIG.md target
- [ ] Edge cases are tested (null, empty, invalid, boundary)
- [ ] Error handling is tested
- [ ] No test dependencies (each test runs independently)

### 3. Code Quality
- [ ] Code follows established patterns from architecture docs
- [ ] No commented-out code
- [ ] No TODO/FIXME without a corresponding backlog item
- [ ] Functions are small and single-purpose
- [ ] Names are clear and descriptive
- [ ] No hardcoded values that should be configuration

### 4. Documentation
- [ ] Relevant docs are updated
- [ ] CHANGELOG.md reflects the change
- [ ] AGENT_NOTES.md records any decisions made

## Sprint-Level Review

### 5. Full Suite
- Run the complete test suite
- Run the coverage report
- Check for flaky tests (run twice if suspicious)

### 6. Integration Verification
- Do all features work together?
- Any conflicts between features built in this sprint?

### 7. Documentation Audit
- README.md is current
- SETUP.md instructions work from scratch
- API docs match implementation
- Architecture docs reflect current state

### 8. Generate Report
Save to `reports/` using the report template.
Present to human for sign-off.
