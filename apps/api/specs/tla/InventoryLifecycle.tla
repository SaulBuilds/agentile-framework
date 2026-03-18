---- MODULE InventoryLifecycle ----
\* Formal verification of InventoryItem status transitions.
\* Invariants: only valid transitions, no item in invalid state.

EXTENDS Naturals, FiniteSets

CONSTANTS Items

VARIABLES status

vars == <<status>>

ValidStates == {
    "NONE", "INTAKE", "GRADING", "AVAILABLE", "RESERVED",
    "QUARANTINED", "SHIPPED", "DELIVERED", "REJECTED", "DISPOSED"
}

TypeOK == status \in [Items -> ValidStates]

Init == status = [i \in Items |-> "NONE"]

\* Item arrives at warehouse
Intake(i) ==
    /\ status[i] = "NONE"
    /\ status' = [status EXCEPT ![i] = "INTAKE"]

\* Operator begins grading
BeginGrade(i) ==
    /\ status[i] = "INTAKE"
    /\ status' = [status EXCEPT ![i] = "GRADING"]

\* Grading result: accept
Accept(i) ==
    /\ status[i] = "GRADING"
    /\ status' = [status EXCEPT ![i] = "AVAILABLE"]

\* Grading result: reject
Reject(i) ==
    /\ status[i] = "GRADING"
    /\ status' = [status EXCEPT ![i] = "REJECTED"]

\* Grading result: quarantine
Quarantine(i) ==
    /\ status[i] = "GRADING"
    /\ status' = [status EXCEPT ![i] = "QUARANTINED"]

\* Quarantine resolution: release to available
ReleaseFromQuarantine(i) ==
    /\ status[i] = "QUARANTINED"
    /\ status' = [status EXCEPT ![i] = "AVAILABLE"]

\* Quarantine resolution: reject
RejectFromQuarantine(i) ==
    /\ status[i] = "QUARANTINED"
    /\ status' = [status EXCEPT ![i] = "REJECTED"]

\* User reserves an available item
Reserve(i) ==
    /\ status[i] = "AVAILABLE"
    /\ status' = [status EXCEPT ![i] = "RESERVED"]

\* Reservation expires, item returns to available
UnReserve(i) ==
    /\ status[i] = "RESERVED"
    /\ status' = [status EXCEPT ![i] = "AVAILABLE"]

\* Item is shipped out
Ship(i) ==
    /\ status[i] = "RESERVED"
    /\ status' = [status EXCEPT ![i] = "SHIPPED"]

\* Item is delivered
Deliver(i) ==
    /\ status[i] = "SHIPPED"
    /\ status' = [status EXCEPT ![i] = "DELIVERED"]

\* Rejected item disposed
Dispose(i) ==
    /\ status[i] = "REJECTED"
    /\ status' = [status EXCEPT ![i] = "DISPOSED"]

Next ==
    \E i \in Items :
        \/ Intake(i)
        \/ BeginGrade(i)
        \/ Accept(i)
        \/ Reject(i)
        \/ Quarantine(i)
        \/ ReleaseFromQuarantine(i)
        \/ RejectFromQuarantine(i)
        \/ Reserve(i)
        \/ UnReserve(i)
        \/ Ship(i)
        \/ Deliver(i)
        \/ Dispose(i)

Spec == Init /\ [][Next]_vars

\* ── Invariants ──────────────────────────────────────────

\* Items are always in a valid state
AllStatesValid == \A i \in Items : status[i] \in ValidStates

\* DELIVERED and DISPOSED are terminal
TerminalStates ==
    \A i \in Items :
        status[i] \in {"DELIVERED", "DISPOSED"} =>
            status'[i] = status[i]

\* No backward transition from SHIPPED to AVAILABLE
NoBackwardFromShipped ==
    \A i \in Items :
        status[i] = "SHIPPED" =>
            status'[i] \in {"SHIPPED", "DELIVERED"}

\* AVAILABLE can only come from GRADING, QUARANTINED, or RESERVED
AvailableOnlyFromValid ==
    \A i \in Items :
        (status[i] # "AVAILABLE" /\ status'[i] = "AVAILABLE") =>
            status[i] \in {"GRADING", "QUARANTINED", "RESERVED"}

====
