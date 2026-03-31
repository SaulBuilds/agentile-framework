import React, { useState, useEffect } from 'react';

const federationABI = [
  "function createFederation(uint256 minStake) public returns (uint256)",
  "function joinFederation(uint256 federationId) public",
  "function leaveFederation(uint256 federationId) public",
  "function stakeSeed(uint256 federationId, uint256 tokenId, uint256 amount) public",
  "function unstakeSeed(uint256 federationId, uint256 tokenId, uint256 amount) public",
  "function addReward(uint256 federationId, uint256 amount) public",
  "function distributeRewards(uint256 federationId) public",
  "function updateTotalScore(uint256 federationId, uint256 scoreToAdd) public",
  "function federationCreator(uint256 federationId) public view returns (address)",
  "function federationMinStake(uint256 federationId) public view returns (uint256)",
  "function federationRewardPool(uint256 federationId) public view returns (uint256)",
  "function federationTotalScore(uint256 federationId) public view returns (uint256)",
  "function isMember(uint256 federationId, address player) public view returns (bool)",
  "function memberStake(uint256 federationId, address player) public view returns (uint256)",
  "function getTotalScore(uint256 federationId) public view returns (uint256)",
  "event FederationCreated(uint256 indexed federationId, address indexed creator, uint256 minStake)",
  "event PlayerJoinedFederation(uint256 indexed federationId, address indexed player)",
  "event PlayerLeftFederation(uint256 indexed federationId, address indexed player)",
  "event SeedStaked(uint256 indexed federationId, address indexed player, uint256 indexed tokenId, uint256 amount)",
  "event SeedUnstaked(uint256 indexed federationId, address indexed player, uint256 indexed tokenId, uint256 amount)",
  "event RewardAdded(uint256 indexed federationId, uint256 amount)",
  "event RewardDistributed(uint256 indexed federationId, uint256 amount)"
];

const ortTokenABI = [
  "function stake(uint256 amount) public",
  "function unstake(uint256 amount) public",
  "function stakeSeedInFederation(uint256 federationId, uint256 tokenId, uint256 amount) public",
  "function unstakeSeedFromFederation(uint256 federationId, uint256 tokenId, uint256 amount) public",
  "function balanceOf(address account) public view returns (uint256)",
  "function stakedBalance() public view returns (uint256)",
  "event Staked(address indexed player, uint256 amount)",
  "event Unstaked(address indexed player, uint256 amount)",
  "event SeedStakedInFederation(address indexed player, uint256 federationId, uint256 tokenId, uint256 amount)",
  "event SeedUnstakedFromFederation(address indexed player, uint256 federationId, uint256 tokenId, uint256 amount)"
];

