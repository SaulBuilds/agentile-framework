# TLA+ Specification Index

> Index of all formal verification specifications in the project.

## Specifications

| Spec | Path | Invariants | Domain | Status |
|------|------|-----------|--------|--------|
| *(no specifications yet)* | -- | -- | -- | -- |

## Summary

| Metric | Value |
|--------|-------|
| Total specs | 0 |
| Total invariants | 0 |
| Total states explored | 0 |
| Violations found | 0 |

## When to Add a New Spec

Per [FORMAL_VERIFICATION_RULES.md](../rules/FORMAL_VERIFICATION_RULES.md):

- **MUST**: Any change to consensus logic, finality, or proposer election
- **SHOULD**: State machine changes, economic rule changes, new protocol flows
- **MAY**: Complex data structure invariants, UI state machines

## How to Run

```bash
# Install TLC (TLA+ model checker)
# Download from https://github.com/tlaplus/tlaplus/releases

# Run a spec
cd <spec-directory>
tlc SpecName.tla -config SpecName.cfg
```

## Adding a New Spec

1. Create the `.tla` file in the appropriate location
2. Create a `.cfg` file with model parameters and invariant list
3. Run TLC and verify 0 violations
4. Add a row to the table above
5. Update the summary metrics
