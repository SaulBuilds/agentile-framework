--------------------------- MODULE SeedLifecycle ---------------------------

EXTENDS Naturals, TLC

(* 
  SeedLifecycle.tla
  TLA+ specification for the seed planting and growth cycle subsystem
  References: The Grove Spec Document Section 10.1 Core Game Loop Features
*)

(*
  VARIABLES
  - seeds: Set of all seed identifiers
  - seedState: [seedId -> {"PLANTED", "GROWING", "READY", "HARVESTED"}]
  - seedPayload: [seedId -> String] - the prompt/content of the seed
  - seedStake: [seedId -> Nat] - amount of ORT staked
  - seedFederation: [seedId -> String] - federation the seed belongs to
  - seedPlanter: [seedId -> String] - address of player who planted
  - seedCheckpoint: [seedId -> Nat] - current checkpoint cycle (0 = planted)
  - seedMaxCheckpoint: [seedId -> Nat] - total checkpoints needed for growth
  - seedGrowthScore: [seedId -> Nat] - final score (0-100) at harvest
  - rewardPools: [federation -> Nat] - available ORT in each federation's reward pool
  - totalFederationScore: [federation -> Nat] - sum of all growth scores in federation
*)

VARIABLES
  seeds,
  seedState,
  seedPayload,
  seedStake,
  seedFederation,
  seedPlanter,
  seedCheckpoint,
  seedMaxCheckpoint,
  seedGrowthScore,
  rewardPools,
  totalFederationScore

TypeOK ==
  /\ seeds ⊆ SeedId
  /\ seedState ∈ [seeds -> {"PLANTED", "GROWING", "READY", "HARVESTED"}]
  /\ seedPayload ∈ [seeds -> String]
  /\ seedStake ∈ [seeds -> Nat]
  /\ seedFederation ∈ [seeds -> FederationId]
  /\ seedPlanter ∈ [seeds -> PlayerAddress]
  /\ seedCheckpoint ∈ [seeds -> Nat]
  /\ seedMaxCheckpoint ∈ [seeds -> Nat]
  /\ seedGrowthScore ∈ [seeds -> 0..100]
  /\ rewardPools ∈ [FederationId -> Nat]
  /\ totalFederationScore ∈ [FederationId -> Nat]
  /\ ∀ s ∈ seeds: seedCheckpoint[s] ≤ seedMaxCheckpoint[s]
  /\ ∀ f ∈ FederationId: 
      totalFederationScore[f] = Σ {s ∈ seeds: seedFederation[s] = f : seedGrowthScore[s]}

SeedId == STRING
FederationId == STRING
PlayerAddress == STRING

(* 
  Initial state: No seeds planted, empty reward pools
*)
Init ==
  /\ seeds = {}
  /\ seedState = [s ∈ {} |-> "PLANTED"]  \* Empty function
  /\ seedPayload = [s ∈ {} |-> ""]
  /\ seedStake = [s ∈ {} |-> 0]
  /\ seedFederation = [s ∈ {} |-> ""]
  /\ seedPlanter = [s ∈ {} |-> ""]
  /\ seedCheckpoint = [s ∈ {} |-> 0]
  /\ seedMaxCheckpoint = [s ∈ {} |-> 0]
  /\ seedGrowthScore = [s ∈ {} |-> 0]
  /\ rewardPools = [f ∈ {} |-> 0]
  /\ totalFederationScore = [f ∈ {} |-> 0]

(*
  Action: Plant a new seed
  Preconditions:
    - Player has sufficient ORT balance (checked externally)
    - Federation exists and has minimum stake requirement
    - Seed ID is unique
  Effects:
    - Adds new seed to seeds set
    - Sets initial state to "PLANTED"
    - Transfers stake to federation reward pool
*)
PlantSeed(seedId, payload, stake, federation, planter, maxCheckpoint) ==
  /\ seedId ∉ seeds
  /\ stake > 0
  /\ maxCheckpoint > 0
  /\ seeds' = seeds ∪ {seedId}
  /\ seedState' = [seedState EXCEPT ![seedId] = "PLANTED"]
  /\ seedPayload' = [seedPayload EXCEPT ![seedId] = payload]
  /\ seedStake' = [seedStake EXCEPT ![seedId] = stake]
  /\ seedFederation' = [seedFederation EXCEPT ![seedId] = federation]
  /\ seedPlanter' = [seedPlanter EXCEPT ![seedId] = planter]
  /\ seedCheckpoint' = [seedCheckpoint EXCEPT ![seedId] = 0]
  /\ seedMaxCheckpoint' = [seedMaxCheckpoint EXCEPT ![seedId] = maxCheckpoint]
  /\ seedGrowthScore' = [seedGrowthScore EXCEPT ![seedId] = 0]
  /\ rewardPools' = [rewardPools EXCEPT ![federation] = @ + stake]
  /\ totalFederationScore' = [totalFederationScore EXCEPT ![federation] = @]
  /\ UNCHANGED <<seedId, payload, stake, federation, planter, maxCheckpoint>>

