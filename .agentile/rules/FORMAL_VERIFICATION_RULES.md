# Formal Verification Rules

> Formal methods reduce the risk of catastrophic bugs in consensus, state machines, and critical business logic.

---

## Requirement Tiers

| Change Category | TLA+ Spec | Invariant Tests | Requirement Level |
|----------------|-----------|-----------------|-------------------|
| Consensus / ordering changes | Required | Required | **MUST** |
| State machine transitions | Recommended | Required | **SHOULD** |
| Financial / economic logic | Recommended | Recommended | **SHOULD** |
| Cryptographic primitive changes | Recommended | Required | **SHOULD** |
| API endpoint changes | Not applicable | Recommended | **MAY** |

### Definitions

- **MUST**: The change cannot merge without the specified verification. **BLOCKER.**
- **SHOULD**: Exceptions require documented justification in the PR body.
- **MAY**: Verification is encouraged but not required.

---

## TLA+ Specifications

### When to Write a TLA+ Spec

Write a TLA+ spec when:
1. The change modifies consensus or ordering logic.
2. The change modifies finality or commitment mechanisms.
3. The change introduces a new state machine.
4. The change modifies leader/proposer election.

### Spec Location

```
.agentile/formal/
├── VERIFICATION_WORKFLOW.md
├── specs/
│   ├── <component_a>.tla
│   └── <component_b>.tla
└── models/
    ├── <component_a>.cfg
    └── <component_b>.cfg
```

### Spec Requirements

Every TLA+ spec must include:

1. **Module declaration** matching the file name.
2. **Constants** section defining parameters.
3. **Variables** section defining the state space.
4. **Init** predicate defining the initial state.
5. **Next** predicate defining all possible state transitions.
6. **Invariants** -- properties that must hold in every reachable state.
7. **Liveness properties** (where applicable).

### Model Checking

**GATE:** Every TLA+ spec must be model-checked with TLC before the corresponding code change can merge.

Verification steps:
1. Define a model configuration (`.cfg` file) with finite bounds.
2. Run TLC.
3. TLC must report zero invariant violations.
4. Record the TLC output (states explored, runtime) in the PR body.

If TLC finds a violation:
- The invariant violation trace becomes a test case.
- The code must be fixed before merge.
- **BLOCKER.** DO NOT PROCEED with code that violates its own specification.

---

## Invariant Tests

### When to Write Invariant Tests

Write invariant tests (property-based tests) when:
1. A function operates on collections or ranges of inputs.
2. A state machine has transitions that must preserve invariants.
3. Financial calculations must satisfy conservation laws.
4. Cryptographic operations must satisfy algebraic properties.

### Process

**For critical logic changes:**
```
1. Draft TLA+ spec (or update existing)
2. Run TLC model checker
3. If violations found -> fix spec or algorithm -> re-run TLC
4. TLC passes -> write property-based tests
5. Tests pass -> implement production code (TDD cycle)
6. All tests pass -> PR with TLC output in body
```

**For state machine changes:**
```
1. Write property-based invariant tests
2. Implement production code (TDD cycle)
3. Optional: write TLA+ spec for complex state machines
4. All tests pass -> PR
```

---

## Maintaining Specs

- TLA+ specs are living documents. When the algorithm changes, the spec must be updated in the same PR.
- Outdated specs must be marked as such and updated or archived.

**GATE:** A TLA+ spec marked as "verified" that no longer matches the implementation is a documentation bug.
