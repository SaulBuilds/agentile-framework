# Git Rules

> Git conventions for the repository. Consistency in commits, branches, and PRs makes history navigable and automation reliable.

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

The scope identifies the affected crate or area. Define your project's scopes in CONFIG.md.

### Examples

```
feat(consensus): add committee checkpoint finality

Implements deterministic committee selection via VRF seed.
Committee of N validators with quorum Q.

Closes WP-3.4
Co-Authored-By: Agent Name <email>
```

```
fix(api): correct pending nonce calculation for sequential transactions
```

```
test(execution): add reentrancy guard tests for precompile calls
```

**GATE:** Commits that do not follow conventional commit format will be rejected by CI (if linting is enabled) or flagged in PR review. DO NOT PROCEED with non-conventional commit messages.

---

## Branch Naming

```
<type>/<short-description>
```

| Branch Type | Pattern | Example |
|-------------|---------|---------|
| Feature | `feature/<description>` | `feature/committee-checkpoints` |
| Bug fix | `fix/<description>` | `fix/pending-nonce-calculation` |
| Documentation | `docs/<description>` | `docs/wallet-guide-update` |
| Refactor | `refactor/<description>` | `refactor/storage-persistence` |
| Sprint work | `sprint/<sprint-id>` | `sprint/sprint-s-persistence` |
| Hotfix | `hotfix/<description>` | `hotfix/panic-fix` |

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

| Check | Command | Gate Level |
|-------|---------|------------|
| Tests pass | Project test command | **BLOCKER** |
| Lint clean | Project lint command | **BLOCKER** |
| Formatted | Project format command | **GATE** |
| Frontend tests pass (if changed) | Frontend test command | **BLOCKER** |
| Contract tests pass (if changed) | Contract test command | **BLOCKER** |
| Docs updated (if API changed) | Manual review | **GATE** |

---

## Protected Branches

| Branch | Protection |
|--------|-----------|
| `main` | No direct pushes. PRs required. Force push prohibited. |
| `release/*` | No direct pushes. PRs required. Release manager approval. |

**BLOCKER:** Never force push to `main` or any `release/*` branch. If you need to undo a change, use `git revert` to create a new commit.

---

## AI Contributions

All commits that include AI-generated code must include a `Co-Authored-By` footer:

```
Co-Authored-By: <AI Agent Name> <email>
```

This is not optional. It provides attribution and traceability for AI-assisted work.

---

## Commit Hygiene

- **One logical change per commit.** Do not mix a feature, a bug fix, and a refactor in one commit.
- **Write the body when the diff is not self-explanatory.** A one-liner rename needs no body. An algorithm change needs context.
- **Reference the sprint WP in the footer.** `Closes WP-3.4` or `Refs WP-2.1`.
- **Do not commit generated files.** Add build artifacts, compiled outputs, and IDE config to `.gitignore`.
- **Do not commit secrets.** Never commit `.env` files, private keys, API tokens, or credentials. If accidentally committed, rotate the secret immediately.

---

## Rebase vs. Merge

- **Feature branches**: Rebase onto main before merging to keep history linear.
- **Long-running branches**: Merge main into the branch periodically to avoid large conflicts.
- **Never rebase commits that have been pushed to a shared branch.**
- **Squash merges are acceptable** for small PRs (1-3 commits). For larger PRs, preserve individual commits for traceability.
