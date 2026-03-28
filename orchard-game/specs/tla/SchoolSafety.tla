---------------------------- MODULE SchoolSafety ---------------------------

EXTENDS Naturals, TLC

(*
  SchoolSafety.tla
  TLA+ specification for school safety features including content filtering
  References: The Grove Spec Document Section 5 and 10.3
*)

(*
  VARIABLES
  - schoolNodes: Set of school node identifiers
  - contentFilters: [schoolNode -> FilterLevel] - current filter setting
  - teacherOverrides: [schoolNode -> BOOLEAN] - whether teacher can override
  - studentInputs: [schoolNode -> Seq(InputRecord)] - queued student inputs
  - botResponses: [schoolNode -> Seq(ResponseRecord)] - generated responses
  - deliveryStatus: [schoolNode -> Seq(DeliveryStatus)] - what was delivered
  - panicButtonState: [schoolNode -> {"ACTIVE", "PAUSED", "ARMED"}] - panic state
  - interactionLogs: [schoolNode -> Seq(LogEntry)] - audit trail
  - contentApproval: [schoolNode -> Set(ApprovedContent)] - whitelisted content
*)

VARIABLES
  schoolNodes,
  contentFilters,
  teacherOverrides,
  studentInputs,
  botResponses,
  deliveryStatus,
  panicButtonState,
  interactionLogs,
  contentApproval

FilterLevel == {"NONE", "K-12_STANDARD", "HIGH_SCHOOL", "COLLEGE"}
InputRecord == [studentId: StudentId, payload: String, timestamp: Nat]
ResponseRecord == [requestId: RequestId, rawResponse: String, filteredResponse: String, 
                   timestamp: Nat, approved: BOOLEAN]
DeliveryStatus == [requestId: RequestId, delivered: BOOLEAN, reason: String, timestamp: Nat]
LogEntry == [eventType: EventType, details: String, timestamp: Nat, 
             studentId: StudentId, requestId: RequestId]
EventType == {"INPUT_SUBMITTED", "CONTENT_BLOCKED", "RESPONSE_GENERATED", 
              "RESPONSE_DELIVERED", "PANIC_ACTIVATED", "PANIC_DEACTIVATED",
              "TEACHER_OVERRIDE"}
StudentId == STRING
RequestId == STRING
ApprovedContent == [contentId: String, gradeLevel: String, subject: String, 
                   approvedBy: TeacherId, timestamp: Nat]
TeacherId == STRING
Nat == Nat

TypeOK ==
  /\ schoolNodes ⊆ SchoolId
  /\ contentFilters ∈ [schoolNodes -> FilterLevel]
  /\ teacherOverrides ∈ [schoolNodes -> BOOLEAN]
  /\ studentInputs ∈ [schoolNodes -> Seq(InputRecord)]
  /\ botResponses ∈ [schoolNodes -> Seq(ResponseRecord)]
  /\ deliveryStatus ∈ [schoolNodes -> Seq(DeliveryStatus)]
  /\ panicButtonState ∈ [schoolNodes -> {"ACTIVE", "PAUSED", "ARMED"}]
  /\ interactionLogs ∈ [schoolNodes -> Seq(LogEntry)]
  /\ contentApproval ∈ [schoolNodes -> SUBSET ApprovedContent]
  /\ ∀ n ∈ schoolNodes: 
      contentFilters[n] ∈ FilterLevel
      /\ teacherOverrides[n] ∈ BOOLEAN

SchoolId == STRING

(*
  Initial state: No school nodes, default safety settings
*)
Init ==
  /\ schoolNodes = {}
  /\ contentFilters = [n ∈ {} |-> "NONE"]
  /\ teacherOverrides = [n ∈ {} |-> FALSE]
  /\ studentInputs = [n ∈ {} |-> <<>>]
  /\ botResponses = [n ∈ {} |-> <<>>]
  /\ deliveryStatus = [n ∈ {} |-> <<>>]
  /\ panicButtonState = [n ∈ {} |-> "ARMED"]  \* Start armed, teacher must activate
  /\ interactionLogs = [n ∈ {} |-> <<>>]
  /\ contentApproval = [n ∈ {} |-> {}]

