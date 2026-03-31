import React, { useState, useEffect } from 'react';

function MysteryBoxUI({ account }) {
  const [boxes, setBoxes] = useState([]);
  const [availableBoxes, setAvailableBoxes] = useState([
    { id: 0, name: 'Bronze Box', minValue: 10, maxValue: 100, probability: 5000, available: 100 },
    { id: 1, name: 'Silver Box', minValue: 100, maxValue: 500, probability: 3000, available: 50 },
    { id: 2, name: 'Gold Box', minValue: 500, maxValue: 2000, probability: 1500, available: 20 },
    { id: 3, name: 'Platinum Box', minValue: 2000, maxValue: 10000, probability: 500, available: 5 }
  ]);
  const [myBoxes, setMyBoxes] = useState([]);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState(null);

  useEffect(() => {
    const saved = localStorage.getItem('orchardGameBoxes');
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        setMyBoxes(parsed);
      } catch (e) {
        console.warn('Failed to load boxes:', e);
      }
    }
  }, []);

  useEffect(() => {
    try {
      localStorage.setItem('orchardGameBoxes', JSON.stringify(myBoxes));
    } catch (e) {
      console.warn('Failed to save boxes:', e);
    }
  }, [myBoxes]);

  const claimDailyBox = () => {
    setLoading(true);
    
    const boxTypes = [
      { id: 0, name: 'Bronze Box', minValue: 10, maxValue: 100 },
      { id: 1, name: 'Silver Box', minValue: 100, maxValue: 500 },
      { id: 2, name: 'Gold Box', minValue: 500, maxValue: 2000 },
      { id: 3, name: 'Platinum Box', minValue: 2000, maxValue: 10000 }
    ];
    
    const roll = Math.random() * 10000;
    let selectedBox;
    if (roll < 5000) selectedBox = boxTypes[0];
    else if (roll < 8000) selectedBox = boxTypes[1];
    else if (roll < 9500) selectedBox = boxTypes[2];
    else selectedBox = boxTypes[3];
    
    const reward = selectedBox.minValue + Math.floor(Math.random() * (selectedBox.maxValue - selectedBox.minValue));
    
    const newBox = {
      id: Date.now(),
      typeId: selectedBox.id,
      name: selectedBox.name,
      reward: reward,
      awardedAt: Date.now(),
      status: 'CLAIMED'
    };
    
    setMyBoxes(prev => [...prev, newBox]);
    setMessage(`You received a ${selectedBox.name} with ${reward} ORT!`);
    setLoading(false);
  };

  const openBox = (boxId) => {
    setLoading(true);
    
    setMyBoxes(prev => prev.map(box => {
      if (box.id === boxId) {
        return { ...box, status: 'OPENED', openedAt: Date.now() };
      }
      return box;
    }));
    
    const box = myBoxes.find(b => b.id === boxId);
    setMessage(`You opened ${box.name} and received ${box.reward} ORT!`);
    setLoading(false);
  };

  const claimReward = (boxId) => {
    setLoading(true);
    
    const box = myBoxes.find(b => b.id === boxId);
    setMessage(`You claimed ${box.reward} ORT from ${box.name}!`);
    
    setMyBoxes(prev => prev.filter(b => b.id !== boxId));
    setLoading(false);
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'CLAIMED': return 'status-claimed';
      case 'OPENED': return 'status-opened';
      default: return '';
    }
  };

  const claimedToday = () => {
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    return myBoxes.some(box => box.awardedAt >= today.getTime());
  };

  return (
    <div className="mystery-box-ui">
      <h2>Mystery Boxes</h2>
      
      {message && (
        <div className="box-message">
          <p>{message}</p>
          <button onClick={() => setMessage(null)}>×</button>
        </div>
      )}

      <div className="daily-box">
        <h3>Daily Free Box</h3>
        <p>Claim a free mystery box once per day!</p>
        <button 
          onClick={claimDailyBox} 
          disabled={loading || claimedToday()}
          className="daily-button"
        >
          {claimedToday() ? 'Claimed Today!' : 'Claim Free Box'}
        </button>
      </div>

      <div className="box-types">
        <h3>Available Box Types</h3>
        <div className="box-grid">
          {availableBoxes.map(box => (
            <div key={box.id} className={`box-type type-${box.id}`}>
              <h4>{box.name}</h4>
              <p className="range">{box.minValue} - {box.maxValue} ORT</p>
              <p className="chance">{(box.probability / 100).toFixed(1)}% max value</p>
              <p className="available">{box.available} remaining</p>
            </div>
          ))}
        </div>
      </div>

      <div className="my-boxes">
        <h3>My Boxes</h3>
        {myBoxes.length === 0 ? (
          <p className="empty-message">No boxes yet. Claim your daily box!</p>
        ) : (
          <div className="boxes-list">
            {myBoxes.map(box => (
              <div key={box.id} className={`box-item ${getStatusColor(box.status)}`}>
                <h4>{box.name}</h4>
                <p className="reward">Reward: {box.reward} ORT</p>
                <p className="date">
                  Awarded: {new Date(box.awardedAt).toLocaleDateString()}
                </p>
                <p className="status">Status: {box.status}</p>
                <div className="box-actions">
                  {box.status === 'CLAIMED' && (
                    <button onClick={() => openBox(box.id)} disabled={loading}>
                      Open Box
                    </button>
                  )}
                  {box.status === 'OPENED' && (
                    <button onClick={() => claimReward(box.id)} disabled={loading}>
                      Claim {box.reward} ORT
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="box-stats">
        <div className="stat-card">
          <h4>Total Boxes</h4>
          <p>{myBoxes.length}</p>
        </div>
        <div className="stat-card">
          <h4>Claimed Today</h4>
          <p>{claimedToday() ? 'Yes' : 'No'}</p>
        </div>
        <div className="stat-card">
          <h4>Total Value</h4>
          <p>{myBoxes.reduce((sum, b) => sum + b.reward, 0)} ORT</p>
        </div>
      </div>
    </div>
  );
}

export default MysteryBoxUI;
