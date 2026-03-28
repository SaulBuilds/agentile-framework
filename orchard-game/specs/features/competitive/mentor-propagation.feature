Feature: Mentor Propagation
  # @TLA+ ../tla/MentorPropagation.tla
  # @tags @competitive @federated @learning
  
  Background:
    Given federation "Marine Biology 101" is active
    And there are 5 models in the federation with scores [100, 85, 70, 60, 50]
    And the top 2 models are mentors (scores 100 and 85)
    And the bottom 3 models are mentees (scores 70, 60, 50)

  Scenario: Mentors are selected based on blue scores
    When the mentor selection process runs with mentorCount = 2
    Then the models with scores 100 and 85 are designated as mentors
    And the models with scores 70, 60, and 50 are designated as mentees
    And no model is both a mentor and a mentee

  Scenario: LoRA propagation is queued from mentor to mentee
    Given Mentor Model A has a trained LoRA adapter "adapter-123"
    And Mentee Model B has not recently received "adapter-123"
    When the propagation queue process runs
    Then a propagation record is added for transferring "adapter-123" from Model A to Model B
    And the record is marked as not yet applied

  Scenario: Queued LoRA propagation is applied
    Given there is a queued propagation from Mentor Model A to Mentee Model B for "adapter-123"
    And Model A possesses the "adapter-123" to share
    When the apply propagation process runs on that queue item
    Then the propagation is marked as applied
    And Model B's LoRA is updated to include knowledge from "adapter-123"
    And "adapter-123" is recorded in Model B's adapter history

  Scenario: Applied propagations are cleaned from queue
    Given there are applied propagations in the queue
    When the cleanup queue process runs
    Then only unapplied propagations remain in the queue
    And applied propagations are removed

  Scenario: Duplicate propagations are prevented in queue
    Given there is already a queued propagation from Model A to Model B for "adapter-123"
    When another attempt is made to queue the same propagation
    Then no additional propagation record is added
    And the queue still contains only one record for that mentor-mentee-adapter triplet

  Scenario: Mentor and mentee sets remain disjoint
    At all times
    Then no model is simultaneously in both the mentor set and mentee set
    And all mentors and mentees are valid, existing models