# Formal Verification Workflow

## When to Write a TLA+ Spec

| Change Type | Requirement | Gate |
|-------------|-------------|------|
| Consensus logic | **MUST** have spec | BLOCKER -- PR cannot merge |
| Proposer election / leader selection | **MUST** have spec | BLOCKER |
| Economic rules (rewards, slashing, governance) | **SHOULD** have spec | Reviewer discretion |
| State machine changes | **SHOULD** have spec | Reviewer discretion |
| Network protocol changes | **MAY** have spec | Optional |
| Data structure invariants | **MAY** have spec | Optional |

## Workflow

### Step 1: Identify Invariants

Before writing code, list the properties that MUST hold:
- Safety: "Bad thing X never happens"
- Liveness: "Good thing Y eventually happens"
- Ordering: "A always precedes B"

### Step 2: Write the Spec

Follow existing specs or use the FORMAL_VERIFIER zooid's template.

A spec must define:
- **Variables**: The state space
- **Init**: Initial state predicate
- **Next**: Transition relation
- **Invariants**: Safety properties to check
- **Config (.cfg)**: Model parameters and invariant list

### Step 3: Model Check

```bash
tlc YourSpec.tla -config YourSpec.cfg
```

The model checker must complete with:
- 0 invariant violations
- 0 deadlock states (unless deadlock is expected and documented)

### Step 4: Document Results

Add the spec to [SPEC_INDEX.md](SPEC_INDEX.md) with:
- Invariant count
- States explored
- Any interesting findings

### Step 5: Link to Code

Add a comment in the relevant source file referencing the spec:
```
// Formal spec: path/to/YourSpec.tla
// Invariants: InvariantName1, InvariantName2
```

## Gate Enforcement

**BLOCKER**: If a PR touches consensus or election logic and does not include a TLA+ spec (new or updated), the FORMAL_VERIFIER zooid must flag it and request spec work before merge.

**Exception**: Hotfixes for production incidents may merge without a spec, but a follow-up spec MUST be filed as a backlog item within 48 hours.
