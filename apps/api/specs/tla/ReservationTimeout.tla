---- MODULE ReservationTimeout ----
\* Formal verification that expired reservations cannot be finalized.
\* Models time progression and reservation expiry.

EXTENDS Naturals, FiniteSets

CONSTANTS Users, Items, MaxTime

VARIABLES reservations, time, finalized

vars == <<reservations, time, finalized>>

\* A reservation maps to {holder, expiresAt, status}
ReservationRecord == [holder: Users, expiresAt: 0..MaxTime, status: {"ACTIVE", "EXPIRED", "FINALIZED"}]

TypeOK ==
    /\ reservations \in [Items -> ReservationRecord \cup {<<>>}]
    /\ time \in 0..MaxTime
    /\ finalized \subseteq Items

Init ==
    /\ reservations = [i \in Items |-> <<>>]
    /\ time = 0
    /\ finalized = {}

\* User reserves an item
MakeReservation(u, i) ==
    /\ reservations[i] = <<>>
    /\ i \notin finalized
    /\ time < MaxTime
    /\ reservations' = [reservations EXCEPT ![i] = [holder |-> u, expiresAt |-> time + 2, status |-> "ACTIVE"]]
    /\ UNCHANGED <<time, finalized>>

\* Time advances
Tick ==
    /\ time < MaxTime
    /\ time' = time + 1
    /\ \* Expire any reservations past their deadline
       reservations' = [i \in Items |->
           IF reservations[i] # <<>> /\ reservations[i].status = "ACTIVE" /\ time' > reservations[i].expiresAt
           THEN [reservations[i] EXCEPT !.status = "EXPIRED"]
           ELSE reservations[i]
       ]
    /\ UNCHANGED finalized

\* Finalize an active (non-expired) reservation
Finalize(i) ==
    /\ reservations[i] # <<>>
    /\ reservations[i].status = "ACTIVE"
    /\ time <= reservations[i].expiresAt
    /\ reservations' = [reservations EXCEPT ![i] = [reservations[i] EXCEPT !.status = "FINALIZED"]]
    /\ finalized' = finalized \cup {i}
    /\ UNCHANGED time

\* Cancel a reservation
Cancel(i) ==
    /\ reservations[i] # <<>>
    /\ reservations[i].status = "ACTIVE"
    /\ reservations' = [reservations EXCEPT ![i] = <<>>]
    /\ UNCHANGED <<time, finalized>>

Next ==
    \/ Tick
    \/ \E u \in Users, i \in Items : MakeReservation(u, i)
    \/ \E i \in Items : Finalize(i)
    \/ \E i \in Items : Cancel(i)

Spec == Init /\ [][Next]_vars

\* ── Invariants ──────────────────────────────────────────

\* Expired reservations are never finalized
ExpiredNeverFinalized ==
    \A i \in Items :
        reservations[i] # <<>> /\ reservations[i].status = "EXPIRED" =>
            i \notin finalized

\* Finalized items were reserved by someone
FinalizedWereReserved ==
    \A i \in Items :
        i \in finalized => reservations[i] # <<>>

\* No item finalized after expiry
NoFinalizeAfterExpiry ==
    \A i \in Items :
        (reservations[i] # <<>> /\ reservations[i].status = "FINALIZED") =>
            reservations[i].expiresAt >= time - 1

====