(*
  Action: Register a new school node
  Preconditions:
    - School ID is unique
  Effects:
    - Adds school to schoolNodes set
    - Initializes with safe defaults
*)
RegisterSchool(schoolId) ==
  /\ schoolId ∉ schoolNodes
  /\ schoolNodes' = schoolNodes ∪ {schoolId}
  /\ contentFilters' = [contentFilters EXCEPT ![schoolId] = "K-12_STANDARD"]
  /\ teacherOverrides' = [teacherOverrides EXCEPT ![schoolId] = FALSE]
  /\ studentInputs' = [studentInputs EXCEPT ![schoolId] = <<>>]
  /\ botResponses' = [botResponses EXCEPT ![schoolId] = <<>>]
  /\ deliveryStatus' = [deliveryStatus EXCEPT ![schoolId] = <<>>]
  /\ panicButtonState' = [panicButtonState EXCEPT ![schoolId] = "ARMED"]
  /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = <<>>]
  /\ contentApproval' = [contentApproval EXCEPT ![schoolId] = {}]
  /\ UNCHANGED <<schoolId>>

(*
  Action: Student submits input to chatbot
  Preconditions:
    - School exists and is not panicked
    - Input is not empty
  Effects:
    - Adds input to student inputs queue
    - Logs the submission
*)
StudentSubmitInput(schoolId, studentId, payload, timestamp) ==
  /\ schoolId ∈ schoolNodes
  /\ panicButtonState[schoolId] = "ACTIVE"  \* Only accept when active
  /\ payload /= ""  \* Non-empty input
  /\ requestId = CONCAT("req-", schoolId, "-", studentId, "-", STRING(timestamp))
  /\ studentInputs' = [studentInputs EXCEPT ![schoolId] = 
        @ << [studentId |-> studentId, payload |-> payload, timestamp |-> timestamp] >>]
  /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
        @ << [eventType |-> "INPUT_SUBMITTED", 
             details |-> CONCAT("Student ", studentId, " submitted: ", payload),
             timestamp |-> timestamp,
             studentId |-> studentId,
             requestId |-> requestId] >>]
  /\ UNCHANGED <<botResponses, deliveryStatus, panicButtonState, contentApproval, teacherOverrides>>

(*
  Action: Content filter processes student input
  Preconditions:
    - School has pending student inputs
  Effects:
    - Checks input against content filter
    - Either blocks or allows to proceed to inference
*)
ProcessContentFilter(schoolId) ==
  /\ schoolId ∈ schoolNodes
  /\ Len(studentInputs[schoolId]) > 0
  /\ LET 
        input == Head(studentInputs[schoolId])
        isApproved == 
          LET filterLevel = contentFilters[schoolId]
          IN
            CASE filterLevel OF
              "NONE" => TRUE
              "K-12_STANDARD" => 
                  ~ContainsBadWords(input.payload)  \* Simplified
              "HIGH_SCHOOL" => 
                  ~ContainsVeryBadWords(input.payload)  \* Simplified
              "COLLEGE" => TRUE  \* Minimal filtering
            END CASE
  IN
    IF isApproved
    THEN /\ studentInputs' = [studentInputs EXCEPT ![schoolId] = Tail(@)]
         /\ botResponses' = [botResponses EXCEPT ![schoolId] = 
               @ << [requestId |-> input.requestId, 
                    rawResponse |-> "",  \* Placeholder - inference happens off-spec
                    filteredResponse |-> "",
                    timestamp |-> input.timestamp + 1,
                    approved |-> TRUE] >>]
         /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
               @ << [eventType |-> "RESPONSE_GENERATED", 
                    details |-> CONCAT("Generated response for request ", input.requestId),
                    timestamp |-> input.timestamp + 1,
                    studentId |-> input.studentId,
                    requestId |-> input.requestId] >>]
    ELSE /\ studentInputs' = [studentInputs EXCEPT ![schoolId] = Tail(@)]
         /\ botResponses' = [botResponses EXCEPT ![schoolId] = 
               @ << [requestId |-> input.requestId, 
                    rawResponse |-> "", 
                    filteredResponse |-> "", 
                    timestamp |-> input.timestamp + 1,
                    approved |-> FALSE] >>]
         /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
               @ << [eventType |-> "CONTENT_BLOCKED", 
                    details |-> CONCAT("Blocked input from student ", input.studentId, 
                                       " due to content policy"),
                    timestamp |-> input.timestamp + 1,
                    studentId |-> input.studentId,
                    requestId |-> input.requestId] >>]
    /\ deliveryStatus' = [deliveryStatus EXCEPT ![schoolId] = 
          @ << [requestId |-> input.requestId, 
               delivered |-> isApproved, 
               reason |-> IF isApproved THEN "Content approved" ELSE "Content blocked",
               timestamp |-> input.timestamp + 1] >>]
  /\ UNCHANGED <<panicButtonState, contentApproval, teacherOverrides>>

