# Workflow: SPRINT

A sprint is the unit of planned delivery and reflective memory.

## Sprint Directory

Each active sprint should have:

```text
.agentile/sprints/active/sprint-<id>-<name>/
├── SPRINT.md
├── DAILY.md
├── JOURNAL.md
└── REPORT.md
```

## Phase 1: PLAN

1. Create the sprint directory.
2. Write `SPRINT.md` from `templates/SPRINT.template.md`.
3. Write the initial `DAILY.md`, `JOURNAL.md`, and `REPORT.md` from templates.
4. Update `sprints/CURRENT.md`.
5. Record the test baseline.

Plan gate:
- sprint goal exists
- work packages exist
- acceptance criteria exist
- current sprint points here
- test baseline is recorded

## Phase 2: EXECUTE

For each work package:
1. follow `FEATURE.md` or `DEBUGGING.md` as appropriate
2. update `DAILY.md` after each meaningful session
3. update work package state in `SPRINT.md`
4. commit with work package traceability

Execution gate:
- tests do not regress
- lint stays clean
- sprint records reflect reality

## Phase 3: REVIEW

Before marking the sprint complete:
1. run the full verification suite
2. confirm all work packages are complete or explicitly blocked
3. confirm documentation is current
4. confirm the journal requirement is satisfied
5. confirm any novel insight has either been captured or explicitly marked as not needed

Review gate:
- tests pass
- lint is clean
- test count is at or above baseline
- `DAILY.md` is current
- `JOURNAL.md` exists

## Phase 4: REFLECT

Finalize `JOURNAL.md` with:
- what happened
- what was learned
- what was fragile
- what changed your mind
- what should carry into the next sprint

If the sprint produced a durable lesson, create:
- a session journal
- an essay
- a case study

Reflection gate:
- journal is honest
- next actions are clear
- durable lessons have been promoted when warranted

## Phase 5: ARCHIVE

1. Finalize `REPORT.md`.
2. Move the sprint directory from `active/` to `completed/`.
3. Treat the archived sprint as immutable.
4. Update `sprints/CURRENT.md`.

Archive gate:
- report exists
- archive location is correct
- no further edits are expected except via a new corrective artifact

## Summary

```text
PLAN -> EXECUTE -> REVIEW -> REFLECT -> ARCHIVE
```
