import { expect } from "chai";

describe("MentorProtocol", function () {
  let mentorProtocol;
  let owner, addr1, addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();
    const MentorProtocol = await ethers.getContractFactory("MentorProtocol");
    mentorProtocol = await MentorProtocol.deploy();
    await mentorProtocol.waitForDeployment();
  });

  it("should register as mentor with sufficient score", async function () {
    await mentorProtocol.connect(addr1).registerAsMentor(80);
    expect(await mentorProtocol.isMentor(addr1.address)).to.equal(true);
  });

  it("should reject registration with insufficient score", async function () {
    await expect(
      mentorProtocol.connect(addr1).registerAsMentor(50)
    ).to.be.revertedWith("Score below threshold");
  });

  it("should reject duplicate registration", async function () {
    await mentorProtocol.connect(addr1).registerAsMentor(80);
    await expect(
      mentorProtocol.connect(addr1).registerAsMentor(85)
    ).to.be.revertedWith("Already a mentor");
  });

  it("should allow mentee to request mentorship", async function () {
    await mentorProtocol.connect(owner).registerAsMentor(80);
    await mentorProtocol.connect(addr1).requestMentorship(owner.address);
    const mentors = await mentorProtocol.getMenteesMentors(addr1.address);
    expect(mentors).to.include(owner.address);
  });

  it("should create knowledge adapter", async function () {
    await mentorProtocol.connect(addr1).registerAsMentor(80);
    await mentorProtocol.connect(addr1).createAdapter(85);
    const adapters = await mentorProtocol.getPlayerAdapters(addr1.address);
    expect(adapters.length).to.equal(1);
  });

  it("should reject adapter with invalid quality", async function () {
    await mentorProtocol.connect(addr1).registerAsMentor(80);
    await expect(
      mentorProtocol.connect(addr1).createAdapter(150)
    ).to.be.revertedWith("Quality too high");
  });

  it("should allow deregistration", async function () {
    await mentorProtocol.connect(addr1).registerAsMentor(80);
    await mentorProtocol.connect(addr1).deregisterAsMentor();
    expect(await mentorProtocol.isMentor(addr1.address)).to.equal(false);
  });
});
