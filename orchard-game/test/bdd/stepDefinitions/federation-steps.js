const { Given, When, Then } = require('@cucumber/cucumber');
const { ethers } = require('ethers');
const { assert } = require('chai');

let federation;
let federationId;
let signer;
let member;

Given('the Federation contract is deployed', async function () {
  const Federation = await ethers.getContractFactory('Federation');
  federation = await Federation.deploy();
  await federation.waitForDeployment();
  const signers = await ethers.getSigners();
  signer = signers[0];
});

When('I create a federation with minimum stake {int}', async function (minStake) {
  const tx = await federation.createFederation(minStake);
  const receipt = await tx.wait();
  const event = receipt.logs.find(log => log.fragment && log.fragment.name === 'FederationCreated');
  federationId = event ? event.args.federationId : 0;
});

Then('the federation should be created with me as creator', async function () {
  const creator = await federation.federationCreator(federationId);
  assert.equal(creator, signer.address);
});

Then('the minimum stake should be {int}', async function (expectedMinStake) {
  const minStake = await federation.federationMinStake(federationId);
  assert.equal(minStake, expectedMinStake);
});

Given('a federation exists with minimum stake {int}', async function (minStake) {
  const Federation = await ethers.getContractFactory('Federation');
  federation = await Federation.deploy();
  await federation.waitForDeployment();
  const tx = await federation.createFederation(minStake);
  const receipt = await tx.wait();
  const event = receipt.logs.find(log => log.fragment && log.fragment.name === 'FederationCreated');
  federationId = event ? event.args.federationId : 0;
  const signers = await ethers.getSigners();
  member = signers[1];
});

When('I join the federation', async function () {
  const federationsWithSigner = federation.connect(member);
  await federationsWithSigner.joinFederation(federationId);
});

Then('I should be a member of the federation', async function () {
  const isMember = await federation.isMember(federationId, member.address);
  assert.isTrue(isMember);
});

Given('I am a member of a federation', async function () {
  const isMember = await federation.isMember(federationId, signer.address);
  assert.isTrue(isMember);
});

Given('I have no seeds staked', async function () {
  const stake = await federation.memberStake(federationId, signer.address);
  assert.equal(stake, 0);
});

When('I leave the federation', async function () {
  await federation.leaveFederation(federationId);
});

Then('I should not be a member of the federation', async function () {
  const isMember = await federation.isMember(federationId, signer.address);
  assert.isFalse(isMember);
});

Given('I am a member of a federation with minimum stake {int}', async function (minStake) {
  const tx = await federation.createFederation(minStake);
  const receipt = await tx.wait();
  const event = receipt.logs.find(log => log.fragment && log.fragment.name === 'FederationCreated');
  federationId = event ? event.args.federationId : 0;
});

When('I stake a seed with tokenId {int} and amount {int}', async function (tokenId, amount) {
  await federation.stakeSeed(federationId, tokenId, amount);
});

Then('my stake in the federation should be {int}', async function (expectedStake) {
  const stake = await federation.memberStake(federationId, signer.address);
  assert.equal(stake, expectedStake);
});

Given('I have staked {int} in a federation', async function (amount) {
  await federation.stakeSeed(federationId, 1, amount);
});

When('I unstake my seed with amount {int}', async function (amount) {
  await federation.unstakeSeed(federationId, 1, amount);
});

Given('I am the creator of a federation', async function () {
  const tx = await federation.createFederation(100);
  const receipt = await tx.wait();
  const event = receipt.logs.find(log => log.fragment && log.fragment.name === 'FederationCreated');
  federationId = event ? event.args.federationId : 0;
});

When('I add {int} ORT to the reward pool', async function (amount) {
  await federation.addReward(federationId, amount);
});

Then('the federation reward pool should be {int}', async function (expectedReward) {
  const reward = await federation.federationRewardPool(federationId);
  assert.equal(reward, expectedReward);
});
