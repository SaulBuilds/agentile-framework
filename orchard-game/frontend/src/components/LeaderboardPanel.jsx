import React, { useState, useEffect } from 'react';

function LeaderboardPanel({ seeds, federations }) {
  const [playerRankings, setPlayerRankings] = useState([]);
  const [federationRankings, setFederationRankings] = useState([]);
  const [activeTab, setActiveTab] = useState('players');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const saved = localStorage.getItem('orchardGameLeaderboard');
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        setPlayerRankings(parsed.players || []);
        setFederationRankings(parsed.federations || []);
      } catch (e) {
        console.warn('Failed to load leaderboard:', e);
      }
    }
  }, []);

  useEffect(() => {
    try {
      const harvestedSeeds = seeds.filter(s => s.state === 'HARVESTED');
      const playerScores = {};
      
      harvestedSeeds.forEach(seed => {
        const planter = seed.planter || 'player1';
        if (!playerScores[planter]) {
          playerScores[planter] = { address: planter, score: 0, harvests: 0 };
        }
        playerScores[planter].score += seed.growthScore || 0;
        playerScores[planter].harvests++;
      });
      
      const rankings = Object.values(playerScores).sort((a, b) => b.score - a.score);
      setPlayerRankings(rankings);
      
      const fedScores = {};
      harvestedSeeds.forEach(seed => {
        if (seed.federation > 0) {
          if (!fedScores[seed.federation]) {
            fedScores[seed.federation] = { id: seed.federation, score: 0, harvests: 0 };
          }
          fedScores[seed.federation].score += seed.growthScore || 0;
          fedScores[seed.federation].harvests++;
        }
      });
      
      const fedRankings = Object.values(fedScores).sort((a, b) => b.score - a.score);
      setFederationRankings(fedRankings);
      
      localStorage.setItem('orchardGameLeaderboard', JSON.stringify({
        players: rankings,
        federations: fedRankings
      }));
    } catch (e) {
      console.warn('Failed to update leaderboard:', e);
    }
  }, [seeds]);

  const getRankBadge = (index) => {
    if (index === 0) return '🥇';
    if (index === 1) return '🥈';
    if (index === 2) return '🥉';
    return `#${index + 1}`;
  };

  const getRankClass = (index) => {
    if (index === 0) return 'gold';
    if (index === 1) return 'silver';
    if (index === 2) return 'bronze';
    return '';
  };

  return (
    <div className="leaderboard-panel">
      <h2>Leaderboard</h2>
      
      <div className="leaderboard-tabs">
        <button 
          className={activeTab === 'players' ? 'active' : ''} 
          onClick={() => setActiveTab('players')}
        >
          Players
        </button>
        <button 
          className={activeTab === 'federations' ? 'active' : ''} 
          onClick={() => setActiveTab('federations')}
        >
          Federations
        </button>
      </div>

      {activeTab === 'players' && (
        <div className="leaderboard-list">
          {playerRankings.length === 0 ? (
            <p className="empty-message">No player rankings yet</p>
          ) : (
            playerRankings.map((player, index) => (
              <div key={player.address} className={`leaderboard-item ${getRankClass(index)}`}>
                <span className="rank">{getRankBadge(index)}</span>
                <span className="address">
                  {player.address === 'player1' ? 'You' : player.address.slice(0, 6) + '...'}
                </span>
                <span className="score">{player.score} pts</span>
                <span className="detail">{player.harvests} harvests</span>
              </div>
            ))
          )}
        </div>
      )}

      {activeTab === 'federations' && (
        <div className="leaderboard-list">
          {federationRankings.length === 0 ? (
            <p className="empty-message">No federation rankings yet</p>
          ) : (
            federationRankings.map((fed, index) => (
              <div key={fed.id} className={`leaderboard-item ${getRankClass(index)}`}>
                <span className="rank">{getRankBadge(index)}</span>
                <span className="address">Federation #{fed.id}</span>
                <span className="score">{fed.score} pts</span>
                <span className="detail">{fed.harvests} harvests</span>
              </div>
            ))
          )}
        </div>
      )}

      <div className="leaderboard-stats">
        <div className="stat-card">
          <h4>Total Players</h4>
          <p>{playerRankings.length}</p>
        </div>
        <div className="stat-card">
          <h4>Total Federations</h4>
          <p>{federationRankings.length}</p>
        </div>
        <div className="stat-card">
          <h4>Top Score</h4>
          <p>{playerRankings.length > 0 ? playerRankings[0].score : 0}</p>
        </div>
      </div>
    </div>
  );
}

export default LeaderboardPanel;
