export const POOL_REGISTRY_ADDRESS = '0x509Bd43690F20eE36f9C4816e79aaC3fCb5F340d' as const;
export const ITEM_RECEIPT_NFT_ADDRESS = '0x7a45FDcEf785CE342f96d24Df16B94c8C38dD3Fa' as const;
export const POOL_CLAIM_TOKEN_ADDRESS = '0x8915B405610a75b3C1909B6fB9448de5Db58f016' as const;
export const INVENTORY_COMMITMENT_ADDRESS = '0x3E413Dd53da0729af8745160CC482Ba6bd4bB777' as const;
export const DISPUTE_RESOLVER_ADDRESS = '0x8b4F1f72c95dDbF59f3B8FD665a3134a088f5F89' as const;
export const ESCROW_SETTLEMENT_ADDRESS = '0x41ba22ee648fB63Ecd0BFF7472BE972F2842E3D3' as const;

export const poolRegistryAbi = [
  {
    type: 'function',
    name: 'getPool',
    inputs: [{ name: 'poolId', type: 'uint256' }],
    outputs: [
      {
        name: '',
        type: 'tuple',
        components: [
          { name: 'hue', type: 'uint8' },
          { name: 'band', type: 'uint8' },
          { name: 'region', type: 'string' },
          { name: 'qualityTier', type: 'uint8' },
          { name: 'active', type: 'bool' },
          { name: 'paused', type: 'bool' },
        ],
      },
    ],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'nextPoolId',
    inputs: [],
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
  },
  {
    type: 'function',
    name: 'isPoolActive',
    inputs: [{ name: 'poolId', type: 'uint256' }],
    outputs: [{ name: '', type: 'bool' }],
    stateMutability: 'view',
  },
] as const;

export const poolClaimTokenAbi = [
  {
    type: 'function',
    name: 'balanceOf',
    inputs: [
      { name: 'account', type: 'address' },
      { name: 'id', type: 'uint256' },
    ],
    outputs: [{ name: '', type: 'uint256' }],
    stateMutability: 'view',
  },
] as const;

export const inventoryCommitmentAbi = [
  {
    type: 'function',
    name: 'getInventory',
    inputs: [{ name: 'inventoryId', type: 'uint256' }],
    outputs: [
      {
        name: '',
        type: 'tuple',
        components: [
          { name: 'receiptId', type: 'uint256' },
          { name: 'poolId', type: 'uint256' },
          { name: 'reservationId', type: 'uint256' },
          { name: 'state', type: 'uint8' },
          { name: 'trackingRef', type: 'bytes32' },
          { name: 'proofHash', type: 'bytes32' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;

export const escrowSettlementAbi = [
  {
    type: 'function',
    name: 'getReservation',
    inputs: [{ name: 'reservationId', type: 'uint256' }],
    outputs: [
      {
        name: '',
        type: 'tuple',
        components: [
          { name: 'user', type: 'address' },
          { name: 'poolId', type: 'uint256' },
          { name: 'inventoryId', type: 'uint256' },
          { name: 'claimAmount', type: 'uint256' },
          { name: 'createdAt', type: 'uint256' },
          { name: 'expiresAt', type: 'uint256' },
          { name: 'status', type: 'uint8' },
        ],
      },
    ],
    stateMutability: 'view',
  },
] as const;
