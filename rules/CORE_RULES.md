# Core Rules

> 10 non-negotiable rules for every contributor -- human or AI.
> Violations are marked as **BLOCKER** (stops all work) or **GATE** (stops the current step).

---

## Rule 0: Read AGENT_ENTRY.md Before Any Work

**Statement:** Every session begins by reading `AGENT_ENTRY.md`. No exceptions.

**Why:** Context drift is the primary cause of wasted work. Cold-starting without orientation produces code that conflicts with active sprints, duplicates existing implementations, or violates architectural decisions.

**Verification:**
- The first action in any session log is reading `AGENT_ENTRY.md`.
- CONFIG.md constants are referenced correctly in all produced artifacts.

**On violation:** **BLOCKER.** All work produced without reading AGENT_ENTRY.md is suspect and must be re-reviewed from scratch.

---

## Rule 1: Plan Before You Code

**Statement:** Check `sprints/CURRENT.md` and the active sprint's `SPRINT.md` before writing any code. If no sprint exists for the work, create one first.

**Why:** Unplanned work creates orphan code that nobody expects, nobody reviews, and nobody maintains. Sprint files are how the team (and future agents) know what happened and why.

**Verification:**
- Every commit traces to a Work Package (WP) in an active sprint.
- `sprints/CURRENT.md` reflects the work being done.

**On violation:** **GATE.** Code written without a sprint item will not be merged. Create the sprint entry retroactively and justify the gap.

---

## Rule 2: No Mocks, Stubs, or TODOs in Production Code

**Statement:** All delivered code must be fully functional and production-ready. Do not create mock implementations, placeholder functions, TODO comments, or stub methods.

**Why:** Incomplete code is tech debt with compound interest. It misleads other contributors into thinking functionality exists when it does not. In a production system, stubs can mask critical safety gaps.

**Verification:**
- `grep -rn "TODO\|FIXME\|HACK\|STUB\|unimplemented!\|todo!" src/` returns zero hits in new code.
- All public functions have complete implementations with error handling.

**On violation:** **GATE.** PR will not be approved. Replace every stub with a working implementation or remove the dead code entirely.

---

## Rule 3: Test Count Only Increases (Test Ratchet)

**Statement:** The total number of passing tests must increase or stay the same with every sprint. It must never decrease.

**Why:** Removing tests to make the build green is a hallmark of codebase decay. If a test is genuinely obsolete, it must be replaced with an equivalent or better test in the same commit.

**Verification:**
- Record baseline in `coverage/BASELINE.md` at sprint start.
- Record final count in sprint `REPORT.md` at sprint end.
- Test result output shows count >= previous sprint.

**On violation:** **BLOCKER.** Sprint cannot close with fewer passing tests than it started with. Restore or replace removed tests before proceeding.

---

## Rule 4: Every Feature Traces to a Sprint Item

**Statement:** Every feature, bug fix, or refactor must be linked to a Work Package in an active sprint. Untracked work is not permitted.

**Why:** Traceability is how we maintain accountability, generate accurate reports, and enable future agents to understand the provenance of every change.

**Verification:**
- Commit messages reference a WP (e.g., `feat(WP-3.2): add proposer election`).
- Sprint `SPRINT.md` has a corresponding entry with status updates.

**On violation:** **GATE.** Commit must be amended with a WP reference, or a sprint entry must be created retroactively.

---

## Rule 5: All Code Must Pass Linting with Zero Warnings

**Statement:** Code must compile cleanly under the project's linting tool with warnings treated as errors. No suppressions without documented justification.

**Why:** Lint warnings catch real bugs, performance issues, and idiomatic violations. Suppressing them without cause hides problems that compound over time.

**Verification:**
- CI runs the linter with warnings-as-errors on every PR.
- Any lint suppression annotation must have an adjacent comment explaining why.

**On violation:** **GATE.** PR will not merge. Fix the warnings or document the suppression justification.

---

## Rule 6: Audits Are Dated and Immutable

**Statement:** Audit reports are created in dated directories (`audits/YYYY-MM-DD-name/`) and are never modified after creation. Corrections go in new audit files.

**Why:** Audit integrity depends on immutability. If audits can be retroactively edited, they lose all evidentiary value. Historical accuracy requires that past findings remain exactly as they were recorded.

**Verification:**
- Audit directories follow `YYYY-MM-DD-name` format.
- Git log shows no modifications to files in `audits/` directories after their creation date.
- New findings produce new audit files, not edits to old ones.

**On violation:** **BLOCKER.** Revert the edit. Create a new audit file with the correction and a reference to the original.

---

## Rule 7: Documentation Updates Accompany Code Changes

**Statement:** Code changes that affect public APIs, configuration, architecture, or user-facing behavior must include corresponding documentation updates in the same PR.

**Why:** Documentation that lags behind code is worse than no documentation -- it actively misleads. Keeping docs and code in the same commit ensures they stay synchronized.

**Verification:**
- PRs that touch public functions, structs, config files, or CLI arguments include doc updates.
- Crate/module READMEs reflect the current API surface.
- CHANGELOG has an entry for user-facing changes.

**On violation:** **GATE.** PR will not merge until documentation is updated. Reviewers check for doc changes alongside code changes.

---

## Rule 8: Security-Sensitive Changes Require Review

**Statement:** Changes to consensus, cryptography, key management, transaction validation, or access control must be reviewed by at least one other contributor (human or qualified agent) before merge.

**Why:** Security bugs in a production system are exploitable and potentially irreversible. The cost of a missed vulnerability far exceeds the cost of a review cycle.

**Verification:**
- PRs touching security-sensitive paths require an approving review.
- The reviewer is recorded in the PR metadata.
- Formal verification (TLA+ or invariant tests) is strongly recommended for consensus changes (see `FORMAL_VERIFICATION_RULES.md`).

**On violation:** **BLOCKER.** Merge is blocked. No self-approvals for security-sensitive paths.

---

## Rule 9: The Sprint File Is the Source of Truth

**Statement:** The sprint `SPRINT.md` file is the canonical record of what was planned, what was done, and what the outcome was. Not your memory. Not a chat log. Not an internal plan file.

**Why:** Agents have no persistent memory across sessions. Humans forget. Chat logs are unsearchable and ephemeral. The sprint file is the only artifact that survives and is accessible to all future contributors.

**Verification:**
- Sprint status is updated in `SPRINT.md` after every work session.
- `DAILY.md` entries exist for each active work day.
- Completed sprints have a `REPORT.md` and `RETRO.md`.

**On violation:** **GATE.** Work that is not recorded in the sprint file did not happen. Update the file before claiming completion.

---

## Quick Reference

| # | Rule | Enforcement |
|---|------|-------------|
| 0 | Read AGENT_ENTRY.md first | BLOCKER |
| 1 | Plan before you code | GATE |
| 2 | No mocks/stubs/TODOs | GATE |
| 3 | Test ratchet (count never decreases) | BLOCKER |
| 4 | Every feature traces to a sprint item | GATE |
| 5 | Linting clean (zero warnings) | GATE |
| 6 | Audits are immutable | BLOCKER |
| 7 | Docs accompany code changes | GATE |
| 8 | Security changes need review | BLOCKER |
| 9 | Sprint file is source of truth | GATE |
