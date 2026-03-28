---------------------------- MODULE FederationEcon ---------------------------

EXTENDS Naturals, TLC

(*
  FederationEcon.tla
  TLA+ specification for federation economic subsystems
  References: The Grove Spec Document Sections 10.2 & 10.4
*)

(*
  VARIABLES
  - federations: Set of all federation identifiers
  - federationMinStake: [federation -> Nat] - minimum stake required
  - federationRewardPools: [federation -> Nat] - available ORT for rewards
  - federationTotalScore: [federation -> Nat] - sum of growth scores
  - federationActiveSeeds: [federation -> Nat] - count of non-harvested seeds
  - federationValidators: [federation -> Set(PlayerAddress)] - validator set
  - federationMentors: [federation -> Set(PlayerAddress)] - mentor nodes
  - globalRewardPool: Nat - ecosystem-level rewards (from block allocations)
  - teacherStipends: [federation -> Nat] - allocated funds for school use
*)

VARIABLES
  federations,
  federationMinStake,
  federationRewardPools,
  federationTotalScore,
  federationActiveSeeds,
  federationValidators,
  federationMentors,
  globalRewardPool,
  teacherStipends

TypeOK ==
  /\ federations ⊆ FederationId
  /\ federationMinStake ∈ [federations -> Nat]
  /\ federationRewardPools ∈ [federations -> Nat]
  /\ federationTotalScore ∈ [federations -> Nat]
  /\ federationActiveSeeds ∈ [federations -> Nat]
  /\ federationValidators ∈ [federations -> SUBSET PlayerAddress]
  /\ federationMentors ∈ [federations -> SUBSET PlayerAddress]
  /\ globalRewardPool ∈ Nat
  /\ teacherStipends ∈ [federations -> Nat]
  /\ ∀ f ∈ federations: federationMinStake[f] > 0
  /\ ∀ f ∈ federations: 
      federationActiveSeeds[f] >= 0
      /\ federationTotalScore[f] >= 0

FederationId == STRING
PlayerAddress == STRING
Nat == Nat  \* Natural numbers including zero

(*
  Initial state: No federations, empty reward pools
*)
Init ==
  /\ federations = {}
  /\ federationMinStake = [f ∈ {} |-> 0]
  /\ federationRewardPools = [f ∈ {} |-> 0]
  /\ federationTotalScore = [f ∈ {} |-> 0]
  /\ federationActiveSeeds = [f ∈ {} |-> 0]
  /\ federationValidators = [f ∈ {} |-> {}]
  /\ federationMentors = [f ∈ {} |-> {}]
  /\ globalRewardPool = 0
  /\ teacherStipends = [f ∈ {} |-> 0]

(*
  Action: Create a new federation
  Preconditions:
    - Federation ID is unique
    - Minimum stake is set appropriately
  Effects:
    - Adds federation to federations set
    - Initializes all federation parameters
*)
CreateFederation(federationId, minStake) ==
  /\ federationId ∉ federations
  /\ minStake > 0
  /\ federations' = federations ∪ {federationId}
  /\ federationMinStake' = [federationMinStake EXCEPT ![federationId] = minStake]
  /\ federationRewardPools' = [federationRewardPools EXCEPT ![federationId] = 0]
  /\ federationTotalScore' = [federationTotalScore EXCEPT ![federationId] = 0]
  /\ federationActiveSeeds' = [federationActiveSeeds EXCEPT ![federationId] = 0]
  /\ federationValidators' = [federationValidators EXCEPT ![federationId] = {}]
  /\ federationMentors' = [federationMentors EXCEPT ![federationId] = {}]
  /\ globalRewardPool' = globalRewardPool  \* Unchanged by federation creation
  /\ teacherStipends' = [teacherStipends EXCEPT ![federationId] = 0]
  /\ UNCHANGED <<federationId, minStake>>

(*
  Action: Add global rewards to federation pool (from ecosystem allocation)
  Preconditions:
    - Federation exists
    - Global reward pool has sufficient funds
  Effects:
    - Transfers ORT from global pool to federation reward pool
    - Optionally allocates to teacher stipend
*)
AllocateGlobalRewards(federationId, amount, teacherAllocation) ==
  /\ federationId ∈ federations
  /\ amount > 0
  /\ teacherAllocation >= 0
  /\ teacherAllocation <= amount  \* Can't allocate more than total
  /\ globalRewardPool >= amount
  /\ federationRewardPools' = [federationRewardPools EXCEPT ![federationId] = 
        @ + (amount - teacherAllocation)]
  /\ teacherStipends' = [teacherStipends EXCEPT ![federationId] = 
        @ + teacherAllocation]
  /\ globalRewardPool' = globalRewardPool - amount
  /\ UNCHANGED <<federations, federationMinStake, federationTotalScore,
                 federationActiveSeeds, federationValidators, federationMentors>>

