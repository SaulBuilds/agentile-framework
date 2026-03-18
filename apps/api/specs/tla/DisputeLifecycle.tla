---- MODULE DisputeLifecycle ----
\* Formal verification of the Dispute state machine.
\* Invariants: valid transitions, terminal states final.

EXTENDS Naturals, FiniteSets

CONSTANTS Users, Objects

VARIABLES disputes, evidence

vars == <<disputes, evidence>>

DisputeStates == {"NONE", "OPEN", "UNDER_REVIEW", "RESOLVED", "DENIED"}

TypeOK ==
    /\ disputes \in [Users \X Objects -> DisputeStates]
    /\ evidence \in [Users \X Objects -> Nat]

Init ==
    /\ disputes = [u \in Users, o \in Objects |-> "NONE"]
    /\ evidence = [u \in Users, o \in Objects |-> 0]

\* User opens a dispute
Open(u, o) ==
    /\ disputes[u, o] = "NONE"
    /\ disputes' = [disputes EXCEPT ![u, o] = "OPEN"]
    /\ UNCHANGED evidence

\* User or admin submits evidence (only on open/under review)
SubmitEvidence(u, o) ==
    /\ disputes[u, o] \in {"OPEN", "UNDER_REVIEW"}
    /\ evidence' = [evidence EXCEPT ![u, o] = evidence[u, o] + 1]
    /\ UNCHANGED disputes

\* Dispute moves to under review
Review(u, o) ==
    /\ disputes[u, o] = "OPEN"
    /\ disputes' = [disputes EXCEPT ![u, o] = "UNDER_REVIEW"]
    /\ UNCHANGED evidence

\* Admin resolves the dispute (approve)
Resolve(u, o) ==
    /\ disputes[u, o] \in {"OPEN", "UNDER_REVIEW"}
    /\ disputes' = [disputes EXCEPT ![u, o] = "RESOLVED"]
    /\ UNCHANGED evidence

\* Admin denies the dispute
Deny(u, o) ==
    /\ disputes[u, o] \in {"OPEN", "UNDER_REVIEW"}
    /\ disputes' = [disputes EXCEPT ![u, o] = "DENIED"]
    /\ UNCHANGED evidence

Next ==
    \E u \in Users, o \in Objects :
        \/ Open(u, o)
        \/ SubmitEvidence(u, o)
        \/ Review(u, o)
        \/ Resolve(u, o)
        \/ Deny(u, o)

Spec == Init /\ [][Next]_vars

\* ── Invariants ──────────────────────────────────────────

\* RESOLVED and DENIED are terminal — no further transitions
TerminalStatesAreFinal ==
    \A u \in Users, o \in Objects :
        disputes[u, o] \in {"RESOLVED", "DENIED"} =>
            disputes'[u, o] = disputes[u, o]

\* No evidence can be submitted to resolved/denied disputes
NoEvidenceOnTerminal ==
    \A u \in Users, o \in Objects :
        disputes[u, o] \in {"RESOLVED", "DENIED"} =>
            evidence'[u, o] = evidence[u, o]

\* RESOLVED can only come from OPEN or UNDER_REVIEW
ResolvedOnlyFromValid ==
    \A u \in Users, o \in Objects :
        (disputes[u, o] # "RESOLVED" /\ disputes'[u, o] = "RESOLVED") =>
            disputes[u, o] \in {"OPEN", "UNDER_REVIEW"}

\* DENIED can only come from OPEN or UNDER_REVIEW
DeniedOnlyFromValid ==
    \A u \in Users, o \in Objects :
        (disputes[u, o] # "DENIED" /\ disputes'[u, o] = "DENIED") =>
            disputes[u, o] \in {"OPEN", "UNDER_REVIEW"}

\* Cannot open a dispute on an already-disputed object
NoDuplicateDisputes ==
    \A u \in Users, o \in Objects :
        disputes'[u, o] = "OPEN" =>
            disputes[u, o] = "NONE"

====
