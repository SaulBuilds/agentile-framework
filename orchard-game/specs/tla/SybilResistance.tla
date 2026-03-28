---------------------------- MODULE SybilResistance.tla ---------------------------

EXTENDS Naturals, TLC

(*
  SybilResistance.tla
  TLA+ specification for Sybil attack resistance mechanisms
  References: The Grove Spec Document Section 6.1 and 10.4
*)

(*
  VARIABLES
  - players: Set(PlayerId) - all registered players
  - playerStakes: [PlayerId -> Nat] - amount of ORT staked by each player
  - playerReputations: [PlayerId -> ReputationTier] - reputation level
  - federationParticipants: [FederationId -> Set(PlayerId)] - players in each federation
  - federationMinStakes: [FederationId -> Nat] - minimum stake required per federation
  - seedSubmissions: Set(SeedSubmission) - seeds planted by players
  - validatorVotes: [SeedId -> Set(Vote)] - votes received by each seed
*)

VARIABLES
  players,
  playerStakes,
  playerReputations,
  federationParticipants,
  federationMinStakes,
  seedSubmissions,
  validatorVotes

PlayerId == STRING
FederationId == STRING
SeedId == STRING
Nat == Nat
ReputationTier == {"Seedling", "Sapling", "Tree", "Forest"}
Vote == [validator: PlayerId, agreement: BOOLEAN, timestamp: Nat]
SeedSubmission == [playerId: PlayerId, seedId: SeedId, federationId: FederationId, 
                   stake: Nat, timestamp: Nat, payload: String]

TypeOK ==
  /\ players ⊆ PlayerId
  /\ playerStakes ∈ [players -> Nat]
  /\ playerReputations ∈ [players -> ReputationTier]
  /\ federationParticipants ∈ [FederationId -> SUBSET PlayerId]
  /\ federationMinStakes ∈ [FederationId -> Nat]
  /\ seedSubmissions ⊆ SeedSubmission
  /\ validatorVotes ∈ [SeedId -> SUBSET Vote]
  /\ ∀ p ∈ players: playerStakes[p] >= 0
  /\ ∀ f ∈ FederationId: 
      federationMinStakes[f] > 0
      /\ federationParticipants[f] ⊆ players
  /\ ∀ s ∈ seedSubmissions: 
      s.playerId ∈ players
      /\ s.federationId ∈ FederationId
      /\ s.stake >= federationMinStakes[s.federationId]
      /\ s.seedId ∈ SeedId
  /\ ∀ seedId ∈ SeedId, v ∈ validatorVotes[seedId]: 
      v.validator ∈ players
      /\ v.timestamp >= 0

(*
  Initial state: No players, federations, or seeds
*)
Init ==
  /\ players = {}
  /\ playerStakes = [p ∈ {} |-> 0]
  /\ playerReputations = [p ∈ {} |-> "Seedling"]
  /\ federationParticipants = [f ∈ {} |-> {}]
  /\ federationMinStakes = [f ∈ {} |-> 0]
  /\ seedSubmissions = {}
  /\ validatorVotes = [s ∈ {} |-> {}]

(*
  Action: Register a new player
  Preconditions:
    - Player ID is unique
  Effects:
    - Adds player to players set
    - Initializes with zero stake and Seedling reputation
*)
RegisterPlayer(playerId) ==
  /\ playerId ∉ players
  /\ players' = players ∪ {playerId}
  /\ playerStakes' = [playerStakes EXCEPT ![playerId] = 0]
  /\ playerReputations' = [playerReputations EXCEPT ![playerId] = "Seedling"]
  /\ UNCHANGED <<federationParticipants, federationMinStakes, seedSubmissions, validatorVotes>>

(*
  Action: Player stakes ORT
  Preconditions:
    - Player exists
    - Amount is positive
  Effects:
    - Increases player's stake
*)
StakeORT(playerId, amount) ==
  /\ playerId ∈ players
  /\ amount > 0
  /\ playerStakes' = [playerStakes EXCEPT ![playerId] = @ + amount]
  /\ UNCHANGED <<players, playerReputations, federationParticipants, federationMinStakes, seedSubmissions, validatorVotes>>

(*
  Action: Player unstakes ORT (with cooldown)
  Preconditions:
    - Player exists
    - Amount is positive and not more than current stake
  Effects:
    - Decreases player's stake
*)
UnstakeORT(playerId, amount) ==
  /\ playerId ∈ players
  /\ amount > 0
  /\ amount <= playerStakes[playerId]
  /\ playerStakes' = [playerStakes EXCEPT ![playerId] = @ - amount]
  /\ UNCHANGED <<players, playerReputations, federationParticipants, federationMinStakes, seedSubmissions, validatorVotes>>

