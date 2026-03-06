# DOCUMENTATION_RULES.md — Documentation Standards

> **Documentation is a first-class deliverable in Agentile. Every documentation requirement below has an enforceable gate.**

---

## Documentation is Not Optional

In Agentile, documentation is written BEFORE code and updated DURING development. Undocumented work is incomplete work.

**GATE: A feature is not "done" until its documentation is complete. A sprint is not "done" until all docs are current. Missing docs block phase advancement just like failing tests.**

---

## Document Types and Locations

### Planset Documents (`planset/`)
Created during initialization, updated throughout the project.

**Executive Summary** (`planset/executive-summary/`)
- `VISION.md` — What are we building and why?
- `SCOPE.md` — What's in scope, what's explicitly out of scope?
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

---

## When to Document

| Event | Action | Gate |
|-------|--------|------|
| Project initialized | Create all planset documents | Init is not complete without all planset docs |
| Architecture decision made | Write an ADR | Decision cannot be implemented without an ADR |
| Feature completed | Update CHANGELOG.md | Feature not marked DONE without CHANGELOG entry |
| Sprint completed | Generate sprint report | Sprint not archived without report |
| Bug discovered | Add to TROUBLESHOOTING.md | Bug fix commit must include doc update |
| Environment changes | Update SETUP.md | Config change commit must include SETUP.md update |
| Decision made during coding | Note in AGENT_NOTES.md | Decisions not logged are decisions lost |

**GATE: For each event above, the corresponding documentation update must happen in the same commit or immediately after. Stale docs are treated as a review failure.**

---

## Documentation Quality Rules

1. **Write for a new developer** — Someone with no context should understand the project from docs alone
2. **No stale docs** — If code changes, docs change in the same commit
3. **Concrete over abstract** — Include examples, commands, file paths
4. **Link, don't duplicate** — Reference other docs instead of copying content
5. **Date your entries** — AGENT_NOTES.md and CHANGELOG.md entries must be dated

**GATE: During sprint review (REVIEW_WORKFLOW.md), every doc is checked for staleness. A doc that references code that no longer exists, APIs that have changed, or setup steps that no longer work fails review.**

---

## Architecture Decision Records (ADRs)

Use `templates/ARCHITECTURE_DECISION.template.md` for every significant technical decision:
- Choosing a framework
- Database design choices
- API patterns
- Authentication strategy
- Anything the team might question later

ADRs are **immutable once accepted**. If a decision changes, create a new ADR that supersedes the old one.

**GATE: Every major technology choice must have an ADR before implementation begins. "Major" means: a new dependency, a new pattern, a data model change, or an infrastructure choice. If you're about to `npm install` a new package or introduce a new architectural pattern, write the ADR first.**

---

## README.md Maintenance

The project root `README.md` must always contain:
1. What the project does (one paragraph)
2. How to install and run it
3. How to run tests
4. Link to `.agentile/` for detailed documentation
5. License

**GATE: README must be updated by end of every sprint. If the project's setup, test command, or purpose has changed, the README must reflect it before the sprint is archived.**

---

## Completeness Checklist

**At any point, the documentation state can be audited against this list:**

- [ ] Planset docs exist and describe the current project (not aspirational, not stale)
- [ ] Every feature in the current sprint has a `.feature` file
- [ ] CHANGELOG.md has an entry for every user-facing change
- [ ] AGENT_NOTES.md has entries for every decision made since last session
- [ ] Sprint tracker in `sprints/active/` reflects the true current state
- [ ] README.md setup instructions actually work
- [ ] No code exists without a corresponding planset trace

**GATE: If the documentation audit fails any item, fix it before proceeding with new work. Documentation debt is treated with the same urgency as test debt.**
