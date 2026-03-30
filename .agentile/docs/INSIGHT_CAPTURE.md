# Insight Capture Protocol

The goal of insight capture is simple: if a fix, interpretation, or failure taught something worth reusing, do not leave it trapped in chat history.

## The Artifact Ladder

Use the lightest artifact that preserves the value.

| Artifact | Scope | Use It When |
|----------|-------|-------------|
| Sprint `JOURNAL.md` | One sprint | You need a retrospective and introspective record of what changed |
| `docs/journals/*.md` | One session, review, or incident | A debugging loop, review cycle, or handoff produced insight worth preserving |
| `docs/essays/*.md` | General principle | The lesson should shape future work beyond this one repo state |
| `docs/case_studies/*.md` | Concrete incident | A failure mode and remedy are clearer when tied to a real episode |

## Write Something When Any Of These Are True

- a review changed your design or interpretation
- a CI failure exposed a hidden dependency or assumption
- a debugging cycle produced a non-obvious root cause
- a rule needed clarification to stop surface-level compliance
- a human counterpart corrected an agent's false confidence
- two agents or reviewers disagreed in a way that taught something useful
- a fix feels novel enough that future contributors would otherwise rediscover it the hard way

## Do Not Escalate Everything Into An Essay

Do not write long-form just to sound thoughtful.

Use:
- a journal when the lesson is local
- an essay when the lesson generalizes
- a case study when the evidence matters as much as the conclusion

## Required Honesty Fields

Every non-trivial journal, essay, or case study should answer:

1. What I thought was true
2. What was actually true
3. What evidence changed my mind
4. What fix or decision followed
5. What remains uncertain
6. What the next contributor should try, avoid, or verify

## Novelty Test

Before writing, ask:

1. Would the next contributor likely repeat this mistake without help?
2. Did this change how I interpret a rule, architecture choice, or failure mode?
3. Can I point to evidence rather than vibes?

If two or more answers are yes, capture it.

## Default Bias

When in doubt, write the shorter artifact now and promote it later if the idea proves durable.
