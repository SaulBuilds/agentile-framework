---------------------------- MODULE MysteryBoxRNG ---------------------------

EXTENDS Naturals, TLC

(*
  MysteryBoxRNG.tla
  TLA+ specification for Mystery Box random reward distribution
  References: The Grove Spec Document Sections 2 and 10.1
*)

(*
  VARIABLES
  - mysteryBoxes: Set(BoxId) - available mystery boxes
  - boxContents: [BoxId -> Reward] - what each box contains
  - boxStates: [BoxId -> {"UNASSIGNED", "AWARDED", "CLAIMED"}] - state of each box
  - awardedBoxes: Set(BoxId) - boxes that have been awarded
  - claimedBoxes: Set(BoxId) - boxes that have been claimed by players
  - totalRewardsDistributed: Nat - total ORT distributed from mystery boxes
  - rngState: Nat - state of the random number generator
*)

VARIABLES
  mysteryBoxes,
  boxContents,
  boxStates,
  awardedBoxes,
  claimedBoxes,
  totalRewardsDistributed,
  rngState

Reward == [amount: Nat, itemType: ItemType]
ItemType == {"KnowledgeNFT", "BoostToken", "AccessPass", "Currency"}
BoxId == STRING
Nat == Nat

TypeOK ==
  /\ mysteryBoxes ⊆ BoxId
  /\ boxContents ∈ [mysteryBoxes -> Reward]
  /\ boxStates ∈ [mysteryBoxes -> {"UNASSIGNED", "AWARDED", "CLAIMED"}]
  /\ awardedBoxes ⊆ mysteryBoxes
  /\ claimedBoxes ⊆ mysteryBoxes
  /\ totalRewardsDistributed ∈ Nat
  /\ rngState ∈ Nat
  /\ awardedBoxes ⊆ {b ∈ mysteryBoxes : boxStates[b] = "AWARDED"}
  /\ claimedBoxes ⊆ {b ∈ mysteryBoxes : boxStates[b] = "CLAIMED"}
  /\ ∀ b ∈ mysteryBoxes: 
      boxContents[b].amount >= 0

(*
  Initial state: No mystery boxes
*)
Init ==
  /\ mysteryBoxes = {}
  /\ boxContents = [b ∈ {} |-> [amount |-> 0, itemType |-> "Currency"]]
  /\ boxStates = [b ∈ {} |-> "UNASSIGNED"]
  /\ awardedBoxes = {}
  /\ claimedBoxes = {}
  /\ totalRewardsDistributed = 0
  /\ rngState = 12345  \* Initial seed

(*
  Action: Create a new mystery box
  Preconditions:
    - Box ID is unique
    - Reward amount is non-negative
  Effects:
    - Adds box to mysteryBoxes set
    - Initializes box state as UNASSIGNED
*)
CreateMysteryBox(boxId, amount, itemType) ==
  /\ boxId ∉ mysteryBoxes
  /\ amount >= 0
  /\ mysteryBoxes' = mysteryBoxes ∪ {boxId}
  /\ boxContents' = [boxContents EXCEPT ![boxId] = [amount |-> amount, itemType |-> itemType]]
  /\ boxStates' = [boxStates EXCEPT ![boxId] = "UNASSIGNED"]
  /\ UNCHANGED <<awardedBoxes, claimedBoxes, totalRewardsDistributed, rngState>>

(*
  Action: Award a mystery box (triggered by events like Blight defense success)
  Preconditions:
    - Box exists and is UNASSIGNED
    - RNG selects this box for awarding
  Effects:
    - Sets box state to AWARDED
    - Adds to awardedBoxes set
*)
AwardMysteryBox(boxId) ==
  /\ boxId ∈ mysteryBoxes
  /\ boxStates[boxId] = "UNASSIGNED"
  /\ LET 
        rngResult == (rngState * 1664525 + 1013904223) % 2^32  \* Simple LCG
        shouldAward == rngResult % 100 < 30  \* 30% chance to award (example probability)
  IN
    IF shouldAward
    THEN /\ boxStates' = [boxStates EXCEPT ![boxId] = "AWARDED"]
         /\ awardedBoxes' = awardedBoxes ∪ {boxId}
         /\ rngState' = (rngState * 1664525 + 1013904223) % 2^32
    ELSE /\ SKIP  \* Box remains unassigned
    /\ UNCHANGED <<mysteryBoxes, boxContents, claimedBoxes, totalRewardsDistributed>>

