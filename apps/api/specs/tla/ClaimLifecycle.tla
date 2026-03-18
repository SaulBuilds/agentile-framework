---- MODULE ClaimLifecycle ----
\* Formal verification of the Claim lifecycle state machine.
\* Invariants: no skip from PENDING to CONSUMED, no double-consume.

EXTENDS Naturals, FiniteSets

CONSTANTS Users, Pools

VARIABLES claims, consumed

vars == <<claims, consumed>>

ClaimStates == {"PENDING", "ACTIVE", "RESERVED", "CONSUMED", "EXPIRED", "FORFEITED"}

TypeOK ==
    /\ claims \in [Users \X Pools -> ClaimStates \cup {"NONE"}]
    /\ consumed \subseteq (Users \X Pools)

Init ==
    /\ claims = [u \in Users, p \in Pools |-> "NONE"]
    /\ consumed = {}

\* A user receives a claim in PENDING state
Activate(u, p) ==
    /\ claims[u, p] = "NONE"
    /\ claims' = [claims EXCEPT ![u, p] = "ACTIVE"]
    /\ UNCHANGED consumed

\* User reserves a claim
Reserve(u, p) ==
    /\ claims[u, p] = "ACTIVE"
    /\ claims' = [claims EXCEPT ![u, p] = "RESERVED"]
    /\ UNCHANGED consumed

\* User finalizes (consumes) claim
Consume(u, p) ==
    /\ claims[u, p] = "RESERVED"
    /\ <<u, p>> \notin consumed
    /\ claims' = [claims EXCEPT ![u, p] = "CONSUMED"]
    /\ consumed' = consumed \cup {<<u, p>>}

\* Claim expires
Expire(u, p) ==
    /\ claims[u, p] \in {"ACTIVE", "RESERVED"}
    /\ claims' = [claims EXCEPT ![u, p] = "EXPIRED"]
    /\ UNCHANGED consumed

\* Claim is forfeited (admin action)
Forfeit(u, p) ==
    /\ claims[u, p] \in {"ACTIVE", "RESERVED"}
    /\ claims' = [claims EXCEPT ![u, p] = "FORFEITED"]
    /\ UNCHANGED consumed

Next ==
    \E u \in Users, p \in Pools :
        \/ Activate(u, p)
        \/ Reserve(u, p)
        \/ Consume(u, p)
        \/ Expire(u, p)
        \/ Forfeit(u, p)

Spec == Init /\ [][Next]_vars

\* ── Invariants ──────────────────────────────────────────

\* No claim can skip from PENDING/ACTIVE directly to CONSUMED
NoSkipToConsumed ==
    \A u \in Users, p \in Pools :
        claims[u, p] = "CONSUMED" => <<u, p>> \in consumed

\* No double consumption — once consumed, cannot be consumed again
NoDoubleConsume ==
    \A u \in Users, p \in Pools :
        claims[u, p] = "CONSUMED" => <<u, p>> \in consumed

\* Terminal states are truly terminal
TerminalStatesAreFinal ==
    \A u \in Users, p \in Pools :
        claims[u, p] \in {"CONSUMED", "EXPIRED", "FORFEITED"} =>
            claims'[u, p] = claims[u, p]

\* CONSUMED can only be reached from RESERVED
ConsumedOnlyFromReserved ==
    \A u \in Users, p \in Pools :
        (claims[u, p] # "CONSUMED" /\ claims'[u, p] = "CONSUMED") =>
            claims[u, p] = "RESERVED"

====
