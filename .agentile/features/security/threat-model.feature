@sprint-2 @priority-high @security
Feature: Threat Model
  As the security team
  I want a comprehensive threat model covering all system layers
  So that we identify and mitigate risks before implementation begins

  Scenario: Smart contract threats are identified
    Given the protocol has on-chain state
    When threats are analyzed
    Then reentrancy attacks on settlement functions are mitigated by guards
    And claim inflation attacks are prevented by the one-claim-per-item invariant
    And unauthorized minting is prevented by role-based access
    And signature replay is prevented by EIP-712 typed data and nonce tracking
    And front-running of reservations is mitigated by commit-reveal or access control

  Scenario: Backend threats are identified
    Given the backend handles sensitive operations
    When threats are analyzed
    Then injection attacks are mitigated by parameterized queries via Prisma
    And unauthorized access is prevented by role-based guards on every endpoint
    And media upload attacks are mitigated by signed URLs and malware scanning
    And API abuse is mitigated by rate limiting

  Scenario: Warehouse threats are identified
    Given warehouses handle physical items
    When threats are analyzed
    Then internal theft is mitigated by role separation and CCTV
    And item substitution is mitigated by evidence photos and serial tracking
    And collusion is mitigated by dual control for high-value items

  Scenario: User-facing threats are identified
    Given users interact with the platform
    When threats are analyzed
    Then account takeover is mitigated by MFA and wallet verification
    And fake submissions are mitigated by pre-submission declarations and warehouse verification
    And social engineering is mitigated by address privacy controls
