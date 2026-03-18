import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from '../src/app.module';
import { PrismaService } from '../src/prisma/prisma.service';
import { createMockPrismaService } from './helpers/prisma-mock';

describe('App (e2e)', () => {
  let app: INestApplication;

  beforeAll(async () => {
    const moduleFixture: TestingModule = await Test.createTestingModule({
      imports: [AppModule],
    })
      .overrideProvider(PrismaService)
      .useValue(createMockPrismaService())
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
  });

  afterAll(async () => {
    await app.close();
  });

  it('GET /api/v1/health should return ok', () => {
    return request(app.getHttpServer())
      .get('/api/v1/health')
      .expect(200)
      .expect((res) => {
        expect(res.body.status).toBe('ok');
      });
  });

  it('POST /api/v1/auth/register should create user and return token', async () => {
    const res = await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({
        email: 'e2e-test@example.com',
        password: 'SecurePassword123!',
      })
      .expect(201);

    expect(res.body.userId).toBeDefined();
    expect(res.body.token).toBeDefined();
  });

  it('POST /api/v1/auth/login should return token for valid credentials', async () => {
    // First register
    await request(app.getHttpServer())
      .post('/api/v1/auth/register')
      .send({
        email: 'login-test@example.com',
        password: 'SecurePassword123!',
      });

    // Then login
    const res = await request(app.getHttpServer())
      .post('/api/v1/auth/login')
      .send({
        email: 'login-test@example.com',
        password: 'SecurePassword123!',
      })
      .expect(201);

    expect(res.body.token).toBeDefined();
  });

  it('should reject unauthenticated requests to protected endpoints', async () => {
    await request(app.getHttpServer())
      .get('/api/v1/claims')
      .expect(401);
  });

  it('should reject requests with invalid token', async () => {
    await request(app.getHttpServer())
      .get('/api/v1/claims')
      .set('Authorization', 'Bearer invalid-token')
      .expect(401);
  });
});
