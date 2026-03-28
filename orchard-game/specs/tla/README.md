# TLA+ Specifications for Orchard Game

This directory contains TLA+ specifications for the concurrent subsystems of the Orchard Game.

## Files

- `SeedLifecycle.tla` - Basic specification for seed planting and growth cycles
- `SeedLifecycleEnhanced.tla` - **ENHANCED VERSION** with comprehensive modeling of billions of states, deep adversarial coverage, and rich integration points
- `FederationEcon.tla` - Specification for federation economic subsystems
- `BelnapAggregation.tla` - Specification for Belnap logic-based validation
- `SchoolSafety.tla` - Specification for school safety features
- `MentorPropagation.tla` - Specification for LoRA adapter propagation in mentor-mentee protocol
- `MysteryBoxRNG.tla` - Specification for Mystery Box random reward distribution
- `SybilResistance.tla` - Specification for Sybil attack resistance mechanisms
- `SeasonTransition.tla` - Specification for federation season lifecycle and transitions

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
tlc SeedLifecycleEnhanced.tla
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

The **Enhanced SeedLifecycle** specification goes significantly beyond the basic version by:
- Adding parametric modeling for billions of state combinations
- Incorporating deep adversarial modeling for specific attack vectors
- Adding rich integration points with other subsystems
- Including comprehensive validation history tracking
- Modeling mentor influence networks
- Tracking knowledge frontier progression
- Adding economic conservation laws
- Modeling temporal dynamics with seasons

## Relationship to Gherkin Features

Each TLA+ module has corresponding Gherkin feature files in `../features/`:
- SeedLifecycleEnhanced.tla <-> seed-planting.feature, growth-cycle.feature, harvest.feature
- FederationEcon.tla <-> reward-distribution.feature, sybil-resistance.feature
- BelnapAggregation.tla <-> (validated through the above feature files)
- SchoolSafety.tla <-> school-content-safety.feature
- MentorPropagation.tla <-> mentor-propagation.feature
- MysteryBoxRNG.tla <-> (validated through economic and visual features)
- SybilResistance.tla <-> sybil-resistance.feature
- SeasonTransition.tla <-> (validated through visual and economic features)

The verification workflow is:
1. Write TLA+ specs (invariants that must always hold)
2. Write Gherkin features (observable behaviors in specific scenarios)
3. Implement code that satisfies both