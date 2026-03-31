------------------- MODULE AdversarialFederation -------------------

\* Adversarial specifications for Federation contract
\* Tests attack vectors against the economic subsystem

EXTENDS Naturals, Sequences, FiniteSets

CONSTANTS 
    MaxFederations,
    MaxPlayers,
    MaxStake

VARIABLES
    federations,
    players,
    stakes,
    rewards,
    attacked

\* Initialize state
AdversarialInit ==
    /\ federations = {}
    /\ players = {}
    /\ stakes = [p \in players |-> 0]
    /\ rewards = [f \in federations |-> 0]
    /\ attacked = FALSE

\* Sybil attack: attacker creates many federations with minimum stake
SybilAttack ==
    /\ ~attacked
    /\ LET newFedIds == {i \in 1..1000 : i \notin federations} IN
        /\ federations' = federations \cup newFedIds
        /\ rewards' = [f \in federations' |-> 0]
    /\ attacked' = TRUE
    /\ UNCHANGED <<players, stakes>>

\* Stake grinding: attacker tries to minimize stake to join many federations
StakeGrindingAttack ==
    /\ attacked
    /\ \E f \in federations :
        /\ stakes'[players] = [p \in players |-> MinStake(f)]
    /\ UNCHANGED <<federations, rewards, attacked>>

\* Reward pool exhaustion: attacker drains all rewards
RewardPoolExhaustionAttack ==
    /\ \E f \in federations :
        /\ rewards[f] > 0
        /\ rewards' = [rewards EXCEPT ![f] = 0]
    /\ UNCHANGED <<federations, players, stakes, attacked>>

\* Griefing: attacker joins federations without contributing
GriefingAttack ==
    /\ \E f \in federations :
        /\ \E p \in players :
            /\ p \notin federations[f].members
            /\ federations' = [federations EXCEPT 
                ![f].members' = federations[f].members \cup {p}]
    /\ UNCHANGED <<players, stakes, rewards, attacked>>

\* Invariant: Positive reward pools
RewardPoolPositive ==
    \A f \in federations : rewards[f] >= 0

\* Invariant: Valid minimum stake requirements
ValidMinStake ==
    \A f \in federations : federations[f].minStake > 0

\* Invariant: Player stake is non-negative
NonNegativeStake ==
    \A p \in players : stakes[p] >= 0

\* Invariant: Federation has valid creator
ValidCreator ==
    \A f \in federations : federations[f].creator \in players

\* Property: Conservation of stake value
StakeConservation ==
    \A p \in players : 
        OriginalStake(p) = CurrentStake(p) + WithdrawnStake(p)

============================================================
