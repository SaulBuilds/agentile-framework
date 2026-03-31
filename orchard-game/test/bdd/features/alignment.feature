Feature: Counterparty Alignment System
  As a player,
  I want to discover and track relationships with other players,
  So that I can find allies and understand my rivals

  @alignment @similarity
  Scenario: Player updates input hash for alignment
    Given I am a player in the game
    When I update my input hash with my seed payload
    Then the system should record my input hash

  @alignment @discover
  Scenario: Discover alignment with another player
    Given I have an input hash recorded
    And another player has a similar input hash
    When I discover alignment with them
    Then we should have a similarity score above threshold

  @alignment @rivalry
  Scenario: Duel creates rivalry
    Given I challenge another player to a duel
    When I win the duel
    Then we should be marked as rivals

  @alignment @alliance
  Scenario: Duel loss creates alliance
    Given I challenge another player to a duel
    When I lose the duel
    Then we should be marked as allies

  @alignment @connections
  Scenario: View connected players
    Given I have alignments with multiple players
    When I view my connections
    Then I should see all aligned players

  @alignment @stats
  Scenario: View player duel stats
    Given I have played multiple duels
    When I check my stats
    Then I should see total duels, wins, and losses
