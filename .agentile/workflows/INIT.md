# Workflow: INIT

Use this workflow when entering a healthy repo cold and establishing the project contract before normal sprint work.

## Step 1: Bootstrap The Repo Contract

1. Read `../SPIRIT.md` if it exists.
2. If `SPIRIT.md` is missing or generic, create or rewrite it from `../SPIRIT_GUIDE.md`.
3. Refactor `../README.md` if it still describes the starter rather than the actual project.
4. Ensure the root instruction files point agents into `.agentile/AGENT_ENTRY.md`.
5. Ensure `CHANGELOG.md` exists.

Gate: the repo should describe itself, not the framework starter.

## Step 2: Read The Canon

Read:
- `AGENT_ENTRY.md`
- `CONFIG.md`
- `rules/CORE_RULES.md`
- `sprints/CURRENT.md`

Gate: you can explain the project, the current repo state, the hard rules, and the active sprint situation.

## Step 3: Onboarding Or Skip

Take the quiz or use the skip protocol:
- `onboarding/QUIZ_SPEC.md`
- `onboarding/SKIP_PROTOCOL.md`

Gate: you know your tier and role boundaries.

## Step 4: Choose The Repo-State Path

Pick the next workflow based on reality:

| Repo State | Next Workflow |
|------------|---------------|
| Healthy and planned | `SPRINT.md` then `FEATURE.md` |
| Existing repo adoption | `RETROFIT.md` |
| Failing CI, review fallout, or debugging | `REVIEW.md` then `DEBUGGING.md` |

Gate: do not treat debugging or hardening as ordinary greenfield feature work.

## Step 5: Create The First Sprint Scaffold

Create:

```text
.agentile/sprints/active/sprint-<id>-<name>/
├── SPRINT.md
├── DAILY.md
├── JOURNAL.md
└── REPORT.md
```

Use:
- `templates/SPRINT.template.md`
- `templates/DAILY.template.md`
- `templates/JOURNAL.template.md`
- `templates/REPORT.template.md`

Update `sprints/CURRENT.md` to point to the sprint.

Gate: no untracked work.

## Step 6: Record The Test Baseline

Run the relevant test suite and record the result in:
- `coverage/BASELINE.md`
- the sprint `SPRINT.md`

Gate: the test ratchet has a real starting number.

## Step 7: Start The Right Workflow

From here, work should happen through the selected workflow and be reported back into sprint artifacts and journals.

Gate: if work is not reflected in sprint records, it did not happen.
