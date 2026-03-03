# MANIFEST.md — Framework Index

This is the complete index of the Agentile framework. Use this to navigate the system.

## Entry Point
| File | Purpose |
|------|---------|
| `AGENT_ENTRY.md` | First file the agent reads. Orientation and decision tree. |
| `MANIFEST.md` | This file. Directory of everything. |
| `CONFIG.md` | Project-specific settings (language, framework, test runner, etc.) |

## Rules (Non-Negotiable)
| File | Purpose |
|------|---------|
| `rules/CORE_RULES.md` | Fundamental operating principles |
| `rules/BDD_RULES.md` | Gherkin syntax and feature file standards |
| `rules/TDD_RULES.md` | Red-Green-Refactor cycle rules |
| `rules/DOCUMENTATION_RULES.md` | What to document, when, and where |
| `rules/GIT_RULES.md` | Commit messages, branching, PR conventions |

## Roles (Agent Personas)
| File | Purpose |
|------|---------|
| `roles/ARCHITECT.md` | System design, ADRs, tech stack decisions |
| `roles/DEVELOPER.md` | Feature implementation via TDD |
| `roles/QA_ENGINEER.md` | Test coverage, edge cases, quality gates |
| `roles/SCRUM_MASTER.md` | Sprint management, velocity, reporting |
| `roles/TECH_WRITER.md` | README, API docs, changelogs |

## Workflows (Step-by-Step)
| File | Purpose |
|------|---------|
| `workflows/INIT_WORKFLOW.md` | New project initialization |
| `workflows/SPRINT_WORKFLOW.md` | Sprint lifecycle |
| `workflows/FEATURE_WORKFLOW.md` | Feature development lifecycle |
| `workflows/RETROFIT_WORKFLOW.md` | Adopting Agentile in an existing repo |
| `workflows/REVIEW_WORKFLOW.md` | Code review and quality gates |

## Templates (Copy & Fill)
| File | Purpose |
|------|---------|
| `templates/FEATURE.template.md` | Gherkin feature file template |
| `templates/SPRINT.template.md` | Sprint planning document |
| `templates/REPORT.template.md` | Sprint/task completion report |
| `templates/ARCHITECTURE_DECISION.template.md` | ADR template |
| `templates/USER_STORY.template.md` | User story with acceptance criteria |

## Living Directories
| Directory | Purpose |
|-----------|---------|
| `planset/executive-summary/` | Project vision, whitepaper, business case |
| `planset/architecture/` | System design, diagrams, ADRs |
| `features/` | Gherkin `.feature` files |
| `sprints/backlog/` | Unscheduled work items |
| `sprints/active/` | Current sprint plan and progress |
| `sprints/completed/` | Archived sprints with retrospectives |
| `reports/` | Agent-generated progress reports |
| `docs/` | Living project documentation |
