import { expect } from "chai";

describe("ORTToken", function () {
  let ortToken;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    const ORTToken = await ethers.getContractFactory("ORTToken");
    ortToken = await ORTToken.deploy();
    await ortToken.waitForDeployment();
  });

  describe("Token Basics", function () {
    it("Should have correct name and symbol", async function () {
      expect(await ortToken.name()).to.equal("Orchard Token");
      expect(await ortToken.symbol()).to.equal("ORT");
    });

    it("Should have correct decimals", async function () {
      expect(await ortToken.decimals()).to.equal(18);
    });

    it("Should have correct initial supply", async function () {
      const initialSupply = await ortToken.INITIAL_SUPPLY();
      expect(initialSupply).to.equal(BigInt(1_000_000) * BigInt(10 ** 18));
    });

    it("Should mint initial supply to owner", async function () {
      const ownerBalance = await ortToken.balanceOf(owner.address);
      const initialSupply = await ortToken.INITIAL_SUPPLY();
      expect(ownerBalance).to.equal(initialSupply);
    });
  });

  describe("Staking", function () {
    it("Should allow staking tokens", async function () {
      await ortToken.stake(100);

      expect(await ortToken.balanceOf(owner.address)).to.equal(
        (await ortToken.INITIAL_SUPPLY()) - BigInt(100)
      );
      expect(await ortToken.balanceOf(await ortToken.getAddress())).to.equal(100);
    });

    it("Should not allow staking zero amount", async function () {
      await expect(ortToken.stake(0)).to.be.revertedWith(
        "Amount must be greater than zero"
      );
    });

    it("Should not allow staking more than balance", async function () {
      const balance = await ortToken.balanceOf(owner.address);
      await expect(ortToken.stake(balance + BigInt(1))).to.be.revertedWith(
        "Insufficient balance"
      );
    });

    it("Should emit Staked event", async function () {
      await expect(ortToken.stake(100))
        .to.emit(ortToken, "Staked")
        .withArgs(owner.address, 100);
    });
  });

  describe("Unstaking", function () {
    beforeEach(async function () {
      await ortToken.stake(100);
    });

    it("Should allow unstaking tokens", async function () {
      await ortToken.unstake(50);

      expect(await ortToken.balanceOf(owner.address)).to.equal(
        (await ortToken.INITIAL_SUPPLY()) - BigInt(50)
      );
      expect(await ortToken.balanceOf(await ortToken.getAddress())).to.equal(50);
    });

    it("Should not allow unstaking zero amount", async function () {
      await expect(ortToken.unstake(0)).to.be.revertedWith(
        "Amount must be greater than zero"
      );
    });

    it("Should not allow unstaking more than staked", async function () {
      await expect(ortToken.unstake(200)).to.be.revertedWith(
        "Insufficient staked balance"
      );
    });
  });

  describe("Rewards", function () {
    beforeEach(async function () {
      await ortToken.stake(1000);
    });

    it("Should allow rewarding tokens", async function () {
      await ortToken.reward(addr1.address, 50);

      expect(await ortToken.balanceOf(addr1.address)).to.equal(50);
    });

    it("Should not allow rewarding zero amount", async function () {
      await expect(ortToken.reward(addr1.address, 0)).to.be.revertedWith(
        "Amount must be greater than zero"
      );
    });

    it("Should not allow rewarding more than contract balance", async function () {
      await expect(ortToken.reward(addr1.address, 2000)).to.be.revertedWith(
        "Insufficient reward balance"
      );
    });
  });

  describe("Federation Staking", function () {
    it("Should allow staking seed in federation", async function () {
      await ortToken.stakeSeedInFederation(0, 1, 100);

      expect(await ortToken.balanceOf(owner.address)).to.equal(
        (await ortToken.INITIAL_SUPPLY()) - BigInt(100)
      );
    });

    it("Should allow unstaking seed from federation", async function () {
      await ortToken.stakeSeedInFederation(0, 1, 100);
      await ortToken.unstakeSeedFromFederation(0, 1, 50);

      expect(await ortToken.balanceOf(owner.address)).to.equal(
        (await ortToken.INITIAL_SUPPLY()) - BigInt(50)
      );
    });
  });

  describe("View Functions", function () {
    it("Should return staked balance", async function () {
      await ortToken.stake(100);
      expect(await ortToken.stakedBalance()).to.equal(100);
    });
  });
});
