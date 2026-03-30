# Workflow: REVIEW

Use these gates before marking work complete, closing a sprint, shipping a release, or landing a hotfix.

## Feature Review Gate

Run before marking a work package complete.

```text
FEATURE REVIEW -- WP-<id>

Code Quality
[ ] Relevant tests pass
[ ] Full required suite passes
[ ] Linter and typecheck clean
[ ] Formatting clean
[ ] No TODOs, FIXMEs, mocks, or stubs in production code

Traceability
[ ] Change maps to an active work package
[ ] Commit or PR references the work package
[ ] Real data source or contract is named where relevant

Documentation
[ ] Module README updated if API changed
[ ] Root README updated if project-facing behavior changed
[ ] Root SPIRIT updated if governance or collaboration norms changed
[ ] CHANGELOG updated if user-facing behavior changed

Insight Capture
[ ] Sprint DAILY.md updated
[ ] Sprint JOURNAL.md updated or queued
[ ] Novel fix or interpretation captured, or explicitly noted as not needed

Security / Formality
[ ] Sensitive changes reviewed appropriately
[ ] Formal verification or explicit justification added when required
```

Gate: all applicable items pass.

## Sprint Review Gate

Run before closing a sprint.

```text
SPRINT REVIEW -- Sprint <id>

Completion
[ ] All work packages complete or explicitly blocked
[ ] Carried work moved to backlog or next sprint
[ ] Sprint goal achieved or honestly marked partial

Quality
[ ] Full suite passes
[ ] Linter clean
[ ] Formatting clean
[ ] Final test count >= baseline

Artifacts
[ ] SPRINT.md reflects reality
[ ] DAILY.md reflects actual work sessions
[ ] JOURNAL.md exists and is honest
[ ] REPORT.md is ready or drafted

Docs
[ ] Public docs updated where needed
[ ] CHANGELOG updated where needed

Learning
[ ] Any durable lesson promoted to journal, essay, or case study
[ ] If nothing rose to that level, the sprint journal says so
```

Blocker:
- failing tests
- decreased test count without replacement
- missing sprint journal

## Release Review Gate

Run before tagging a release.

```text
RELEASE REVIEW -- v<version>

Build And Test
[ ] Release build succeeds
[ ] Full test suite passes
[ ] CI parity command passes

Governance
[ ] README reflects the actual product
[ ] SPIRIT reflects current collaboration and protection norms
[ ] CHANGELOG is current

Security
[ ] Sensitive changes reviewed
[ ] No known credential leaks
[ ] Critical logic has required verification or explicit waiver

Reflection
[ ] Significant release lessons are captured
```

## Hotfix Review Gate

For urgent fixes that cannot wait.

```text
HOTFIX REVIEW -- <description>

Urgency
[ ] This cannot wait for the normal sprint cycle

Fix Quality
[ ] Regression verified
[ ] Relevant tests pass
[ ] Linter clean
[ ] Scope stayed minimal

Artifacts
[ ] Sprint records updated
[ ] CHANGELOG updated
[ ] Incident journal or case study created if the event taught something durable
```
