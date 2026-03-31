------------------- MODULE AdversarialDuel -------------------

\* Adversarial specifications for DuelManager contract
\* Tests attack vectors against the duel/PVP subsystem

EXTENDS Naturals, Sequences, FiniteSets

CONSTANTS 
    MaxDuels,
    MaxPlayers,
    CooldownPeriod

VARIABLES
    duels,
    players,
    cooldowns,
    attacked

\* Initialize state
AdversarialInit ==
    /\ duels = {}
    /\ players = {}
    /\ cooldowns = [p \in players |-> 0]
    /\ attacked = FALSE

\* Front-running attack: initiate duel before target's cooldown expires
FrontRunningAttack ==
    /\ ~attacked
    /\ \E p1, p2 \in players :
        /\ cooldowns[p2] > block.timestamp
        /\ duels' = duels \cup {Duel(p1, p2, block.timestamp)}
    /\ attacked' = TRUE
    /\ UNCHANGED <<players, cooldowns>>

\* Cooldown manipulation: attacker tries to force opponent on cooldown
CooldownManipulationAttack ==
    /\ attacked
    /\ \E p1, p2 \in players :
        /\ p1 \notin cooldowns
        /\ cooldowns' = [cooldowns EXCEPT ![p2] = block.timestamp + CooldownPeriod]
    /\ UNCHANGED <<duels, players, attacked>>

\* Duel spam: attacker initiates many duels to grief opponent
DuelSpamAttack ==
    /\ \E p1, p2 \in players :
        /\ Cardinality({d \in duels : d.initiator = p1}) < 100
        /\ duels' = duels \cup {Duel(p1, p2, block.timestamp)}
    /\ UNCHANGED <<players, cooldowns, attacked>>

\* Response deadline manipulation: try to extend response time
DeadlineManipulationAttack ==
    /\ \E d \in duels :
        /\ d.responseDeadline < block.timestamp + 1000
        /\ duels' = [duels EXCEPT ![d].responseDeadline = block.timestamp + 1000]
    /\ UNCHANGED <<players, cooldowns, attacked>>

\* Invariant: All duels have valid participants
ValidParticipants ==
    \A d \in duels : 
        /\ d.playerA \in players
        /\ d.playerB \in players
        /\ d.playerA # d.playerB

\* Invariant: Duel response deadline is in the future or just passed
ValidDeadline ==
    \A d \in duels : d.responseDeadline >= d.startTime

\* Invariant: Cooldown timestamps are non-negative
ValidCooldowns ==
    \A p \in players : cooldowns[p] >= 0

\* Invariant: Duel IDs are unique
UniqueDuelIds ==
    Cardinality(duels) = Cardinality({d.id : d \in duels})

\* Property: Player can only be in one active duel at a time
OneActiveDuelPerPlayer ==
    \A p \in players :
        Cardinality({d \in duels : /\ d.playerA = p \/ d.playerB = p /\ ~d.completed}) <= 1

============================================================
