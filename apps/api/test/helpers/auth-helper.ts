import { INestApplication } from '@nestjs/common';
import * as request from 'supertest';
import { randomUUID } from 'crypto';

export async function registerUser(
  app: INestApplication,
  overrides?: { email?: string; role?: string },
): Promise<{ token: string; userId: string }> {
  const email = overrides?.email || `test-${randomUUID()}@test.com`;

  const res = await request(app.getHttpServer())
    .post('/api/v1/auth/register')
    .send({
      email,
      password: 'TestPassword123!',
    });

  if (res.status !== 201) {
    throw new Error(`Registration failed: ${res.status} ${JSON.stringify(res.body)}`);
  }

  return {
    token: res.body.token,
    userId: res.body.userId,
  };
}

export function authHeader(token: string) {
  return { Authorization: `Bearer ${token}` };
}