(*
  Action: Claim an awarded mystery box
  Preconditions:
    - Box exists and is AWARDED
    - Player interacts with the box
  Effects:
    - Sets box state to CLAIMED
    - Adds to claimedBoxes set
    - Increments total rewards distributed
*)
ClaimMysteryBox(boxId) ==
  /\ boxId ∈ mysteryBoxes
  /\ boxStates[boxId] = "AWARDED"
  /\ boxStates' = [boxStates EXCEPT ![boxId] = "CLAIMED"]
  /\ claimedBoxes' = claimedBoxes ∪ {boxId}
  /\ totalRewardsDistributed' = totalRewardsDistributed + boxContents[boxId].amount
  /\ UNCHANGED <<mysteryBoxes, boxContents, awardedBoxes, rngState>>

(*
  Action: Update RNG state (called periodically)
*)
UpdateRNG ==
  /\ rngState' = (rngState * 1664525 + 1013904223) % 2^32
  /\ UNCHANGED <<mysteryBoxes, boxContents, boxStates, awardedBoxes, claimedBoxes, totalRewardsDistributed>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: Box states are consistent
*)
BoxStateConsistency ==
  ∀ b ∈ mysteryBoxes: 
      (b ∈ awardedBoxes <=> boxStates[b] = "AWARDED") 
      /\ (b ∈ claimedBoxes <=> boxStates[b] = "CLAIMED")
      /\ (b ∉ awardedBoxes /\ b ∉ claimedBoxes <=> boxStates[b] = "UNASSIGNED")

(* 
  Invariant: Claimed boxes are a subset of awarded boxes
*)
ClaimedSubsetOfAwarded ==
  claimedBoxes ⊆ awardedBoxes

(* 
  Invariant: Total rewards distributed is non-negative and consistent
*)
RewardsDistributedConsistent ==
  totalRewardsDistributed >= 0
  /\ totalRewardsDistributed = Σ {b ∈ claimedBoxes : boxContents[b].amount}

(* 
  Invariant: RNG state is always within bounds
*)
RNGStateBounds ==
  0 <= rngState /\ rngState < 2^32

(* 
  Invariant: No box can be awarded twice without being claimed in between
  (Prevents double-awarding before claiming)
*)
NoDoubleAwardWithoutClaim ==
  ∀ b ∈ mysteryBoxes: 
      [] (boxStates[b] = "AWARDED" => <> (boxStates[b] = "CLAIMED" /\ boxStates[b] # "AWARDED"))
      \* This is a liveness property - in practice we'd check that awarded boxes eventually get claimed
      \* For safety, we just ensure claimed boxes don't exceed awarded boxes (handled above)

(*
  Spec Definition
*)
MysteryBoxRNGSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ boxId ∈ BoxId, amount ∈ Nat, itemType ∈ ItemType: 
        CreateMysteryBox(boxId, amount, itemType)
  \/ ∃ boxId ∈ BoxId: AwardMysteryBox(boxId)
  \/ ∃ boxId ∈ BoxId: ClaimMysteryBox(boxId)
  \/ UpdateRNG

vars == <<mysteryBoxes, boxContents, boxStates, awardedBoxes, claimedBoxes, 
          totalRewardsDistributed, rngState>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM MysteryBoxRNGSpec => []TypeOK
THEOREM MysteryBoxRNGSpec => []BoxStateConsistency
THEOREM MysteryBoxRNGSpec => []ClaimedSubsetOfAwarded
THEOREM MysteryBoxRNGSpec => []RewardsDistributedConsistent
THEOREM MysteryBoxRNGSpec => []RNGStateBounds

=============================================================================