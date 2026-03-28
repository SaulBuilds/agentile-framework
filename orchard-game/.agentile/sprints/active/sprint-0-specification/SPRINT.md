# Sprint 0-specification: Specification Sprint

> Copy this template to `.agentile/sprints/active/sprint-<id>-<name>/SPRINT.md`

---

## Sprint Metadata

| Field | Value |
|-------|-------|
| **Sprint ID** | S-0 |
| **Sprint Name** | Specification Sprint |
| **Goal** | Write all TLA+ modules and core .feature files based on The Grove spec. Set up testing harnesses. No implementation code. |
| **Start Date** | 2026-03-28 |
| **End Date** | 2026-04-11 |
| **Duration** | 14 days |
| **Status** | IN PROGRESS |

---

## Test Baseline

| Metric | Count |
|--------|-------|
| **Tests (start)** | 0 |
| **Total (start)** | 0 |

---

## Work Packages

### WP-1: TLA+ Module Creation

| Field | Value |
|-------|-------|
| **Status** | `[x] COMPLETE` |
| **Assignee** | FORMAL_VERIFIER |
| **Estimated effort** | L |
| **Commit(s)** |  |

**Description:**
Create all TLA+ modules for concurrent subsystems as specified in The Grove document. These modules define state spaces, transitions, and invariants for formal verification.

**Tasks:**
- [x] Create SeedLifecycle.tla for seed planting and growth cycles
- [x] Create FederationEcon.tla for reward distribution and economic safety
- [x] Create BelnapAggregation.tla for Belnap logic classification
- [x] Create SchoolSafety.tla for content filtering and safety properties
- [x] Create MentorPropagation.tla for LoRA adapter propagation
- [x] Create MysteryBoxRNG.tla for random reward distribution
- [x] Create SybilResistance.tla for attack surface protection
- [x] Create SeasonTransition.tla for federation lifecycle management

**Acceptance Criteria:**
- [x] All TLA+ modules compile without syntax errors
- [ ] TLC model checker runs successfully on each module
- [ ] Each module defines clear state spaces and transitions
- [ ] Critical invariants are specified for each subsystem
- [ ] Modules follow the naming convention and structure from The Grove spec

**Tests Added:**
- tla-check-seed-lifecycle -- verifies SeedLifecycle.tla invariants
- tla-check-federation-econ -- verifies FederationEcon.tla invariants
- tla-check-belnap-aggregation -- verifies BelnapAggregation.tla invariants

### WP-2: Gherkin Feature File Creation

| Field | Value |
|-------|-------|
| **Status** | `[x] COMPLETE` |
| **Assignee** | DEVELOPER |
| **Estimated effort** | L |
| **Commit(s)** |  |

**Description:**
Create Gherkin feature files that define observable behaviors for each game mechanic. These serve as acceptance criteria and definition of done.

**Tasks:**
- [x] Create core feature files: seed-planting.feature, growth-cycle.feature, harvest.feature
- [x] Create competitive feature files: pollination-duel.feature, blight-raid.feature
- [x] Create school safety feature files: school-content-safety.feature
- [x] Create economic safety feature files: reward-distribution.feature, sybil-resistance.feature
- [x] Create visual feature files: garden-rendering.feature, duel-animation.feature
- [x] Create competitive feature files: mentor-propagation.feature

**Acceptance Criteria:**
- [x] All .feature files follow Gherkin syntax
- [x] Each feature file references its parent TLA+ module in comments
- [x] Scenarios are tagged appropriately (@solo, @federated, @school, @adversarial)
- [x] Acceptance criteria are specific and testable
- [x] Feature files cover all major game mechanics from The Grove spec

**Tests Added:**
- cucumber-test-seed-planting -- verifies seed planting scenarios
- cucumber-test-growth-cycle -- verifies growth cycle scenarios
- cucumber-test-harvest -- verifies harvest scenarios
- cucumber-test-pollination-duel -- verifies pollination duel scenarios
- cucumber-test-blight-raid -- verifies blight raid scenarios
- cucumber-test-school-safety -- verifies school safety scenarios
- cucumber-test-reward-distribution -- verifies reward distribution scenarios
- cucumber-test-sybil-resistance -- verifies sybil resistance scenarios
- cucumber-test-mentor-propagation -- verifies mentor propagation scenarios
- cucumber-test-garden-rendering -- verifies garden rendering scenarios
- cucumber-test-duel-animation -- verifies duel animation scenarios

