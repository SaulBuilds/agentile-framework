import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from '../src/app.module';
import { PrismaService } from '../src/prisma/prisma.service';
import { createMockPrismaService } from './helpers/prisma-mock';

describe('Courier (e2e)', () => {
  let app: INestApplication;
  let mockPrisma: any;
  let courierToken: string;
  let courierId: string;
  let taskId: string;

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

    // Register courier
    const courierRes = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'courier@test.com', password: 'Password123!' });
    courierToken = courierRes.body.token;
    courierId = courierRes.body.userId;

    // Set courier role and approval
    const courierUser = mockPrisma._store.users.find(
      (u: any) => u.id === courierId,
    );
    if (courierUser) {
      courierUser.role = 'COURIER';
      courierUser.kycStatus = 'APPROVED';
    }

    // Re-login to get COURIER role token
    const loginRes = await request(app.getHttpServer())
      .post('/api/v1/auth/login')
      .send({ email: 'courier@test.com', password: 'Password123!' });
    courierToken = loginRes.body.token;

    // Pre-seed a shipment and courier task
    mockPrisma._store.shipments.push({
      id: 'ship-courier-1',
      direction: 'OUTBOUND',
      status: 'CREATED',
      inventoryId: 'inv-1',
    });

    taskId = 'task-e2e-1';
    mockPrisma._store.courierTasks.push({
      id: taskId,
      shipmentId: 'ship-courier-1',
      courierId: null,
      pickupLocation: {
        street: '100 Warehouse Blvd',
        city: 'Portland',
        state: 'OR',
        region: 'OR',
      },
      dropoffLocation: {
        street: '456 Main St',
        city: 'Portland',
        state: 'OR',
        region: 'OR',
      },
      fee: 9.99,
      tip: null,
      status: 'POSTED',
      timeWindowStart: new Date(),
      timeWindowEnd: new Date(Date.now() + 4 * 3600000),
      createdAt: new Date(),
    });
  });

  afterAll(async () => {
    await app.close();
  });

  it('should list available tasks with redacted addresses', async () => {
    const res = await request(app.getHttpServer())
      .get('/api/v1/courier/tasks')
      .set('Authorization', `Bearer ${courierToken}`)
      .expect(200);

    expect(res.body.data.length).toBeGreaterThanOrEqual(1);
    // Addresses should be redacted (no street)
    expect(res.body.data[0].pickupLocation).not.toHaveProperty('street');
    expect(res.body.data[0].pickupLocation.city).toBe('Portland');
  });

  it('should accept a task', async () => {
    const res = await request(app.getHttpServer())
      .post(`/api/v1/courier/tasks/${taskId}/accept`)
      .set('Authorization', `Bearer ${courierToken}`)
      .expect(201);

    expect(res.body.status).toBe('ACCEPTED');
  });

  it('should confirm pickup with QR code', async () => {
    const res = await request(app.getHttpServer())
      .post(`/api/v1/courier/tasks/${taskId}/pickup`)
      .set('Authorization', `Bearer ${courierToken}`)
      .send({ qrCode: 'QR-SCAN-E2E-TEST' })
      .expect(201);

    expect(res.body.status).toBe('PICKUP_VERIFIED');
    expect(res.body.proofHash).toBeDefined();
    expect(res.body.proofHash.length).toBe(64);
  });

  it('should report a milestone', async () => {
    const res = await request(app.getHttpServer())
      .post(`/api/v1/courier/tasks/${taskId}/milestone`)
      .set('Authorization', `Bearer ${courierToken}`)
      .send({
        location: { lat: 45.5, lng: -122.6 },
        notes: 'Passing downtown',
      })
      .expect(201);

    expect(res.body.status).toBe('IN_TRANSIT_COURIER');
  });

  it('should confirm delivery with proof', async () => {
    const res = await request(app.getHttpServer())
      .post(`/api/v1/courier/tasks/${taskId}/deliver`)
      .set('Authorization', `Bearer ${courierToken}`)
      .send({
        proofMethod: 'photo',
        proofData: 'base64-delivery-photo-data',
      })
      .expect(201);

    expect(res.body.status).toBe('DELIVERED_COURIER');
    expect(res.body.proofHash).toBeDefined();
  });

  it('should get courier earnings', async () => {
    const res = await request(app.getHttpServer())
      .get('/api/v1/courier/earnings')
      .set('Authorization', `Bearer ${courierToken}`)
      .expect(200);

    expect(res.body.data).toBeDefined();
    expect(res.body.summary).toBeDefined();
    expect(res.body.summary.totalEarned).toBeDefined();
  });

  it('should reject non-courier users', async () => {
    // Register a regular user
    const userRes = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({ email: 'regular@test.com', password: 'Password123!' });

    await request(app.getHttpServer())
      .get('/api/v1/courier/tasks')
      .set('Authorization', `Bearer ${userRes.body.token}`)
      .expect(403);
  });
});
