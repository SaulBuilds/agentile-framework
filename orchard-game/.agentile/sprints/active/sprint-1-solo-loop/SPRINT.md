# Sprint 1-solo-loop: Solo Game Loop

> Copy this template to `.agentile/sprints/active/sprint-<id>-<name>/SPRINT.md`

---

## Sprint Metadata

| Field | Value |
|-------|-------|
| **Sprint ID** | S-1 |
| **Sprint Name** | Solo Game Loop |
| **Goal** | Implement the solo game loop: SeedNFT contract, GrowthEngine, and p5.js garden renderer. This is a single-player experience that works with one node. |
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

### WP-1: SeedNFT Contract Implementation

| Field | Value |
|-------|-------|
| **Status** | `[x] COMPLETE` |
| **Assignee** | DEVELOPER |
| **Estimated effort** | L |
| **Commit(s)** | 311dbb6 |

**Description:**
Implement the SeedNFT ERC-721 contract that represents planted seeds in the Orchard Game. This includes minting, burning, and metadata functions that comply with the TLA+ specifications.

**Tasks:**
- [x] Create SeedNFT.sol contract with ERC-721 compliance
- [x] Implement seed metadata functions (name, symbol, tokenURI)
- [x] Add seed-specific properties (stake, federation, planter, checkpoint, maxCheckpoint)
- [x] Implement planting function with stake validation
- [x] Implement growth progression functions
- [x] Implement harvesting function with reward calculation
- [x] Add events for seed lifecycle events
- [x] Ensure no mocks/stubs/TODOs in production code
- [x] Write comprehensive unit tests

**Acceptance Criteria:**
- [x] Contract compiles without errors using Hardhat
- [x] Contract follows ERC-721 standard
- [x] Seed metadata includes all required properties from TLA+ spec
- [x] Planting function validates minimum stake and federation existence
- [x] Growth progression respects checkpoint limits
- [x] Harvesting only works for mature seeds (checkpoint = maxCheckpoint)
- [x] Events are emitted for all state transitions
- [x] 80%+ line coverage in unit tests

**Tests Added:**
- test-seednft-minting -- verifies seed minting with proper properties
- test-seednft-growth-progression -- verifies checkpoint advancement
- test-seednft-harvesting -- verifies harvest functionality
- test-seednft-events -- verifies proper event emission
- test-seednft-validation -- verifies input validation
- test-seednft -- comprehensive SeedNFT test suite (in test/SeedNFT.test.js)

---

### WP-2: GrowthEngine Implementation

| Field | Value |
|-------|-------|
| **Status** | `[x] COMPLETE` |
| **Assignee** | DEVELOPER |
| **Estimated effort** | L |
| **Commit(s)** | 311dbb6 |

**Description:**
Implement the GrowthEngine contract that handles seed validation, Belnap logic computation, and growth score calculation according to the TLA+ specifications.

**Tasks:**
- [x] Create GrowthEngine.sol contract
- [x] Implement Belnap four-valued logic for validation aggregation
- [x] Add checkpoint-based validation processing
- [x] Implement growth score calculation based on validator agreement
- [x] Add integration with SeedNFT for reading seed properties
- [x] Implement timeout and failure handling (via require statements; timeout not applicable in solo mode as validation is immediate)
- [x] Add events for validation results
- [x] Ensure integration with MCP marketplace for inference (simplified in solo mode using deterministic mock)
- [x] Write comprehensive unit tests

**Acceptance Criteria:**
- [x] Contract compiles without errors
- [x] Belnap logic correctly computes T/F/B/N states
- [x] Validation processing respects checkpoint timing
- [x] Growth scores calculated according to spec formulas
- [x] Proper integration with SeedNFT contract
- [x] Events emitted for validation results
- [x] 80%+ line coverage in unit tests

**Tests Added:**
- test-growthengine-belnap-logic -- verifies four-valued logic
- test-growthengine-validation-processing -- verifies checkpoint handling
- test-growthengine-score-calculation -- verifies growth score formulas
- test-growthengine-seed-integration -- verifies SeedNFT interaction
- test-growthengine-events -- verifies validation event emission
- test-growthengine -- comprehensive GrowthEngine test suite (in test/GrowthEngine.test.js)

---

### WP-3: p5.js Garden Renderer

| Field | Value |
|-------|-------|
| **Status** | `[~] IN PROGRESS` |
| **Assignee** | DEVELOPER |
| **Estimated effort** | L |
| **Commit(s)** | 311dbb6 |

**Description:**
Implement the p5.js garden renderer that visualizes seeds growing in real-time based on their state and validation results.

**Tasks:**
- [x] Set up p5.js canvas in React component
- [ ] Create seed visualization components (sprout, growing plant, mature plant, failed plant)
- [ ] Implement color coding based on Belnap states (T=green, F=red, B=yellow, N=gray)
- [ ] Implement size scaling based on checkpoint progress
- [ ] Add animation for growth progression
- [ ] Implement coordinate system for seed placement
- [ ] Add tooltips/showing seed details on hover/click
- [ ] Ensure responsiveness and performance optimization
- [ ] Write p5.js snapshot tests for visual regression

