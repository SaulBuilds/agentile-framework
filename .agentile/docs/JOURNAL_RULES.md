# Journaling Rules

Journals are not optional mood boards. They are the project's reflective memory.

Daily logs record activity. Journals record interpretation.

## Rule 1: Every Sprint Gets A `JOURNAL.md`

Each sprint directory should contain:
- `SPRINT.md`
- `DAILY.md`
- `JOURNAL.md`
- `REPORT.md`

`JOURNAL.md` is the sprint retrospective and introspective log. It should exist before the sprint is archived.

Gate: a sprint is not complete until its journal exists.

## Rule 2: Reviews And Debugging Can Trigger Extra Journals

Write a session journal in `.agentile/docs/journals/` when:
- a review changes the direction of the work
- failing CI exposes a hidden assumption
- a debugging cycle produces a non-obvious root cause
- a handoff would otherwise lose important context

If the sprint journal is enough, link to it. If not, create a dedicated session journal.

## Rule 3: Promote Durable Lessons

Promote an insight when it clearly outlives the immediate sprint:

- write an essay when the lesson generalizes into a principle
- write a case study when the incident details matter as much as the conclusion

See [`INSIGHT_CAPTURE.md`](INSIGHT_CAPTURE.md) for the decision logic.

## Rule 4: Required Sections

Every meaningful journal should answer:

1. What happened
2. What I thought was true
3. What was actually true
4. What evidence changed my mind
5. What fix, interpretation, or decision followed
6. What still feels fragile
7. What the next contributor should do

Use [`../templates/JOURNAL.template.md`](../templates/JOURNAL.template.md) as the default shape.

## Rule 5: Be Intellectually Honest

Do not:
- smooth over fragility with upbeat language
- hide uncertainty behind passive voice
- present guesses as facts
- omit the human or reviewer input that changed the outcome

Do:
- state the mistaken belief plainly
- cite the evidence that corrected it
- separate what is proven from what is inferred
- name remaining uncertainty and next verification steps

## Rule 6: Ask For Human Angle When Available, Auto-Write When Not

Human context is valuable, but it should not block the journal.

If a human counterpart is available, ask for angle or emphasis.

If not, write the journal anyway. A missing angle is not a waiver.

## Rule 7: Keep The Ladder Light

The default bias is:
- short journal first
- essay only if the lesson generalizes
- case study only if the concrete incident teaches more than an abstraction would

## Rule 8: Configurable By Project

Projects may tune:

| Setting | Default |
|---------|---------|
| journal_frequency | every sprint, plus notable review/debugging moments |
| journal_location | sprint-local `JOURNAL.md` and `.agentile/docs/journals/` |
| essay_threshold | generalizable principle with evidence |
| case_study_threshold | concrete incident with durable teaching value |
| honesty_level | direct |

## Why This Matters

Without journals, future contributors see a diff and a green check.

With journals, they inherit:
- what almost went wrong
- what the model misunderstood
- what the human corrected
- what remains risky even after the fix
