Feature: Game Economics and Farming
  As a player,
  I want to stake ORT and earn rewards through farming,
  So that I can grow my assets over time

  @economics @staking
  Scenario: Player stakes ORT tokens
    Given I have 100 ORT in my wallet
    When I stake 50 ORT
    Then my wallet balance should be 50 ORT
    And my staked balance should be 50 ORT

  @economics @farming
  Scenario: Farming multiplier increases over time
    Given I have staked 100 ORT
    When 30 days pass
    Then my farming multiplier should be 130%

  @economics @farming-max
  Scenario: Farming multiplier caps at 200%
    Given I have staked 100 ORT
    When 150 days pass
    Then my farming multiplier should be 200%

  @economics @rewards
  Scenario: Player harvests seed and claims rewards
    Given I have a seed with stake 100 ORT
    And the seed has growth score 85
    When I harvest the seed
    Then I should receive rewards based on stake and score

  @economics @federation-bonus
  Scenario: Federation members receive bonus rewards
    Given I am in a federation with 5 members
    And I have a seed with stake 100 ORT
    When I harvest the seed with score 70
    Then I should receive federation bonus

  @economics @duel-bonus
  Scenario: Duel winner receives bonus
    Given I win a duel against an opponent
    When I harvest my seed with score 60
    Then I should receive duel win bonus

  @economics @withdraw
  Scenario: Player withdraws staked tokens
    Given I have staked 100 ORT
    When I withdraw 50 ORT
    Then my staked balance should be 50 ORT
    And my wallet balance should increase by 50 ORT
