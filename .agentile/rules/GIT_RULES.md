# Git Rules

Git hygiene matters because governance lives in the repo, not just the code.

## Conventional Commits

Commit format:

```text
<type>(<scope>): <description>
```

Common types:
- `feat`
- `fix`
- `test`
- `docs`
- `refactor`
- `perf`
- `chore`
- `revert`

Gate: commits should follow a conventional structure and trace to a work package when applicable.

## Branch Naming

Use short lowercase branch names such as:
- `feature/<description>`
- `fix/<description>`
- `docs/<description>`
- `sprint/<sprint-id>`
- `hotfix/<description>`

## Pull Request Minimum

Every PR should include:
- summary
- test plan
- sprint reference where applicable
- documentation impact
- insight-capture impact if the change taught something durable

Use `templates/PR.template.md`.

## Protected Branches

At minimum:
- no direct pushes to `main`
- no force pushes to protected branches
- PRs required for merge

## Governance Files Need Extra Care

Treat these as governance paths:
- `SPIRIT.md`
- `SPIRIT_GUIDE.md`
- `.agentile/AGENT_ENTRY.md`
- `.agentile/rules/`
- `.agentile/workflows/`

Recommended gate:
- CODEOWNERS
- required reviews
- signed commits or tags when possible

See `docs/SPIRIT_PROTECTION.md`.

## AI Contributions

If your team wants explicit attribution, use a `Co-Authored-By` footer or equivalent metadata policy consistently.

## Commit Hygiene

- one logical change per commit when feasible
- do not mix unrelated work
- do not commit secrets
- do not commit generated noise unless the project requires it
