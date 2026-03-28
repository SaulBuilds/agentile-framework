---------------------------- MODULE MentorPropagation ---------------------------

EXTENDS Naturals, TLC

(*
  MentorPropagation.tla
  TLA+ specification for LoRA adapter propagation in the mentor-mentee protocol
  References: The Grove Spec Document Section 2 and Citrate Papers II & III
*)

(*
  VARIABLES
  - models: Set of model identifiers
  - modelScores: [modelId -> Nat] - performance score (blue score composite)
  - modelLoRAs: [modelId -> LoRA] - LoRA adapter weights for each model
  - mentorSet: Set(modelId) - current mentor models
  - menteeSet: Set(modelId) - current mentee models
  - propagationQueue: Seq(PropagationRecord) - pending LoRA transfers
  - appliedAdapters: [modelId -> Set(AdapterId)] - history of applied adapters
*)

VARIABLES
  models,
  modelScores,
  modelLoRAs,
  mentorSet,
  menteeSet,
  propagationQueue,
  appliedAdapters

LoRA == [rank: Nat, weights: Seq(Real), bias: Seq(Real)]
AdapterId == STRING
ModelId == STRING
Nat == Nat
Real == Real  \* Simplified - in practice would be floating point with precision constraints
PropagationRecord == [from: ModelId, to: ModelId, adapter: AdapterId, timestamp: Nat, applied: BOOLEAN]

TypeOK ==
  /\ models ⊆ ModelId
  /\ modelScores ∈ [models -> Nat]
  /\ modelLoRAs ∈ [models -> LoRA]
  /\ mentorSet ⊆ models
  /\ menteeSet ⊆ models
  /\ propagationQueue ⊆ PropagationRecord
  /\ appliedAdapters ∈ [models -> SUBSET AdapterId]
  /\ mentorSet ∩ menteeSet = {}  \* A model can't be both mentor and mentee
  /\ ∀ m ∈ models: modelScores[m] >= 0
  /\ ∀ p ∈ propagationQueue: p.from ∈ models /\ p.to ∈ models /\ p.from # p.to

(*
  Initial state: No models, empty mentor/mentee sets
*)
Init ==
  /\ models = {}
  /\ modelScores = [m ∈ {} |-> 0]
  /\ modelLoRAs = [m ∈ {} |-> [rank |-> 0, weights |-> <<>>, bias |-> <<>>]]
  /\ mentorSet = {}
  /\ menteeSet = {}
  /\ propagationQueue = <<>>
  /\ appliedAdapters = [m ∈ {} |-> {}]

(*
  Action: Register a new model
  Preconditions:
    - Model ID is unique
  Effects:
    - Adds model to models set
    - Initializes with zero score and empty LoRA
*)
RegisterModel(modelId) ==
  /\ modelId ∉ models
  /\ models' = models ∪ {modelId}
  /\ modelScores' = [modelScores EXCEPT ![modelId] = 0]
  /\ modelLoRAs' = [modelLoRAs EXCEPT ![modelId] = [rank |-> 0, weights |-> <<>>, bias |-> <<>>]]
  /\ UNCHANGED <<mentorSet, menteeSet, propagationQueue, appliedAdapters>>

(*
  Action: Update model score (based on accuracy, latency, cooperation)
  Preconditions:
    - Model exists
    - New score is valid
  Effects:
    - Updates model's performance score
*)
UpdateModelScore(modelId, newScore) ==
  /\ modelId ∈ models
  /\ newScore >= 0
  /\ modelScores' = [modelScores EXCEPT ![modelId] = newScore]
  /\ UNCHANGED <<models, modelLoRAs, mentorSet, menteeSet, propagationQueue, appliedAdapters>>

(*
  Action: Update model LoRA adapter
  Preconditions:
    - Model exists
    - LoRA is valid
  Effects:
    - Updates model's LoRA adapter weights
*)
UpdateModelLoRA(modelId, newLoRA) ==
  /\ modelId ∈ models
  /\ newLoRA.rank >= 0
  /\ modelLoRAs' = [modelLoRAs EXCEPT ![modelId] = newLoRA]
  /\ UNCHANGED <<models, modelScores, mentorSet, menteeSet, propagationQueue, appliedAdapters>>

(*
  Action: Select mentors based on blue score (top performers)
  Preconditions:
    - At least one model exists
  Effects:
    - Updates mentor and mentee sets based on scores
*)
SelectMentors(mentorCount) ==
  /\ mentorCount > 0
  /\ LET
        sortedModels == 
          LET modelsSeq == <<m : m ∈ models>> 
          IN
            SortSeq(modelsSeq, 
                    Lambda(m1, m2: modelScores[m1] > modelScores[m2]))
        newMentors == 
          IF Cardinality(modelsSeq) < mentorCount
          THEN SET modelsSeq  \* If fewer models than requested, all are mentors
          ELSE SUBSEQ(modelsSeq, 1, mentorCount)  \* Take top mentorCount models
        newMentees == models \ newMentors
  IN
    /\ mentorSet' = newMentors
    /\ menteeSet' = newMentees
    /\ UNCHANGED <<models, modelScores, modelLoRAs, propagationQueue, appliedAdapters>>

