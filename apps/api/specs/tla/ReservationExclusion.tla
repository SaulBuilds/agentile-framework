---- MODULE ReservationExclusion ----
\* Formal verification of mutual exclusion for inventory reservations.
\* Invariant: No item reserved by two users simultaneously.

EXTENDS Naturals, FiniteSets

CONSTANTS Users, Items

VARIABLES reservedBy, itemStatus

vars == <<reservedBy, itemStatus>>

TypeOK ==
    /\ reservedBy \in [Items -> Users \cup {"NONE"}]
    /\ itemStatus \in [Items -> {"AVAILABLE", "RESERVED", "SHIPPED"}]

Init ==
    /\ reservedBy = [i \in Items |-> "NONE"]
    /\ itemStatus = [i \in Items |-> "AVAILABLE"]

\* User reserves an available item
Reserve(u, i) ==
    /\ itemStatus[i] = "AVAILABLE"
    /\ reservedBy[i] = "NONE"
    /\ reservedBy' = [reservedBy EXCEPT ![i] = u]
    /\ itemStatus' = [itemStatus EXCEPT ![i] = "RESERVED"]

\* Reservation expires or is cancelled
Release(i) ==
    /\ itemStatus[i] = "RESERVED"
    /\ reservedBy' = [reservedBy EXCEPT ![i] = "NONE"]
    /\ itemStatus' = [itemStatus EXCEPT ![i] = "AVAILABLE"]

\* Item is shipped (finalized)
Ship(i) ==
    /\ itemStatus[i] = "RESERVED"
    /\ reservedBy[i] # "NONE"
    /\ itemStatus' = [itemStatus EXCEPT ![i] = "SHIPPED"]
    /\ UNCHANGED reservedBy

Next ==
    \/ \E u \in Users, i \in Items : Reserve(u, i)
    \/ \E i \in Items : Release(i)
    \/ \E i \in Items : Ship(i)

Spec == Init /\ [][Next]_vars

\* ── Invariants ──────────────────────────────────────────

\* No item is reserved by more than one user at a time
\* (Implicit since reservedBy is a function, but we verify the guard)
MutualExclusion ==
    \A i \in Items :
        itemStatus[i] = "RESERVED" => reservedBy[i] # "NONE"

\* Available items have no holder
AvailableMeansUnreserved ==
    \A i \in Items :
        itemStatus[i] = "AVAILABLE" => reservedBy[i] = "NONE"

\* Shipped items have a holder
ShippedHasHolder ==
    \A i \in Items :
        itemStatus[i] = "SHIPPED" => reservedBy[i] # "NONE"

\* No two users can hold the same item
\* Since reservedBy is a function Items -> Users, this is inherently true.
\* But we verify the transition guard: you can't reserve a non-available item.
NoDoubleReservation ==
    \A u \in Users, i \in Items :
        (reservedBy[i] # "NONE" /\ reservedBy[i] # u) =>
            reservedBy'[i] # u \/ itemStatus'[i] = "AVAILABLE"

====
