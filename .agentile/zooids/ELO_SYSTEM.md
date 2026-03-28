# ELO Scoring System

> **Every contribution has a score. Every score tells a story.**

The Agentile ELO system tracks contributor competence over time. It governs which zooids a contributor may operate as, what gates they must pass, and how much autonomy they have. ELO scores are earned through demonstrated work -- not seniority, not credentials, not promises.

---

## Tiers

| Tier | ELO Range | Autonomy Level | Available Zooids |
|------|-----------|----------------|-----------------|
| **Novice** | 0 - 799 | Guided work only. All PRs require Expert+ review. | TECH_WRITER |
| **Journeyman** | 800 - 1199 | Standard workflow with mandatory review gates. | DEVELOPER, QA_ENGINEER, TECH_WRITER |
| **Expert** | 1200 - 1599 | Relaxed gates. Can review others. Architecture input. | All except FORMAL_VERIFIER |
| **Master** | 1600+ | Full autonomy. Can propose architecture. Can mentor. | All zooids |

### Tier Transitions

- **Promotion**: When ELO crosses a tier boundary upward, the contributor gains access immediately
- **Demotion**: When ELO drops below a tier boundary, access is lost at the end of the current sprint (grace period)
- **Floor**: ELO cannot drop below 0

---

## Scoring Events

### Positive Events (ELO Gains)

| Event | Points | Conditions |
|-------|--------|------------|
| **Test passes on first commit** | +5 | All tests green on first run after committing |
| **PR merged without changes requested** | +15 | Zero revision requests |
| **PR merged with minor changes** | +10 | One round of minor feedback |
| **Sprint completed on time** | +20 | All WPs complete by sprint end date |
| **Coverage increased by 5%+** | +10 | Module-level coverage increase |
| **TLA+ spec verified (0 violations)** | +25 | TLC completes with zero violations |
| **Bug found in review** | +10 | Genuine bug (not style nit) found |
| **Documentation updated with code** | +5 | Same PR includes doc updates |
| **Fuzz target finds a bug** | +15 | Fuzz target discovers a crash or logic error |
| **Zero lint warnings on large PR** | +5 | PR touches 200+ lines, zero new warnings |
| **Mentored a Novice to Journeyman** | +20 | Mentee crosses 800 ELO |
| **Regression test prevents re-occurrence** | +10 | Test catches re-introduction in later PR |

### Negative Events (ELO Penalties)

| Event | Points | Conditions |
|-------|--------|------------|
| **Test regression introduced** | -15 | Previously passing test fails |
| **Lint warning introduced** | -10 | Linter fails after commit |
| **PR rejected (quality)** | -10 | Closed without merge due to quality |
| **Stale documentation left** | -5 | Code change merged without doc update |
| **TODO/stub committed** | -20 | TODO, FIXME, or stub in committed code |
| **Force push to main** | -50 | Destructive action |
| **Skipped tests without reason** | -15 | Tests marked skip without documentation |
| **Coverage decreased** | -10 | Coverage drops after PR merges |
| **Sprint WP missed without escalation** | -10 | Not completed, no blocker reported |
| **Spec committed with known violations** | -25 | TLC reports violations |

---

## Starting ELO

| Entry Path | Starting ELO | Starting Tier |
|------------|-------------|---------------|
| **Quiz: 0 correct** | 0 | Novice |
| **Quiz: 1 correct** | 200 | Novice |
| **Quiz: 2 correct** | 400 | Novice |
| **Quiz: 3 correct** | 600 | Novice |
| **Quiz: 4 correct** | 800 | Journeyman |
| **Quiz: 5 correct** | 1000 | Journeyman |
| **Skip Protocol** | 800 | Journeyman |

Maximum starting ELO is 1000. Expert and Master must be earned through contributions.

---

## Tracking

### REGISTRY.md Format

All ELO scores are tracked in `.agentile/zooids/REGISTRY.md`:

```markdown
# Contributor Registry

| Name / Agent ID | Starting ELO | Current ELO | Tier | Active Zooid | Entry Date | Last Event Date |
|-----------------|-------------|-------------|------|-------------|------------|-----------------|
| contributor-1 | 800 | 1245 | Expert | DEVELOPER | 2026-01-15 | 2026-03-18 |

## Event Log

| Date | Contributor | Event | Points | New ELO | Evidence |
|------|------------|-------|--------|---------|----------|
| 2026-03-18 | contributor-1 | PR merged without changes | +15 | 1245 | PR #142 |
```

---

## Audit Trail

The REGISTRY.md file is the source of truth for ELO scores. It must:
- Never have entries deleted (append-only)
- Have every change traceable to a git commit
- Be reviewed at each sprint boundary
