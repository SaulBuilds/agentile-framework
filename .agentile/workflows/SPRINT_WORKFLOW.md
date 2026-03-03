# SPRINT_WORKFLOW.md — Sprint Lifecycle

## Sprint Planning

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

### 4. Human Approval (BLOCKER)
Present the sprint plan. Wait for "go ahead."

## Sprint Execution

### Daily Loop
```
1. Check sprints/active/SPRINT-N.md for current task
2. Execute FEATURE_WORKFLOW.md for the task
3. Update sprint tracker with progress
4. Move to next task
5. If blocked → surface to human
6. If all tasks done → trigger Sprint Review
```

### Progress Tracking
Update the sprint file after each completed feature:
```markdown
- [x] user-login — ✅ 95% coverage — GREEN
- [ ] user-registration — 🔴 IN PROGRESS — writing tests
- [ ] password-reset — ⬜ NOT STARTED
```

## Sprint Review

When all items are complete (or time runs out):

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

### 4. Documentation Sweep
Activate TECH_WRITER role:
- Update README.md
- Update CHANGELOG.md
- Verify docs match implementation

## Sprint Retrospective

### 5. Write Retrospective
Append to the sprint report:
- What went well?
- What slowed things down?
- What should change for next sprint?
- Any Agentile framework improvements suggested?

### 6. Archive Sprint
Move `sprints/active/SPRINT-N.md` to `sprints/completed/SPRINT-N.md`

### 7. Groom Backlog
- Remove completed items from backlog
- Add newly discovered items
- Re-prioritize based on learnings

### 8. Plan Next Sprint
Return to Sprint Planning (top of this document).