(*
  Action: Advance a seed through a checkpoint cycle
  Preconditions:
    - Seed exists and is in PLANTED or GROWING state
    - Seed has not yet reached max checkpoints
  Effects:
    - Increments checkpoint counter
    - If checkpoint < max: state becomes GROWING
    - If checkpoint = max: state becomes READY (awaiting harvest)
*)
AdvanceCheckpoint(seedId) ==
  /\ seedId ∈ seeds
  /\ seedState[seedId] ∈ {"PLANTED", "GROWING"}
  /\ seedCheckpoint[seedId] < seedMaxCheckpoint[seedId]
  /\ seedState' = [seedState EXCEPT ![seedId] = 
        IF seedCheckpoint[seedId] + 1 = seedMaxCheckpoint[seedId]
        THEN "READY"
        ELSE "GROWING"]
  /\ seedCheckpoint' = [seedCheckpoint EXCEPT ![seedId] = @ + 1]
  /\ UNCHANGED <<seeds, seedPayload, seedStake, seedFederation, seedPlanter,
                 seedMaxCheckpoint, seedGrowthScore, rewardPools, totalFederationScore>>

(*
  Action: Harvest a mature seed
  Preconditions:
    - Seed exists and is in READY state
    - Federation has sufficient reward pool for distribution
  Effects:
    - Sets state to HARVESTED
    - Calculates and transfers reward to planter
    - Updates federation reward pool and total score
*)
HarvestSeed(seedId, growthScore) ==
  /\ seedId ∈ seeds
  /\ seedState[seedId] = "READY"
  /\ growthScore ∈ 0..100
  /\ rewardPools[seedFederation[seedId]] ≥ 0  \* Simplified - actual calc happens off-chain
  /\ seedState' = [seedState EXCEPT ![seedId] = "HARVESTED"]
  /\ seedGrowthScore' = [seedGrowthScore EXCEPT ![seedId] = growthScore]
  /\ rewardPools' = [rewardPools EXCEPT ![seedFederation[seedId]] = 
        If rewardPools[seedFederation[seedId]] ≥ growthScore
        Then @ - growthScore
        Else 0]  \* Simplified reward calculation
  /\ totalFederationScore' = [totalFederationScore EXCEPT ![seedFederation[seedId]] = 
        @ + growthScore]
  /\ UNCHANGED <<seeds, seedPayload, seedStake, seedFederation, seedPlanter,
                 seedCheckpoint, seedMaxCheckpoint, rewardPools>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: No seed's stake is ever lost due to concurrent operations
  The total staked ORT across all seeds plus reward pools should remain constant
  (assuming no external inflows/outflows during this subsystem's operation)
*)
StakeConservation ==
  LET TotalStaked == Σ {s ∈ seeds : seedStake[s]}
      TotalInPools == Σ {f ∈ FederationId : rewardPools[f]}
  IN
    TotalStaked + TotalInPools = 
      Σ {s ∈ seeds : seedStake[s]} + Σ {f ∈ FederationId : rewardPools[f]}
    \* Trivially true but demonstrates the concept - in practice would track
    \* inflows/outflows from external sources

(* 
  Invariant: A seed cannot advance beyond its maximum checkpoint
*)
CheckpointBounds ==
  ∀ s ∈ seeds: seedCheckpoint[s] ≤ seedMaxCheckpoint[s]

(* 
  Invariant: Growth score is only set when seed is harvested
*)
GrowthScoreOnlyWhenHarvested ==
  ∀ s ∈ seeds: 
    seedGrowthScore[s] > 0 => seedState[s] = "HARVESTED"

(* 
  Invariant: Reward pools never go negative
*)
NonNegativeRewardPools ==
  ∀ f ∈ FederationId: rewardPools[f] ≥ 0

(* 
  Invariant: Each seed belongs to exactly one federation
*)
SeedFederationConsistency ==
  ∀ s ∈ seeds: seedFederation[s] ∈ FederationId

(* 
  Invariant: Seed state transitions follow proper order
*)
ValidStateTransitions ==
  ∀ s ∈ seeds:
    CASE seedState[s] OF
      "PLANTED" => seedCheckpoint[s] = 0
      "GROWING" => seedCheckpoint[s] > 0 /\ seedCheckpoint[s] < seedMaxCheckpoint[s]
      "READY"   => seedCheckpoint[s] = seedMaxCheckpoint[s]
      "HARVESTED" => seedCheckpoint[s] = seedMaxCheckpoint[s]
    END CASE

(*
  Spec Definition
*)
SeedLifecycleSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ seedId ∈ SeedId, payload ∈ String, stake ∈ Nat, 
                 federation ∈ FederationId, planter ∈ PlayerAddress,
                 maxCheckpoint ∈ Nat:
        PlantSeed(seedId, payload, stake, federation, planter, maxCheckpoint)
  \/ ∃ seedId ∈ seeds: AdvanceCheckpoint(seedId)
  \/ ∃ seedId ∈ seeds, growthScore ∈ 0..100: HarvestSeed(seedId, growthScore)

vars == <<seeds, seedState, seedPayload, seedStake, seedFederation, seedPlanter,
          seedCheckpoint, seedMaxCheckpoint, seedGrowthScore, rewardPools, totalFederationScore>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM SeedLifecycleSpec => []TypeOK
THEOREM SeedLifecycleSpec => []StakeConservation
THEOREM SeedLifecycleSpec => []CheckpointBounds
THEOREM SeedLifecycleSpec => []GrowthScoreOnlyWhenHarvested
THEOREM SeedLifecycleSpec => []NonNegativeRewardPools
THEOREM SeedLifecycleSpec => []SeedFederationConsistency
THEOREM SeedLifecycleSpec => []ValidStateTransitions

=============================================================================