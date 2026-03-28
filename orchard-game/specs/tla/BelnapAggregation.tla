---------------------------- MODULE BelnapAggregation ---------------------------

EXTENDS Naturals, TLC

(*
  BelnapAggregation.tla
  TLA+ specification for Belnap logic-based validation and scoring
  References: The Grove Spec Document Section 3 (Critique) and 10.1
*)

(*
  VARIABLES
  - validations: Set of validation records for seed assessments
  - validationMap: [seedId -> Set(Validation)] - validations per seed
  - belnapState: [seedId -> {"T", "F", "B", "N"}] - Belnap classification per seed
  - validatorStake: [validatorAddress -> Nat] - stake weight of each validator
  - validationThreshold: Nat - minimum agreement for T/F classification
  - seedCheckpoints: [seedId -> Nat] - current checkpoint of seed (for timing)
*)

VARIABLES
  validations,
  validationMap,
  belnapState,
  validatorStake,
  validationThreshold,
  seedCheckpoints

TypeOK ==
  /\ validations ⊆ Validation
  /\ validationMap ∈ [SeedId -> SUBSET Validation]
  /\ belnapState ∈ [SeedId -> {"T", "F", "B", "N"}]
  /\ validatorStake ∈ [PlayerAddress -> Nat]
  /\ validationThreshold ∈ Nat
  /\ seedCheckpoints ∈ [SeedId -> Nat]
  /\ ∀ v ∈ validations: 
      v.seedId ∈ SeedId
      /\ v.validator ∈ PlayerAddress
      /\ v.response ∈ String
      /\ v.checkpoint ∈ Nat
      /\ v.timestamp ∈ TIME  \* Simplified as Nat for now
  /\ validationThreshold > 0

Validation == [seedId: SeedId, validator: PlayerAddress, response: String, 
               checkpoint: Nat, timestamp: Nat]
SeedId == STRING
PlayerAddress == STRING
Nat == Nat
TIME == Nat  \* Simplified time as natural numbers

(*
  Initial state: No validations, empty Belnap states
*)
Init ==
  /\ validations = {}
  /\ validationMap = [s ∈ SeedId |-> {}]
  /\ belnapState = [s ∈ SeedId |-> "N"]  \* Start with No information
  /\ validatorStake = [a ∈ PlayerAddress |-> 0]
  /\ validationThreshold = 2  \* Require at least 2 agreeing validators for T/F
  /\ seedCheckpoints = [s ∈ SeedId |-> 0]

(*
  Action: Submit a validation for a seed at a checkpoint
  Preconditions:
    - Seed exists
    - Validator is known (has stake)
    - Validation is for current checkpoint of seed
  Effects:
    - Adds validation to validations set
    - Adds validation to seed's validation map
*)
SubmitValidation(seedId, validator, response, checkpoint, timestamp) ==
  /\ seedId ∈ SeedId
  /\ validator ∈ PlayerAddress
  /\ validatorStake[validator] > 0  \* Validator must have stake
  /\ checkpoint = seedCheckpoints[seedId]  \* Must validate at current checkpoint
  /\ timestamp >= 0
  /\ validations' = validations ∪ 
        {[seedId |-> seedId, validator |-> validator, response |-> response, 
          checkpoint |-> checkpoint, timestamp |-> timestamp]}
  /\ validationMap' = [validationMap EXCEPT ![seedId] = 
        @ ∪ {[seedId |-> seedId, validator |-> validator, response |-> response, 
             checkpoint |-> checkpoint, timestamp |-> timestamp]}]
  /\ UNCHANGED <<belnapState, validatorStake, validationThreshold, seedCheckpoints>>

(*
  Action: Update Belnap state for a seed based on current validations
  Preconditions:
    - Seed exists
    - At least one validation exists for current checkpoint (or we use all)
  Effects:
    - Computes Belnap state based on agreement of responses
    - T: All valid responses agree (above threshold)
    - F: All valid responses disagree (above threshold) - simplified
    - B: Some agreement, some disagreement (above threshold)
    - N: Not enough validations (below threshold)
*)
UpdateBelnapState(seedId) ==
  /\ seedId ∈ SeedId
  /\ LET 
        validationsForSeed == validationMap[seedId]
        responsesForSeed == {v.response : v ∈ validationsForSeed}
        agreementCount == 
          LET mostCommonResponse == 
                IF responsesForSeed = {} THEN "" 
                ELSE CHOOSE r ∈ responsesForSeed: 
                     |{v ∈ validationsForSeed : v.response = r}| 
                     >= |{v ∈ validationsForSeed : v.response = s}| 
                     /\ s ∈ responsesForSeed
            IN
              |{v ∈ validationsForSeed : v.response = mostCommonResponse}|
        totalValidations == Cardinality(validationsForSeed)
  IN
    IF totalValidations < validationThreshold
    THEN belnapState' = [belnapState EXCEPT ![seedId] = "N"]
    ELSE IF agreementCount * 2 >= totalValidations  \* Simple majority agreement
         THEN belnapState' = [belnapState EXCEPT ![seedId] = "T"]
         ELSE IF agreementCount = 0  \* All responses different (simplified)
              THEN belnapState' = [belnapState EXCEPT ![seedId] = "F"]
              ELSE belnapState' = [belnapState EXCEPT ![seedId] = "B"]
    END IF
  /\ UNCHANGED <<validations, validationMap, validatorStake, validationThreshold, seedCheckpoints>>

