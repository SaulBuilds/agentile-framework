import { expect } from "chai";

describe("Federation", function () {
  let federation;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    const Federation = await ethers.getContractFactory("Federation");
    federation = await Federation.deploy();
    await federation.waitForDeployment();
  });

  describe("Federation Creation", function () {
    it("Should create a federation with correct properties", async function () {
      await federation.createFederation(100);

      expect(await federation.federationCreator(0)).to.equal(owner.address);
      expect(await federation.federationMinStake(0)).to.equal(100);
      expect(await federation.federationRewardPool(0)).to.equal(0);
      expect(await federation.federationTotalScore(0)).to.equal(0);
      expect(await federation.isMember(0, owner.address)).to.equal(true);
    });

    it("Should reject creating a federation with insufficient min stake", async function () {
      await expect(
        federation.createFederation(50)
      ).to.be.revertedWith("Minimum stake too low");
    });

    it("Should increment federation IDs correctly", async function () {
      await federation.createFederation(100);
      await federation.createFederation(200);

      expect(await federation.federationMinStake(0)).to.equal(100);
      expect(await federation.federationMinStake(1)).to.equal(200);
    });
  });

  describe("Joining and Leaving", function () {
    beforeEach(async function () {
      await federation.createFederation(100);
    });

    it("Should allow a player to join a federation", async function () {
      await federation.connect(addr1).joinFederation(0);

      expect(await federation.isMember(0, addr1.address)).to.equal(true);
    });

    it("Should not allow joining twice", async function () {
      await federation.connect(addr1).joinFederation(0);

      await expect(
        federation.connect(addr1).joinFederation(0)
      ).to.be.revertedWith("Already a member");
    });

    it("Should allow a player to leave a federation", async function () {
      await federation.connect(addr1).joinFederation(0);
      await federation.connect(addr1).leaveFederation(0);

      expect(await federation.isMember(0, addr1.address)).to.equal(false);
    });

    it("Should not allow leaving with staked seeds", async function () {
      await federation.connect(addr1).joinFederation(0);
      await federation.connect(addr1).stakeSeed(0, 1, 100);

      await expect(
        federation.connect(addr1).leaveFederation(0)
      ).to.be.revertedWith("Must unstake all seeds before leaving federation");
    });
  });

  describe("Seed Staking", function () {
    beforeEach(async function () {
      await federation.createFederation(100);
    });

    it("Should allow a member to stake a seed", async function () {
      await federation.connect(addr1).joinFederation(0);
      await federation.connect(addr1).stakeSeed(0, 1, 100);

      expect(await federation.memberStake(0, addr1.address)).to.equal(100);
    });

    it("Should not allow staking below minimum", async function () {
      await federation.connect(addr1).joinFederation(0);

      await expect(
        federation.connect(addr1).stakeSeed(0, 1, 50)
      ).to.be.revertedWith("Stake below minimum");
    });

    it("Should allow unstaking a seed", async function () {
      await federation.connect(addr1).joinFederation(0);
      await federation.connect(addr1).stakeSeed(0, 1, 100);
      await federation.connect(addr1).unstakeSeed(0, 1, 100);

      expect(await federation.memberStake(0, addr1.address)).to.equal(0);
    });

    it("Should not allow unstaking more than staked", async function () {
      await federation.connect(addr1).joinFederation(0);
      await federation.connect(addr1).stakeSeed(0, 1, 100);

      await expect(
        federation.connect(addr1).unstakeSeed(0, 1, 200)
      ).to.be.revertedWith("Insufficient stake in federation");
    });
  });

  describe("Reward Management", function () {
    beforeEach(async function () {
      await federation.createFederation(100);
    });

    it("Should allow creator to add rewards", async function () {
      await federation.addReward(0, 1000);

      expect(await federation.federationRewardPool(0)).to.equal(1000);
    });

    it("Should not allow non-creator to add rewards", async function () {
      await expect(
        federation.connect(addr1).addReward(0, 1000)
      ).to.be.revertedWith("Only federation creator can add rewards");
    });

    it("Should allow creator to distribute rewards", async function () {
      await federation.addReward(0, 1000);
      await federation.distributeRewards(0);

      expect(await federation.federationRewardPool(0)).to.equal(0);
    });

    it("Should not allow non-creator to distribute rewards", async function () {
      await federation.addReward(0, 1000);

      await expect(
        federation.connect(addr1).distributeRewards(0)
      ).to.be.revertedWith("Only federation creator can distribute rewards");
    });
  });

  describe("Score Management", function () {
    beforeEach(async function () {
      await federation.createFederation(100);
    });

    it("Should update total score", async function () {
      await federation.updateTotalScore(0, 500);

      expect(await federation.getTotalScore(0)).to.equal(500);
    });

    it("Should accumulate total score", async function () {
      await federation.updateTotalScore(0, 300);
      await federation.updateTotalScore(0, 200);

      expect(await federation.getTotalScore(0)).to.equal(500);
    });
  });
});
