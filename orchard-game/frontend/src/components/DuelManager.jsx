import React, { useState, useEffect } from 'react';

const duelABI = [
  "function initiateDuel(uint256 seedIdA, address target, uint256 seedIdB) public returns (uint256)",
  "function acceptDuel(uint256 duelId) public",
  "function rejectDuel(uint256 duelId) public",
  "function completeDuel(uint256 duelId, uint256 growthScoreA, uint256 growthScoreB) public",
  "function duelTimedOut(uint256 duelId) public",
  "function isOnCooldown(address player) public view returns (bool)",
  "function timeUntilCooldownOver(address player) public view returns (uint256)",
  "function getDuel(uint256 duelId) public view returns (uint256 seedIdA, uint256 seedIdB, address playerA, address playerB, uint256 startTime, uint256 responseDeadline, bool accepted, bool completed, uint8 result, bool growthBonusApplied)",
  "event DuelInitiated(uint256 indexed duelId, address indexed initiator, uint256 seedIdA, address indexed target, uint256 seedIdB)",
  "event DuelAccepted(uint256 indexed duelId, address indexed responder)",
  "event DuelRejected(uint256 indexed duelId, address indexed responder)",
  "event DuelCompleted(uint256 indexed duelId, uint8 result, uint256 growthBonusAmount)",
  "event DuelTimedOut(uint256 indexed duelId, address indexed initiator)",
  "event DuelCooldownActive(address indexed player, uint256 secondsRemaining)"
];

