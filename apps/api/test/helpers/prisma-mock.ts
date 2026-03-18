import { randomUUID } from 'crypto';

interface Store {
  users: any[];
  wallets: any[];
  pools: any[];
  itemSubmissions: any[];
  itemMedia: any[];
  itemReceipts: any[];
  inventoryItems: any[];
  inventoryStatusEvents: any[];
  claims: any[];
  claimConsumptions: any[];
  shipments: any[];
  trackingEvents: any[];
  courierTasks: any[];
  courierEvents: any[];
  disputes: any[];
  disputeEvidence: any[];
  notifications: any[];
  operatorActions: any[];
  warehouses: any[];
  restrictedItemRules: any[];
  indexerState: any[];
}

function createModelMock(store: any[], modelName: string, allStores: Record<string, any[]> = {}) {
  return {
    findUnique: jest.fn(async ({ where, include }: any) => {
      const item = store.find((i: any) => {
        if (where.id) return i.id === where.id;
        if (where.claimId) return i.claimId === where.claimId;
        return Object.entries(where).every(([k, v]) => i[k] === v);
      });
      if (!item) return null;
      if (include) {
        const result = { ...item };
        // Resolve included relations from allStores
        for (const [rel, opts] of Object.entries(include)) {
          if (opts === false) continue;
          const relStore = allStores[rel];
          if (relStore && Array.isArray(relStore)) {
            // Look for foreign key pattern: parentId -> modelName + 'Id'
            const fk = modelName + 'Id';
            result[rel] = relStore.filter((r: any) => r[fk] === item.id);
          }
        }
        return result;
      }
      return item;
    }),

    findFirst: jest.fn(async ({ where }: any = {}) => {
      return store.find((i: any) => matchWhere(i, where)) || null;
    }),

    findMany: jest.fn(async ({ where, orderBy, skip, take, include }: any = {}) => {
      let results = where ? store.filter((i: any) => matchWhere(i, where)) : [...store];
      if (skip) results = results.slice(skip);
      if (take) results = results.slice(0, take);
      return results;
    }),

    create: jest.fn(async ({ data }: any) => {
      const record = { id: randomUUID(), createdAt: new Date(), ...data };
      store.push(record);
      return record;
    }),

    update: jest.fn(async ({ where, data }: any) => {
      const idx = store.findIndex((i: any) => i.id === where.id);
      if (idx >= 0) {
        store[idx] = { ...store[idx], ...data };
        return store[idx];
      }
      return null;
    }),

    delete: jest.fn(async ({ where }: any) => {
      const idx = store.findIndex((i: any) => i.id === where.id);
      if (idx >= 0) return store.splice(idx, 1)[0];
      return null;
    }),

    count: jest.fn(async ({ where }: any = {}) => {
      if (!where) return store.length;
      return store.filter((i: any) => matchWhere(i, where)).length;
    }),

    aggregate: jest.fn(async ({ where, _sum, _count }: any = {}) => {
      const filtered = where ? store.filter((i: any) => matchWhere(i, where)) : [...store];
      const result: any = {};
      if (_sum) {
        result._sum = {};
        for (const key of Object.keys(_sum)) {
          result._sum[key] = filtered.reduce((acc: number, i: any) => acc + (Number(i[key]) || 0), 0);
        }
      }
      if (_count !== undefined) {
        result._count = filtered.length;
      }
      return result;
    }),
  };
}

function matchWhere(item: any, where: any): boolean {
  if (!where) return true;
  for (const [key, value] of Object.entries(where)) {
    if (key === 'OR') {
      return (value as any[]).some((cond) => matchWhere(item, cond));
    }
    if (key === 'AND') {
      return (value as any[]).every((cond) => matchWhere(item, cond));
    }
    if (value && typeof value === 'object' && !Array.isArray(value)) {
      const op = value as any;
      if ('in' in op) {
        if (!op.in.includes(item[key])) return false;
        continue;
      }
      if ('not' in op) {
        if (item[key] === op.not) return false;
        continue;
      }
      // Nested relation check - skip for simplicity
      continue;
    }
    if (item[key] !== value) return false;
  }
  return true;
}

export function createMockPrismaService() {
  const store: Store = {
    users: [],
    wallets: [],
    pools: [],
    itemSubmissions: [],
    itemMedia: [],
    itemReceipts: [],
    inventoryItems: [],
    inventoryStatusEvents: [],
    claims: [],
    claimConsumptions: [],
    shipments: [],
    trackingEvents: [],
    courierTasks: [],
    courierEvents: [],
    disputes: [],
    disputeEvidence: [],
    notifications: [],
    operatorActions: [],
    warehouses: [],
    restrictedItemRules: [],
    indexerState: [],
  };

  // Map relation names to their store arrays for include resolution
  const relStores: Record<string, any[]> = {
    evidence: store.disputeEvidence,
    media: store.itemMedia,
    trackingEvents: store.trackingEvents,
    courierEvents: store.courierEvents,
    submissions: store.itemSubmissions,
    claims: store.claims,
    notifications: store.notifications,
    openedBy: store.users,
    resolvedBy: store.users,
    submission: store.itemSubmissions,
    inventory: store.inventoryItems,
    shipment: store.shipments,
    receipt: store.itemReceipts,
  };

  const prisma: any = {
    _store: store,
    user: createModelMock(store.users, 'user', relStores),
    wallet: createModelMock(store.wallets, 'wallet', relStores),
    pool: createModelMock(store.pools, 'pool', relStores),
    itemSubmission: createModelMock(store.itemSubmissions, 'itemSubmission', relStores),
    itemMedia: createModelMock(store.itemMedia, 'itemMedia', relStores),
    itemReceipt: createModelMock(store.itemReceipts, 'itemReceipt', relStores),
    inventoryItem: createModelMock(store.inventoryItems, 'inventoryItem', relStores),
    inventoryStatusEvent: createModelMock(store.inventoryStatusEvents, 'inventoryStatusEvent', relStores),
    claim: createModelMock(store.claims, 'claim', relStores),
    claimConsumption: createModelMock(store.claimConsumptions, 'claimConsumption', relStores),
    shipment: createModelMock(store.shipments, 'shipment', relStores),
    trackingEvent: createModelMock(store.trackingEvents, 'trackingEvent', relStores),
    courierTask: createModelMock(store.courierTasks, 'courierTask', relStores),
    courierEvent: createModelMock(store.courierEvents, 'courierEvent', relStores),
    dispute: createModelMock(store.disputes, 'dispute', relStores),
    disputeEvidence: createModelMock(store.disputeEvidence, 'disputeEvidence', relStores),
    notification: createModelMock(store.notifications, 'notification', relStores),
    operatorAction: createModelMock(store.operatorActions, 'operatorAction', relStores),
    warehouse: createModelMock(store.warehouses, 'warehouse', relStores),
    restrictedItemRule: createModelMock(store.restrictedItemRules, 'restrictedItemRule', relStores),
    indexerState: createModelMock(store.indexerState, 'indexerState', relStores),
    $transaction: jest.fn(async (fn: any) => fn(prisma)),
  };

  return prisma;
}
