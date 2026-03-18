@sprint-2 @priority-medium @compliance
Feature: Privacy & Data Retention Policy
  As the compliance team
  I want defined data lifecycle policies for all user data
  So that the platform handles PII and evidence responsibly

  Scenario: PII collection is minimized and justified
    Given the platform collects user data
    When data collection points are reviewed
    Then only data necessary for platform operation is collected
    And collection purpose is disclosed at each point
    And users consent before data is collected

  Scenario: Data retention periods are defined
    Given different data types have different lifespans
    When retention policy is applied
    Then active user profiles are retained while account is active
    And item photos and grading evidence are retained for 2 years after trade completion
    And CCTV footage is retained for 90 days minimum
    And dispute evidence is retained for 3 years after resolution
    And audit logs are retained for 5 years
    And deleted account data is purged within 30 days except where legally required

  Scenario: On-chain data permanence is disclosed
    Given blockchain data cannot be deleted
    When users interact with on-chain features
    Then they are informed that wallet addresses and transaction hashes are permanent
    And no PII is stored on-chain (only hashes and references)
    And off-chain evidence can be deleted per retention policy while hashes remain
