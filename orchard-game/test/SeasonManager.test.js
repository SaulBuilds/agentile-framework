import { expect } from "chai";
import hardhat from "hardhat";
const { ethers } = hardhat;

describe("SeasonManager", function () {
  let seasonManager;
  let owner;

  beforeEach(async function () {
    [owner] = await ethers.getSigners();
    const SeasonManager = await ethers.getContractFactory("SeasonManager");
    seasonManager = await SeasonManager.deploy();
    await seasonManager.waitForDeployment();
  });

  it("should create a new season", async function () {
    const duration = 86400 * 30;
    const frontier = 100;
    await seasonManager.startSeason(duration, frontier);
    const info = await seasonManager.getCurrentSeasonInfo();
    expect(info.seasonId).to.equal(0);
    expect(info.state).to.equal(1);
  });

  it("should report inactive initially", async function () {
    expect(await seasonManager.isSeasonActive()).to.equal(false);
  });

  it("should report active after starting season", async function () {
    await seasonManager.startSeason(86400 * 30, 100);
    expect(await seasonManager.isSeasonActive()).to.equal(true);
  });

  it("should expand knowledge frontier", async function () {
    await seasonManager.startSeason(86400 * 30, 100);
    await seasonManager.expandKnowledgeFrontier(150);
    const info = await seasonManager.getCurrentSeasonInfo();
    expect(info.frontierSize).to.equal(150);
  });

  it("should update concept mastery", async function () {
    await seasonManager.startSeason(86400 * 30, 100);
    await seasonManager.updateConceptMastery(0, 500);
    const mastery = await seasonManager.getConceptMastery(0);
    expect(mastery).to.equal(500);
  });

  it("should get time remaining", async function () {
    await seasonManager.startSeason(86400 * 30, 100);
    const timeRemaining = await seasonManager.getTimeRemainingInSeason();
    expect(timeRemaining).to.be.gt(86400 * 29);
  });
});
