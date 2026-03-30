# Workflow: DEBUGGING

Use this workflow when the system is failing, the repo state is unclear, CI is red, or pre-prod hardening reveals unstable behavior.

## The Cycle

```text
OBSERVE -> HYPOTHESIZE -> VERIFY -> ISOLATE -> FIX -> REGRESS
```

## 1. OBSERVE

Do not change code yet. Map the current state:
- failing checks
- logs
- configs
- processes
- connected services
- user-visible behavior

Gate: you have a written state snapshot.

## 2. HYPOTHESIZE

Write a falsifiable claim about the root cause.

Gate: you can explain how to disprove your theory.

## 3. VERIFY

Use evidence, not edits:
- inspect data
- run commands
- compare expected vs actual
- read logs and configs

Gate: the hypothesis is confirmed or rejected by evidence.

## 4. ISOLATE

Find the smallest reproduction or narrowest broken assumption.

Gate: you can explain the bug in one precise sentence.

## 5. FIX

Apply the smallest change that addresses the root cause.

Gate: the fix is targeted and traceable.

## 6. REGRESS

Verify:
- the original issue is fixed
- nearby behavior still works
- the relevant suite is green

Gate: the system is more understood than before, not merely quieter.

## Insight Capture

If the debugging cycle taught something non-obvious:
- update the sprint `JOURNAL.md`
- write a session journal in `docs/journals/`
- promote to a case study or essay if the lesson is durable

If no extra artifact is warranted, note that explicitly in the sprint journal.