(*
  Action: Queue LoRA propagation from mentor to mentee
  Preconditions:
    - Mentor and mentee both exist
    - Mentor has a LoRA to share
    - Mentee hasn't already received this adapter recently
  Effects:
    - Adds propagation record to queue
*)
QueuePropagation(mentorId, menteeId, adapterId) ==
  /\ mentorId ∈ mentorSet
  /\ menteeId ∈ menteeSet
  /\ adapterId ∈ AdapterId
  /\ adapterId ∉ appliedAdapters[menteeId]  \* Prevent immediate re-propagation of same adapter
  /\ propagationQueue' = Append(propagationQueue, 
        [from |-> mentorId, to |-> menteeId, adapter |-> adapterId, 
         timestamp |-> 0, applied |-> FALSE])
  /\ UNCHANGED <<models, modelScores, modelLoRAs, mentorSet, menteeSet, appliedAdapters>>

(*
  Action: Apply queued LoRA propagation
  Preconditions:
    - There is a pending propagation
    - Source model has the adapter to propagate
  Effects:
    - Marks propagation as applied
    - Updates mentee's LoRA (simplified - in practice would blend adapters)
    - Records adapter in mentee's history
*)
ApplyPropagation(index) ==
  /\ index ∈ 1..Len(propagationQueue)
  /\ LET 
        prop == propagationQueue[index]
        fromModel == prop.from
        toModel == prop.to
        adapterId == prop.adapter
        canApply == 
          adapterId ∈ modelLoRAs[fromModel].weights  \* Simplified - would check if mentor has this adapter
  IN
    IF canApply
    THEN /\ propagationQueue' = 
               [propagationQueue EXCEPT ![index] = 
                  [prop EXCEPT ![applied] = TRUE]]
         /\ modelLoRAs' = [modelLoRAs EXCEPT ![toModel] = 
                  [rank |-> modelLoRAs[toModel].rank + 1,  \* Simplified rank increase
                   weights |-> Append(modelLoRAs[toModel].weights, 0.0),  \* Add dummy weight
                   bias |-> modelLoRAs[toModel].bias]]
         /\ appliedAdapters' = [appliedAdapters EXCEPT ![toModel] = 
                  @ ∪ {adapterId}]
    ELSE /\ SKIP  \* Can't apply, leave queue unchanged
    /\ UNCHANGED <<models, modelScores, mentorSet, menteeSet, appliedAdapters>>

(*
  Action: Remove applied propagations from queue (garbage collection)
*)
CleanupQueue ==
  /\ propagationQueue' = 
        <<p ∈ propagationQueue : p.applied = FALSE>>  \* Keep only unapplied
  /\ UNCHANGED <<models, modelScores, modelLoRAs, mentorSet, menteeSet, appliedAdapters>>

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: Mentor and mentee sets are disjoint
*)
DisjointSets ==
  mentorSet ∩ menteeSet = {}

(* 
  Invariant: All models in mentor/mentee sets exist in models set
*)
ValidMentorMentee ==
  mentorSet ⊆ models /\ menteeSet ⊆ models

(* 
  Invariant: Propagation queue only contains valid model references
*)
ValidPropagationQueue ==
  ∀ p ∈ propagationQueue: 
      p.from ∈ models /\ p.to ∈ models /\ p.from # p.to

(* 
  Invariant: Applied adapters are tracked correctly
*)
AdapterHistoryConsistent ==
  ∀ m ∈ models: 
      appliedAdapters[m] ⊆ AdapterId  \* Simplified - would check against actual adapters used

(* 
  Invariant: No duplicate propagations in queue for same mentor-mentee-adapter triplet
*)
NoDuplicatePropagations ==
  ∀ i ∈ 1..Len(propagationQueue)-1, j ∈ i+1..Len(propagationQueue):
      propagationQueue[i].from # propagationQueue[j].from 
      \/ propagationQueue[i].to # propagationQueue[j].to
      \/ propagationQueue[i].adapter # propagationQueue[j].adapter

(*
  Spec Definition
*)
MentorPropagationSpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ modelId ∈ ModelId: RegisterModel(modelId)
  \/ ∃ modelId ∈ ModelId, newScore ∈ Nat: UpdateModelScore(modelId, newScore)
  \/ ∃ modelId ∈ ModelId, newLoRA ∈ LoRA: UpdateModelLoRA(modelId, newLoRA)
  \/ ∃ mentorCount ∈ Nat+: SelectMentors(mentorCount)
  \/ ∃ mentorId ∈ ModelId, menteeId ∈ ModelId, adapterId ∈ AdapterId: 
        QueuePropagation(mentorId, menteeId, adapterId)
  \/ ∃ index ∈ Nat: ApplyPropagation(index)
  \/ \*: CleanupQueue  \* This can happen anytime
vars == <<models, modelScores, modelLoRAs, mentorSet, menteeE, propagationQueue, appliedAdapters>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM MentorPropagationSpec => []TypeOK
THEOREM MentorPropagationSpec => []DisjointSets
THEOREM MentorPropagationSpec => []ValidMentorMentee
THEOREM MentorPropagationSpec => []ValidPropagationQueue
THEOREM MentorPropagationSpec => []AdapterHistoryConsistent
THEOREM MentorPropagationSpec => []NoDuplicatePropagations

=============================================================================