import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import { PrismaModule } from './prisma/prisma.module';
import { AuthModule } from './auth/auth.module';
import { HealthModule } from './health/health.module';
import { SubmissionsModule } from './submissions/submissions.module';
import { WarehouseModule } from './warehouse/warehouse.module';
import { PoolsModule } from './pools/pools.module';
import { ClaimsModule } from './claims/claims.module';
import { ShipmentsModule } from './shipments/shipments.module';
import { NotificationsModule } from './notifications/notifications.module';
import { DeliveryModule } from './delivery/delivery.module';
import { CourierModule } from './courier/courier.module';
import { DisputesModule } from './disputes/disputes.module';

@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    PrismaModule,
    AuthModule,
    HealthModule,
    SubmissionsModule,
    WarehouseModule,
    PoolsModule,
    ClaimsModule,
    ShipmentsModule,
    NotificationsModule,
    DeliveryModule,
    CourierModule,
    DisputesModule,
  ],
})
export class AppModule {}