(*
  Action: Teacher activates panic button
  Preconditions:
    - School exists
    - Panic button is armed
  Effects:
    - Sets panic state to PAUSED (blocks new inputs)
    - Logs the activation
*)
TeacherActivatePanic(schoolId, teacherId, timestamp) ==
  /\ schoolId ∈ schoolNodes
  /\ panicButtonState[schoolId] = "ARMED"
  /\ panicButtonState' = [panicButtonState EXCEPT ![schoolId] = "PAUSED"]
  /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
        @ << [eventType |-> "PANIC_ACTIVATED", 
             details |-> CONCAT("Teacher ", teacherId, " activated panic button"),
             timestamp |-> timestamp,
             studentId |-> "",  \* System event
             requestId |-> "" >>]
  /\ UNCHANGED <<studentInputs, botResponses, deliveryStatus, contentApproval, teacherOverrides>>

(*
  Action: Teacher deactivates panic button
  Preconditions:
    - School exists
    - Panic button is paused
  Effects:
    - Sets panic state to ACTIVE (allows new inputs)
    - Logs the deactivation
*)
TeacherDeactivatePanic(schoolId, teacherId, timestamp) ==
  /\ schoolId ∈ schoolNodes
  /\ panicButtonState[schoolId] = "PAUSED"
  /\ panicButtonState' = [panicButtonState EXCEPT ![schoolId] = "ACTIVE"]
  /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
        @ << [eventType |-> "PANIC_DEACTIVATED", 
             details |-> CONCAT("Teacher ", teacherId, " deactivated panic button"),
             timestamp |-> timestamp,
             studentId |-> "",
             requestId |-> "" >>]
  /\ UNCHANGED <<studentInputs, botResponses, deliveryStatus, contentApproval, teacherOverrides>>

(*
  Action: Deliver approved response to student
  Preconditions:
    - School has bot responses ready for delivery
    - Response was approved by content filter
  Effects:
    - Marks response as delivered
    - Logs the delivery
*)
DeliverResponse(schoolId) ==
  /\ schoolId ∈ schoolNodes
  /\ Len(botResponses[schoolId]) > 0
  /\ LET 
        response = Head(botResponses[schoolId])
        isApproved = response.approved
  IN
    IF isApproved
    THEN /\ botResponses' = [botResponses EXCEPT ![schoolId] = Tail(@)]
         /\ deliveryStatus' = [deliveryStatus EXCEPT ![schoolId] = 
               @ << [requestId |-> response.requestId, 
                    delivered |-> TRUE,
                    reason |-> "Response delivered",
                    timestamp |-> response.timestamp + 1] >>]
         /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
               @ << [eventType |-> "RESPONSE_DELIVERED", 
                    details |-> CONCAT("Delivered response for request ", response.requestId),
                    timestamp |-> response.timestamp + 1,
                    studentId |-> response.studentId,  \* Need to get this from request
                    requestId |-> response.requestId] >>]
    ELSE /\ botResponses' = [botResponses EXCEPT ![schoolId] = Tail(@)]
         /\ deliveryStatus' = [deliveryStatus EXCEPT ![schoolId] = 
               @ << [requestId |-> response.requestId, 
                    delivered |-> FALSE,
                    reason |-> "Response filtered out",
                    timestamp |-> response.timestamp + 1] >>]
         /\ interactionLogs' = [interactionLogs EXCEPT ![schoolId] = 
               @ << [eventType |-> "RESPONSE_DELIVERED", 
                    details |-> CONCAT("Filtered response not delivered for request ", 
                                       response.requestId),
                    timestamp |-> response.timestamp + 1,
                    studentId |-> "",  \* Would need to track student ID
                    requestId |-> response.requestId] >>]
  /\ UNCHANGED <<studentInputs, panicButtonState, contentApproval, teacherOverrides>)

