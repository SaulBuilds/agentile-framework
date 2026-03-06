# SPRINT_WORKFLOW.md — Sprint Lifecycle

> **Every phase transition has an enforceable gate. Do not skip gates. Do not proceed until they pass.**

---

## Sprint Planning

### Entry Gate

**Before planning a sprint, verify:**
- [ ] Project is initialized (planset exists, CONFIG.md is populated)
- [ ] Backlog has at least one item in `sprints/backlog/`
- [ ] No active sprint is in progress (or previous sprint has been archived to `sprints/completed/`)

**GATE: If the project isn't initialized, run INIT_WORKFLOW.md first. If the backlog is empty, work with the user to create items. If a prior sprint is still active, complete or archive it first.**

---

### 1. Review Backlog
- Read all items in `sprints/backlog/`
- Check for priority labels and dependencies
- Identify what's blocked vs. ready

### 2. Select Items
Pull items based on:
- Priority (high first)
- Dependencies (unblocked items first)
- Capacity (don't overcommit — 3-5 features per sprint is typical)

### 3. Create Sprint File
Copy `templates/SPRINT.template.md` to `sprints/active/SPRINT-N.md`
Fill in:
- Sprint number and dates
- Sprint goal (one sentence)
- Selected items with acceptance criteria
- Dependencies and risks

**GATE: The sprint file must contain: a goal, at least 2 items, each with priority and size. Every item must trace to a backlog item. Verify the file is complete before presenting.**

### 4. Human Approval (BLOCKER)
Present the sprint plan. Wait for "go ahead."

**GATE: Human must approve the sprint plan. Do not begin execution until approval is received. If the user requests changes, update the sprint file and re-present.**

---

## Sprint Execution

### Daily Loop
```
1. Read sprints/active/SPRINT-N.md — find the next incomplete item
2. Execute FEATURE_WORKFLOW.md for that item (follow ALL its gates)
3. Update sprint tracker with progress
4. Move to next item
5. If blocked → surface to human immediately
6. If all items done → trigger Sprint Review
```

### Progress Tracking
Update the sprint file after each completed feature:
```markdown
- [x] user-login — 95% coverage — GREEN
- [ ] user-registration — IN PROGRESS — writing tests
- [ ] password-reset — NOT STARTED
```

**GATE: After each feature completes, the sprint file must be updated immediately. If the sprint file doesn't reflect current reality, stop and update it before starting the next feature.**

---

## Sprint Review

**Entry Gate: All sprint items must be either DONE or explicitly deferred (with reason documented). Do not start review with items silently incomplete.**

### 1. Collect Metrics
- Features completed vs. planned
- Test coverage (overall and per feature)
- Tests passing/failing
- Number of blockers encountered

### 2. Generate Report
Use `templates/REPORT.template.md` and save to `reports/SPRINT-N-REPORT.md`

### 3. Run Full Quality Audit
Activate QA_ENGINEER role:
- Full test suite run
- Coverage check
- Regression check

**GATE: Full test suite must pass. Coverage must meet CONFIG.md target across all features delivered in this sprint. If either fails, fix before completing the review.**

### 4. Documentation Sweep
Activate TECH_WRITER role:
- Update README.md
- Update CHANGELOG.md
- Verify docs match implementation

**GATE: README and CHANGELOG must reflect all changes made in the sprint. Any new public APIs must be documented. Verify before proceeding to retro.**

---

## Sprint Retrospective

### 5. Write Retrospective
Append to the sprint report:
- What went well?
- What slowed things down?
- What should change for next sprint?
- Any Agentile framework improvements suggested?

### 6. Archive Sprint
Move `sprints/active/SPRINT-N.md` to `sprints/completed/SPRINT-N.md`

**GATE: The completed sprint file must include: all item statuses, metrics, report reference, and retrospective. Verify the file is complete before archiving.**

### 7. Groom Backlog
- Remove completed items from backlog
- Add newly discovered items
- Re-prioritize based on learnings

### 8. Plan Next Sprint
Return to Sprint Planning (top of this document).

---

## Sprint Completion Checklist

**All must pass before the sprint is considered done:**

- [ ] All planned features are DONE or explicitly deferred with documented reason
- [ ] Full test suite passes
- [ ] Coverage meets target
- [ ] Sprint report exists in `reports/SPRINT-N-REPORT.md`
- [ ] Retrospective is written
- [ ] Sprint file is archived to `sprints/completed/`
- [ ] Backlog is groomed
- [ ] README and CHANGELOG are current
- [ ] Human has reviewed the sprint report
