const { Given, When, Then } = require('@cucumber/cucumber');
const { ethers } = require('ethers');
const { assert } = require('chai');

let seedNFT;
let signer;
let seedId;
const seeds = [];

Given('the SeedNFT contract is deployed', async function () {
  const SeedNFT = await ethers.getContractFactory('SeedNFT');
  seedNFT = await SeedNFT.deploy();
  await seedNFT.waitForDeployment();
  const signers = await ethers.getSigners();
  signer = signers[0];
});

When('I plant a seed with payload {string}, stake {int}, federation {int}, maxCheckpoints {int}', async function (payload, stake, federation, maxCheckpoints) {
  const tx = await seedNFT.plantSeed(payload, stake, federation, maxCheckpoints);
  const receipt = await tx.wait();
  const event = receipt.logs.find(log => log.fragment && log.fragment.name === 'SeedPlanted');
  seedId = event ? event.args.tokenId : 0;
  seeds.push({ id: seedId, owner: signer.address, stake, federation, maxCheckpoints });
});

Then('the seed should be owned by me', async function () {
  const owner = await seedNFT.ownerOf(seedId);
  assert.equal(owner, signer.address);
});

Then('the seed checkpoint should be {int}', async function (expectedCheckpoint) {
  const checkpoint = await seedNFT.checkpointOf(seedId);
  assert.equal(checkpoint, expectedCheckpoint);
});

Then('the seed stake should be {int}', async function (expectedStake) {
  const stake = await seedNFT.stakeOf(seedId);
  assert.equal(stake, expectedStake);
});

Given('I have planted a seed with maxCheckpoints {int}', async function (maxCheckpoints) {
  const tx = await seedNFT.plantSeed("test", 50, 1, maxCheckpoints);
  const receipt = await tx.wait();
  const event = receipt.logs.find(log => log.fragment && log.fragment.name === 'SeedPlanted');
  seedId = event ? event.args.tokenId : 0;
  seeds.push({ id: seedId, maxCheckpoints });
});

When('I advance the checkpoint {int} times', async function (times) {
  for (let i = 0; i < times; i++) {
    await seedNFT.advanceCheckpoint(seedId);
  }
});

Then('I should not be able to advance past max checkpoint', async function () {
  try {
    await seedNFT.advanceCheckpoint(seedId);
    assert.fail('Should have reverted');
  } catch (e) {
    assert(e.message.includes('max checkpoint'));
  }
});

Given('I have advanced the checkpoint to the maximum', async function () {
  const maxCp = await seedNFT.maxCheckpointOf(seedId);
  for (let i = 0; i < maxCp; i++) {
    await seedNFT.advanceCheckpoint(seedId);
  }
});

When('I harvest the seed with growth score {int}', async function (score) {
  await seedNFT.harvestSeed(seedId, score);
});

Then('the seed should be marked as harvested', async function () {
  const isHarvested = await seedNFT.isHarvested(seedId);
  assert.isTrue(isHarvested);
});

Then('the growth score should be {int}', async function (expectedScore) {
  const score = await seedNFT.growthScoreOf(seedId);
  assert.equal(score, expectedScore);
});

When('I fail the seed with reason {string}', async function (reason) {
  await seedNFT.failSeed(seedId, reason);
});

Then('the seed should be marked as failed', async function () {
  const isFailed = await seedNFT.isFailed(seedId);
  assert.isTrue(isFailed);
});
