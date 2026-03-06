# GIT_RULES.md — Version Control Conventions

> **Every commit must pass the pre-commit gate. Every merge must pass the merge gate. No exceptions.**

---

## Commit Message Format

Use Conventional Commits:
```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

### Types
| Type | When |
|------|------|
| `feat` | New feature implementation (GREEN phase) |
| `test` | Adding or updating tests (RED phase) |
| `refactor` | Code improvement without behavior change (REFACTOR phase) |
| `docs` | Documentation changes |
| `fix` | Bug fix |
| `chore` | Tooling, config, dependencies |
| `plan` | Planset or sprint planning updates |

### TDD Phase Tags
Append the TDD phase when relevant:
- `test(red): auth - user login scenario` — Failing test committed
- `feat(green): auth - user login implementation` — Test now passes
- `refactor: auth - extract token validation utility` — Cleanup

---

## Pre-Commit Gate

**Before EVERY commit, verify ALL of the following. If any fail, do not commit.**

- [ ] The commit addresses exactly ONE logical change (not bundled)
- [ ] The commit message follows Conventional Commits format
- [ ] All tests pass (run the full suite, not just the changed files)
- [ ] No unfinished code is included (no `TODO` without backlog item, no commented-out blocks, no placeholder implementations)
- [ ] No skipped or pending tests are included (tests must be complete and passing, or not committed at all)
- [ ] No secrets, credentials, or `.env` files are staged
- [ ] Documentation is updated if behavior changed

**GATE: A commit that includes broken tests, unfinished code, or skipped tests is a framework violation. Do not commit incomplete work. Either finish it or don't commit it.**

### Prohibited in commits:
- Tests marked as `.skip()`, `.only()`, `@skip`, `pending`, or `xit`/`xdescribe`
- Implementation stubs that return hardcoded values (unless in RED → GREEN and it's the minimum passing implementation)
- `console.log` or debug statements left in production code
- Merge conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`)
- Generated files that should be in `.gitignore`

---

## Branching Strategy

```
main
├── develop
│   ├── feature/sprint-1/user-login
│   ├── feature/sprint-1/user-registration
│   └── feature/sprint-2/dashboard-layout
└── hotfix/critical-auth-bug
```

- `main` — Production-ready. Only merges from `develop` after sprint completion.
- `develop` — Integration branch. Features merge here.
- `feature/sprint-N/feature-name` — One branch per feature.
- `hotfix/description` — Emergency fixes branched from `main`.

---

## Merge Gate

**Before merging a feature branch to develop, ALL must be true:**

- [ ] All tests pass on the feature branch
- [ ] Coverage meets CONFIG.md target
- [ ] Feature has passed REVIEW_WORKFLOW.md
- [ ] No merge conflicts (resolve before merging, not after)
- [ ] Sprint tracker reflects the feature as DONE
- [ ] CHANGELOG.md is updated

**GATE: A feature branch that fails any merge gate condition cannot be merged. Fix the issue on the feature branch first.**

---

## Sprint Merge Gate

**Before merging develop to main (sprint release), ALL must be true:**

- [ ] Sprint review is complete (SPRINT_WORKFLOW.md review phase passed)
- [ ] Full test suite passes on develop
- [ ] Coverage meets or exceeds target
- [ ] Sprint report exists in `reports/`
- [ ] README.md is current
- [ ] Version tag is prepared

**GATE: A sprint that hasn't completed its review workflow cannot be released to main. No shortcuts.**

---

## Rules

1. **Never commit directly to `main` or `develop`** — always use feature branches
2. **Every feature branch must have passing tests before merge**
3. **Squash feature branches into a single meaningful commit on merge to develop**
4. **Tag releases**: `v0.1.0`, `v0.2.0`, etc. following semver
5. **Commit often, push per feature** — Small, atomic commits within the branch

**GATE: If any rule is violated, the merge is blocked. Fix the violation before retrying.**

---

## .gitignore Considerations

The `.agentile/` folder SHOULD be committed — it's the project's planning brain. Exceptions:
- `reports/` may be gitignored if reports are ephemeral
- `docs/AGENT_NOTES.md` should be committed — it's valuable project history

---

## Incomplete Work Protocol

If you need to stop work mid-feature (session ending, context limit, user request):

1. **Do NOT commit half-finished code**
2. Save your progress notes to `docs/AGENT_NOTES.md` with clear status:
   ```markdown
   ### [DATE] [Feature Name] — PAUSED
   - Completed: [what's done]
   - In progress: [what was being worked on]
   - Next steps: [what to do when resuming]
   - Branch: [branch name]
   ```
3. If tests were written but implementation isn't done, that's okay — commit the tests (RED phase) but NOT partial implementation
4. Never leave the repo in a state where tests fail on the main working branch
