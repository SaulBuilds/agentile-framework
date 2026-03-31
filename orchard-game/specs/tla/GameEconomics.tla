------------------- MODULE GameEconomics -------------------

\* Economic system specification for Orchard Game
\* Models staking, rewards, farming mechanics, and counterparty alignment

EXTENDS Naturals, Sequences, FiniteSets, Reals

CONSTANTS 
    MaxPlayers,
    MaxSeeds,
    MaxFederations,
    InitialSupply

VARIABLES
    players,
    seeds,
    federations,
    balances,
    staked,
    rewards,
    alignments,
    seasons

\* ============================================
\* INITIALIZATION
\* ============================================

Init ==
    /\ players = {}
    /\ seeds = {}
    /\ federations = {}
    /\ balances = [p \in Players |-> 0]
    /\ staked = [p \in Players |-> 0]
    /\ rewards = [p \in Players |-> 0]
    /\ alignments = {}
    /\ seasons = 0

\* ============================================
\* PLAYER ACTIONS
\* ============================================

PlantSeed(p, payload, stake, federation) ==
    /\ p \in players
    /\ stake >= MinStake
    /\ balances[p] >= stake
    /\ seeds' = seeds \cup {Seed(p, payload, federation, 0, stake, "planted")}
    /\ balances' = [balances EXCEPT ![p] = balances[p] - stake]
    /\ staked' = [staked EXCEPT ![p] = staked[p] + stake]
    /\ UNCHANGED <<players, federations, rewards, alignments, seasons>>

AdvanceCheckpoint(seedId) ==
    /\ \E s \in seeds :
        /\ s.id = seedId
        /\ s.owner = msg.sender
        /\ s.status = "growing"
        /\ s.checkpoint < s.maxCheckpoint
        /\ seeds' = [seeds EXCEPT ![seedId].checkpoint = s.checkpoint + 1]
    /\ UNCHANGED <<players, balances, staked, rewards, alignments, seasons>>

HarvestSeed(seedId, score) ==
    /\ \E s \in seeds :
        /\ s.id = seedId
        /\ s.status = "ready"
        /\ CalculateReward(s.stake, score) > 0
        /\ seeds' = [seeds EXCEPT ![seedId].status = "harvested"]
        /\ rewards' = [rewards EXCEPT ![s.owner] = rewards[s.owner] + CalculateReward(s.stake, score)]
        /\ staked' = [staked EXCEPT ![s.owner] = staked[s.owner] - s.stake]
    /\ UNCHANGED <<players, balances, federations, alignments, seasons>>

Stake(p, amount) ==
    /\ p \in players
    /\ amount > 0
    /\ balances[p] >= amount
    /\ balances' = [balances EXCEPT ![p] = balances[p] - amount]
    /\ staked' = [staked EXCEPT ![p] = staked[p] + amount]
    /\ UNCHANGED <<players, seeds, federations, rewards, alignments, seasons>>

Unstake(p, amount) ==
    /\ p \in players
    /\ amount > 0
    /\ staked[p] >= amount
    /\ staked' = [staked EXCEPT ![p] = staked[p] - amount]
    /\ balances' = [balances EXCEPT ![p] = balances[p] + amount]
    /\ UNCHANGED <<players, seeds, federations, rewards, alignments, seasons>>

\* ============================================
\* ECONOMIC MECHANICS
\* ============================================

CalculateReward(stake, score) ==
    LET baseReward == stake * score / 100
    LET farmingBonus == baseReward * GetFarmingMultiplier(msg.sender) / 100
    LET federationBonus == baseReward * GetFederationBonus(msg.sender) / 100
    IN baseReward + farmingBonus + federationBonus

GetFarmingMultiplier(p) ==
    LET daysStaked == (Now - p.stakeStartTime) / Day
    IN Min(100 + daysStaked, 200) \* 1% per day, max 2x

GetFederationBonus(p) ==
    LET memberCount == Cardinality(p.federations)
    IN memberCount * 10  \* 10% per member

GetFarmingMultiplier(p) ==
    LET daysStaked == (Now - p.stakeStartTime) / Day
    IN Min(100 + daysStaked, 200)

GetFarmingMultiplier(p) ==
    LET daysStaked == (Now - p.stakeStartTime) / Day
    IN Min(100 + daysStaked, 200)

\* ============================================
\* COUNTERPARTY ALIGNMENT
\* ============================================

UpdateInputHash(p, hash) ==
    /\ players' = [players EXCEPT ![p].inputHash = hash]
    /\ UNCHANGED <<seeds, federations, balances, staked, rewards, alignments, seasons>>

DiscoverAlignment(p1, p2) ==
    /\ Similarity(p1, p2) >= SimilarityThreshold
    /\ alignments' = alignments \cup {Alignment(p1, p2, Similarity(p1, p2))}
    /\ UNCHANGED <<players, seeds, federations, balances, staked, rewards, seasons>>

Similarity(p1, p2) ==
    LET matchBits == CountMatchingBits(players[p1].inputHash, players[p2].inputHash)
    IN (matchBits * 100) / 256

CountMatchingBits(h1, h2) ==
    Cardinality({i \in 0..255 : ((h1 >> i) & 1) = ((h2 >> i) & 1)})

RecordDuelOutcome(p1, p2, p1Won) ==
    /\ alignments' = [alignments EXCEPT 
        ![AlignmentKey(p1, p2)] = 
            IF p1Won 
            THEN {...!, isRival = TRUE, alignmentStrength = 100}
            ELSE {...!, isAlly = TRUE, alignmentStrength = 50}]
    /\ UNCHANGED <<players, seeds, federations, balances, staked, rewards, seasons>>

\* ============================================
\* INVARIANTS
\* ============================================

\* All balances are non-negative
InvBalancesNonNegative ==
    \A p \in players : balances[p] >= 0

\* Total supply is conserved
InvSupplyConservation ==
    Sum(balances) + Sum(staked) + Sum(rewards) = InitialSupply

\* Stake never exceeds balance
InvStakeBound ==
    \A p \in players : staked[p] <= balances[p] + InitialSupply

\* Farming multiplier bounded
InvFarmingMultiplier ==
    \A p \in players : 100 <= GetFarmingMultiplier(p) <= 200

\* Similarity score between 0-100
InvSimilarityBound ==
    \A a \in alignments : 0 <= a.similarityScore <= 100

\* Rewards non-negative
InvRewardsNonNegative ==
    \A p \in players : rewards[p] >= 0

\* Alignment strength bounded
InvAlignmentStrength ==
    \A a \in alignments : 0 <= a.alignmentStrength <= 100

\* ============================================
\* TEMPORAL PROPERTIES
\* ============================================

\* Eventually someone harvests
PropEventuallyHarvested ==
    <>(\E s \in seeds : s.status = "harvested")

\* No seed stays in growing state forever (if valid)
PropNoInfiniteGrowth ==
    [](s \in seeds : s.status = "growing" ~> s.status = "harvested" \/ s.status = "failed")

\* Economic activity eventually happens
PropEconomicLiveness ==
    <>(\E p \in players : staked[p] > 0)

============================================================