(*
  Action: Advance seed checkpoint (triggers Belnap update at new checkpoint)
  Preconditions:
    - Seed exists
  Effects:
    - Increments checkpoint
    - Triggers Belnap state update for the new checkpoint
*)
AdvanceCheckpointWithBelnap(seedId) ==
  /\ seedId ∈ SeedId
  /\ seedCheckpoints' = [seedCheckpoints EXCEPT ![seedId] = @ + 1]
  /\ belnapState' = [belnapState EXCEPT ![seedId] = 
        IF validationMap[seedId] = {} THEN "N"  \* No validations yet
        ELSE LET 
              validationsAtCheckpoint == 
                {v ∈ validationMap[seedId] : v.checkpoint = @ + 1}
              IN
                IF Cardinality(validationsAtCheckpoint) < validationThreshold
                THEN "N"
                ELSE LET
                      responses == {v.response : v ∈ validationsAtCheckpoint}
                      mostCommonResponse == 
                        CHOOSE r ∈ responses : 
                              |{v ∈ validationsAtCheckpoint : v.response = r}| 
                              >= |{v ∈ validationsAtCheckpoint : v.response = s}|
                              /\ s ∈ responses
                      agreementCount == |{v ∈ validationsAtCheckpoint : v.response = mostCommonResponse}|
                      total == Cardinality(validationsAtCheckpoint)
                    IN
                      IF agreementCount * 2 >= total THEN "T"
                      ELSE IF agreementCount = 0 THEN "F"
                      ELSE "B"
                END IF
        ]
  /\ UNCHANGED <<validations, validationMap, validatorStake, validationThreshold>>

(*
  Action: Set validator stake
*)
SetValidatorStake(validator, stakeAmount) ==
  /\ validator ∈ PlayerAddress
  /\ stakeAmount >= 0
  /\ validatorStake' = [validatorStake EXCEPT ![validator] = stakeAmount]
  /\ UNCHANGED <<validations, validationMap, belnapState, validationThreshold, seedCheckpoints>>

(*
  Action: Set validation threshold
*)
SetValidationThreshold(newThreshold) ==
  /\ newThreshold > 0
  /\ validationThreshold' = newThreshold
  /\ UNCHANGED <<validations, validationMap, belnapState, validatorStake, seedCheckpoints>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: Belnap state is always one of T, F, B, N
*)
ValidBelnapState ==
  ∀ s ∈ SeedId: belnapState[s] ∈ {"T", "F", "B", "N"}

(* 
  Invariant: Validation threshold is positive
*)
PositiveThreshold ==
  validationThreshold > 0

(* 
  Invariant: Validator stakes are non-negative
*)
NonNegativeValidatorStake ==
  ∀ v ∈ PlayerAddress: validatorStake[v] >= 0

(* 
  Invariant: Each validation is for a known seed
*)
ValidValidationSeedIds ==
  ∀ v ∈ validations: v.seedId ∈ SeedId

(* 
  Invariant: Each validation is by a known validator
*)
ValidValidationValidators ==
  ∀ v ∈ validations: v.validator ∈ PlayerAddress

(* 
  Invariant: Validation map contains exactly the validations for each seed
*)
ValidationMapConsistency ==
  ∀ s ∈ SeedId: validationMap[s] = {v ∈ validations : v.seedId = s}

(*
  Spec Definition
*)
BelnapAggregationSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ seedId ∈ SeedId, validator ∈ PlayerAddress, response ∈ String, 
                 checkpoint ∈ Nat, timestamp ∈ Nat:
        SubmitValidation(seedId, validator, response, checkpoint, timestamp)
  \/ ∃ seedId ∈ SeedId: UpdateBelnapState(seedId)
  \/ ∃ seedId ∈ SeedId: AdvanceCheckpointWithBelnap(seedId)
  \/ ∃ validator ∈ PlayerAddress, stakeAmount ∈ Nat: SetValidatorStake(validator, stakeAmount)
  \/ ∃ newThreshold ∈ Nat+: SetValidationThreshold(newThreshold)

vars == <<validations, validationMap, belnapState, validatorStake, 
          validationThreshold, seedCheckpoints>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM BelnapAggregationSpec => []TypeOK
THEOREM BelnapAggregationSpec => []ValidBelnapState
THEOREM BelnapAggregationSpec => []PositiveThreshold
THEOREM BelnapAggregationSpec => []NonNegativeValidatorStake
THEOREM BelnapAggregationSpec => []ValidValidationSeedIds
THEOREM BelnapAggregationSpec => []ValidValidationValidators
THEOREM BelnapAggregationSpec => []ValidationMapConsistency

=============================================================================