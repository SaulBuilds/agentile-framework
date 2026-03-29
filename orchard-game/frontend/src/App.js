import React, { useState, useEffect } from 'react';
import { ethers } from 'ethers';
import GardenRenderer from './components/GardenRenderer';
import SeedNFT from '../contracts/SeedNFT.sol';
import GrowthEngine from '../contracts/GrowthEngine.sol';
import './index.css';

// ABI for SeedNFT (simplified for the functions we need)
const seedNFTABI = [
  "function plantSeed(string _payload, uint256 _stake, uint256 _federation, uint256 _maxCheckpoints) public returns (uint256)",
  "function advanceCheckpoint(uint256 tokenId) public",
  "function harvestSeed(uint256 tokenId, uint256 _growthScore) public",
  "function failSeed(uint256 tokenId, string memory _reason) public",
  "function planterOf(uint256 tokenId) public view returns (address)",
  "function stakeOf(uint256 tokenId) public view returns (uint256)",
  "function federationOf(uint256 tokenId) public view returns (uint256)",
  "function checkpointOf(uint256 tokenId) public view returns (uint256)",
  "function maxCheckpointOf(uint256 tokenId) public view returns (uint256)",
  "function growthScoreOf(uint256 tokenId) public view returns (uint256)",
  "function isHarvested(uint256 tokenId) public view returns (bool)",
  "function isFailed(uint256 tokenId) public view returns (bool)",
  "event SeedPlanted(uint256 indexed tokenId, address indexed planter, uint256 stake, uint256 federation, uint256 maxCheckpoints)",
  "event SeedAdvanced(uint256 indexed tokenId, uint256 newCheckpoint)",
  "event SeedHarvested(uint256 indexed tokenId, uint256 growthScore)",
  "event SeedFailed(uint256 indexed tokenId, string reason)"
];

// ABI for GrowthEngine (simplified)
const growthEngineABI = [
  "function processValidation(uint256 tokenId, uint256 checkpoint) public",
  "event ValidationProcessed(uint256 indexed tokenId, uint256 checkpoint, uint8 belnapState, uint256 agreementPercentage)",
  "event GrowthScoreCalculated(uint256 indexed tokenId, uint256 growthScore)"
];