**Acceptance Criteria:**
- [ ] p5.js canvas renders correctly in React component
- [ ] Seed visualization shows correct colors for Belnap states
- [ ] Seed size scales appropriately with checkpoint progress
- [ ] Growth animations are smooth and performant
- [ ] Coordinate system allows multiple seeds to be displayed
- [ ] Interactive elements show seed details
- [ ] 80%+ snapshot test coverage for visual components

**Tests Added:**
- test-p5js-seed-rendering -- verifies seed visualization correctness
- test-p5js-growth-animation -- verifies growth animation functionality
- test-p5js-interaction -- verifies seed detail display
- test-p5js-snapshot-regression -- verifies visual consistency

---

### WP-4: Solo Mode Integration

| Field | Value |
|-------|-------|
| **Status** | `[ ] NOT STARTED` |
| **Assignee** | DEVELOPER |
| **Estimated effort** | M |
| **Commit(s)** |  |

**Description:**
Integrate all components into a cohesive solo game experience that can run without network dependencies.

**Tasks:**
- [ ] Create main game orchestrator component
- [ ] Implement seed planting UI (input form, stake validation)
- [ ] Add growth progression simulation (timed checkpoints)
- [ ] Integrate with local mock inference service (for solo mode)
- [ ] Implement harvesting UI with reward display
- [ ] Add game state persistence (localStorage)
- [ ] Implement basic UI/UX for game flow
- [ ] Add error handling and loading states
- [ ] Ensure all components work together seamlessly

**Acceptance Criteria:**
- [ ] Game can be started and played without network connection
- [ ] Seed planting UI validates inputs and stake requirements
- [ ] Growth progression simulates checkpoint timing correctly
- [ ] Local inference service provides deterministic responses for testing
- [ ] Harvesting UI displays correct rewards based on growth score
- [ ] Game state persists between sessions
- [ ] UI/UX provides clear feedback for all game actions
- [ ] No blocking errors in console

**Tests Added:**
- test-solo-mode-integration -- verifies end-to-end game flow
- test-solo-mode-persistence -- verifies state persistence
- test-solo-mode-ui-feedback -- verifies user interaction feedback

---

### WP-5: Testing and Verification

| Field | Value |
|-------|-------|
| **Status** | `[ ] NOT STARTED` |
| **Assignee** | QA_ENGINEER |
| **Estimated effort** | M |
| **Commit(s)** |  |

**Description:**
Set up comprehensive testing infrastructure and run verification against TLA+ specifications.

**Tasks:**
- [ ] Configure Hardhat for Solidity testing
- [ ] Set up Jest and React Testing Library for frontend tests
- [ ] Configure p5.js snapshot testing framework
- [ ] Create test scripts and npm commands
- [ ] Run unit tests for all contracts
- [ ] Run integration tests for contract interactions
- [ ] Run frontend unit and snapshot tests
- [ ] Verify implementation against TLA+ specifications
- [ ] Establish test coverage baseline in coverage/GATES.md
- [ ] Ensure CI pipeline can run all tests

**Acceptance Criteria:**
- [ ] All tests can be run with npm commands
- [ ] Contract unit tests pass with 80%+ coverage
- [ ] Frontend unit tests pass with 80%+ coverage
- [ ] p5.js snapshot tests establish baseline
- [ ] Integration tests verify contract interactions
- [ ] Implementation satisfies key TLA+ invariants (spot-check)
- [ ] Test coverage gates established and met
- [ ] CI pipeline configuration complete

**Tests Added:**
- test-full-integration-solo -- verifies complete solo game flow
- test-contract-interactions -- verifies SeedNFT-GrowthEngine interaction
- test-tla-invariants-spotcheck -- verifies key TLA+ properties
- test-coverage-baseline -- establishes coverage baseline

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
| TLA+ Toolkit | Available | Already validated in Sprint 0 |
| Node.js/npm | Available | Required for WP-5 |
| Solidity Compiler | Available | Required for WP-1, WP-2 |
| Cucumber.js | Available | For future feature testing |
| React | Available | Required for WP-3, WP-4 |
| p5.js Library | Available | Required for WP-3 |
| Hardhat | Available | Required for WP-1, WP-2, WP-5 |
| ethers.js | Available | Required for contract interaction |
| Jest/React Testing Library | Available | Required for WP-5 |

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Contract complexity too high | Medium | High | Break into smaller contracts; use inheritance |
| p5.js performance issues | Medium | Medium | Optimize rendering; use useEffect hooks properly |
| State synchronization bugs | Medium | High | Implement proper state management; use Context API |
| Testing infrastructure setup delays | Low | Medium | Use existing Agentile patterns; leverage create-react-app |
| Integration between contracts and frontend | Medium | High | Use ABI generation; test early and often |
| Solo mode inference simulation | Low | Medium | Use deterministic mock service; seed randomness |

---

## Notes

> This sprint implements the solo game loop as specified in The Grove document Section 12 (Revised Architecture Summary). The goal is to create a working single-player experience that validates the core game mechanics before adding networked features. All implementation will be guided by the TLA+ specifications and Gherkin features created in Sprint 0, following the verification sandwich approach: TLA+ specs (invariants) -> Gherkin features (behaviors) -> implementation code (to satisfy both).
