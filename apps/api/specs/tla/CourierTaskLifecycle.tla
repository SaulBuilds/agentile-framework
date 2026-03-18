---- MODULE CourierTaskLifecycle ----
\* Formal verification of courier task state machine.
\* Invariants: ordered transitions, no backward moves.

EXTENDS Naturals, FiniteSets

CONSTANTS Tasks, Couriers

VARIABLES taskStatus, taskCourier

vars == <<taskStatus, taskCourier>>

States == {
    "NONE", "POSTED", "ACCEPTED", "PICKUP_VERIFIED",
    "IN_TRANSIT_COURIER", "DELIVERED_COURIER", "COMPLETED", "FAILED"
}

\* Define ordering of non-terminal states
StateOrder(s) ==
    CASE s = "NONE" -> 0
      [] s = "POSTED" -> 1
      [] s = "ACCEPTED" -> 2
      [] s = "PICKUP_VERIFIED" -> 3
      [] s = "IN_TRANSIT_COURIER" -> 4
      [] s = "DELIVERED_COURIER" -> 5
      [] s = "COMPLETED" -> 6
      [] s = "FAILED" -> 7
      [] OTHER -> -1

TypeOK ==
    /\ taskStatus \in [Tasks -> States]
    /\ taskCourier \in [Tasks -> Couriers \cup {"NONE"}]

Init ==
    /\ taskStatus = [t \in Tasks |-> "NONE"]
    /\ taskCourier = [t \in Tasks |-> "NONE"]

\* Task is posted to board
Post(t) ==
    /\ taskStatus[t] = "NONE"
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "POSTED"]
    /\ UNCHANGED taskCourier

\* Courier accepts task
Accept(t, c) ==
    /\ taskStatus[t] = "POSTED"
    /\ taskCourier[t] = "NONE"
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "ACCEPTED"]
    /\ taskCourier' = [taskCourier EXCEPT ![t] = c]

\* Courier verifies pickup
VerifyPickup(t, c) ==
    /\ taskStatus[t] = "ACCEPTED"
    /\ taskCourier[t] = c
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "PICKUP_VERIFIED"]
    /\ UNCHANGED taskCourier

\* Courier reports milestone (stays in transit)
ReportMilestone(t, c) ==
    /\ taskStatus[t] \in {"PICKUP_VERIFIED", "IN_TRANSIT_COURIER"}
    /\ taskCourier[t] = c
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "IN_TRANSIT_COURIER"]
    /\ UNCHANGED taskCourier

\* Courier confirms delivery
ConfirmDelivery(t, c) ==
    /\ taskStatus[t] \in {"PICKUP_VERIFIED", "IN_TRANSIT_COURIER"}
    /\ taskCourier[t] = c
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "DELIVERED_COURIER"]
    /\ UNCHANGED taskCourier

\* Task is completed (payout)
Complete(t) ==
    /\ taskStatus[t] = "DELIVERED_COURIER"
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "COMPLETED"]
    /\ UNCHANGED taskCourier

\* Task fails (emergency, timeout, etc.)
Fail(t) ==
    /\ taskStatus[t] \in {"POSTED", "ACCEPTED", "PICKUP_VERIFIED", "IN_TRANSIT_COURIER"}
    /\ taskStatus' = [taskStatus EXCEPT ![t] = "FAILED"]
    /\ UNCHANGED taskCourier

Next ==
    \/ \E t \in Tasks : Post(t)
    \/ \E t \in Tasks, c \in Couriers : Accept(t, c)
    \/ \E t \in Tasks, c \in Couriers : VerifyPickup(t, c)
    \/ \E t \in Tasks, c \in Couriers : ReportMilestone(t, c)
    \/ \E t \in Tasks, c \in Couriers : ConfirmDelivery(t, c)
    \/ \E t \in Tasks : Complete(t)
    \/ \E t \in Tasks : Fail(t)

Spec == Init /\ [][Next]_vars

\* ── Invariants ──────────────────────────────────────────

\* No backward transitions (except to FAILED)
NoBackwardMoves ==
    \A t \in Tasks :
        taskStatus'[t] # "FAILED" =>
            StateOrder(taskStatus'[t]) >= StateOrder(taskStatus[t])

\* COMPLETED and FAILED are terminal
TerminalStates ==
    \A t \in Tasks :
        taskStatus[t] \in {"COMPLETED", "FAILED"} =>
            taskStatus'[t] = taskStatus[t]

\* Accepted tasks must have a courier
AcceptedHasCourier ==
    \A t \in Tasks :
        StateOrder(taskStatus[t]) >= StateOrder("ACCEPTED") /\ taskStatus[t] # "FAILED" =>
            taskCourier[t] # "NONE"

\* Only the assigned courier can progress the task
CourierConsistency ==
    \A t \in Tasks :
        taskCourier[t] # "NONE" =>
            taskCourier'[t] = taskCourier[t]

====