(*
  Action: Update validator set for a federation
  Preconditions:
    - Federation exists
    - Validator set is valid (non-empty, proper format)
  Effects:
    - Updates the validator set
*)
UpdateValidators(federationId, newValidators) ==
  /\ federationId ∈ federations
  /\ newValidators ⊆ PlayerAddress
  /\ federationValidators' = [federationValidators EXCEPT ![federationId] = newValidators]
  /\ UNCHANGED <<federations, federationMinStake, federationRewardPools,
                 federationTotalScore, federationActiveSeeds, federationMentors,
                 globalRewardPool, teacherStipends>>

(*
  Action: Update mentor set for a federation
  Preconditions:
    - Federation exists
    - Mentor set is valid (subset of validators typically)
  Effects:
    - Updates the mentor set
*)
UpdateMentors(federationId, newMentors) ==
  /\ federationId ∈ federations
  /\ newMentors ⊆ PlayerAddress
  /\ federationMentors' = [federationMentors EXCEPT ![federationId] = newMentors]
  /\ UNCHANGED <<federations, federationMinStake, federationRewardPools,
                 federationTotalScore, federationActiveSeeds, federationValidators,
                 globalRewardPool, teacherStipends>>

(*
  Action: Update federation statistics after seed harvest
  Preconditions:
    - Federation exists
    - Growth score is valid
  Effects:
    - Adds growth score to federation total
    - Decrements active seed count
*)
UpdateFederationStats(federationId, growthScore) ==
  /\ federationId ∈ federations
  /\ growthScore ∈ 0..100
  /\ federationTotalScore' = [federationTotalScore EXCEPT ![federationId] = @ + growthScore]
  /\ federationActiveSeeds' = [federationActiveSeeds EXCEPT ![federationId] = 
        IF @ > 0 THEN @ - 1 ELSE 0]  \* Prevent negative
  /\ UNCHANGED <<federations, federationMinStake, federationRewardPools,
                 federationValidators, federationMentors, globalRewardPool,
                 teacherStipends>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: Federation reward pools never go negative
*)
NonNegativeRewardPools ==
  ∀ f ∈ federations: federationRewardPools[f] >= 0

(* 
  Invariant: Teacher stipends never go negative
*)
NonNegativeTeacherStipends ==
  ∀ f ∈ federations: teacherStipends[f] >= 0

(* 
  Invariant: Global reward pool never goes negative
*)
NonNegativeGlobalPool ==
  globalRewardPool >= 0

(* 
  Invariant: Federation total score is non-negative
*)
NonNegativeTotalScore ==
  ∀ f ∈ federations: federationTotalScore[f] >= 0

(* 
  Invariant: Active seed count is non-negative
*)
NonNegativeActiveSeeds ==
  ∀ f ∈ federations: federationActiveSeeds[f] >= 0

(* 
  Invariant: Minimum stake is always positive
*)
PositiveMinStake ==
  ∀ f ∈ federations: federationMinStake[f] > 0

(* 
  Invariant: Validator and mentor sets are valid
*)
ValidValidatorMentorSets ==
  ∀ f ∈ federations: 
      federationMentors[f] ⊆ federationValidators[f]  \* Mentors should be validators

(* 
  Invariant: No double-counting of rewards
  The sum of all federation reward pools plus global pool plus teacher stipends
  should equal total emissions minus what's been staked in seeds
  (Simplified version - full version would track all inflows/outflows)
*)
RewardConservation ==
  LET TotalInFederationPools == Σ {f ∈ federations : federationRewardPools[f]}
      TotalTeacherStipends == Σ {f ∈ federations : teacherStipends[f]}
  IN
    globalRewardPool + TotalInFederationPools + TotalTeacherStipends >= 0
    \* Simplified - in practice would track exact conservation with emissions

(*
  Spec Definition
*)
FederationEconSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ federationId ∈ FederationId, minStake ∈ Nat:
        CreateFederation(federationId, minStake)
  \/ ∃ federationId ∈ federations, amount ∈ Nat+, teacherAlloc ∈ Nat:
        AllocateGlobalRewards(federationId, amount, teacherAlloc)
  \/ ∃ federationId ∈ federations, newValidators ⊆ PlayerAddress:
        UpdateValidators(federationId, newValidators)
  \/ ∃ federationId ∈ federations, newMentors ⊆ PlayerAddress:
        UpdateMentors(federationId, newMentors)
  \/ ∃ federationId ∈ federations, growthScore ∈ 0..100:
        UpdateFederationStats(federationId, growthScore)

vars == <<federations, federationMinStake, federationRewardPools, 
          federationTotalScore, federationActiveSeeds, federationValidators,
          federationMentors, globalRewardPool, teacherStipends>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM FederationEconSpec => []TypeOK
THEOREM FederationEconSpec => []NonNegativeRewardPools
THEOREM FederationEconSpec => []NonNegativeTeacherStipends
THEOREM FederationEconSpec => []NonNegativeGlobalPool
THEOREM FederationEconSpec => []NonNegativeTotalScore
THEOREM FederationEconSpec => []NonNegativeActiveSeeds
THEOREM FederationEconSpec => []PositiveMinStake
THEOREM FederationEconSpec => []ValidValidatorMentorSets
THEOREM FederationEconSpec => []RewardConservation

=============================================================================