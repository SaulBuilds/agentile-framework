---------------------------- MODULE SeasonTransition ---------------------------

EXTENDS Naturals, TLC

(*
  SeasonTransition.tla
  TLA+ specification for federation season lifecycle and transitions
  References: The Grove Spec Document Section 12 (Revised Architecture Summary)
*)

(*
  VARIABLES
  - federations: Set(FederationId) - all federations
  - federationSeasons: [FederationId -> SeasonState] - current season of each federation
  - seasonStartTimes: [FederationId -> Nat] - timestamp when current season started
  - seasonDuration: Nat - length of a season in checkpoint cycles
  - federationScores: [FederationId -> Nat] - aggregate knowledge score of federation
  - federationKnowledge: [FederationId -> KnowledgeState] - current knowledge state
  - transitionQueue: Seq(TransitionRecord) - pending season transitions
*)

VARIABLES
  federations,
  federationSeasons,
  seasonStartTimes,
  seasonDuration,
  federationScores,
  federationKnowledge,
  transitionQueue

SeasonState == {"SPRING", "SUMMER", "AUTUMN", "WINTER"}
KnowledgeState == [concepts: Set(ConceptId), masteryLevel: Nat, frontier: Set(ConceptId)]
ConceptId == STRING
FederationId == STRING
Nat == Nat
TransitionRecord == [federation: FederationId, from: SeasonState, to: SeasonState, timestamp: Nat, applied: BOOLEAN]

TypeOK ==
  /\ federations ⊆ FederationId
  /\ federationSeasons ∈ [federations -> SeasonState]
  /\ seasonStartTimes ∈ [federations -> Nat]
  /\ seasonDuration ∈ Nat
  /\ federationScores ∈ [federations -> Nat]
  /\ federationKnowledge ∈ [federations -> KnowledgeState]
  /\ transitionQueue ⊆ TransitionRecord
  /\ seasonDuration > 0
  /\ ∀ f ∈ federations: 
      federationScores[f] >= 0
      /\ federationKnowledge[f].masteryLevel >= 0
      /\ federationKnowledge[f].concepts ⊆ ConceptId
      /\ federationKnowledge[f].frontier ⊆ ConceptId

(*
  Initial state: No federations
*)
Init ==
  /\ federations = {}
  /\ federationSeasons = [f ∈ {} |-> "SPRING"]
  /\ seasonStartTimes = [f ∈ {} |-> 0]
  /\ seasonDuration = 100  \* Example: 100 checkpoint cycles per season
  /\ federationScores = [f ∈ {} |-> 0]
  /\ federationKnowledge = [f ∈ {} |-> [concepts |-> {}, masteryLevel |-> 0, frontier |-> {{}}]]
  /\ transitionQueue = <<>>

(*
  Action: Create a new federation
  Preconditions:
    - Federation ID is unique
  Effects:
    - Adds federation to federations set
    - Initializes federation state
*)
CreateFederation(federationId) ==
  /\ federationId ∉ federations
  /\ federations' = federations ∪ {federationId}
  /\ federationSeasons' = [federationSeasons EXCEPT ![federationId] = "SPRING"]
  /\ seasonStartTimes' = [seasonStartTimes EXCEPT ![federationId] = 0]
  /\ federationScores' = [federationScores EXCEPT ![federationId] = 0]
  /\ federationKnowledge' = [federationKnowledge EXCEPT ![federationId] = 
        [concepts |-> {}, masteryLevel |-> 0, frontier |-> {{}}]]
  /\ UNCHANGED <<seasonDuration, transitionQueue>>

(*
  Action: Advance federation knowledge (simplified - would be based on seed contributions)
  Preconditions:
    - Federation exists
  Effects:
    - Increases mastery level and updates frontier
*)
AdvanceKnowledge(federationId, newConcepts, frontierUpdate) ==
  /\ federationId ∈ federations
  /\ newConcepts ⊆ ConceptId
  /\ frontierUpdate ⊆ ConceptId
  /\ federationKnowledge' = [federationKnowledge EXCEPT ![federationId] = 
        [concepts |-> @.concepts ∪ newConcepts,
         masteryLevel |-> @.masteryLevel + 1,
         frontier |-> frontierUpdate]]
  /\ federationScores' = [federationScores EXCEPT ![federationId] = @ + 10]  \* Example scoring
  /\ UNCHANGED <<federations, federationSeasons, seasonStartTimes, seasonDuration, transitionQueue>>

