import React, { useState, useEffect } from 'react';

function SeasonDisplay({ provider, account }) {
  const [currentSeason, setCurrentSeason] = useState({
    seasonId: 0,
    state: 'INACTIVE',
    startTime: 0,
    endTime: 0,
    epoch: 0,
    frontierSize: 10
  });
  const [timeRemaining, setTimeRemaining] = useState(0);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const saved = localStorage.getItem('orchardGameSeason');
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        setCurrentSeason(parsed);
      } catch (e) {
        console.warn('Failed to load season data:', e);
      }
    }

    const savedTimeRemaining = localStorage.getItem('orchardGameTimeRemaining');
    if (savedTimeRemaining) {
      setTimeRemaining(parseInt(savedTimeRemaining));
    }
  }, []);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameSeason', JSON.stringify(currentSeason));
    } catch (e) {
      console.warn('Failed to save season:', e);
    }
  }, [currentSeason]);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameTimeRemaining', timeRemaining.toString());
    } catch (e) {
      console.warn('Failed to save time remaining:', e);
    }
  }, [timeRemaining]);

  const startSeason = () => {
    setLoading(true);
    const newSeason = {
      seasonId: currentSeason.seasonId + 1,
      state: 'ACTIVE',
      startTime: Date.now(),
      endTime: Date.now() + (30 * 24 * 60 * 60 * 1000),
      epoch: 0,
      frontierSize: 10
    };
    setCurrentSeason(newSeason);
    setTimeRemaining(30 * 24 * 60 * 60);
    setLoading(false);
  };

  const advanceEpoch = () => {
    setLoading(true);
    setCurrentSeason(prev => ({
      ...prev,
      epoch: prev.epoch + 1
    }));
    setLoading(false);
  };

  const formatTimeRemaining = (seconds) => {
    const days = Math.floor(seconds / (24 * 60 * 60));
    const hours = Math.floor((seconds % (24 * 60 * 60)) / (60 * 60));
    const minutes = Math.floor((seconds % (60 * 60)) / 60);
    return `${days}d ${hours}h ${minutes}m`;
  };

  return (
    <div className="season-display">
      <h2>Season Display</h2>
      
      <div className="season-stats">
        <div className="stat-card">
          <h4>Season ID</h4>
          <p>{currentSeason.seasonId}</p>
        </div>
        <div className="stat-card">
          <h4>Status</h4>
          <p className={`status-${currentSeason.state.toLowerCase()}`}>{currentSeason.state}</p>
        </div>
        <div className="stat-card">
          <h4>Epoch</h4>
          <p>{currentSeason.epoch}</p>
        </div>
        <div className="stat-card">
          <h4>Knowledge Frontier</h4>
          <p>{currentSeason.frontierSize} concepts</p>
        </div>
      </div>

      {currentSeason.state === 'ACTIVE' && (
        <div className="season-timer">
          <h3>Time Remaining</h3>
          <p className="timer">{formatTimeRemaining(timeRemaining)}</p>
        </div>
      )}

      {currentSeason.state === 'INACTIVE' && (
        <div className="season-actions">
          <button onClick={startSeason} disabled={loading}>
            Start New Season
          </button>
        </div>
      )}

      {currentSeason.state === 'ACTIVE' && (
        <div className="season-actions">
          <button onClick={advanceEpoch} disabled={loading}>
            Advance Epoch
          </button>
        </div>
      )}

      <div className="season-progress">
        <h3>Season Progress</h3>
        {currentSeason.state === 'ACTIVE' ? (
          <div className="progress-bar">
            <div 
              className="progress-fill" 
              style={{ width: `${Math.min(100, ((Date.now() - currentSeason.startTime) / (currentSeason.endTime - currentSeason.startTime)) * 100)}%` }}
            ></div>
          </div>
        ) : (
          <p className="no-season">No active season</p>
        )}
      </div>
    </div>
  );
}

export default SeasonDisplay;
