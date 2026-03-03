# GIT_RULES.md — Version Control Conventions

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

## Rules

1. **Never commit directly to `main` or `develop`**
2. **Every feature branch must have passing tests before merge**
3. **Squash feature branches into a single meaningful commit on merge to develop**
4. **Tag releases**: `v0.1.0`, `v0.2.0`, etc. following semver
5. **Commit often, push per feature** — Small, atomic commits within the branch

## .gitignore Considerations

The `.agentile/` folder SHOULD be committed — it's the project's planning brain. Exceptions:
- `reports/` may be gitignored if reports are ephemeral
- `docs/AGENT_NOTES.md` should be committed — it's valuable project history
