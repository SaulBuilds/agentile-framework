# Git Rules

> Git conventions for consistent commits, branches, and PRs.

---

## Conventional Commits

All commit messages follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
Co-Authored-By: <name> <email>
```

### Types

| Type | When to Use |
|------|-------------|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `test` | Adding or modifying tests (no production code change) |
| `docs` | Documentation-only changes |
| `refactor` | Code restructuring without behavior change |
| `perf` | Performance improvement |
| `chore` | Build, CI, dependency updates, tooling |
| `style` | Formatting, whitespace (no logic change) |
| `revert` | Reverting a previous commit |

### Scope

The scope identifies the affected module or area. Define your project's scopes in CONFIG.md.

### Examples

```
feat(auth): add two-factor authentication flow

Implements TOTP-based 2FA with backup codes.
Closes WP-3.4
Co-Authored-By: Claude <noreply@anthropic.com>
```

```
fix(api): correct pagination offset calculation

The offset was 1-indexed but the database query expected 0-indexed.
Fixes WP-1.2
```

**GATE:** Commits that do not follow conventional commit format will be flagged in PR review. DO NOT PROCEED with non-conventional commit messages.

---

## Branch Naming

```
<type>/<short-description>
```

| Branch Type | Pattern | Example |
|-------------|---------|---------|
| Feature | `feature/<description>` | `feature/two-factor-auth` |
| Bug fix | `fix/<description>` | `fix/pagination-offset` |
| Documentation | `docs/<description>` | `docs/api-guide-update` |
| Refactor | `refactor/<description>` | `refactor/storage-layer` |
| Sprint work | `sprint/<sprint-id>` | `sprint/sprint-5-hardening` |
| Hotfix | `hotfix/<description>` | `hotfix/auth-bypass` |

**Rules:**
- Use lowercase and hyphens only. No underscores, no uppercase.
- Keep branch names under 50 characters.
- Delete branches after merge.

---

## Pull Request Requirements

Every PR must include:

1. **Title** following conventional commit format (under 70 characters)
2. **Summary** section with 1-3 bullet points explaining the change
3. **Test plan** section describing how the change was tested
4. **Sprint reference** linking to the WP in the active sprint

Use the PR template at `templates/PR.template.md`.

### Required Checks Before Merge

| Check | Gate Level |
|-------|------------|
| Tests pass | **BLOCKER** |
| Linter clean | **BLOCKER** |
| Formatted | **GATE** |
| Docs updated (if API changed) | **GATE** |

---

## Protected Branches

| Branch | Protection |
|--------|-----------|
| `main` | No direct pushes. PRs required. Force push prohibited. |
| `release/*` | No direct pushes. PRs required. Release manager approval. |

**BLOCKER:** Never force push to `main` or any `release/*` branch. Use `git revert` to undo changes.

---

## AI Contributions

All commits that include AI-generated code must include a `Co-Authored-By` footer. This provides attribution and traceability for AI-assisted work.

---

## Commit Hygiene

- **One logical change per commit.** Do not mix a feature, a bug fix, and a refactor in one commit.
- **Write the body when the diff is not self-explanatory.**
- **Reference the sprint WP in the footer.** `Closes WP-3.4` or `Refs WP-2.1`.
- **Do not commit generated files.** Add build artifacts to `.gitignore`.
- **Do not commit secrets.** Never commit private keys, API tokens, or credentials.
