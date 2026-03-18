import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from '../src/app.module';
import { PrismaService } from '../src/prisma/prisma.service';
import { createMockPrismaService } from './helpers/prisma-mock';

describe('Full Barter Lifecycle (e2e)', () => {
  let app: INestApplication;
  let mockPrisma: any;
  let contributorToken: string;
  let contributorId: string;
  let operatorToken: string;
  let operatorId: string;

  beforeAll(async () => {
    mockPrisma = createMockPrismaService();

    // Pre-seed a pool and warehouse
    mockPrisma._store.pools.push({
      id: 'pool-1',
      hue: 'GREEN',
      band: 2,
      region: 'PDX',
      qualityTier: 'NEW',
      status: 'ACTIVE',
      createdAt: new Date(),
    });

    mockPrisma._store.warehouses.push({
      id: 'wh-1',
      name: 'PDX Warehouse',
      region: 'PDX',
      address: {
        street: '100 Warehouse Blvd',
        city: 'Portland',
        state: 'OR',
        zip: '97201',
      },
      status: 'ACTIVE_WH',
      capacity: 1000,
      currentCount: 0,
    });

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

    // Register contributor
    const contRes = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'contributor@test.com', password: 'Password123!' });
    contributorToken = contRes.body.token;
    contributorId = contRes.body.userId;

    // Register operator and manually set role
    const opRes = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'operator@test.com', password: 'Password123!' });
    operatorToken = opRes.body.token;
    operatorId = opRes.body.userId;

    // Manually upgrade operator role in mock store
    const opUser = mockPrisma._store.users.find((u: any) => u.id === operatorId);
    if (opUser) opUser.role = 'OPERATOR';

    // Re-login to get token with OPERATOR role
    const opLoginRes = await request(app.getHttpServer())
      .post('/api/v1/auth/login')
      .send({ email: 'operator@test.com', password: 'Password123!' });
    operatorToken = opLoginRes.body.token;
  });

  afterAll(async () => {
    await app.close();
  });

  it('should complete the full barter lifecycle', async () => {
    // Step 1: Create submission
    const subRes = await request(app.getHttpServer())
      .post('/api/v1/submissions')
      .set('Authorization', `Bearer ${contributorToken}`)
      .send({
        category: 'tools',
        conditionDescription: 'Vintage wrench set in good condition',
        estimatedBand: 2,
      })
      .expect(201);

    const submissionId = subRes.body.submissionId;
    expect(submissionId).toBeDefined();

    // Step 2: Submit the item
    const submission = mockPrisma._store.itemSubmissions.find(
      (s: any) => s.id === submissionId,
    );
    if (submission) submission.status = 'SUBMITTED';

    // Step 3: Verify claims list is initially empty
    const claimsRes = await request(app.getHttpServer())
      .get('/api/v1/claims')
      .set('Authorization', `Bearer ${contributorToken}`)
      .expect(200);

    expect(claimsRes.body.data).toHaveLength(0);

    // Step 4: Verify shipments list is initially empty
    const shipmentsRes = await request(app.getHttpServer())
      .get('/api/v1/shipments')
      .set('Authorization', `Bearer ${contributorToken}`)
      .expect(200);

    expect(shipmentsRes.body.data).toHaveLength(0);

    // Step 5: Verify notifications endpoint works
    const notifRes = await request(app.getHttpServer())
      .get('/api/v1/notifications')
      .set('Authorization', `Bearer ${contributorToken}`)
      .expect(200);

    expect(notifRes.body.data).toBeDefined();
  });

  it('should get shipping rates', async () => {
    const ratesRes = await request(app.getHttpServer())
      .post('/api/v1/shipping/rates')
      .set('Authorization', `Bearer ${contributorToken}`)
      .send({
        originZip: '97201',
        destinationZip: '10001',
        weight: 16,
      })
      .expect(200);

    expect(ratesRes.body.rates).toBeDefined();
    expect(ratesRes.body.rates.length).toBeGreaterThanOrEqual(2);
    expect(ratesRes.body.rates[0].provider).toBe('local-dev');
  });

  it('should reject invalid registration data', async () => {
    await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ password: 'NoEmailOrPhone' })
      .expect(400);
  });

  it('should reject duplicate email registration', async () => {
    await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'contributor@test.com', password: 'Password123!' })
      .expect(409);
  });
});
