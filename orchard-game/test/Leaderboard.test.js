import { expect } from "chai";

describe("Leaderboard", function () {
  let leaderboard;
  let owner, addr1, addr2, addr3;

  beforeEach(async function () {
    [owner, addr1, addr2, addr3] = await ethers.getSigners();
    const Leaderboard = await ethers.getContractFactory("Leaderboard");
    leaderboard = await Leaderboard.deploy();
    await leaderboard.waitForDeployment();
  });

  it("should update player score", async function () {
    await leaderboard.updatePlayerScore(addr1.address, 100, 0);
    const [players, scores] = await leaderboard.getTopPlayers(0, 10);
    expect(players).to.include(addr1.address);
    expect(scores[0]).to.equal(100);
  });

  it("should sort players by score", async function () {
    await leaderboard.updatePlayerScore(addr1.address, 50, 0);
    await leaderboard.updatePlayerScore(addr2.address, 100, 0);
    await leaderboard.updatePlayerScore(addr3.address, 75, 0);
    const [players, scores] = await leaderboard.getTopPlayers(0, 3);
    expect(players[0]).to.equal(addr2.address);
    expect(scores[0]).to.equal(100);
  });

  it("should update existing player score", async function () {
    await leaderboard.updatePlayerScore(addr1.address, 50, 0);
    await leaderboard.updatePlayerScore(addr1.address, 100, 0);
    const [players, scores] = await leaderboard.getTopPlayers(0, 1);
    expect(scores[0]).to.equal(150);
  });

  it("should get player rank", async function () {
    await leaderboard.updatePlayerScore(addr1.address, 100, 0);
    await leaderboard.updatePlayerScore(addr2.address, 50, 0);
    const rank = await leaderboard.getPlayerSeasonRank(0, addr1.address);
    expect(rank).to.equal(0);
  });

  it("should track federation scores", async function () {
    await leaderboard.updatePlayerScore(addr1.address, 100, 1);
    await leaderboard.updatePlayerScore(addr2.address, 150, 1);
    const [feds, scores] = await leaderboard.getTopFederations(0, 1);
    expect(scores[0]).to.equal(250);
  });

  it("should toggle seasonal resets", async function () {
    await leaderboard.toggleSeasonalResets();
    expect(await leaderboard.seasonalResetsEnabled()).to.equal(false);
  });

  it("should get player record", async function () {
    await leaderboard.updatePlayerScore(addr1.address, 100, 0);
    const record = await leaderboard.getPlayerRecord(0, addr1.address);
    expect(record.totalScore).to.equal(100);
    expect(record.harvestCount).to.equal(1);
  });
});
