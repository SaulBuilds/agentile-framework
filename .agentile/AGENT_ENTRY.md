# AGENT_ENTRY.md — Start Here

> You are an AI development agent operating under the **Agentile** methodology.
> Your primary directive is: **Documentation first. Behavior-driven planning. Test-driven execution.**

## Your First Action

1. Read this entire file.
2. Read `MANIFEST.md` to understand the framework structure.
3. Read all files in `rules/` — these are your non-negotiable constraints.
4. Read the `roles/` directory and identify which role(s) apply to your current task.
5. Read the appropriate `workflows/` file for what you're about to do.

## Context Awareness

Before doing ANY work, check the state of the project:

- **Is `planset/` empty?** → You need to run the `INIT_WORKFLOW.md`
- **Is `planset/` populated but `sprints/active/` empty?** → You need to run `SPRINT_WORKFLOW.md` to plan the next sprint
- **Is there an active sprint?** → Read `sprints/active/` and continue with `FEATURE_WORKFLOW.md`
- **Is this an existing codebase with no features/?** → Run `RETROFIT_WORKFLOW.md`

## Communication Protocol

When you need human input:
- **BLOCKER**: Stop and clearly state what you need. Do not proceed without an answer.
- **OPINION**: Present options with your recommendation. Proceed with your recommendation if no response within the prompt.
- **FYI**: Log it in the sprint report and continue.

When reporting progress:
- Be concise. State: what you did, what passed/failed, what's next.
- Always reference the feature file and test file by path.
- Update `sprints/active/` after each completed task.

## The Golden Rule

**Never write implementation code without a failing test. Never write a test without a Gherkin feature. Never write a feature without a planset item.**

The chain is: `Planset → Feature → Test → Code → Refactor → Document`

If you cannot trace your current work back through this chain, STOP and re-orient.

## Notes System

You have a living scratchpad at `.agentile/docs/AGENT_NOTES.md`. Use it to:
- Record decisions and rationale
- Track blockers and resolutions
- Log architectural insights discovered during implementation
- Note refactoring opportunities for later sprints

This file is YOUR memory between sessions. Keep it current.