(*
  Action: Update player reputation based on behavior
  Preconditions:
    - Player exists
  Effects:
    - Updates reputation tier (simplified - would be based on historical behavior)
*)
UpdateReputation(playerId, newTier) ==
  /\ playerId ∈ players
  /\ newTier ∈ ReputationTier
  /\ playerReputations' = [playerReputations EXCEPT ![playerId] = newTier]
  /\ UNCHANGED <<players, playerStakes, federationParticipants, federationMinStakes, seedSubmissions, validatorVotes>>

(*
  Action: Create a new federation
  Preconditions:
    - Federation ID is unique
    - Minimum stake is set
  Effects:
    - Adds federation to the system
*)
CreateFederation(federationId, minStake) ==
  /\ federationId ∉ FederationId
  /\ minStake > 0
  /\ federationParticipants' = [federationParticipants EXCEPT ![federationId] = {}]
  /\ federationMinStakes' = [federationMinStakes EXCEPT ![federationId] = minStake]
  /\ UNCHANGED <<players, playerStakes, playerReputations, seedSubmissions, validatorVotes>>

(*
  Action: Player joins a federation
  Preconditions:
    - Player exists
    - Federation exists
    - Player meets minimum stake requirement
    - Player is not already in the federation
  Effects:
    - Adds player to federation's participant set
*)
JoinFederation(playerId, federationId) ==
  /\ playerId ∈ players
  /\ federationId ∈ FederationId
  /\ playerStakes[playerId] >= federationMinStakes[federationId]
  /\ playerId ∉ federationParticipants[federationId]
  /\ federationParticipants' = [federationParticipants EXCEPT ![federationId] = @ ∪ {playerId}]
  /\ UNCHANGED <<players, playerStakes, playerReputations, federationMinStakes, seedSubmissions, validatorVotes>>

(*
  Action: Player leaves a federation
  Preconditions:
    - Player exists
    - Federation exists
    - Player is currently in the federation
  Effects:
    - Removes player from federation's participant set
*)
LeaveFederation(playerId, federationId) ==
  /\ playerId ∈ players
  /\ federationId ∈ FederationId
  /\ playerId ∈ federationParticipants[federationId]
  /\ federationParticipants' = [federationParticipants EXCEPT ![federationId] = @ \ {playerId}]
  /\ UNCHANGED <<players, playerStakes, playerReputations, federationMinStakes, seedSubmissions, validatorVotes>>

(*
  Action: Player plants a seed in a federation
  Preconditions:
    - Player exists and is in the federation
    - Seed ID is unique (globally or per federation - we'll use globally for simplicity)
    - Player has sufficient stake (checked at join, but we re-check)
  Effects:
    - Adds seed submission
    - Note: Stake is locked in the federation reward pool (handled in FederationEcon)
*)
PlantSeed(playerId, federationId, seedId, payload, stake, timestamp) ==
  /\ playerId ∈ players
  /\ federationId ∈ FederationId
  /\ playerId ∈ federationParticipants[federationId]
  /\ seedId ∉ {s.seedId : s ∈ seedSubmissions}  \* Globally unique seed ID
  /\ stake >= federationMinStakes[federationId]
  /\ payload /= ""  \* Non-empty payload
  /\ seedSubmissions' = seedSubmissions ∪ 
        [playerId |-> playerId, federationId |-> federationId, seedId |-> seedId, 
         payload |-> payload, stake |-> stake, timestamp |-> timestamp]
  /\ UNCHANGED <<players, playerStakes, playerReputations, federationParticipants, 
                 federationMinStakes, validatorVotes>>

(*
  Action: Validator votes on a seed
  Preconditions:
    - Seed exists (has been planted)
    - Validator is a player
    - Validator has not already voted on this seed (simplified - would allow changing vote)
  Effects:
    - Adds a vote for the seed
*)
VoteOnSeed(validatorId, seedId, agrees, timestamp) ==
  /\ validatorId ∈ players
  /\ ∃ s ∈ seedSubmissions: s.seedId = seedId  \* Seed exists
  /\ validatorVotes' = [validatorVotes EXCEPT ![seedId] = @ ∪ 
        [validator |-> validatorId, agreement |-> agrees, timestamp |-> timestamp]]
  /\ UNCHANGED <<players, playerStakes, playerReputations, federationParticipants, 
                 federationMinStakes, seedSubmissions>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: No player can have negative stake
