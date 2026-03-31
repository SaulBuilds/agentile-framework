import { expect } from "chai";
import hardhat from "hardhat";
const { ethers } = hardhat;

describe("MysteryBox", function () {
  let mysteryBox;
  let owner, addr1, addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();
    const MysteryBox = await ethers.getContractFactory("MysteryBox");
    mysteryBox = await MysteryBox.deploy();
    await mysteryBox.waitForDeployment();
  });

  const MIN_VALUE = ethers.parseEther("10");
  const MAX_VALUE = ethers.parseEther("10000");

  it("should create box type", async function () {
    await mysteryBox.createBoxType("Bronze Box", MIN_VALUE, MAX_VALUE, 5000, 100);
    const details = await mysteryBox.getBoxTypeDetails(0);
    expect(details.name).to.equal("Bronze Box");
  });

  it("should reject invalid min value", async function () {
    await expect(
      mysteryBox.createBoxType("Test", 1, 100, 5000, 100)
    ).to.be.revertedWith("Min value too low");
  });

  it("should reject invalid max value", async function () {
    const hugeValue = ethers.parseEther("20000");
    await expect(
      mysteryBox.createBoxType("Test", MIN_VALUE, hugeValue, 5000, 100)
    ).to.be.revertedWith("Max value too high");
  });

  it("should reject invalid probability", async function () {
    await expect(
      mysteryBox.createBoxType("Test", MIN_VALUE, MAX_VALUE, 10001, 100)
    ).to.be.revertedWith("Probability too high");
  });

  it("should award box to recipient", async function () {
    await mysteryBox.createBoxType("Bronze Box", MIN_VALUE, MAX_VALUE, 5000, 100);
    const commitHash = ethers.keccak256(ethers.toUtf8Bytes("test"));
    await mysteryBox.awardBox(addr1.address, 0, commitHash);
    const boxes = await mysteryBox.getRecipientBoxes(addr1.address);
    expect(boxes.length).to.equal(1);
  });

  it("should batch award boxes", async function () {
    await mysteryBox.createBoxType("Bronze Box", MIN_VALUE, MAX_VALUE, 5000, 100);
    await mysteryBox.batchAwardBoxes([addr1.address, addr2.address], 0);
    const boxes1 = await mysteryBox.getRecipientBoxes(addr1.address);
    const boxes2 = await mysteryBox.getRecipientBoxes(addr2.address);
    expect(boxes1.length).to.equal(1);
    expect(boxes2.length).to.equal(1);
  });

  it("should track box details", async function () {
    await mysteryBox.createBoxType("Bronze Box", MIN_VALUE, MAX_VALUE, 5000, 100);
    const commitHash = ethers.keccak256(ethers.toUtf8Bytes("test"));
    await mysteryBox.awardBox(addr1.address, 0, commitHash);
    const details = await mysteryBox.getBoxDetails(0);
    expect(details.recipient).to.equal(addr1.address);
    expect(details.state).to.equal(1);
  });
});