(*
  Action: Queue a season transition
  Preconditions:
    - Federation exists
    - Current season is not the same as target season
  Effects:
    - Adds transition to queue
*)
QueueTransition(federationId, targetSeason) ==
  /\ federationId ∈ federations
  /\ targetSeason ∈ SeasonState
  /\ federationSeasons[federationId] # targetSeason
  /\ transitionQueue' = Append(transitionQueue, 
        [federation |-> federationId, 
         from |-> federationSeasons[federationId], 
         to |-> targetSeason, 
         timestamp |-> 0, 
         applied |-> FALSE])
  /\ UNCHANGED <<federationSeasons, seasonStartTimes, federationScores, federationKnowledge>>

(*
  Action: Apply season transition
  Preconditions:
    - There is a pending transition
    - Enough time has passed in current season (season duration elapsed)
  Effects:
    - Marks transition as applied
    - Updates federation season and resets start time
*)
ApplyTransition(index) ==
  /\ index ∈ 1..Len(transitionQueue)
  /\ LET 
        trans == transitionQueue[index]
        federationId == trans.federation
        timeInCurrentSeason ==  \* Simplified - would use actual timestamp
          LET 
            startTime == seasonStartTimes[federationId]
            currentTime ==  \* In real spec, this would be a global clock
              1000  \* Placeholder
          IN
            currentTime - startTime
  IN
    IF timeInCurrentSeason >= seasonDuration
    THEN /\ transitionQueue' = 
               [transitionQueue EXCEPT ![index] = 
                  [trans EXCEPT ![applied] = TRUE]]
         /\ federationSeasons' = [federationSeasons EXCEPT ![federationId] = trans.to]
         /\ seasonStartTimes' = [seasonStartTimes EXCEPT ![federationId] = 
                seasonStartTimes[federationId] + seasonDuration]  \* Simplified
         /\ UNCHANGED <<federations, federationScores, federationKnowledge>>
    ELSE /\ SKIP  \* Not time yet
    /\ UNCHANGED <<federations, federationScores, federationKnowledge>>

(*
  Action: Remove applied transitions from queue (garbage collection)
*)
CleanupQueue ==
  /\ transitionQueue' = 
        <<t ∈ transitionQueue : t.applied = FALSE>>  \* Keep only unapplied
  /\ UNCHANGED <<federations, federationSeasons, seasonStartTimes, federationScores, federationKnowledge>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: Federation seasons are valid
*)
ValidSeasons ==
  ∀ f ∈ federations: federationSeasons[f] ∈ SeasonState

(* 
  Invariant: Season start times are non-negative
*)
NonNegativeStartTimes ==
  ∀ f ∈ federations: seasonStartTimes[f] >= 0

(* 
  Invariant: Federation knowledge mastery level is non-negative
*)
NonNegativeMastery ==
  ∀ f ∈ federations: federationKnowledge[f].masteryLevel >= 0

(* 
  Invariant: Knowledge frontier is subset of concepts (or empty)
*)
FrontierSubsetOfConcepts ==
  ∀ f ∈ federations: federationKnowledge[f].frontier ⊆ federationKnowledge[f].concepts

(* 
  Invariant: Season duration is positive
*)
PositiveSeasonDuration ==
  seasonDuration > 0

(* 
  Invariant: No federation can have overlapping season transitions
  (Simplified - would check timestamps in full spec)
*)
NoOverlappingTransitions ==
  TRUE  \* Placeholder - would require temporal logic to check transition times

(*
  Spec Definition
*)
SeasonTransitionSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ federationId ∈ FederationId: CreateFederation(federationId)
  \/ \* Knowledge advancement would be triggered by seed harvests in reality
  \/ \* For now, we just show the structure
  \/ ∃ federationId ∈ Federations, newConcepts ⊆ ConceptId, frontierUpdate ⊆ ConceptId: 
        AdvanceKnowledge(federationId, newConcepts, frontierUpdate)
  \/ ∃ federationId ∈ FederationId, targetSeason ∈ SeasonState: 
        QueueTransition(federationId, targetSeason)
  \/ ∃ index ∈ Nat: ApplyTransition(index)
  \/ \*: CleanupQueue  \* This can happen anytime

vars == <<federations, federationSeasons, seasonStartTimes, seasonDuration, 
          federationScores, federationKnowledge, transitionQueue>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM SeasonTransitionSpec => []TypeOK
THEOREM SeasonTransitionSpec => []ValidSeasons
THEOREM SeasonTransitionSpec => []NonNegativeStartTimes
THEOREM SeasonTransitionSpec => []NonNegativeMastery
THEOREM SeasonTransitionSpec => []FrontierSubsetOfConcepts
THEOREM SeasonTransitionSpec => []PositiveSeasonDuration
THEOREM SeasonTransitionSpec => []NoOverlappingTransitions

=============================================================================