function DuelManager({ account, seeds }) {
  const [duels, setDuels] = useState([]);
  const [myDuels, setMyDuels] = useState([]);
  const [cooldowns, setCooldowns] = useState({});
  const [selectedOpponent, setSelectedOpponent] = useState('');
  const [selectedMySeed, setSelectedMySeed] = useState('');
  const [selectedOpponentSeed, setSelectedOpponentSeed] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [info, setInfo] = useState(null);
  const [activeView, setActiveView] = useState('available');

  useEffect(() => {
    const saved = localStorage.getItem('orchardGameDuels');
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        if (Array.isArray(parsed)) {
          setDuels(parsed);
        }
      } catch (e) {
        console.warn('Failed to load duels from localStorage:', e);
      }
    }

    const savedMyDuels = localStorage.getItem('orchardGameMyDuels');
    if (savedMyDuels) {
      try {
        const parsed = JSON.parse(savedMyDuels);
        if (Array.isArray(parsed)) {
          setMyDuels(parsed);
        }
      } catch (e) {
        console.warn('Failed to load my duels from localStorage:', e);
      }
    }
  }, []);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameDuels', JSON.stringify(duels));
    } catch (e) {
      console.warn('Failed to save duels to localStorage:', e);
    }
  }, [duels]);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameMyDuels', JSON.stringify(myDuels));
    } catch (e) {
      console.warn('Failed to save my duels to localStorage:', e);
    }
  }, [myDuels]);

  const mySeeds = seeds.filter(s => s.state === 'READY' || s.state === 'GROWING' || s.state === 'PLANTED');

  const handleInitiateDuel = async () => {
    setLoading(true);
    setError(null);

    if (!selectedMySeed || !selectedOpponent || !selectedOpponentSeed) {
      setError('Please select your seed, opponent, and opponent\'s seed');
      setLoading(false);
      return;
    }

    const mySeed = seeds.find(s => s.tokenId === parseInt(selectedMySeed));
    if (!mySeed) {
      setError('Selected seed not found');
      setLoading(false);
      return;
    }

    if (mySeed.checkpoint === 0) {
      setError('Your seed must advance at least one checkpoint before dueling');
      setLoading(false);
      return;
    }

    if (cooldowns[account || 'player1']) {
      setError('You are on cooldown. Wait before initiating another duel.');
      setLoading(false);
      return;
    }

    try {
      const newDuelId = duels.length;
      const newDuel = {
        id: newDuelId,
        seedIdA: parseInt(selectedMySeed),
        seedIdB: parseInt(selectedOpponentSeed),
        playerA: account || 'player1',
        playerB: selectedOpponent,
        startTime: Date.now(),
        responseDeadline: Date.now() + 60000,
        accepted: false,
        rejected: false,
        completed: false,
        result: null,
        growthBonusApplied: false
      };

      setDuels(prev => [...prev, newDuel]);
      setMyDuels(prev => [...prev, newDuelId]);
      setCooldowns(prev => ({ ...prev, [account || 'player1']: true }));

      setTimeout(() => {
        setCooldowns(prev => {
          const newCooldowns = { ...prev };
          delete newCooldowns[account || 'player1'];
          return newCooldowns;
        });
      }, 86400000);

      setInfo(`Duel #${newDuelId} initiated! Waiting for opponent response.`);
      setSelectedMySeed('');
      setSelectedOpponent('');
      setSelectedOpponentSeed('');
    } catch (err) {
      console.error('Error initiating duel:', err);
      setError('Failed to initiate duel');
    } finally {
      setLoading(false);
    }
  };

  const handleAcceptDuel = async (duelId) => {
    setLoading(true);
    setError(null);

    const duel = duels.find(d => d.id === duelId);
    if (!duel) {
      setError('Duel not found');
      setLoading(false);
      return;
    }

    if (duel.accepted || duel.rejected || duel.completed) {
      setError('Duel already responded to');
      setLoading(false);
      return;
    }

    try {
      setDuels(prev => prev.map(d => {
        if (d.id === duelId) {
          return { ...d, accepted: true };
        }
        return d;
      }));

      setInfo(`Accepted duel #${duelId}!`);
    } catch (err) {
      console.error('Error accepting duel:', err);
      setError('Failed to accept duel');
    } finally {
      setLoading(false);
    }
  };

  const handleRejectDuel = async (duelId) => {
    setLoading(true);
    setError(null);

    try {
      setDuels(prev => prev.map(d => {
        if (d.id === duelId) {
          return { ...d, rejected: true };
        }
        return d;
      }));

      setInfo(`Rejected duel #${duelId}`);
    } catch (err) {
      console.error('Error rejecting duel:', err);
      setError('Failed to reject duel');
    } finally {
      setLoading(false);
    }
  };

  const handleCompleteDuel = async (duelId) => {
    setLoading(true);
    setError(null);

    const duel = duels.find(d => d.id === duelId);
    if (!duel) {
      setError('Duel not found');
      setLoading(false);
      return;
    }

    if (!duel.accepted) {
      setError('Duel not accepted yet');
      setLoading(false);
      return;
    }

    if (duel.completed) {
      setError('Duel already completed');
      setLoading(false);
      return;
    }

    const mySeed = seeds.find(s => s.tokenId === duel.seedIdA);
    const opponentSeed = seeds.find(s => s.tokenId === duel.seedIdB);

    if (!mySeed || !opponentSeed) {
      setError('Seeds not found');
      setLoading(false);
      return;
    }

    const scoreA = mySeed.growthScore || 50;
    const scoreB = opponentSeed.growthScore || 50;

    let result;
    if (scoreA > scoreB) {
      result = 1;
    } else if (scoreB > scoreA) {
      result = 2;
    } else {
      result = 0;
    }

    const bonusAmount = Math.abs(scoreA - scoreB);

    try {
      setDuels(prev => prev.map(d => {
        if (d.id === duelId) {
          return {
            ...d,
            completed: true,
            result: result,
            growthBonusApplied: bonusAmount > 0,
            scoreA: scoreA,
            scoreB: scoreB
          };
        }
        return d;
      }));

      const resultText = result === 1 ? 'You Win!' : result === 2 ? 'You Lose!' : 'Draw!';
      setInfo(`Duel #${duelId} completed: ${resultText} Bonus: ${bonusAmount}`);
    } catch (err) {
      console.error('Error completing duel:', err);
      setError('Failed to complete duel');
    } finally {
      setLoading(false);
    }
  };

  const clearMessages = () => {
    setError(null);
    setInfo(null);
  };

  const availableDuels = duels.filter(d => 
    d.playerB === (account || 'player1') && 
    !d.accepted && 
    !d.rejected && 
    !d.completed
  );

  const myInitiatedDuels = duels.filter(d => 
    d.playerA === (account || 'player1') && 
    !d.completed
  );

  const completedDuels = duels.filter(d => d.completed);

  return (
    <div className="duel-manager">
      <h2>Pollination Duels</h2>

      {(error || info) && (
        <div className="duel-messages">
          {error && <span className="status-error">{error}</span>}
          {info && <span className="status-info">{info}</span>}
          <button onClick={clearMessages} className="clear-button">Clear</button>
        </div>
      )}

      <div className="duel-stats">
        <div className="stat-card">
          <h4>Active Duels</h4>
          <p>{duels.filter(d => !d.completed).length}</p>
        </div>
        <div className="stat-card">
          <h4>Completed Duels</h4>
          <p>{completedDuels.length}</p>
        </div>
        <div className="stat-card">
          <h4>On Cooldown</h4>
          <p>{cooldowns[account || 'player1'] ? 'Yes' : 'No'}</p>
        </div>
      </div>

      <div className="duel-actions">
        <div className="action-panel">
          <h3>Initiate Duel</h3>
          <div className="duel-form">
            <div>
              <label>
                Your Seed:
                <select 
                  value={selectedMySeed} 
                  onChange={(e) => setSelectedMySeed(e.target.value)}
                >
                  <option value="">Select seed...</option>
                  {mySeeds.filter(s => s.checkpoint > 0).map(seed => (
                    <option key={seed.tokenId} value={seed.tokenId}>
                      Seed #{seed.tokenId} (Score: {seed.growthScore || 'N/A'})
                    </option>
                  ))}
                </select>
              </label>
            </div>
            <div>
              <label>
                Opponent Address:
                <input 
                  type="text" 
                  placeholder="0x..." 
                  value={selectedOpponent}
                  onChange={(e) => setSelectedOpponent(e.target.value)}
                />
              </label>
            </div>
            <div>
              <label>
                Opponent's Seed ID:
                <input 
                  type="number" 
                  placeholder="Seed ID"
                  value={selectedOpponentSeed}
                  onChange={(e) => setSelectedOpponentSeed(e.target.value)}
                />
              </label>
            </div>
            <button 
              onClick={handleInitiateDuel}
              disabled={loading || !selectedMySeed || !selectedOpponent || !selectedOpponentSeed || cooldowns[account || 'player1']}
            >
              Challenge to Duel
            </button>
          </div>
        </div>
      </div>

      <div className="duel-tabs">
        <button 
          className={activeView === 'available' ? 'active' : ''}
          onClick={() => setActiveView('available')}
        >
          Incoming Challenges ({availableDuels.length})
        </button>
        <button 
          className={activeView === 'initiated' ? 'active' : ''}
          onClick={() => setActiveView('initiated')}
        >
          My Challenges ({myInitiatedDuels.length})
        </button>
        <button 
          className={activeView === 'completed' ? 'active' : ''}
          onClick={() => setActiveView('completed')}
        >
          Completed ({completedDuels.length})
        </button>
      </div>

      <div className="duels-list">
        {activeView === 'available' && (
          <>
            {availableDuels.length === 0 ? (
              <p className="empty-message">No pending duel challenges</p>
            ) : (
              availableDuels.map(duel => (
                <div key={duel.id} className="duel-card incoming">
                  <h4>Duel #{duel.id}</h4>
                  <p><strong>Challenger:</strong> {duel.playerA.slice(0, 6)}...{duel.playerA.slice(-4)}</p>
                  <p><strong>Their Seed:</strong> #{duel.seedIdA}</p>
                  <p><strong>Your Seed:</strong> #{duel.seedIdB}</p>
                  <div className="duel-buttons">
                    <button onClick={() => handleAcceptDuel(duel.id)} disabled={loading}>
                      Accept
                    </button>
                    <button onClick={() => handleRejectDuel(duel.id)} disabled={loading}>
                      Reject
                    </button>
                  </div>
                </div>
              ))
            )}
          </>
        )}

        {activeView === 'initiated' && (
          <>
            {myInitiatedDuels.length === 0 ? (
              <p className="empty-message">No active challenges initiated</p>
            ) : (
              myInitiatedDuels.map(duel => (
                <div key={duel.id} className="duel-card initiated">
                  <h4>Duel #{duel.id}</h4>
                  <p><strong>Opponent:</strong> {duel.playerB.slice(0, 6)}...{duel.playerB.slice(-4)}</p>
                  <p><strong>Your Seed:</strong> #{duel.seedIdA}</p>
                  <p><strong>Their Seed:</strong> #{duel.seedIdB}</p>
                  <p><strong>Status:</strong> {
                    duel.accepted ? 'Accepted - Ready to complete!' :
                    duel.rejected ? 'Rejected' :
                    'Waiting for response...'
                  }</p>
                  {duel.accepted && (
                    <button onClick={() => handleCompleteDuel(duel.id)} disabled={loading}>
                      Complete Duel
                    </button>
                  )}
                </div>
              ))
            )}
          </>
        )}

        {activeView === 'completed' && (
          <>
            {completedDuels.length === 0 ? (
              <p className="empty-message">No completed duels yet</p>
            ) : (
              completedDuels.map(duel => (
                <div key={duel.id} className={`duel-card completed ${duel.result === 1 ? 'win' : duel.result === 2 ? 'loss' : 'draw'}`}>
                  <h4>Duel #{duel.id}</h4>
                  <p><strong>vs:</strong> {duel.playerA === (account || 'player1') ? duel.playerB.slice(0, 6) + '...' + duel.playerB.slice(-4) : duel.playerA.slice(0, 6) + '...' + duel.playerA.slice(-4)}</p>
                  <p><strong>Your Score:</strong> {duel.result === 1 ? duel.scoreA : duel.scoreB}</p>
                  <p><strong>Their Score:</strong> {duel.result === 1 ? duel.scoreB : duel.scoreA}</p>
                  <p><strong>Result:</strong> {
                    duel.result === 1 ? 'You Win!' :
                    duel.result === 2 ? 'You Lose!' :
                    'Draw'
                  }</p>
                  {duel.growthBonusApplied && (
                    <p><strong>Bonus Applied:</strong> Yes!</p>
                  )}
                </div>
              ))
            )}
          </>
        )}
      </div>
    </div>
  );
}

export default DuelManager;