function App() {
  const [provider, setProvider] = useState(null);
  const [seedNFTContract, setSeedNFTContract] = useState(null);
  const [growthEngineContract, setGrowthEngineContract] = useState(null);
  const [account, setAccount] = useState(null);
  const [seeds, setSeeds] = useState([]);
  const [nextTokenId, setNextTokenId] = useState(0);
  const [isCheckpointAdvancing, setIsCheckpointAdvancing] = useState(false);
  const [checkpointInterval, setCheckpointInterval] = useState(null);

  // Connect to Ethereum provider and contracts
  useEffect(() => {
    async function loadWeb3() {
      if (window.ethereum) {
        try {
          // Request account access if needed
          await window.ethereum.request({ method: 'eth_requestAccounts' });
          const provider = new ethers.BrowserProvider(window.ethereum);
          const signer = await provider.getSigner();
          const account = await signer.getAddress();
          setAccount(account);
          setProvider(provider);

          // Get contract addresses from deployment (in a real app, these would be from deployment scripts or environment variables)
          // For now, we'll assume they are deployed at known addresses (we'll use placeholders)
          // In a real setup, we would deploy the contracts in this test or use the addresses from the deployment.
          // Since we are in a local development environment, we can deploy the contracts here or use the ones from the previous deployment.
          // For simplicity, we'll use the deployment from the test network (if any) or we'll deploy mock contracts.
          // Given the complexity, we'll skip the actual contract interaction for now and simulate the state locally.
          // We'll set the contracts to null and rely on local state for the demo.
          // In a real implementation, we would uncomment the following and set the addresses.

          // const seedNFTAddress = "0xYourSeedNFTAddress";
          // const growthEngineAddress = "0xYourGrowthEngineAddress";
          // const seedNFTContract = new ethers.Contract(seedNFTAddress, seedNFTABI, signer);
          // const growthEngineContract = new ethers.Contract(growthEngineAddress, growthEngineABI, signer);
          // setSeedNFTContract(seedNFTContract);
          // setGrowthEngineContract(growthEngineContract);

          // For now, we'll simulate having the contracts by setting them to a mock object or just using local state.
          // We'll set a flag to indicate we are in simulation mode.
          setSeedNFTContract({}); // placeholder
          setGrowthEngineContract({}); // placeholder
        } catch (error) {
          console.error("Failed to connect to Ethereum:", error);
        }
      } else {
        console.log("No Ethereum browser extension detected, using local simulation.");
        setSeedNFTContract({}); // placeholder for simulation
        setGrowthEngineContract({}); // placeholder for simulation
      }
    }

    loadWeb3();
  }, []);

  // Load seeds from blockchain (or local state for simulation)
  useEffect(() => {
    if (provider) {
      // In a real implementation, we would listen for SeedPlanted events and update the seeds array.
      // For now, we'll simulate by setting the seeds array from local state or from a mock.
      // We'll use the nextTokenId to know how many seeds we have.
      // We'll fetch each seed's details from the contract.
      // Since we are simulating, we'll just use the seeds array we have in state.
      // We'll leave this empty for now and rely on the form to add seeds.
    }
  }, [provider, seedNFTContract]);

  // Simulate checkpoint advancement every 5 seconds (for demo purposes)
  useEffect(() => {
    if (isCheckpointAdvancing && !checkpointInterval) {
      const interval = setInterval(() => {
        // Advance the checkpoint for all seeds that are not mature and not harvested/failed
        setSeeds(prevSeeds => {
          return prevSeeds.map(seed => {
            if (seed.state === 'PLANTED' || seed.state === 'GROWING') {
              const newCheckpoint = Math.min(seed.checkpoint + 1, seed.maxCheckpoint);
              let newState = seed.state;
              if (newCheckpoint === seed.maxCheckpoint) {
                newState = 'READY'; // In a real game, we would wait for validation
              }
              return { ...seed, checkpoint: newCheckpoint, state: newState };
            }
            return seed;
          });
        });
      }, 5000); // 5 seconds
      setCheckpointInterval(interval);
      return () => clearInterval(interval);
    }
  }, [isCheckpointAdvancing]);

  // Handle planting a new seed
  const handlePlantSeed = async (e) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const payload = formData.get('payload');
    const stake = parseInt(formData.get('stake'));
    const federation = parseInt(formData.get('federation'));
    const maxCheckpoints = parseInt(formData.get('maxCheckpoints'));

    if (!payload || stake < 10 || isNaN(stake) || federation < 1 || isNaN(federation) || maxCheckpoints < 1 || isNaN(maxCheckpoints) || maxCheckpoints > 1000) {
      alert('Please fill in all fields correctly: payload (non-empty), stake (>=10 ORT), federation (>=1), maxCheckpoints (1-1000)');
      return;
    }

    // In a real implementation, we would call the seedNFTContract.plantSeed function
    // For now, we'll simulate by adding a seed to the local state
    const newSeed = {
      tokenId: nextTokenId,
      payload: payload,
      stake: stake,
      federation: federation,
      checkpoint: 0,
      maxCheckpoint: maxCheckpoints,
      state: 'PLANTED',
      growthScore: 0,
      isHarvested: false,
      isFailed: false
    };

    setSeeds(prevSeeds => [...prevSeeds, newSeed]);
    setNextTokenId(prev => prev + 1);

    // Reset the form
    e.target.reset();
  };

  // Handle harvesting a seed
  const handleHarvestSeed = (tokenId) => {
    // In a real implementation, we would call the seedNFTContract.harvestSeed function with a growth score
    // For now, we'll simulate by setting the seed's state to HARVESTED and setting a random growth score
    // In a real game, we would call the GrowthEngine to process validation and then get the growth score
    setSeeds(prevSeeds => {
      return prevSeeds.map(seed => {
        if (seed.tokenId === tokenId && seed.state === 'READY') {
          // Simulate a growth score based on some validation (for demo, we'll use a random score between 50 and 100)
          const growthScore = Math.floor(Math.random() * 50) + 50;
          return { ...seed, state: 'HARVESTED', growthScore: growthScore };
        }
        return seed;
      });
    });
  };

  // Handle failing a seed
  const handleFailSeed = (tokenId, reason) => {
    setSeeds(prevSeeds => {
      return prevSeeds.map(seed => {
        if (seed.tokenId === tokenId && (seed.state === 'PLANTED' || seed.state === 'GROWING')) {
          return { ...seed, state: 'FAILED' };
        }
        return seed;
      });
    });
  };

  // Start advancing checkpoints
  const startCheckpointAdvance = () => {
    setIsCheckpointAdvancing(true);
  };

  // Stop advancing checkpoints
  const stopCheckpointAdvance = () => {
    setIsCheckpointAdvancing(false);
    if (checkpointInterval) {
      clearInterval(checkpointInterval);
      setCheckpointInterval(null);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Orchard Game - Solo Mode</h1>
      </header>
      <main>
        <div className="game-controls">
          <div className="seed-planting-form">
            <h2>Plant a New Seed</h2>
            <form onSubmit={handlePlantSeed}>
              <div>
                <label>
                  Payload (seed content):
                  <input type="text" name="payload" required />
                </label>
              </div>
              <div>
                <label>
                  Stake (ORT, min 10):
                  <input type="number" name="stake" min="10" required />
                </label>
              </div>
              <div>
                <label>
                  Federation ID:
                  <input type="number" name="federation" min="1" required />
                </label>
              </div>
              <div>
                <label>
                  Max Checkpoints:
                  <input type="number" name="maxCheckpoints" min="1" max="1000" value="5" required />
                </label>
              </div>
              <button type="submit">Plant Seed</button>
            </form>
          </div>
          <div className="game-controls">
            <button onClick={startCheckpointAdvance} disabled={isCheckpointAdvancing}>
              Start Growth
            </button>
            <button onClick={stopCheckpointAdvance} disabled={!isCheckpointAdvancing}>
              Stop Growth
            </button>
          </div>
        </div>

        <div className="game-display">
          <h2>Garden ({seeds.length} seeds)</h2>
          <div className="garden-container">
            <GardenRenderer seeds={seeds} />
          </div>
          {seeds.length > 0 && (
            <div className="seed-actions">
              <h3>Seed Actions</h3>
              {seeds.map(seed => (
                <div key={seed.tokenId} className="seed-card">
                  <h4>Seed #{seed.tokenId}</h4>
                  <p>State: {seed.state}</p>
                  <p>Checkpoint: {seed.checkpoint}/{seed.maxCheckpoint}</p>
                  <p>Stake: {seed.stake} ORT</p>
                  <p>Federation: {seed.federation}</p>
                  {seed.state === 'READY' && (
                    <button onClick={() => handleHarvestSeed(seed.tokenId)}>
                      Harvest (Score: {seed.growthScore || 'Waiting for validation'})
                    </button>
                  )}
                  {(seed.state === 'PLANTED' || seed.state === 'GROWING') && (
                    <>
                      <button onClick={() => handleFailSeed(seed.tokenId, 'Manual fail')}>
                        Fail Seed
                      </button>
                      <button onClick={() => {/* In a real game, we would call the GrowthEngine to process validation here */}}>
                        Process Validation
                      </button>
                    </>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </main>
    </div>
  );
}

export default App;