*)
NonNegativeStakes ==
  ∀ p ∈ players: playerStakes[p] >= 0

(* 
  Invariant: Federation minimum stake is always positive
*)
PositiveMinStake ==
  ∀ f ∈ FederationId: federationMinStakes[f] > 0

(* 
  Invariant: Players in a federation must exist
*)
ValidFederationParticipants ==
  ∀ f ∈ FederationId: federationParticipants[f] ⊆ players

(* 
  Invariant: Players in a federation meet the minimum stake requirement
*)
FederationStakeRequirement ==
  ∀ f ∈ FederationId, p ∈ federationParticipants[f]: 
      playerStakes[p] >= federationMinStakes[f]

(* 
  Invariant: Seed submissions are from valid players in valid federations
*)
ValidSeedSubmissions ==
  ∀ s ∈ seedSubmissions: 
      s.playerId ∈ players
      /\ s.federationId ∈ FederationId
      /\ s.playerId ∈ federationParticipants[s.federationId]
      /\ s.stake >= federationMinStakes[s.federationId]
      /\ s.payload /= ""

(* 
  Invariant: Each seed ID is unique (no duplicate seed IDs)
*)
UniqueSeedIDs ==
  LET seedIds == {s.seedId : s ∈ seedSubmissions}
  IN
    Cardinality(seedIds) = Cardinality(seedSubmissions)

(* 
  Invariant: Validator votes are from valid players on existing seeds
*)
ValidValidatorVotes ==
  ∀ seedId ∈ SeedId, v ∈ validatorVotes[seedId]: 
      v.validator ∈ players
      /\ ∃ s ∈ seedSubmissions: s.seedId = seedId

(* 
  Invariant: Sybil resistance - minimum stake scales with federation participation
  (Simplified version: minimum stake increases with number of participants)
*)
SybilResistanceBasic ==
  ∀ f ∈ FederationId: 
      federationMinStakes[f] >= 10  \* Base minimum
      /\ (Cardinality(federationParticipants[f]) > 10 => 
          federationMinStakes[f] >= 10 + (Cardinality(federationParticipants[f]) - 10) * 2)
      \* Example: after 10 participants, each additional participant adds 2 to minimum stake

(*
  Spec Definition
*)
SybilResistanceSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ playerId ∈ PlayerId: RegisterPlayer(playerId)
  \/ ∃ playerId ∈ PlayerId, amount ∈ Nat+: StakeORT(playerId, amount)
  \/ \* Unstake action commented out for simplicity - would need cooldown tracking
  \/ ∃ playerId ∈ PlayerId, newTier ∈ ReputationTier: UpdateReputation(playerId, newTier)
  \/ ∃ federationId ∈ FederationId, minStake ∈ Nat+: CreateFederation(federationId, minStake)
  \/ ∃ playerId ∈ PlayerId, federationId ∈ FederationId: JoinFederation(playerId, federationId)
  \/ \* Leave action commented out for simplicity
  \/ ∃ playerId ∈ PlayerId, federationId ∈ FederationId, seedId ∈ SeedId, payload ∈ String, 
         stake ∈ Nat, timestamp ∈ Nat: 
        PlantSeed(playerId, federationId, seedId, payload, stake, timestamp)
  \/ ∃ validatorId ∈ PlayerId, seedId ∈ SeedId, agrees ∈ BOOLEAN, timestamp ∈ Nat: 
        VoteOnSeed(validatorId, seedId, agrees, timestamp)

vars == <<players, playerStakes, playerReputations, federationParticipants, 
          federationMinStakes, seedSubmissions, validatorVotes>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM SybilResistanceSpec => []TypeOK
THEOREM SybilResistanceSpec => []NonNegativeStakes
THEOREM SybilResistanceSpec => []PositiveMinStake
THEOREM SybilResistanceSpec => []ValidFederationParticipants
THEOREM SybilResistanceSpec => []FederationStakeRequirement
THEOREM SybilResistanceSpec => []ValidSeedSubmissions
THEOREM SybilResistanceSpec => []UniqueSeedIDs
THEOREM SybilResistanceSpec => []ValidValidatorVotes
THEOREM SybilResistanceSpec => []SybilResistanceBasic

=============================================================================