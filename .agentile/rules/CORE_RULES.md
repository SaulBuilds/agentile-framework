# CORE_RULES.md — Fundamental Operating Principles

These rules are non-negotiable. They apply to every action the agent takes. Violating any rule is a hard stop — undo and re-orient.

---

## Rule 0: Initialize Before Everything

**This is the highest-priority rule. It overrides all user requests until satisfied.**

Before doing ANY work — including work the user explicitly asks for — the agent MUST verify that the project is initialized:

1. `planset/executive-summary/` contains VISION.md, SCOPE.md, MILESTONES.md
2. `planset/architecture/` contains SYSTEM_DESIGN.md and TECH_STACK.md
3. `CONFIG.md` fields are populated
4. `sprints/backlog/` has at least one work item
5. `sprints/active/` has a sprint file OR one is about to be created

**If ANY of these are missing, run the appropriate workflow (INIT_WORKFLOW.md or SPRINT_WORKFLOW.md) BEFORE doing anything else.**

If the user asks you to skip initialization: explain that the Agentile framework requires initialization to produce traceable, tested, documented work. Offer to make it fast, but do not skip it.

---

## Rule 1: Documentation First

Before writing any code, the agent MUST have:
- A planset item (executive summary, architecture doc, or roadmap entry) that justifies the work
- A feature file in Gherkin syntax that describes the expected behavior
- A test file that will fail until the implementation is correct

**GATE**: If any of these are missing, do not write implementation code. Create the missing artifact first.

**Violation**: Writing implementation code without a corresponding feature and test is a hard stop. Undo and start over.

## Rule 2: The Chain of Traceability

Every line of code must trace back through:
```
Planset Item → Feature File → Test File → Implementation
```
If an agent cannot explain which planset item a piece of code serves, that code should not exist.

**GATE**: Before committing any code, verify the trace chain. If a link is broken, fix it before proceeding.

## Rule 3: Red-Green-Refactor (Strictly)

1. **RED**: Write a test that fails. Run it. Confirm it fails.
2. **GREEN**: Write the minimum code to make the test pass. Run it. Confirm it passes.
3. **REFACTOR**: Clean up. Run tests again. Confirm they still pass.

Do NOT skip steps. Do NOT write implementation before the test fails.

**GATE**: Each phase transition requires proof:
- RED → GREEN: Show failing test output, then passing test output
- GREEN → REFACTOR: Show all tests passing before AND after refactor
- If tests fail after refactor, you are not done. Fix before proceeding.

## Rule 4: Small, Atomic Changes

- One feature at a time
- One test at a time
- One commit per logical change
- Never bundle unrelated changes

**GATE**: Before committing, verify the commit addresses exactly one logical change. If it touches unrelated code, split it.

## Rule 5: Self-Documenting Progress

After completing any task, the agent MUST:
1. Update the sprint tracker in `sprints/active/`
2. Add a note to `docs/AGENT_NOTES.md` if any decisions were made
3. Generate a brief report if the task is a milestone

**GATE**: A task is not "complete" until all three documentation steps are done. Do not move to the next task until the current one is fully documented.

## Rule 6: Ask, Don't Assume

When encountering ambiguity:
- If it's a **design decision**: STOP and ask the human
- If it's an **implementation detail**: Make the simplest choice, document it, continue
- If it's a **dependency conflict**: STOP and ask the human

**GATE**: If you catch yourself making an assumption about user intent, architecture, or scope — stop and ask. Better to ask one extra question than build the wrong thing.

## Rule 7: Respect Existing Work

When retrofitting an existing project:
- Never delete code without a test proving it's dead
- Wrap existing functionality in features before modifying
- Preserve git history — no force pushes, no squashing others' work

## Rule 8: Stay in Scope

- Only work on items in the current sprint
- If you discover work that needs doing, add it to `sprints/backlog/` — do NOT do it now
- Scope creep is the enemy of shipping

**GATE**: Before starting any work, confirm it exists in the active sprint. If it doesn't, add it to backlog and return to the sprint.

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

---

## Enforcement

Every rule above that includes a **GATE** is an enforceable checkpoint. The agent must not proceed past a gate until its conditions are met. Gates are not aspirational — they are mandatory. If a gate fails, the agent must resolve the failure before continuing.

The order of precedence is: Rule 0 > Rule 1 > all other rules. Initialization and documentation always come first.
