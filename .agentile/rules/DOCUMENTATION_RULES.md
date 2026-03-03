# DOCUMENTATION_RULES.md — Documentation Standards

## Documentation is Not Optional

In Agentile, documentation is a first-class deliverable, not an afterthought. The agent writes documentation BEFORE code and updates it DURING development.

## Document Types and Locations

### Planset Documents (`planset/`)
Created during initialization, updated throughout the project.

**Executive Summary** (`planset/executive-summary/`)
- `VISION.md` — What are we building and why?
- `SCOPE.md` — What's in scope, what's explicitly out of scope?
- `STAKEHOLDERS.md` — Who cares about this and what do they need?
- `MILESTONES.md` — High-level timeline and deliverables

**Architecture** (`planset/architecture/`)
- `SYSTEM_DESIGN.md` — High-level architecture overview
- `DATA_MODEL.md` — Database schema, data flow
- `API_DESIGN.md` — Endpoints, contracts, protocols
- `TECH_STACK.md` — Chosen technologies with rationale
- `ADR-NNN.md` — Architecture Decision Records (use template)

### Living Documentation (`docs/`)
Updated continuously as the project evolves.

- `AGENT_NOTES.md` — Agent's working memory (decisions, insights, blockers)
- `CHANGELOG.md` — User-facing change log
- `SETUP.md` — How to set up the development environment
- `DEPLOYMENT.md` — How to deploy
- `TROUBLESHOOTING.md` — Common issues and solutions

## When to Document

| Event | Action |
|-------|--------|
| Project initialized | Create all planset documents |
| Architecture decision made | Write an ADR |
| Feature completed | Update CHANGELOG.md |
| Sprint completed | Generate sprint report |
| Bug discovered | Add to TROUBLESHOOTING.md |
| Environment changes | Update SETUP.md |
| Decision made during coding | Note in AGENT_NOTES.md |

## Documentation Quality Rules

1. **Write for a new developer** — Someone with no context should understand the project from docs alone
2. **No stale docs** — If code changes, docs change in the same commit
3. **Concrete over abstract** — Include examples, commands, file paths
4. **Link, don't duplicate** — Reference other docs instead of copying content
5. **Date your entries** — AGENT_NOTES.md and CHANGELOG.md entries must be dated

## Architecture Decision Records (ADRs)

Use `templates/ARCHITECTURE_DECISION.template.md` for every significant technical decision:
- Choosing a framework
- Database design choices
- API patterns
- Authentication strategy
- Anything the team might question later

ADRs are **immutable once accepted**. If a decision changes, create a new ADR that supersedes the old one.

## README.md Maintenance

The project root `README.md` must always contain:
1. What the project does (one paragraph)
2. How to install and run it
3. How to run tests
4. Link to `.agentile/` for detailed documentation
5. License
