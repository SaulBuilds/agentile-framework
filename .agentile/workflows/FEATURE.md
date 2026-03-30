# Workflow: FEATURE

Use this workflow for planned feature work, bug fixes, and scoped refactors.

## Flow

```text
SPECIFY -> RED -> GREEN -> REFACTOR -> VERIFY -> DOCUMENT -> REPORT
```

## Step 1: SPECIFY

Define:
- the work package
- the behavior or bug being addressed
- the acceptance criteria
- the real data source or contract when applicable

If the repo is in failing CI or unknown-state debugging, use `REVIEW.md` or `DEBUGGING.md` first.

Gate: you can state what should become true and how you will know.

## Step 2: RED

Write or identify a failing test, check, or reproducible verification step that fails for the right reason.

Gate: failure is real and relevant, not accidental.

## Step 3: GREEN

Write the minimum production change that makes the check pass.

Gate:
- module tests pass
- no mocks, stubs, or TODOs in production code

## Step 4: REFACTOR

Clean the code without adding new behavior.

Gate: tests still pass and the change is clearer than before.

## Step 5: VERIFY

Run the relevant verification stack:
- tests
- lint / typecheck
- format
- any domain-specific safety checks

Gate: all checks pass and the test ratchet is intact.

## Step 6: DOCUMENT

Update the docs that changed truth:
- module README if API changed
- root `README.md` if project-facing behavior changed
- root `SPIRIT.md` if collaboration or governance norms changed
- `CHANGELOG.md` for user-facing changes
- ADRs or specs for architecture changes

If the repo still carries starter framing in `README.md` or `SPIRIT.md`, fix that before calling the work done.

Gate: public truth matches the new behavior.

## Step 7: REPORT

Record the work in:
- sprint `SPRINT.md`
- sprint `DAILY.md`
- commit metadata

Also record the insight decision:
- write or update `JOURNAL.md` if the sprint meaning changed
- create a session journal, essay, or case study if the change taught something durable
- or explicitly note that no extra insight artifact was warranted

Gate: the work is traceable from diff to sprint to journal.
