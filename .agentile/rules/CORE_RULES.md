# CORE_RULES.md — Fundamental Operating Principles

These rules are non-negotiable. They apply to every action the agent takes.

## Rule 1: Documentation First
Before writing any code, the agent MUST have:
- A planset item (executive summary, architecture doc, or roadmap entry) that justifies the work
- A feature file in Gherkin syntax that describes the expected behavior
- A test file that will fail until the implementation is correct

**Violation**: Writing implementation code without a corresponding feature and test is a hard stop. Undo and start over.

## Rule 2: The Chain of Traceability
Every line of code must trace back through:
```
Planset Item → Feature File → Test File → Implementation
```
If an agent cannot explain which planset item a piece of code serves, that code should not exist.

## Rule 3: Red-Green-Refactor (Strictly)
1. **RED**: Write a test that fails. Run it. Confirm it fails.
2. **GREEN**: Write the minimum code to make the test pass. Run it. Confirm it passes.
3. **REFACTOR**: Clean up. Run tests again. Confirm they still pass.

Do NOT skip steps. Do NOT write implementation before the test fails.

## Rule 4: Small, Atomic Changes
- One feature at a time
- One test at a time
- One commit per logical change
- Never bundle unrelated changes

## Rule 5: Self-Documenting Progress
After completing any task, the agent MUST:
1. Update the sprint tracker in `sprints/active/`
2. Add a note to `docs/AGENT_NOTES.md` if any decisions were made
3. Generate a brief report if the task is a milestone

## Rule 6: Ask, Don't Assume
When encountering ambiguity:
- If it's a **design decision**: STOP and ask the human
- If it's an **implementation detail**: Make the simplest choice, document it, continue
- If it's a **dependency conflict**: STOP and ask the human

## Rule 7: Respect Existing Work
When retrofitting an existing project:
- Never delete code without a test proving it's dead
- Wrap existing functionality in features before modifying
- Preserve git history — no force pushes, no squashing others' work

## Rule 8: Stay in Scope
- Only work on items in the current sprint
- If you discover work that needs doing, add it to `sprints/backlog/` — do NOT do it now
- Scope creep is the enemy of shipping

## Rule 9: Human is Product Owner
- The human sets direction and priorities
- The agent executes within the framework
- When the human says "go ahead," proceed with the documented plan
- When the human says "stop," stop immediately

## Rule 10: Continuous Improvement
After each sprint, the agent writes a retrospective that includes:
- What went well
- What slowed things down
- Suggestions for improving the Agentile configuration
- Coverage metrics and quality observations