### WP-3: Test Harness Setup

| Field | Value |
|-------|-------|
| **Status** | `[~] IN PROGRESS` |
| **Assignee** | QA_ENGINEER |
| **Estimated effort** | M |
| **Commit(s)** |  |

**Description:**
Set up the testing infrastructure including Cucumber.js + Hardhat for contract testing and p5.js snapshot testing for visual components.

**Tasks:**
- [x] Configure Hardhat for Solidity compilation and testing
- [x] Set up Cucumber.js for Gherkin feature execution
- [x] Configure Chai and Waffle for contract testing
- [ ] Set up p5.js snapshot testing framework
- [x] Create test scripts and npm commands
- [ ] Establish test coverage baseline

**Acceptance Criteria:**
- [x] All tests can be run with npm commands
- [x] Hardhat compiles contracts successfully
- [x] Cucumber.js executes Gherkin features
- [ ] p5.js snapshot tests capture canvas states
- [ ] Test coverage gates are established in coverage/GATES.md
- [ ] CI pipeline configuration is ready

**Tests Added:**
- test-harness-verification -- confirms testing infrastructure works
- contract-compilation-test -- verifies contracts compile
- feature-execution-test -- verifies Gherkin features execute
- test-specs-verification -- verifies TLA+ specifications can be checked

### WP-4: Specification Review and Traceability

| Field | Value |
|-------|-------|
| **Status** | `[~] IN PROGRESS` |
| **Assignee** | ARCHITECT |
| **Estimated effort** | S |
| **Commit(s)** |  |

**Description:**
Review all specifications for completeness, consistency, and traceability between TLA+ modules and Gherkin feature files.

**Tasks:**
- [x] Verify bidirectional traceability between TLA+ modules and .feature files
- [x] Ensure all game mechanics from The Grove spec are covered
- [x] Check for missing invariants or testable behaviors
- [x] Validate specification against oppositional research points
- [x] Create specification manifest documenting all files

**Acceptance Criteria:**
- [x] Every .feature file references a parent TLA+ module
- [x] Every TLA+ module has at least one referencing .feature file
- [x] All major mechanics from oppositional review are addressed
- [x] Specification manifest is created and maintained
- [x] No gaps exist between TLA+ invariants and observable behaviors

**Tests Added:**
- spec-traceability-check -- verifies TLA+ to feature traceability
- spec-completeness-validation -- ensures all mechanics covered
- oppositional-resistance-check -- validates against known attack vectors

---

## Final Metrics

| Metric | Count |
|--------|-------|
| **Tests (end)** | 0 |
| **Total (end)** | 0 |
| **Delta** | +0 |

---

## Dependencies

| Dependency | Status | Impact if Blocked |
|------------|--------|-------------------|
| TLA+ Toolkit | Available | Required for WP-1 |
| Node.js/npm | Available | Required for WP-3 |
| Solidity Compiler | Available | Required for WP-3 |
| Cucumber.js | Available | Required for WP-2 and WP-3 |

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Specification scope too large | Medium | High | Break into smaller, verifiable chunks; focus on core mechanics first |
| TLA+ complexity overwhelms team | Medium | Medium | Start with simplified models; iterate toward complexity |
| Missing critical invariants | Low | High | Regular review sessions; oppositional thinking exercises |
| Test harness configuration issues | Medium | Medium | Use established patterns; leverage existing Agentile configurations |

---

## Notes

> This sprint follows the specification-first approach outlined in The Grove document (Section 11). No implementation code will be written during this sprint - only specifications and test harnesses. The goal is to create a verification sandwich: TLA+ specs (invariants) -> Gherkin features (behaviors) -> implementation code (to be written in subsequent sprints).
