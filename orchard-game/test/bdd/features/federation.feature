Feature: Federation Management
  As a player,
  I want to create and join federations,
  So that I can collaborate with other players for shared rewards

  @federation @create
  Scenario: Player creates a federation
    Given the Federation contract is deployed
    When I create a federation with minimum stake 100
    Then the federation should be created with me as creator
    And the minimum stake should be 100

  @federation @join
  Scenario: Player joins a federation
    Given a federation exists with minimum stake 100
    When I join the federation
    Then I should be a member of the federation

  @federation @leave
  Scenario: Player leaves a federation
    Given I am a member of a federation
    And I have no seeds staked
    When I leave the federation
    Then I should not be a member of the federation

  @federation @stake
  Scenario: Player stakes a seed in a federation
    Given I am a member of a federation with minimum stake 50
    When I stake a seed with tokenId 1 and amount 100
    Then my stake in the federation should be 100

  @federation @unstake
  Scenario: Player unstakes a seed from a federation
    Given I have staked 100 in a federation
    When I unstake my seed with amount 100
    Then my stake in the federation should be 0

  @federation @rewards
  Scenario: Federation creator adds rewards
    Given I am the creator of a federation
    When I add 1000 ORT to the reward pool
    Then the federation reward pool should be 1000
