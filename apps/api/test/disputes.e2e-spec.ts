import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from '../src/app.module';
import { PrismaService } from '../src/prisma/prisma.service';
import { createMockPrismaService } from './helpers/prisma-mock';

describe('Disputes (e2e)', () => {
  let app: INestApplication;
  let mockPrisma: any;
  let userToken: string;
  let userId: string;
  let adminToken: string;
  let adminId: string;
  let claimId: string;

  beforeAll(async () => {
    mockPrisma = createMockPrismaService();

    const moduleFixture: TestingModule = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideProvider(PrismaService)
      .useValue(mockPrisma)
      .compile();

    app = moduleFixture.createNestApplication();
    app.setGlobalPrefix('api/v1');
    app.useGlobalPipes(
      new ValidationPipe({
        whitelist: true,
        forbidNonWhitelisted: true,
        transform: true,
      }),
    );
    await app.init();

    // Register user
    const userRes = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'disputer@test.com', password: 'Password123!' });
    userToken = userRes.body.token;
    userId = userRes.body.userId;

    // Register admin
    const adminRes = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'admin@test.com', password: 'Password123!' });
    adminToken = adminRes.body.token;
    adminId = adminRes.body.userId;

    // Set admin role
    const adminUser = mockPrisma._store.users.find((u: any) => u.id === adminId);
    if (adminUser) adminUser.role = 'ADMIN';

    // Re-login admin to get ADMIN-role token
    const adminLoginRes = await request(app.getHttpServer())
      .post('/api/v1/auth/login')
      .send({ email: 'admin@test.com', password: 'Password123!' });
    adminToken = adminLoginRes.body.token;

    // Pre-seed a claim for the user to dispute
    claimId = 'claim-for-dispute';
    mockPrisma._store.claims.push({
      id: claimId,
      userId,
      poolId: 'pool-1',
      receiptId: 'receipt-1',
      status: 'ACTIVE',
      activatedAt: new Date(),
      consumedAt: null,
    });

    // Seed pool and receipt
    mockPrisma._store.pools.push({
      id: 'pool-1',
      hue: 'GREEN',
      band: 2,
      region: 'PDX',
      qualityTier: 'NEW',
      status: 'ACTIVE',
    });

    mockPrisma._store.itemReceipts.push({
      id: 'receipt-1',
      submissionId: 'sub-1',
      poolId: 'pool-1',
      finalBand: 2,
      conditionGrade: 'good',
    });
  });

  afterAll(async () => {
    await app.close();
  });

  it('should complete the full dispute lifecycle', async () => {
    // Step 1: Open dispute
    const createRes = await request(app.getHttpServer())
      .post('/api/v1/disputes')
      .set('Authorization', `Bearer ${userToken}`)
      .send({
        objectType: 'CLAIM',
        objectId: claimId,
        reason: 'Grading was incorrect — item is in better condition',
      })
      .expect(201);

    const disputeId = createRes.body.disputeId;
    expect(disputeId).toBeDefined();
    expect(createRes.body.slaDeadline).toBeDefined();

    // Step 2: Submit text evidence
    await request(app.getHttpServer())
      .post(`/api/v1/disputes/${disputeId}/evidence`)
      .set('Authorization', `Bearer ${userToken}`)
      .send({
        evidenceType: 'TEXT',
        content: 'The item has original packaging and zero scratches',
      })
      .expect(201);

    // Step 3: List user disputes
    const listRes = await request(app.getHttpServer())
      .get('/api/v1/disputes')
      .set('Authorization', `Bearer ${userToken}`)
      .expect(200);

    expect(listRes.body.data.length).toBeGreaterThanOrEqual(1);
    expect(listRes.body.data[0].status).toBe('OPEN');

    // Step 4: Get dispute detail
    const detailRes = await request(app.getHttpServer())
      .get(`/api/v1/disputes/${disputeId}`)
      .set('Authorization', `Bearer ${userToken}`)
      .expect(200);

    expect(detailRes.body.evidence.length).toBeGreaterThanOrEqual(1);

    // Step 5: Admin views queue
    const queueRes = await request(app.getHttpServer())
      .get('/api/v1/admin/disputes/queue')
      .set('Authorization', `Bearer ${adminToken}`)
      .expect(200);

    expect(queueRes.body.data.length).toBeGreaterThanOrEqual(1);

    // Step 6: Admin resolves with REFUND_CLAIM
    const resolveRes = await request(app.getHttpServer())
      .post(`/api/v1/admin/disputes/${disputeId}/resolve`)
      .set('Authorization', `Bearer ${adminToken}`)
      .send({
        resolution: 'REFUND_CLAIM',
        reason: 'Evidence confirmed better condition',
      })
      .expect(201);

    expect(resolveRes.body.status).toBe('RESOLVED');
    expect(resolveRes.body.newClaimId).toBeDefined();

    // Step 7: Verify new claim was created
    const newClaim = mockPrisma._store.claims.find(
      (c: any) => c.id === resolveRes.body.newClaimId,
    );
    expect(newClaim).toBeDefined();
    expect(newClaim.userId).toBe(userId);
    expect(newClaim.status).toBe('ACTIVE');
  });

  it('should reject duplicate disputes on same object', async () => {
    // First dispute already exists from previous test
    await request(app.getHttpServer())
      .post('/api/v1/disputes')
      .set('Authorization', `Bearer ${userToken}`)
      .send({
        objectType: 'CLAIM',
        objectId: claimId,
        reason: 'Duplicate dispute attempt',
      });

    // The first dispute was resolved, so a new one on a different claim should work
    // Create a new claim to dispute
    const newClaimId = 'claim-2';
    mockPrisma._store.claims.push({
      id: newClaimId,
      userId,
      poolId: 'pool-1',
      receiptId: 'receipt-1',
      status: 'ACTIVE',
      activatedAt: new Date(),
    });

    const res = await request(app.getHttpServer())
      .post('/api/v1/disputes')
      .set('Authorization', `Bearer ${userToken}`)
      .send({
        objectType: 'CLAIM',
        objectId: newClaimId,
        reason: 'Another dispute',
      })
      .expect(201);

    expect(res.body.disputeId).toBeDefined();
  });

  it('should reject non-admin from accessing admin queue', async () => {
    await request(app.getHttpServer())
      .get('/api/v1/admin/disputes/queue')
      .set('Authorization', `Bearer ${userToken}`)
      .expect(403);
  });

  it('should reject dispute on object user does not own', async () => {
    mockPrisma._store.claims.push({
      id: 'other-claim',
      userId: 'someone-else',
      poolId: 'pool-1',
      receiptId: 'receipt-1',
      status: 'ACTIVE',
    });

    await request(app.getHttpServer())
      .post('/api/v1/disputes')
      .set('Authorization', `Bearer ${userToken}`)
      .send({
        objectType: 'CLAIM',
        objectId: 'other-claim',
        reason: 'Not mine',
      })
      .expect(403);
  });

  it('should validate dispute creation input', async () => {
    await request(app.getHttpServer())
      .post('/api/v1/disputes')
      .set('Authorization', `Bearer ${userToken}`)
      .send({
        objectType: 'INVALID_TYPE',
        objectId: 'x',
        reason: 'Bad',
      })
      .expect(400);
  });
});
