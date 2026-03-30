# Workflow: RETROFIT

Use this workflow when bringing Agentile into an existing repository or an existing slice of a repository.

## Step 1: Install The Starter Surface

Add:
- `.agentile/`
- root instruction files such as `AGENTS.md` and `CLAUDE.md`
- `SPIRIT_GUIDE.md`

Create or update:
- `SPIRIT.md`
- `README.md`
- `CHANGELOG.md` if missing

Gate: agents and humans have a clear way in, and the repo no longer speaks like a generic starter.

## Step 2: Read The Actual Repo State

Inventory:
- test status
- CI status
- obvious doc drift
- sensitive paths
- open failures or incident conditions

Gate: you know whether this is a healthy retrofit or an unstable one.

## Step 3: If The Repo Is Unstable, Review Before You Build

If CI is red, review feedback is piling up, or the repo is in pre-prod hardening:

1. run `REVIEW.md`
2. run `DEBUGGING.md`
3. create work packages for the discovered fixes

Gate: do not start new features on top of an unknown state.

## Step 4: Create The Baseline

Run the test suite and record:
- total passing tests
- failing tests
- ignored or flaky tests
- obvious untested high-risk areas

Write the baseline in `coverage/BASELINE.md`.

Gate: the current quality floor is explicit.

## Step 5: Create The Retrofit Sprint

Create a sprint that covers:
- spirit and README alignment
- baseline capture
- red CI or review fallout if present
- first hardening or adoption tasks

Use the normal sprint scaffold with `SPRINT.md`, `DAILY.md`, `JOURNAL.md`, and `REPORT.md`.

Gate: retrofit work is tracked like all other work.

## Step 6: Stabilize Before Normal Feature Work

Prioritize:
- failing tests
- broken CI
- untracked risky code
- missing docs for critical paths

Leave lower-risk cleanup for later sprints.

Gate: high-risk uncertainty is reduced before the repo returns to normal planned delivery.

## Step 7: Transition To Standard Workflow

Once the repo has:
- a real `SPIRIT.md`
- a real project README
- a recorded test baseline
- an active sprint
- stable enough review/debug state

move into the normal `SPRINT.md` and `FEATURE.md` cycle.
