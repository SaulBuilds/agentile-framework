# TLA+ Specifications for Orchard Game

This directory contains TLA+ specifications for the concurrent subsystems of the Orchard Game.

## Files

- `SeedLifecycle.tla` - Specification for seed planting and growth cycles
- `FederationEcon.tla` - Specification for federation economic subsystems
- `BelnapAggregation.tla` - Specification for Belnap logic-based validation
- `SchoolSafety.tla` - Specification for school safety features

## Validation

To validate these specifications, you need the TLA+ Toolkit which includes the TLC model checker.

### Installation

1. **Java Requirement**: TLA+ requires Java 8 or later
2. **Download TLA+ Toolkit**: 
   - Visit https://lamport.azurewebsites.net/tla/tla.html
   - Download the TLA+ Toolkit
   - Extract and add the `tla2tools.jar` to your PATH

### Running TLC

To check a specification, run:
```bash
tlc SeedLifecycle.tla
```

This will check all invariants defined in the specification's theorem statements.

### Expected Output

A successful run will show:
```
Model checking completed. No error has been found.
```

If invariants are violated, TLC will provide a counterexample trace.

## Specification Notes

These specifications follow the approach outlined in The Grove specification document:
- They define state spaces, transitions, and invariants for each subsystem
- They focus on critical properties that must always hold (safety properties)
- They are designed to be checked with TLC for bounded verification
- Each spec includes theorems that can be verified by TLC

## Relationship to Gherkin Features

Each TLA+ module has corresponding Gherkin feature files in `../features/`:
- SeedLifecycle.tla <-> seed-planting.feature, growth-cycle.feature, harvest.feature
- FederationEcon.tla <-> reward-distribution.feature, sybil-resistance.feature
- BelnapAggregation.tla <-> (validated through the above feature files)
- SchoolSafety.tla <-> school-content-safety.feature

The verification workflow is:
1. Write TLA+ specs (invariants that must always hold)
2. Write Gherkin features (observable behaviors in specific scenarios)
3. Implement code that satisfies both