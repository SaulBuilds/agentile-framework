# Formal Verification Rules

> Formal methods reduce the risk of catastrophic bugs in consensus, state machines, and financial logic. This document defines when and how to apply them.

---

## Requirement Tiers

| Change Category | TLA+ Spec | Invariant Tests | Requirement Level |
|----------------|-----------|-----------------|-------------------|
| Consensus algorithm changes | Required | Required | **MUST** |
| Finality mechanism changes | Required | Required | **MUST** |
| State machine transitions | Recommended | Required | **SHOULD** |
| Smart contract logic | Not applicable | Required | **SHOULD** |
| Cryptographic primitive changes | Recommended | Required | **SHOULD** |
| Token economics changes | Recommended | Recommended | **MAY** |
| API endpoint changes | Not applicable | Recommended | **MAY** |

### Definitions

- **MUST**: The change cannot merge without the specified verification. **BLOCKER.**
- **SHOULD**: The change should have the specified verification. Exceptions require documented justification in the PR body.
- **MAY**: Verification is encouraged but not required.

---

## TLA+ Specifications

### When to Write a TLA+ Spec

Write a TLA+ spec when:
1. The change modifies the consensus algorithm.
2. The change modifies the finality mechanism.
3. The change introduces a new state machine.
4. The change modifies proposer election or leader selection.

### Spec Location

```
.agentile/formal/
├── README.md              # Index of all specs with status
├── specs/
│   ├── spec_1.tla         # First protocol spec
│   ├── spec_2.tla         # Second protocol spec
│   └── ...
└── models/
    ├── spec_1.cfg         # TLC model checker config
    └── ...
```

### Spec Requirements

Every TLA+ spec must include:

1. **Module declaration** with the specification name matching the file name.
2. **Constants** section defining parameters.
3. **Variables** section defining the state space.
4. **Init** predicate defining the initial state.
5. **Next** predicate defining all possible state transitions.
6. **Invariants** -- properties that must hold in every reachable state.
7. **Liveness properties** (where applicable) -- properties that must eventually hold.

```tla
---- MODULE ProtocolName ----
EXTENDS Integers, Sequences, FiniteSets

CONSTANTS Param1, Param2, Param3
VARIABLES state1, state2, state3

vars == <<state1, state2, state3>>

Init ==
    /\ state1 = InitialValue
    /\ state2 = InitialValue
    /\ state3 = InitialValue

\* ... state transitions ...

SafetyInvariant ==
    \* Property that must hold in every reachable state

LivenessProperty ==
    \* Property that must eventually hold

====
```

### Model Checking

**GATE:** Every TLA+ spec must be model-checked with TLC before the corresponding code change can merge.

Verification steps:
1. Define a model configuration (`.cfg` file) with finite bounds for constants.
2. Run TLC: `java -jar tla2tools.jar -config <spec>.cfg <spec>.tla`
3. TLC must report: "Model checking completed. No error has been found."
4. Record the TLC output (states explored, runtime) in the PR body.

If TLC finds a violation:
- The invariant violation trace becomes a test case.
- The code must be fixed before merge.
- **BLOCKER.** DO NOT PROCEED with code that violates its own specification.

---

## Invariant Tests (Code-Level)

### When to Write Invariant Tests

Invariant tests are property-based tests that verify system-wide invariants hold across many random inputs. They complement unit tests.

Write invariant tests when:
1. A function operates on collections or ranges of inputs.
2. A state machine has transitions that must preserve invariants.
3. Financial calculations must satisfy conservation laws.
4. Cryptographic operations must satisfy algebraic properties.

---

## Process

### For Consensus Changes

```
1. Draft TLA+ spec (or update existing)
2. Run TLC model checker
3. If violations found -> fix spec or algorithm -> re-run TLC
4. TLC passes -> write invariant tests
5. Invariant tests pass -> implement production code (TDD cycle)
6. All tests pass -> PR with TLC output in body
```

**BLOCKER:** Consensus PRs without TLC output will not be merged.

### For State Machine Changes

```
1. Write property-based invariant tests
2. Implement production code (TDD cycle)
3. Optional: write TLA+ spec for complex state machines
4. All tests pass -> PR
```

### For Smart Contract Changes

```
1. Write invariant tests
2. Write unit tests (TDD cycle)
3. All tests pass -> PR
```

---

## Maintaining Specs

- TLA+ specs are living documents. When the algorithm changes, the spec must be updated in the same PR.
- The `formal/SPEC_INDEX.md` must list all specs with their current status (draft, verified, outdated).
- Outdated specs must be marked as such and updated or archived.

**GATE:** A TLA+ spec marked as "verified" that no longer matches the implementation is a documentation bug. Update it before the next release.
