const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("GrowthEngine", function () {
  let growthEngine;
  let seedNFT;
  let owner;
  let addr1;

  beforeEach(async function () {
    [owner, addr1] = await ethers.getSigners();

    // Deploy SeedNFT first
    const SeedNFT = await ethers.getContractFactory("SeedNFT");
    seedNFT = await SeedNFT.deploy();
    await seedNFT.deployed();

    // Deploy GrowthEngine
    const GrowthEngine = await ethers.getContractFactory("GrowthEngine");
    growthEngine = await GrowthEngine.deploy(seedNFT.address);
    await growthEngine.deployed();
  });

  describe("Validation Processing", function () {
    it("Should process validation for a seed", async function () {
      // Plant a seed first
      await seedNFT.plantSeed(
        "ipfs://QmTest", // payload
        50, // stake
        1, // federation
        5 // maxCheckpoints
      );

      // Process validation at checkpoint 0
      await expect(growthEngine.processValidation(0, 0))
        .to.emit(growthEngine, "ValidationProcessed")
        .withArgs(0, 0, /* belnapState */, /* agreementPercentage */);
    });

    it("Should revert for invalid seed", async function () {
      await expect(
        growthEngine.processValidation(999, 0) // non-existent seed
      ).to.be.revertedWith("Seed does not exist");
    });

    it("Should revert for harvested seed", async function () {
      // Plant and harvest a seed
      await seedNFT.plantSeed("ipfs://QmTest", 50, 1, 5);
      await seedNFT.advanceCheckpoint(0); // advance to checkpoint 1
      await seedNFT.advanceCheckpoint(0); // advance to checkpoint 2
      await seedNFT.advanceCheckpoint(0); // advance to checkpoint 3
      await seedNFT.advanceCheckpoint(0); // advance to checkpoint 4
      await seedNFT.advanceCheckpoint(0); // advance to checkpoint 5 (mature)
      await seedNFT.harvestSeed(0, 80); // harvest with score 80

      // Try to validate harvested seed
      await expect(
        growthEngine.processValidation(0, 5)
      ).to.be.revertedWith("Cannot validate harvested seed");
    });

    it("Should calculate growth score at final checkpoint", async function () {
      // Plant a seed with maxCheckpoints = 3
      await seedNFT.plantSeed("ipfs://QmTest", 50, 1, 3);

      // Advance to checkpoint 2
      await seedNFT.advanceCheckpoint(0); // 0 -> 1
      await seedNFT.advanceCheckpoint(0); // 1 -> 2

      // Process validation at final checkpoint (2 -> 3 would make it mature)
      // Actually we need to be AT the max checkpoint to harvest
      await seedNFT.advanceCheckpoint(0); // 2 -> 3 (now at max checkpoint)

      // Process validation should trigger harvest
      await expect(growthEngine.processValidation(0, 3))
        .to.emit(growthEngine, "GrowthScoreCalculated")
        .withArgs(0, /* growthScore */);
    });
  });

  describe("Belnap State Description", function () {
    it("Should return correct description for high validation values", async function () {
      const desc = await growthEngine.getBelnapStateDescription(75);
      expect(desc).to.equal("True (T)");
    });

    it("Should return correct description for low validation values", async function () {
      const desc = await growthEngine.getBelnapStateDescription(15);
      expect(desc).to.equal("False (F)");
    });

    it("Should return correct description for middle validation values", async function () {
      const desc = await growthEngine.getBelnapStateDescription(40);
      expect(desc).to.equal("Both/Contradiction (B)");
    });
  });
});