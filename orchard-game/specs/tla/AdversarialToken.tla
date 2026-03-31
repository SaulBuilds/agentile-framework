------------------- MODULE AdversarialToken -------------------

\* Adversarial specifications for ORTToken contract
\* Tests attack vectors against the token economic subsystem

EXTENDS Naturals, Sequences, FiniteSets

CONSTANTS 
    MaxHolders,
    MaxSupply,
    MaxTransaction

VARIABLES
    balances,
    staked,
    allowances,
    attacked

\* Initialize state
AdversarialInit ==
    /\ balances = [a \in Accounts |-> 0]
    /\ staked = [a \in Accounts |-> 0]
    /\ allowances = [a1 \in Accounts, a2 \in Accounts |-> 0]
    /\ attacked = FALSE

\* Flash loan attack: borrow large amount, manipulate price, repay
FlashLoanAttack ==
    /\ ~attacked
    /\ \E attacker \in Accounts :
        /\ balances[attacker] < MaxSupply
        /\ \E target \in Accounts :
            /\ staked[target] > 0
            /\ balances' = [balances EXCEPT ![attacker] = balances[attacker] + MaxSupply]
    /\ attacked' = TRUE
    /\ UNCHANGED <<staked, allowances>>

\* Balance manipulation: attacker inflates balance
BalanceManipulationAttack ==
    /\ attacked
    /\ \E a \in Accounts :
        /\ balances[a] < MaxSupply
        /\ balances' = [balances EXCEPT ![a] = MaxSupply]
    /\ UNCHANGED <<staked, allowances, attacked>>

\* Double spending: try to transfer same tokens twice
DoubleSpendAttack ==
    /\ \E a1, a2, a3 \in Accounts :
        /\ balances[a1] >= Amount
        /\ a1 # a2
        /\ a1 # a3
        /\ balances' = [balances EXCEPT 
            ![a1] = balances[a1] - Amount,
            ![a2] = balances[a2] + Amount]
    /\ UNCHANGED <<staked, allowances, attacked>>

\* Approval manipulation: attacker increases own allowance
ApprovalManipulationAttack ==
    /\ \E a \in Accounts :
        /\ allowances[a][a] < MaxTransaction
        /\ allowances' = [allowances EXCEPT ![a][a] = MaxTransaction]
    /\ UNCHANGED <<balances, staked, attacked>>

\* Staking drain: drain staked balance
StakingDrainAttack ==
    /\ \E a \in Accounts :
        /\ staked[a] > 0
        /\ staked' = [staked EXCEPT ![a] = 0]
        /\ balances' = [balances EXCEPT ![a] = balances[a] + staked[a]]
    /\ UNCHANGED <<allowances, attacked>>

\* Invariant: All balances are non-negative
NonNegativeBalances ==
    \A a \in Accounts : balances[a] >= 0

\* Invariant: Total supply is conserved
SupplyConservation ==
    Sum(balances) + Sum(staked) = MaxSupply

\* Invariant: Staked amounts are non-negative
NonNegativeStaked ==
    \A a \in Accounts : staked[a] >= 0

\* Invariant: Allowances are non-negative
NonNegativeAllowances ==
    \A a1, a2 \in Accounts : allowances[a1][a2] >= 0

\* Invariant: No account can have more than total supply
TotalSupplyBound ==
    \A a \in Accounts : balances[a] + staked[a] <= MaxSupply

\* Property: Transfer reduces sender balance
TransferReducesSender ==
    /\ A a1, a2 \in Accounts :
        balances'[a1] <= balances[a1]

============================================================
