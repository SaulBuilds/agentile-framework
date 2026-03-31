import { expect } from "chai";

describe("Integration: Contract Interactions", function () {
  let seedNFT;
  let growthEngine;
  let federation;
  let leaderboard;
  let seasonManager;
  let ortToken;
  let duelManager;
  let owner, addr1, addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    // Deploy all contracts
    const SeedNFT = await ethers.getContractFactory("SeedNFT");
    seedNFT = await SeedNFT.deploy();
    await seedNFT.waitForDeployment();

    const GrowthEngine = await ethers.getContractFactory("GrowthEngine");
    growthEngine = await GrowthEngine.deploy(await seedNFT.getAddress());
    await growthEngine.waitForDeployment();

    const Federation = await ethers.getContractFactory("Federation");
    federation = await Federation.deploy();
    await federation.waitForDeployment();

    const Leaderboard = await ethers.getContractFactory("Leaderboard");
    leaderboard = await Leaderboard.deploy();
    await leaderboard.waitForDeployment();

    const SeasonManager = await ethers.getContractFactory("SeasonManager");
    seasonManager = await SeasonManager.deploy();
    await seasonManager.waitForDeployment();

    const ORTToken = await ethers.getContractFactory("ORTToken");
    ortToken = await ORTToken.deploy();
    await ortToken.waitForDeployment();

    const DuelManager = await ethers.getContractFactory("DuelManager");
    duelManager = await DuelManager.deploy(
      await seedNFT.getAddress(),
      await growthEngine.getAddress()
    );
    await duelManager.waitForDeployment();
  });

  describe("SeedNFT -> Federation Integration", function () {
    it("should track federation in seed", async function () {
      await seedNFT.plantSeed("QmTest", 50, 1, 5);
      const fedId = await seedNFT.federationOf(0);
      expect(fedId).to.equal(1);
    });

    it("should allow addr1 to join federation", async function () {
      await federation.createFederation(100);
      await federation.connect(addr1).joinFederation(0);
      expect(await federation.isMember(0, addr1.address)).to.equal(true);
    });
  });

  describe("SeedNFT -> GrowthEngine Integration", function () {
    it("should process validation on seed", async function () {
      await seedNFT.plantSeed("QmTest", 50, 0, 5);
      await seedNFT.advanceCheckpoint(0);
      await seedNFT.advanceCheckpoint(0);
      await seedNFT.advanceCheckpoint(0);
      await seedNFT.advanceCheckpoint(0);
      await seedNFT.advanceCheckpoint(0);
      
      const tx = await growthEngine.processValidation(0, 5);
      expect(tx).to.emit(growthEngine, "ValidationProcessed");
    });
  });

  describe("DuelManager -> SeedNFT Integration", function () {
    it("should validate seed ownership in duels", async function () {
      await seedNFT.connect(addr1).plantSeed("QmTest1", 50, 0, 3);
      await seedNFT.connect(addr2).plantSeed("QmTest2", 50, 0, 3);
      
      await seedNFT.connect(addr1).advanceCheckpoint(0);
      await seedNFT.connect(addr2).advanceCheckpoint(1);
      
      // addr1 initiates duel
      await duelManager.connect(addr1).initiateDuel(0, addr2.address, 1);
      
      const duel = await duelManager.getDuel(0);
      expect(duel.seedIdA).to.equal(0);
      expect(duel.seedIdB).to.equal(1);
    });
  });

  describe("Leaderboard -> Federation Integration", function () {
    it("should track federation scores", async function () {
      await leaderboard.updatePlayerScore(addr1.address, 100, 1);
      await leaderboard.updatePlayerScore(addr2.address, 150, 1);
      
      const [feds, scores] = await leaderboard.getTopFederations(0, 1);
      expect(scores[0]).to.equal(250);
    });
  });

  describe("SeasonManager -> Federation Integration", function () {
    it("should track harvests per season", async function () {
      await seasonManager.startSeason(86400 * 30, 100);
      
      // Simulate harvest
      await seasonManager.recordHarvest(addr1.address, 80);
      
      const stats = await seasonManager.getSeasonStats(0);
      expect(stats.totalHarvests).to.equal(1);
      expect(stats.totalScore).to.equal(80);
    });
  });

  describe("ORTToken -> Federation Integration", function () {
    it("should stake tokens", async function () {
      await ortToken.stake(100);
      expect(await ortToken.stakedBalance()).to.equal(100);
    });

    it("should stake in federation", async function () {
      await ortToken.stakeSeedInFederation(0, 1, 50);
      const balance = await ortToken.balanceOf(owner.address);
      expect(balance).to.be.lt(await ortToken.INITIAL_SUPPLY());
    });
  });

  describe("Full Game Flow Integration", function () {
    it("should complete full game flow", async function () {
      // 1. Plant seed
      await seedNFT.plantSeed("QmTest", 50, 0, 5);
      
      // 2. Advance checkpoints
      for (let i = 0; i < 5; i++) {
        await seedNFT.advanceCheckpoint(0);
      }
      
      // 3. Harvest
      await seedNFT.harvestSeed(0, 85);
      
      // 4. Update leaderboard
      await leaderboard.updatePlayerScore(owner.address, 85, 0);
      
      // 5. Check leaderboard
      const [players, scores] = await leaderboard.getTopPlayers(0, 1);
      expect(players[0]).to.equal(owner.address);
      expect(scores[0]).to.equal(85);
      
      // 6. Verify harvest event
      const isHarvested = await seedNFT.isHarvested(0);
      expect(isHarvested).to.equal(true);
    });
  });
});
