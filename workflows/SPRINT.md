# Workflow: SPRINT -- Sprint Lifecycle

> A sprint is a time-boxed unit of work with a clear goal, defined tasks, and measurable outcomes.
> This workflow governs the full lifecycle: Plan, Execute, Review, Retro, Archive.

---

## Phase 1: PLAN

### Inputs
- Backlog items (prioritized in `sprints/backlog/`)
- Previous sprint's retro (if applicable)
- Current test count baseline

### Steps

1. **Create sprint directory:**
   ```
   .agentile/sprints/active/sprint-<id>-<name>/
   ├── SPRINT.md    # WBS, tasks, acceptance criteria
   ├── DAILY.md     # Progress log (created empty)
   └── RETRO.md     # Retrospective (created empty)
   ```

2. **Write SPRINT.md** using `templates/SPRINT.template.md`:
   - Sprint ID and name
   - Sprint goal (one sentence)
   - Duration (start date, end date)
   - Work Packages (WBS) with task breakdown
   - Acceptance criteria per WP
   - Test count baseline

3. **Update CURRENT.md:**
   - Set the active sprint reference
   - Link to the sprint directory

4. **Record test baseline:**
   - Run the project's test suite
   - Record the total passing test count in `coverage/BASELINE.md` and in the sprint `SPRINT.md`

### GATE: Plan Review

Before proceeding to Execute:
- [ ] Sprint directory exists with all three files
- [ ] SPRINT.md has at least one Work Package with acceptance criteria
- [ ] CURRENT.md points to this sprint
- [ ] Test baseline is recorded

**DO NOT PROCEED to Execute without a complete plan.**

---

## Phase 2: EXECUTE

### Steps

1. **For each Work Package**, follow the [FEATURE.md](FEATURE.md) workflow:
   - Specify (optional Gherkin)
   - RED (failing test)
   - GREEN (implement)
   - REFACTOR
   - VERIFY
   - DOCUMENT
   - REPORT

2. **Update DAILY.md** after each work session:
   ```markdown
   ## YYYY-MM-DD

   ### Completed
   - WP-1.1: Implemented feature X
   - WP-1.2: Added N unit tests

   ### Metrics
   - Tests: N passing (+N from baseline)
   - Lint: clean

   ### Blockers
   - None

   ### Next
   - WP-2.1: Next task description
   ```

3. **Update WP status** in SPRINT.md as work progresses:
   - `[ ] NOT STARTED`
   - `[~] IN PROGRESS`
   - `[x] COMPLETE`
   - `[!] BLOCKED` (with reason)

4. **Commit work** following GIT_RULES.md conventions.

### GATE: Execution Quality

At any point during execution:
- All tests pass
- Linter is clean
- Test count has not decreased from baseline

**BLOCKER:** If tests fail or linter has warnings, stop feature work and fix the regression before continuing.

---

## Phase 3: REVIEW

### Steps

1. **Run full verification suite** (project-specific test, lint, and format commands)

2. **Verify all WPs are complete:**
   - Every WP in SPRINT.md is marked `[x] COMPLETE` or `[!] BLOCKED` with documented reason.
   - Blocked WPs are moved to the backlog for the next sprint.

3. **Verify test ratchet:**
   - Final test count >= baseline test count.
   - Record final count in SPRINT.md.

4. **Verify documentation:**
   - Module READMEs updated where APIs changed.
   - CHANGELOG updated for user-facing changes.
   - Sprint DAILY.md has entries for all work days.

### GATE: Sprint Review Checklist

- [ ] All tests pass
- [ ] Lint clean
- [ ] Formatting clean
- [ ] All WPs complete or explicitly blocked with reason
- [ ] Test count >= baseline
- [ ] DAILY.md is complete
- [ ] Documentation is current

**DO NOT PROCEED to Retro until all checks pass.**

---

## Phase 4: RETRO

### Steps

1. **Write RETRO.md** using `templates/REPORT.template.md` (retro section):
   - What went well
   - What went poorly
   - What to change next sprint
   - Key metrics (test count delta, WPs completed/blocked, velocity)

2. **Extract action items** for the next sprint. Add them to the backlog.

3. **Update CURRENT.md** to mark the sprint as complete.

### GATE: Retro Complete

- [ ] RETRO.md written with all sections
- [ ] Action items added to backlog
- [ ] CURRENT.md updated

---

## Phase 5: ARCHIVE

### Steps

1. **Move sprint directory** from `active/` to `completed/`:
   ```
   .agentile/sprints/completed/sprint-<id>-<name>/
   ```

2. **Sprint files are now immutable.** Do not edit them after archiving.

3. **Generate REPORT.md** using `templates/REPORT.template.md` with final metrics.

### GATE: Archive Complete

- [ ] Sprint directory moved to `completed/`
- [ ] REPORT.md generated
- [ ] No further edits to archived sprint files

---

## Lifecycle Summary

```
PLAN ──gate──> EXECUTE ──gate──> REVIEW ──gate──> RETRO ──gate──> ARCHIVE
  │                                                                   │
  └───────────────── next sprint ─────────────────────────────────────┘
```

| Phase | Key Output | Gate |
|-------|-----------|------|
| Plan | SPRINT.md with WBS | Plan review checklist |
| Execute | Code + tests + daily logs | Tests pass, lint clean |
| Review | Verification suite results | Sprint review checklist |
| Retro | RETRO.md + action items | Retro complete |
| Archive | Immutable sprint record | Archive complete |
