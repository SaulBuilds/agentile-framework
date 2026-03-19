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

- **Promotion**: When ELO crosses a tier boundary upward, the contributor gains access to that tier's zooids immediately
- **Demotion**: When ELO drops below a tier boundary, the contributor loses access to the higher tier's zooids at the end of the current sprint (grace period to complete in-progress work)
- **Floor**: ELO cannot drop below 0. A contributor at 0 ELO can still operate as TECH_WRITER.

---

## Scoring Events

### Positive Events (ELO Gains)

| Event | Points | Conditions | Rationale |
|-------|--------|------------|-----------|
| **Test passes on first commit** | +5 | All tests green on the first test run after committing | Rewards careful, test-driven development |
| **PR merged without changes requested** | +15 | PR approved and merged with zero revision requests | Clean work that meets the bar on first attempt |
| **PR merged with minor changes** | +10 | PR merged after one round of minor feedback | Good work that needed small polish |
| **Sprint completed on time** | +20 | All WPs in the sprint plan marked COMPLETE by the sprint end date | Reliable delivery against commitments |
| **Coverage increased by 5%+** | +10 | Module-level coverage increases by 5+ percentage points in a single PR | Direct investment in code quality |
| **TLA+ spec verified (0 violations)** | +25 | TLC model checker completes with zero invariant violations on a new or updated spec | Formal correctness is the highest bar |
| **Bug found in review** | +10 | Reviewer identifies a genuine bug (not style nit) in another contributor's PR | Catching bugs early saves everyone time |
| **Documentation updated with code** | +5 | PR that changes code also updates relevant documentation in the same PR | Keeps docs and code in sync |
| **Fuzz target finds a bug** | +15 | A fuzz target written by the contributor discovers a crash or logic error | Proactive quality investment |
| **Zero lint warnings on large PR** | +5 | PR touches 200+ lines and introduces zero new lint warnings | Discipline at scale |
| **Mentored a Novice to Journeyman** | +20 | A Novice the contributor mentored crosses the 800 ELO threshold | Investing in the team pays double |
| **Regression test prevents re-occurrence** | +10 | A regression test written for a bug catches a re-introduction in a later PR | Proving the test's value retrospectively |

### Negative Events (ELO Penalties)

| Event | Points | Conditions | Rationale |
|-------|--------|------------|-----------|
| **Test regression introduced** | -15 | A previously passing test fails after the contributor's commit | Breaking existing functionality is costly |
| **Lint warning introduced** | -10 | Linter fails after the contributor's commit | Lint discipline prevents deeper issues |
| **PR rejected (quality)** | -10 | PR closed without merge due to quality issues (not scope change) | Submitting unready work wastes reviewer time |
| **Stale documentation left** | -5 | Code change merged without updating documentation that references the changed behavior | Stale docs are worse than no docs |
| **TODO/stub committed** | -20 | `TODO`, `FIXME`, `unimplemented!()`, `todo!()` found in committed code | Violates the "no stubs" core rule |
| **Force push to main** | -50 | `git push --force` to the `main` branch | Destructive action that can lose work |
| **Skipped tests in commit** | -15 | Tests marked as ignored/skipped without a documented reason linked to a tracking issue | Hiding failures is not fixing them |
| **Coverage decreased** | -10 | Module-level coverage drops by any amount after the contributor's PR merges | Every PR should improve or maintain coverage |
| **Sprint WP missed without escalation** | -10 | A work package assigned to the contributor is not completed by sprint end, and no blocker was reported | Silent failure erodes trust |
| **Spec committed with known violations** | -25 | A TLA+ spec is committed where TLC reports invariant violations | False confidence in correctness is dangerous |

---

## Scoring Mechanics

### How Points Are Applied

1. **Events are recorded in REGISTRY.md** with timestamp, event type, points, and evidence (commit hash, PR number, or TLC output)
2. **Points are applied immediately** -- there is no batch processing
3. **Disputes**: A contributor may dispute a negative event by opening a discussion. A Master-tier contributor adjudicates.

### Multi-Event Scenarios

- A single PR can trigger multiple events (e.g., +15 for clean merge AND +10 for coverage increase AND +5 for docs updated = +30 total)
- Negative events from the same commit stack (e.g., test regression AND lint warning = -25 total)
- A fix PR for a regression does NOT cancel the original penalty -- it earns its own positive score

### Decay

- ELO does not decay over time. A contributor who takes a break returns at their last recorded ELO.
- However, zooid assignment may be re-evaluated if a contributor has been inactive for 90+ days (they take the quiz again at their current ELO as a baseline).

---

## Starting ELO

| Entry Path | Starting ELO | Starting Tier | Zooid |
|------------|-------------|---------------|-------|
| **Onboarding Quiz: 0 correct** | 0 | Novice | TECH_WRITER |
| **Onboarding Quiz: 1 correct** | 200 | Novice | TECH_WRITER |
| **Onboarding Quiz: 2 correct** | 400 | Novice | DEVELOPER (guided) |
| **Onboarding Quiz: 3 correct** | 600 | Novice | DEVELOPER |
| **Onboarding Quiz: 4 correct** | 800 | Journeyman | QA_ENGINEER or DEVELOPER |
| **Onboarding Quiz: 5 correct** | 1000 | Journeyman | Any Journeyman-tier zooid |
| **Skip Protocol** | 800 | Journeyman | Based on background |
| **Returning contributor (90+ day gap)** | Previous ELO | Based on ELO | Re-quiz for zooid assignment |

Note: The quiz caps starting ELO at 1000 (Journeyman). Expert and Master must be earned through contributions. No one starts above Journeyman.

---

## Tracking

### REGISTRY.md Format

All ELO scores are tracked in `.agentile/zooids/REGISTRY.md`:

```markdown
# Contributor Registry

| Name / Agent ID | Starting ELO | Current ELO | Tier | Active Zooid | Entry Date | Last Event Date |
|-----------------|-------------|-------------|------|-------------|------------|-----------------|
| agent-001 | 800 | 1245 | Expert | DEVELOPER | 2026-01-15 | 2026-03-18 |
```

### Event Log Format

Below the registry table, maintain a chronological event log:

```markdown
## Event Log

| Date | Contributor | Event | Points | New ELO | Evidence |
|------|------------|-------|--------|---------|----------|
| 2026-03-18 | agent-001 | PR merged without changes | +15 | 1245 | PR #142 |
```

---

## ELO and Zooid Reassignment

Contributors are not permanently bound to a zooid. They may switch zooids if:

1. Their ELO qualifies them for the new zooid's minimum tier
2. The current sprint does not depend on their current zooid assignment
3. They notify the SCRUM_MASTER of the switch

A contributor may hold multiple zooid qualifications but may only operate as ONE zooid at a time within a single sprint. Switching mid-sprint requires SCRUM_MASTER approval.

---

## Audit Trail

The REGISTRY.md file is the source of truth for ELO scores. It must:

- Never have entries deleted (append-only)
- Have every change traceable to a git commit
- Be reviewed by SCRUM_MASTER at each sprint boundary
- Be backed up before any sprint transition