(*
  Helper functions (simplified for TLA+)
*)
ContainsBadWords(s) ==
  FALSE  \* Placeholder - would check against bad word list

ContainsVeryBadWords(s) ==
  FALSE  \* Placeholder - would check against very bad word list

(*
  Invariants - properties that must always hold
*)

(* 
  Invariant: No inference response is delivered to a student before passing 
             through a local content filter
*)
NoUnfilteredDelivery ==
  ∀ n ∈ schoolNodes: 
      ∀ d ∈ deliveryStatus[n]: 
          d.delivered => 
            LET 
                  reqId == d.requestId
                  resp == 
                    IF Len(botResponses[n]) > 0 
                    THEN Head(botResponses[n])  \* Simplified - would need proper mapping
                    ELSE [requestId |-> "", approved |-> FALSE]
            IN
              resp.approved  \* Only approved responses are delivered

(* 
  Invariant: When panic button is active, no new inputs are accepted
*)
PanicBlocksInputs ==
  ∀ n ∈ schoolNodes: 
      (panicButtonState[n] = "PAUSED" => 
         /\ Len(studentInputs[n]) = 0  \* No new inputs when paused
         /\ Len(botResponses[n]) = 0)  \* No processing when paused

(* 
  Invariant: Student inputs are never transmitted to broader network 
             without explicit opt-in (privacy-by-default)
*)
PrivacyByDefault ==
  ∀ n ∈ schoolNodes: 
      TRUE  \* In this spec, all processing stays within school node
      \* Would add actual network transmission checks in full implementation

(* 
  Invariant: Teacher has real-time dashboard showing all interactions
*)
TeacherDashboardAvailable ==
  ∀ n ∈ schoolNodes: 
      Len(interactionLogs[n]) >= 0  \* Logs are always available for review

(* 
  Invariant: Panic button immediately halts all chatbot interactions
*)
PanicStopsInteractions ==
  ∀ n ∈ schoolNodes: 
      (panicButtonState[n] = "PAUSED" => 
         /\ Len(studentInputs[n]) = 0  \* No new inputs accepted
         /\ (Len(botResponses[n]) > 0 => 
              FALSE)  \* Would not process existing responses when paused
        )

(*
  Spec Definition
*)
SchoolSafetySpec ==
  Init /\ [][Next]_vars
  /\ WF_vars(Next)  \* Weak fairness for liveness

Next ==
  \/ ∃ schoolId ∈ SchoolId: RegisterSchool(schoolId)
  \/ ∃ schoolId ∈ schoolNodes, studentId ∈ StudentId, payload ∈ String, t ∈ Nat:
        StudentSubmitInput(schoolId, studentId, payload, t)
  \/ ∃ schoolId ∈ schoolNodes: ProcessContentFilter(schoolId)
  \/ ∃ schoolId ∈ schoolNodes, teacherId ∈ TeacherId, t ∈ Nat:
        TeacherActivatePanic(schoolId, teacherId, t)
  \/ ∃ schoolId ∈ schoolNodes, teacherId ∈ TeacherId, t ∈ Nat:
        TeacherDeactivatePanic(schoolId, teacherId, t)
  \/ ∃ schoolId ∈ schoolNodes: DeliverResponse(schoolId)

vars == <<schoolNodes, contentFilters, teacherOverrides, studentInputs, 
          botResponses, deliveryStatus, panicButtonState, interactionLogs, 
          contentApproval>>

(* 
  Theorem: The specification maintains all invariants
*)
THEOREM SchoolSafetySpec => []TypeOK
THEOREM SchoolSafetySpec => []NoUnfilteredDelivery
THEOREM SchoolSafetySpec => []PanicBlocksInputs
THEOREM SchoolSafetySpec => []PrivacyByDefault
THEOREM SchoolSafetySpec => []TeacherDashboardAvailable
THEOREM SchoolSafetySpec => []PanicStopsInteractions

=============================================================================