function FederationManager({ provider, account, seeds }) {
  const [federations, setFederations] = useState([]);
  const [selectedFederation, setSelectedFederation] = useState(null);
  const [myFederations, setMyFederations] = useState([]);
  const [balance, setBalance] = useState(10000);
  const [stakedBalance, setStakedBalance] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [info, setInfo] = useState(null);
  const [federationContract, setFederationContract] = useState({});
  const [ortTokenContract, setOrtTokenContract] = useState({});

  useEffect(() => {
    const saved = localStorage.getItem('orchardGameFederations');
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        if (Array.isArray(parsed)) {
          setFederations(parsed);
        }
      } catch (e) {
        console.warn('Failed to load federations from localStorage:', e);
      }
    }

    const savedMyFed = localStorage.getItem('orchardGameMyFederations');
    if (savedMyFed) {
      try {
        const parsed = JSON.parse(savedMyFed);
        if (Array.isArray(parsed)) {
          setMyFederations(parsed);
        }
      } catch (e) {
        console.warn('Failed to load my federations from localStorage:', e);
      }
    }

    const savedBalance = localStorage.getItem('orchardGameORTBalance');
    if (savedBalance) {
      setBalance(parseInt(savedBalance));
    }

    const savedStaked = localStorage.getItem('orchardGameStakedBalance');
    if (savedStaked) {
      setStakedBalance(parseInt(savedStaked));
    }
  }, []);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameFederations', JSON.stringify(federations));
    } catch (e) {
      console.warn('Failed to save federations to localStorage:', e);
    }
  }, [federations]);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameMyFederations', JSON.stringify(myFederations));
    } catch (e) {
      console.warn('Failed to save my federations to localStorage:', e);
    }
  }, [myFederations]);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameORTBalance', balance.toString());
    } catch (e) {
      console.warn('Failed to save ORT balance to localStorage:', e);
    }
  }, [balance]);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameStakedBalance', stakedBalance.toString());
    } catch (e) {
      console.warn('Failed to save staked balance to localStorage:', e);
    }
  }, [stakedBalance]);

  const handleCreateFederation = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    const formData = new FormData(e.target);
    const minStake = parseInt(formData.get('minStake'));

    if (minStake < 100) {
      setError('Minimum stake must be at least 100 ORT');
      setLoading(false);
      return;
    }

    if (balance < minStake) {
      setError('Insufficient ORT balance to create federation');
      setLoading(false);
      return;
    }

    try {
      const newFedId = federations.length;
      const newFederation = {
        id: newFedId,
        creator: account || 'player1',
        minStake: minStake,
        rewardPool: 0,
        totalScore: 0,
        members: [{ address: account || 'player1', stake: minStake, stakedSeeds: [] }],
        createdAt: Date.now()
      };

      setFederations(prev => [...prev, newFederation]);
      setMyFederations(prev => [...prev, newFedId]);
      setBalance(prev => prev - minStake);
      setStakedBalance(prev => prev + minStake);

      e.target.reset();
      setInfo(`Federation #${newFedId} created successfully!`);
    } catch (err) {
      console.error('Error creating federation:', err);
      setError('Failed to create federation');
    } finally {
      setLoading(false);
    }
  };

  const handleJoinFederation = async (federationId) => {
    setLoading(true);
    setError(null);

    const federation = federations.find(f => f.id === federationId);
    if (!federation) {
      setError('Federation not found');
      setLoading(false);
      return;
    }

    if (myFederations.includes(federationId)) {
      setError('Already a member of this federation');
      setLoading(false);
      return;
    }

    if (balance < federation.minStake) {
      setError(`Insufficient ORT balance. Need at least ${federation.minStake} ORT to join.`);
      setLoading(false);
      return;
    }

    try {
      setFederations(prev => prev.map(fed => {
        if (fed.id === federationId) {
          return {
            ...fed,
            members: [...fed.members, { address: account || 'player1', stake: federation.minStake, stakedSeeds: [] }]
          };
        }
        return fed;
      }));
      setMyFederations(prev => [...prev, federationId]);
      setBalance(prev => prev - federation.minStake);
      setStakedBalance(prev => prev + federation.minStake);

      setInfo(`Joined Federation #${federationId} successfully!`);
    } catch (err) {
      console.error('Error joining federation:', err);
      setError('Failed to join federation');
    } finally {
      setLoading(false);
    }
  };

  const handleLeaveFederation = async (federationId) => {
    setLoading(true);
    setError(null);

    const federation = federations.find(f => f.id === federationId);
    if (!federation) {
      setError('Federation not found');
      setLoading(false);
      return;
    }

    const myMember = federation.members.find(m => m.address === (account || 'player1'));
    if (myMember && myMember.stakedSeeds && myMember.stakedSeeds.length > 0) {
      setError('Cannot leave federation with staked seeds. Unstake all seeds first.');
      setLoading(false);
      return;
    }

    try {
      setFederations(prev => prev.map(fed => {
        if (fed.id === federationId) {
          return {
            ...fed,
            members: fed.members.filter(m => m.address !== (account || 'player1'))
          };
        }
        return fed;
      }));
      setMyFederations(prev => prev.filter(id => id !== federationId));
      setBalance(prev => prev + federation.minStake);
      setStakedBalance(prev => prev - federation.minStake);

      setInfo(`Left Federation #${federationId}`);
    } catch (err) {
      console.error('Error leaving federation:', err);
      setError('Failed to leave federation');
    } finally {
      setLoading(false);
    }
  };

  const handleStakeSeed = async (federationId, tokenId, amount) => {
    setLoading(true);
    setError(null);

    const federation = federations.find(f => f.id === federationId);
    if (!federation) {
      setError('Federation not found');
      setLoading(false);
      return;
    }

    const seed = seeds.find(s => s.tokenId === tokenId);
    if (!seed) {
      setError('Seed not found');
      setLoading(false);
      return;
    }

    if (amount < federation.minStake) {
      setError(`Must stake at least ${federation.minStake} ORT`);
      setLoading(false);
      return;
    }

    try {
      setFederations(prev => prev.map(fed => {
        if (fed.id === federationId) {
          return {
            ...fed,
            members: fed.members.map(m => {
              if (m.address === (account || 'player1')) {
                return {
                  ...m,
                  stake: m.stake + amount,
                  stakedSeeds: [...(m.stakedSeeds || []), { tokenId, amount }]
                };
              }
              return m;
            })
          };
        }
        return fed;
      }));

      setInfo(`Staked seed #${tokenId} in Federation #${federationId} for ${amount} ORT`);
    } catch (err) {
      console.error('Error staking seed:', err);
      setError('Failed to stake seed');
    } finally {
      setLoading(false);
    }
  };

  const handleUnstakeSeed = async (federationId, tokenId, amount) => {
    setLoading(true);
    setError(null);

    try {
      setFederations(prev => prev.map(fed => {
        if (fed.id === federationId) {
          return {
            ...fed,
            members: fed.members.map(m => {
              if (m.address === (account || 'player1')) {
                return {
                  ...m,
                  stake: Math.max(0, m.stake - amount),
                  stakedSeeds: (m.stakedSeeds || []).filter(s => s.tokenId !== tokenId)
                };
              }
              return m;
            })
          };
        }
        return fed;
      }));

      setInfo(`Unstaked seed #${tokenId} from Federation #${federationId}`);
    } catch (err) {
      console.error('Error unstaking seed:', err);
      setError('Failed to unstake seed');
    } finally {
      setLoading(false);
    }
  };

  const handleAddReward = async (federationId, amount) => {
    setLoading(true);
    setError(null);

    const federation = federations.find(f => f.id === federationId);
    if (!federation) {
      setError('Federation not found');
      setLoading(false);
      return;
    }

    if (federation.creator !== (account || 'player1')) {
      setError('Only federation creator can add rewards');
      setLoading(false);
      return;
    }

    if (balance < amount) {
      setError('Insufficient ORT balance');
      setLoading(false);
      return;
    }

    try {
      setFederations(prev => prev.map(fed => {
        if (fed.id === federationId) {
          return { ...fed, rewardPool: fed.rewardPool + amount };
        }
        return fed;
      }));
      setBalance(prev => prev - amount);

      setInfo(`Added ${amount} ORT to Federation #${federationId} reward pool`);
    } catch (err) {
      console.error('Error adding reward:', err);
      setError('Failed to add reward');
    } finally {
      setLoading(false);
    }
  };

  const handleDistributeRewards = async (federationId) => {
    setLoading(true);
    setError(null);

    const federation = federations.find(f => f.id === federationId);
    if (!federation) {
      setError('Federation not found');
      setLoading(false);
      return;
    }

    if (federation.creator !== (account || 'player1')) {
      setError('Only federation creator can distribute rewards');
      setLoading(false);
      return;
    }

    if (federation.rewardPool === 0) {
      setError('No rewards to distribute');
      setLoading(false);
      return;
    }

    try {
      const memberCount = federation.members.length;
      if (memberCount === 0) {
        setError('No members to distribute rewards to');
        setLoading(false);
        return;
      }

      const rewardPerMember = Math.floor(federation.rewardPool / memberCount);

      setFederations(prev => prev.map(fed => {
        if (fed.id === federationId) {
          return { ...fed, rewardPool: 0 };
        }
        return fed;
      }));
      setBalance(prev => prev + (rewardPerMember * memberCount));

      setInfo(`Distributed ${rewardPerMember} ORT to each of ${memberCount} members`);
    } catch (err) {
      console.error('Error distributing rewards:', err);
      setError('Failed to distribute rewards');
    } finally {
      setLoading(false);
    }
  };

  const clearMessages = () => {
    setError(null);
    setInfo(null);
  };

  return (
    <div className="federation-manager">
      <h2>Federation Management</h2>

      {(error || info) && (
        <div className="federation-messages">
          {error && <span className="status-error">{error}</span>}
          {info && <span className="status-info">{info}</span>}
          <button onClick={clearMessages} className="clear-button">Clear</button>
        </div>
      )}

      <div className="federation-stats">
        <div className="stat-card">
          <h4>ORT Balance</h4>
          <p>{balance.toLocaleString()} ORT</p>
        </div>
        <div className="stat-card">
          <h4>Staked ORT</h4>
          <p>{stakedBalance.toLocaleString()} ORT</p>
        </div>
        <div className="stat-card">
          <h4>My Federations</h4>
          <p>{myFederations.length}</p>
        </div>
        <div className="stat-card">
          <h4>Total Federations</h4>
          <p>{federations.length}</p>
        </div>
      </div>

      <div className="federation-actions">
        <div className="action-panel">
          <h3>Create Federation</h3>
          <form onSubmit={handleCreateFederation}>
            <div>
              <label>
                Minimum Stake (ORT):
                <input type="number" name="minStake" min="100" defaultValue="100" required />
              </label>
            </div>
            <button type="submit" disabled={loading}>
              Create Federation
            </button>
          </form>
        </div>
      </div>

      <div className="federations-list">
        <h3>Available Federations</h3>
        {federations.length === 0 ? (
          <p className="empty-message">No federations exist yet. Create one above!</p>
        ) : (
          <div className="federation-grid">
            {federations.map(fed => (
              <div key={fed.id} className={`federation-card ${myFederations.includes(fed.id) ? 'my-federation' : ''}`}>
                <h4>Federation #{fed.id}</h4>
                <p><strong>Creator:</strong> {fed.creator === (account || 'player1') ? 'You' : fed.creator.slice(0, 6) + '...' + fed.creator.slice(-4)}</p>
                <p><strong>Min Stake:</strong> {fed.minStake} ORT</p>
                <p><strong>Members:</strong> {fed.members.length}</p>
                <p><strong>Reward Pool:</strong> {fed.rewardPool} ORT</p>
                <p><strong>Total Score:</strong> {fed.totalScore}</p>

                <div className="federation-buttons">
                  {!myFederations.includes(fed.id) ? (
                    <button 
                      onClick={() => handleJoinFederation(fed.id)}
                      disabled={loading || balance < fed.minStake}
                    >
                      Join Federation
                    </button>
                  ) : (
                    <>
                      {fed.creator === (account || 'player1') && (
                        <>
                          <button 
                            onClick={() => {
                              const amount = prompt('Enter reward amount:');
                              if (amount) handleAddReward(fed.id, parseInt(amount));
                            }}
                            disabled={loading}
                          >
                            Add Reward
                          </button>
                          <button 
                            onClick={() => handleDistributeRewards(fed.id)}
                            disabled={loading || fed.rewardPool === 0}
                          >
                            Distribute Rewards
                          </button>
                        </>
                      )}
                      <button 
                        onClick={() => handleLeaveFederation(fed.id)}
                        disabled={loading}
                      >
                        Leave Federation
                      </button>
                    </>
                  )}
                </div>

                {myFederations.includes(fed.id) && (
                  <div className="my-member-info">
                    <h5>Your Membership</h5>
                    {(() => {
                      const myMember = fed.members.find(m => m.address === (account || 'player1'));
                      return myMember ? (
                        <>
                          <p><strong>Your Stake:</strong> {myMember.stake} ORT</p>
                          <p><strong>Staked Seeds:</strong> {myMember.stakedSeeds?.length || 0}</p>
                          {(myMember.stakedSeeds || []).length > 0 && (
                            <div className="staked-seeds">
                              {myMember.stakedSeeds.map((ss, idx) => (
                                <div key={idx} className="staked-seed-item">
                                  <span>Seed #{ss.tokenId}: {ss.amount} ORT</span>
                                  <button
                                    onClick={() => handleUnstakeSeed(fed.id, ss.tokenId, ss.amount)}
                                    disabled={loading}
                                  >
                                    Unstake
                                  </button>
                                </div>
                              ))}
                            </div>
                          )}
                        </>
                      ) : null;
                    })()}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

export default FederationManager;
