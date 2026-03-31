import { expect } from "chai";

describe("DuelManager", function () {
  let duelManager;
  let seedNFT;
  let growthEngine;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    const SeedNFT = await ethers.getContractFactory("SeedNFT");
    seedNFT = await SeedNFT.deploy();
    await seedNFT.waitForDeployment();

    const GrowthEngine = await ethers.getContractFactory("GrowthEngine");
    growthEngine = await GrowthEngine.deploy(await seedNFT.getAddress());
    await growthEngine.waitForDeployment();

    const DuelManager = await ethers.getContractFactory("DuelManager");
    duelManager = await DuelManager.deploy(
      await seedNFT.getAddress(),
      await growthEngine.getAddress()
    );
    await duelManager.waitForDeployment();
  });

  describe("Duel Initiation", function () {
    it("Should allow initiating a duel between two seeds", async function () {
      await seedNFT.connect(addr1).plantSeed("ipfs://QmTest1", 50, 0, 3);
      await seedNFT.connect(addr2).plantSeed("ipfs://QmTest2", 50, 0, 3);
      
      await seedNFT.connect(addr1).advanceCheckpoint(0);
      await seedNFT.connect(addr2).advanceCheckpoint(1);

      await duelManager.connect(addr1).initiateDuel(0, addr2.address, 1);

      const duel = await duelManager.getDuel(0);
      expect(duel.seedIdA).to.equal(0);
      expect(duel.seedIdB).to.equal(1);
      expect(duel.playerA).to.equal(addr1.address);
      expect(duel.playerB).to.equal(addr2.address);
    });

    it("Should not allow dueling yourself", async function () {
      await seedNFT.plantSeed("ipfs://QmTest", 50, 0, 3);
      await seedNFT.advanceCheckpoint(0);

      await expect(
        duelManager.initiateDuel(0, owner.address, 0)
      ).to.be.revertedWith("Cannot duel yourself");
    });

    it("Should not allow duel if target is on cooldown after completing duel", async function () {
      await seedNFT.connect(addr1).plantSeed("ipfs://QmTest1", 50, 0, 3);
      await seedNFT.connect(addr2).plantSeed("ipfs://QmTest2", 50, 0, 3);
      
      await seedNFT.connect(addr1).advanceCheckpoint(0);
      await seedNFT.connect(addr2).advanceCheckpoint(1);

      await duelManager.connect(addr1).initiateDuel(0, addr2.address, 1);
      await duelManager.connect(addr2).acceptDuel(0);
      await duelManager.connect(addr1).completeDuel(0, 60, 60);

      await expect(
        duelManager.connect(addr1).initiateDuel(0, addr2.address, 1)
      ).to.be.revertedWith("You are on cooldown");
    });
  });

  describe("Duel Acceptance", function () {
    beforeEach(async function () {
      await seedNFT.connect(addr1).plantSeed("ipfs://QmTest1", 50, 0, 3);
      await seedNFT.connect(addr2).plantSeed("ipfs://QmTest2", 50, 0, 3);
      
      await seedNFT.connect(addr1).advanceCheckpoint(0);
      await seedNFT.connect(addr2).advanceCheckpoint(1);
      
      await duelManager.connect(addr1).initiateDuel(0, addr2.address, 1);
    });

    it("Should allow target to accept duel", async function () {
      await duelManager.connect(addr2).acceptDuel(0);

      const duel = await duelManager.getDuel(0);
      expect(duel.accepted).to.equal(true);
    });

    it("Should not allow non-target to accept duel", async function () {
      await expect(
        duelManager.connect(addr1).acceptDuel(0)
      ).to.be.revertedWith("Not the target of this duel");
    });

    it("Should allow target to reject duel", async function () {
      await duelManager.connect(addr2).rejectDuel(0);

      const duel = await duelManager.getDuel(0);
      expect(duel.accepted).to.equal(false);
    });
  });

  describe("Duel Completion", function () {
    beforeEach(async function () {
      await seedNFT.connect(addr1).plantSeed("ipfs://QmTest1", 50, 0, 3);
      await seedNFT.connect(addr2).plantSeed("ipfs://QmTest2", 50, 0, 3);
      
      await seedNFT.connect(addr1).advanceCheckpoint(0);
      await seedNFT.connect(addr2).advanceCheckpoint(1);
      
      await duelManager.connect(addr1).initiateDuel(0, addr2.address, 1);
      await duelManager.connect(addr2).acceptDuel(0);
    });

    it("Should complete duel with player A winning", async function () {
      await duelManager.connect(addr1).completeDuel(0, 80, 50);

      const duel = await duelManager.getDuel(0);
      expect(duel.completed).to.equal(true);
      expect(duel.result).to.equal(1);
    });

    it("Should complete duel with player B winning", async function () {
      await duelManager.connect(addr1).completeDuel(0, 50, 80);

      const duel = await duelManager.getDuel(0);
      expect(duel.result).to.equal(2);
    });

    it("Should complete duel as draw", async function () {
      await duelManager.connect(addr1).completeDuel(0, 60, 60);

      const duel = await duelManager.getDuel(0);
      expect(duel.result).to.equal(0);
    });
  });

  describe("Cooldown Management", function () {
    it("Should report cooldown status correctly", async function () {
      expect(await duelManager.isOnCooldown(owner.address)).to.equal(false);
    });

    it("Should track time until cooldown is over", async function () {
      const time = await duelManager.timeUntilCooldownOver(owner.address);
      expect(time).to.equal(0);
    });
  });
});
