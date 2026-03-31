import { expect } from "chai";

describe("SchoolSafety", function () {
  let schoolSafety;
  let owner, educator, user;

  beforeEach(async function () {
    [owner, educator, user] = await ethers.getSigners();
    const SchoolSafety = await ethers.getContractFactory("SchoolSafety");
    schoolSafety = await SchoolSafety.deploy();
    await schoolSafety.waitForDeployment();
  });

  describe("Panic Mode", function () {
    it("should activate panic mode", async function () {
      await schoolSafety.activatePanic("Emergency");
      const status = await schoolSafety.getPanicStatus();
      expect(status.active).to.equal(true);
    });

    it("should reject operations during panic", async function () {
      await schoolSafety.activatePanic("Emergency");
      await expect(
        schoolSafety.submitContent("QmHash123")
      ).to.be.revertedWith("System in panic mode");
    });

    it("should check panic status", async function () {
      const status = await schoolSafety.getPanicStatus();
      expect(status.active).to.equal(false);
    });
  });

  describe("Content Management", function () {
    it("should submit content", async function () {
      await schoolSafety.submitContent("QmTestHash123");
      const content = await schoolSafety.getContent(0);
      expect(content.contentHash).to.equal("QmTestHash123");
    });

    it("should approve content", async function () {
      await schoolSafety.approveEducator(owner.address, 2);
      await schoolSafety.submitContent("QmTestHash123");
      await schoolSafety.approveContent(0, "Approved for use");
      
      expect(await schoolSafety.isContentApproved(0)).to.equal(true);
    });

    it("should reject content", async function () {
      await schoolSafety.approveEducator(owner.address, 2);
      await schoolSafety.submitContent("QmTestHash123");
      await schoolSafety.rejectContent(0, "Inappropriate content");
      
      expect(await schoolSafety.isContentApproved(0)).to.equal(false);
    });

    it("should flag content", async function () {
      await schoolSafety.submitContent("QmTestHash123");
      await schoolSafety.flagContent(0, "Suspicious content");
      
      const content = await schoolSafety.getContent(0);
      expect(content.state).to.equal(3); // FLAGGED
    });
  });

  describe("Educator Management", function () {
    it("should approve educator", async function () {
      await schoolSafety.approveEducator(educator.address, 1);
      expect(await schoolSafety.approvedEducators(educator.address)).to.equal(true);
    });

    it("should revoke educator", async function () {
      await schoolSafety.approveEducator(educator.address, 1);
      await schoolSafety.revokeEducator(educator.address);
      expect(await schoolSafety.approvedEducators(educator.address)).to.equal(false);
    });

    it("should enforce permission levels", async function () {
      await schoolSafety.approveEducator(educator.address, 1);
      // Level 1 can review, let's verify the permission is set
      const perms = await schoolSafety.educatorPermissions(educator.address);
      expect(perms).to.equal(1);
    });
  });

  describe("Privacy", function () {
    it("should allow user to view own data", async function () {
      // User should be able to view their own data
      const canView = await schoolSafety.connect(user).canViewUserData(user.address);
      expect(canView).to.equal(true);
    });

    it("should set privacy exemption", async function () {
      await schoolSafety.setPrivacyExemption(user.address, true);
      const exempted = await schoolSafety.privacyExemptions(user.address);
      expect(exempted).to.equal(true);
    });
  });

  describe("Safety Reporting", function () {
    it("should report safety incident", async function () {
      await expect(schoolSafety.reportIncident("Suspicious activity detected"))
        .to.emit(schoolSafety, "SafetyIncidentReported");
    });
  });
});
