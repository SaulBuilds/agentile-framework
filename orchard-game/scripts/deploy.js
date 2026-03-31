// Deployment script for Orchard Game contracts
const hre = require("hardhat");

async function main() {
    const [deployer] = await hre.ethers.getSigners();
    console.log("Deployer:", deployer.address);

    const confirmations = 2;

    // 1. Deploy ORTToken (must be first)
    console.log("\n1. Deploying ORTToken...");
    const ORTToken = await hre.ethers.getContractFactory("ORTToken");
    const ortToken = await ORTToken.deploy();
    await ortToken.waitForDeploy();
    console.log("ORTToken:", ortToken.target);
    await ortToken.deploymentTransaction().wait(confirmations);

    // 2. Deploy SeedNFT
    console.log("\n2. Deploying SeedNFT...");
    const SeedNFT = await hre.ethers.getContractFactory("SeedNFT");
    const seedNFT = await SeedNFT.deploy();
    await seedNFT.waitForDeploy();
    console.log("SeedNFT:", seedNFT.target);
    await seedNFT.deploymentTransaction().wait(confirmations);

    // 3. Deploy GrowthEngine
    console.log("\n3. Deploying GrowthEngine...");
    const GrowthEngine = await hre.ethers.getContractFactory("GrowthEngine");
    const growthEngine = await GrowthEngine.deploy(seedNFT.target);
    await growthEngine.waitForDeploy();
    console.log("GrowthEngine:", growthEngine.target);
    await growthEngine.deploymentTransaction().wait(confirmations);

    // 4. Deploy Federation
    console.log("\n4. Deploying Federation...");
    const Federation = await hre.ethers.getContractFactory("Federation");
    const federation = await Federation.deploy();
    await federation.waitForDeploy();
    console.log("Federation:", federation.target);
    await federation.deploymentTransaction().wait(confirmations);

    // 5. Deploy DuelManager
    console.log("\n5. Deploying DuelManager...");
    const DuelManager = await hre.ethers.getContractFactory("DuelManager");
    const duelManager = await DuelManager.deploy(seedNFT.target, growthEngine.target);
    await duelManager.waitForDeploy();
    console.log("DuelManager:", duelManager.target);
    await duelManager.deploymentTransaction().wait(confirmations);

    // 6. Deploy Leaderboard
    console.log("\n6. Deploying Leaderboard...");
    const Leaderboard = await hre.ethers.getContractFactory("Leaderboard");
    const leaderboard = await Leaderboard.deploy();
    await leaderboard.waitForDeploy();
    console.log("Leaderboard:", leaderboard.target);
    await leaderboard.deploymentTransaction().wait(confirmations);

    // 7. Deploy EconomicsEngine
    console.log("\n7. Deploying EconomicsEngine...");
    const EconomicsEngine = await hre.ethers.getContractFactory("EconomicsEngine");
    const economicsEngine = await EconomicsEngine.deploy(ortToken.target);
    await economicsEngine.waitForDeploy();
    console.log("EconomicsEngine:", economicsEngine.target);
    await economicsEngine.deploymentTransaction().wait(confirmations);

    // 8. Deploy CounterpartyAlignment
    console.log("\n8. Deploying CounterpartyAlignment...");
    const CounterpartyAlignment = await hre.ethers.getContractFactory("CounterpartyAlignment");
    const counterpartyAlignment = await CounterpartyAlignment.deploy();
    await counterpartyAlignment.waitForDeploy();
    console.log("CounterpartyAlignment:", counterpartyAlignment.target);
    await counterpartyAlignment.deploymentTransaction().wait(confirmations);

    // 9. Deploy x402PaymentGateway
    console.log("\n9. Deploying x402PaymentGateway...");
    const x402Payment = await hre.ethers.getContractFactory("x402PaymentGateway");
    const x402PaymentGateway = await x402Payment.deploy(ortToken.target, ortToken.target); // USDC, ORT - in production use actual USDC address
    await x402PaymentGateway.waitForDeploy();
    console.log("x402PaymentGateway:", x402PaymentGateway.target);
    await x402PaymentGateway.deploymentTransaction().wait(confirmations);

    // 10. Deploy SchoolSafety (if needed)
    console.log("\n10. Deploying SchoolSafety...");
    const SchoolSafety = await hre.ethers.getContractFactory("SchoolSafety");
    const schoolSafety = await SchoolSafety.deploy();
    await schoolSafety.waitForDeploy();
    console.log("SchoolSafety:", schoolSafety.target);
    await schoolSafety.deploymentTransaction().wait(confirmations);

    // 11. Deploy SeasonManager
    console.log("\n11. Deploying SeasonManager...");
    const SeasonManager = await hre.ethers.getContractFactory("SeasonManager");
    const seasonManager = await SeasonManager.deploy();
    await seasonManager.waitForDeploy();
    console.log("SeasonManager:", seasonManager.target);
    await seasonManager.deploymentTransaction().wait(confirmations);

    // 12. Deploy MentorProtocol
    console.log("\n12. Deploying MentorProtocol...");
    const MentorProtocol = await hre.ethers.getContractFactory("MentorProtocol");
    const mentorProtocol = await MentorProtocol.deploy();
    await mentorProtocol.waitForDeploy();
    console.log("MentorProtocol:", mentorProtocol.target);
    await mentorProtocol.deploymentTransaction().wait(confirmations);

    // 13. Deploy MysteryBox
    console.log("\n13. Deploying MysteryBox...");
    const MysteryBox = await hre.ethers.getContractFactory("MysteryBox");
    const mysteryBox = await MysteryBox.deploy(ortToken.target);
    await mysteryBox.waitForDeploy();
    console.log("MysteryBox:", mysteryBox.target);
    await mysteryBox.deploymentTransaction().wait(confirmations);

    console.log("\n=== Deployment Complete ===");
    console.log("\nContract Addresses:");
    console.log("ORTToken:", ortToken.target);
    console.log("SeedNFT:", seedNFT.target);
    console.log("GrowthEngine:", growthEngine.target);
    console.log("Federation:", federation.target);
    console.log("DuelManager:", duelManager.target);
    console.log("Leaderboard:", leaderboard.target);
    console.log("EconomicsEngine:", economicsEngine.target);
    console.log("CounterpartyAlignment:", counterpartyAlignment.target);
    console.log("x402PaymentGateway:", x402PaymentGateway.target);
    console.log("SchoolSafety:", schoolSafety.target);
    console.log("SeasonManager:", seasonManager.target);
    console.log("MentorProtocol:", mentorProtocol.target);
    console.log("MysteryBox:", mysteryBox.target);
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
