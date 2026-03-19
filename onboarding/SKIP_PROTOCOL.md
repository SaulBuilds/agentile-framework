# Skip Protocol -- Experienced Engineer Fast Track

> **For qualified engineers who want to skip the onboarding quiz and start contributing immediately.**

## Overview

The Skip Protocol allows experienced developers to bypass the adaptive quiz and enter the Agentile framework at Journeyman tier (800 ELO). This is a fast track, not a free pass -- contributors who skip the quiz still must follow all core rules and earn Expert/Master through demonstrated work.

---

## Eligibility Criteria

A contributor qualifies for the Skip Protocol if they meet **at least one** of the following:

### Path 1: Merged Pull Requests

- **3 or more merged PRs** in any public project in the same domain
- PRs must be substantive (not typo fixes or dependency bumps)
- PRs must be verifiable via GitHub / GitLab links

### Path 2: Contribution History

- **50 or more domain-related contributions** visible on a public GitHub profile
- Contributions include: commits, PRs, issues, and code reviews in related repositories
- The 50-contribution threshold is approximate -- quality matters more than count

### Path 3: Recognized Expertise

- Published author of a relevant technical paper, security audit, or formal verification report
- Core maintainer or significant contributor to a project with 500+ GitHub stars
- Speaker at a recognized engineering conference
- Author of a widely-used development tool or library

---

## Skip Protocol Process

### Step 1: Declaration

The contributor declares their intent to skip by providing:

```markdown
## Skip Protocol Request

**Name / Agent ID**: [identifier]
**Date**: YYYY-MM-DD
**Eligibility Path**: [1, 2, or 3]
**Evidence**:
- [Link to PR 1]
- [Link to PR 2]
- [Link to PR 3]
- [OR: GitHub profile link showing contribution history]
- [OR: Link to published work]
```

### Step 2: Verification

A contributor at Expert tier (1200+) or higher reviews the evidence and confirms eligibility. If no Expert+ contributor is available (e.g., the project is bootstrapping), the project maintainer verifies.

Verification checks:
- PRs are real, merged, and substantive
- Contributions are to genuine projects (not forks with no community)
- Published work is legitimate and relevant

### Step 3: Agreement

The contributor must explicitly agree to:

1. **Follow CORE_RULES.md** -- All non-negotiable rules apply regardless of experience
2. **Follow the PR template** -- Every PR follows the project's template format
3. **Follow the zooid's hard gates** -- No exemptions from test, lint, or coverage gates
4. **Accept ELO scoring** -- All contributions will be scored per the ELO system
5. **Accept demotion** -- If ELO drops below 800, the contributor returns to Novice tier until they recover

Agreement is recorded by adding a line to REGISTRY.md.

### Step 4: Registration

The contributor is added to REGISTRY.md:

```markdown
| [Name/ID] | 800 | 800 | Journeyman | [Zooid] | YYYY-MM-DD | YYYY-MM-DD |
```

Entry note in the event log:

```markdown
| YYYY-MM-DD | [Name/ID] | Skip Protocol (Path N) | +800 | 800 | [Evidence link] |
```

### Step 5: Zooid Assignment

Skip Protocol contributors choose their starting zooid based on their background:

| Background | Recommended Zooid | Rationale |
|-----------|-------------------|-----------|
| Systems developer | DEVELOPER | Direct path to core implementation |
| Test/QA engineer | QA_ENGINEER | Direct path to test infrastructure |
| Technical writer / DevRel | TECH_WRITER | Direct path to documentation (starts at 800, above Novice) |
| Generalist / Full-stack | DEVELOPER | Most versatile starting point |

The contributor may choose any Journeyman-tier zooid regardless of background. The table above is a recommendation, not a requirement.

---

## What the Skip Protocol Does NOT Grant

- **Expert tier**: Starting ELO is 800 (Journeyman). Expert (1200+) must be earned.
- **Master tier**: 1600+ must be earned through sustained excellent contributions.
- **ARCHITECT access**: Requires Expert tier (1200+). Must be earned.
- **FORMAL_VERIFIER access**: Requires Master tier (1600+). Must be earned.
- **Self-merge privileges**: All Journeyman PRs require Expert+ review.
- **Exemption from hard gates**: TDD, lint, coverage, and documentation gates apply to everyone.

---

## Earning Expert and Master After Skipping

A Skip Protocol contributor reaches Expert (1200+) the same way as any other contributor -- through the ELO scoring events:

**Fastest path from 800 to 1200 (400 points needed):**

| Scenario | Points | Count | Total |
|----------|--------|-------|-------|
| PRs merged without changes | +15 each | 10 PRs | +150 |
| Tests pass on first commit | +5 each | 10 commits | +50 |
| Coverage increased by 5%+ | +10 each | 3 PRs | +30 |
| Sprint completed on time | +20 each | 2 sprints | +40 |
| Docs updated with code | +5 each | 10 PRs | +50 |
| Bugs found in review | +10 each | 5 reviews | +50 |
| Fuzz target finds a bug | +15 each | 2 targets | +30 |

Total: +400 points across approximately 2-3 sprints of active contribution.

This is intentionally achievable but not trivial. A competent engineer who writes clean, tested, documented code will reach Expert within a few weeks of active work.

---

## Revoking Skip Protocol Status

The Skip Protocol entry is permanent -- it cannot be revoked. However, ELO can decrease through negative events, potentially dropping the contributor below Journeyman:

- If ELO drops below 800, the contributor returns to Novice tier
- They retain their REGISTRY.md entry and event history
- They can regain Journeyman through positive events (no need to re-skip or re-quiz)
- The ELO floor is 0

This is by design. The Skip Protocol trusts contributors based on their past work, but the ELO system validates that trust through ongoing contributions.

---

## FAQ

**Q: Can I skip the quiz and start at Expert or Master?**
A: No. The maximum starting ELO via any entry path is 1000 (quiz) or 800 (skip). Expert and Master are earned, not granted.

**Q: I have extensive formal verification experience. Can I start as FORMAL_VERIFIER?**
A: No. FORMAL_VERIFIER requires Master tier (1600+). You start at Journeyman and earn your way up. Your formal methods expertise will accelerate your path through high-value TLA+ contributions (+25 per verified spec).

**Q: What if my evidence is in a private repository?**
A: Private repository contributions can be verified by a project maintainer who has access, or by providing screenshots/exports of PR descriptions and review threads (with sensitive information redacted).

**Q: Can an AI agent use the Skip Protocol?**
A: Yes. An AI agent with a documented history of code generation in the relevant domain qualifies under Path 1 or Path 2. The verification step still applies -- another contributor must confirm the evidence.
