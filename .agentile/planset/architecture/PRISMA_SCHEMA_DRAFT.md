# Prisma Schema Draft

This is the draft schema to be translated into `prisma/schema.prisma` during Sprint 5.

```prisma
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

// ============================================================
// ENUMS
// ============================================================

enum UserRole {
  CONTRIBUTOR
  OPERATOR
  GRADER
  COURIER
  ADMIN
  SUPER_ADMIN
}

enum KycStatus {
  NONE
  PENDING
  APPROVED
  REJECTED
}

enum PoolHue {
  GREEN
  BLUE
  AMBER
  VIOLET
  TEAL
  RED
}

enum QualityTier {
  NEW
  REFURBISHED
  USED_GOOD
  COLLECTIBLE
}

enum PoolStatus {
  ACTIVE
  PAUSED
  SATURATED
}

enum SubmissionStatus {
  DRAFT
  SUBMITTED
  IN_TRANSIT
  RECEIVED
  GRADED
  ACCEPTED
  REJECTED
  QUARANTINED
}

enum InventoryStatus {
  AVAILABLE
  RESERVED
  OUTBOUND
  DELIVERED
  DISPUTED
  QUARANTINED
  LIQUIDATED
  DESTROYED
  RETURNED
}

enum ClaimStatus {
  PENDING
  ACTIVE
  CONSUMED
  EXPIRED
  DISPUTED
}

enum ShipmentDirection {
  INBOUND
  OUTBOUND
}

enum ShipmentStatus {
  CREATED
  PICKED_UP
  IN_TRANSIT
  DELIVERED
  EXCEPTION
}

enum CourierTaskStatus {
  POSTED
  ACCEPTED
  PICKUP_VERIFIED
  IN_TRANSIT_COURIER
  DELIVERED_COURIER
  COMPLETED
  FAILED
  DISPUTED_COURIER
}

enum CourierEventType {
  ACCEPTED
  PICKUP_SCAN
  MILESTONE
  DROPOFF_PROOF
  COMPLETED
  EXCEPTION
}

enum DisputeStatus {
  OPEN
  UNDER_REVIEW
  RESOLVED
  DENIED
}

enum DisputeResolution {
  REFUND_CLAIM
  REPLACEMENT
  DENY
  GOODWILL_CREDIT
}

enum DisputeObjectType {
  SUBMISSION
  RECEIPT
  INVENTORY
  CLAIM
  SHIPMENT
  COURIER_TASK
}

enum EvidenceType {
  PHOTO
  VIDEO
  TEXT
  DOCUMENT
}

enum MediaType {
  PHOTO
  VIDEO
}

enum NotificationChannel {
  EMAIL
  SMS
  PUSH
}

enum RestrictedRuleType {
  BLOCKED
  MANUAL_REVIEW
  RECALL
}

enum WarehouseStatus {
  ACTIVE_WH
  PAUSED_WH
  DECOMMISSIONED
}

// ============================================================
// MODELS
// ============================================================

model User {
  id            String    @id @default(uuid())
  email         String?   @unique
  phone         String?   @unique
  passwordHash  String
  displayName   String?
  role          UserRole  @default(CONTRIBUTOR)
  kycStatus     KycStatus @default(NONE)
  kycVerifiedAt DateTime?
  createdAt     DateTime  @default(now())
  updatedAt     DateTime  @updatedAt

  wallets             Wallet[]
  submissions         ItemSubmission[]
  claims              Claim[]
  courierTasks        CourierTask[]     @relation("CourierTasks")
  disputesOpened      Dispute[]         @relation("DisputesOpened")
  disputesResolved    Dispute[]         @relation("DisputesResolved")
  operatorActions     OperatorAction[]
  notifications       Notification[]
  taxProfile          TaxReportingProfile?
  inventoryReserved   InventoryItem[]   @relation("ReservedBy")
  receiptGraded       ItemReceipt[]     @relation("GradedBy")
  disputeEvidence     DisputeEvidence[] @relation("EvidenceSubmitter")
  inventoryEvents     InventoryStatusEvent[] @relation("EventActor")

  @@map("users")
}

model Wallet {
  id        String   @id @default(uuid())
  userId    String
  address   String   @unique
  chainId   Int
  verified  Boolean  @default(false)
  linkedAt  DateTime @default(now())

  user User @relation(fields: [userId], references: [id], onDelete: Cascade)

  @@map("wallets")
}

model Pool {
  id              String     @id @default(uuid())
  hue             PoolHue
  band            Int
  region          String
  qualityTier     QualityTier
  onChainPoolId   String?
  status          PoolStatus @default(ACTIVE)
  createdAt       DateTime   @default(now())

  rules           PoolRule[]
  submissions     ItemSubmission[]
  receipts        ItemReceipt[]
  inventoryItems  InventoryItem[]
  claims          Claim[]
  bandSnapshots   PricingBandSnapshot[]

  @@unique([hue, band, region, qualityTier])
  @@map("pools")
}

model PoolRule {
  id        String  @id @default(uuid())
  poolId    String
  ruleType  String
  ruleValue Json
  active    Boolean @default(true)

  pool Pool @relation(fields: [poolId], references: [id], onDelete: Cascade)

  @@map("pool_rules")
}

model ItemSubmission {
  id                   String           @id @default(uuid())
  userId               String
  category             String
  estimatedBand        Int
  conditionDescription String
  status               SubmissionStatus @default(DRAFT)
  targetPoolId         String?
  submissionHash       String?
  createdAt            DateTime         @default(now())
  updatedAt            DateTime         @updatedAt

  user       User            @relation(fields: [userId], references: [id])
  targetPool Pool?           @relation(fields: [targetPoolId], references: [id])
  media      ItemMedia[]
  receipt    ItemReceipt?
  shipments  Shipment[]      @relation("InboundShipments")
  qrLabel    QrLabel?

  @@index([userId])
  @@index([status])
  @@map("item_submissions")
}

model ItemMedia {
  id           String   @id @default(uuid())
  submissionId String
  mediaType    MediaType
  s3Key        String
  uploadedAt   DateTime @default(now())

  submission ItemSubmission @relation(fields: [submissionId], references: [id], onDelete: Cascade)

  @@map("item_media")
}

model ItemReceipt {
  id              String  @id @default(uuid())
  submissionId    String  @unique
  onChainTokenId  String?
  onChainTxHash   String?
  poolId          String
  finalBand       Int
  conditionGrade  String
  graderId        String
  gradingNotes    String?
  evidenceHash    String?
  restrictionFlags Json?
  createdAt       DateTime @default(now())

  submission    ItemSubmission @relation(fields: [submissionId], references: [id])
  pool          Pool           @relation(fields: [poolId], references: [id])
  grader        User           @relation("GradedBy", fields: [graderId], references: [id])
  inventoryItem InventoryItem?
  claims        Claim[]

  @@index([poolId])
  @@map("item_receipts")
}

model InventoryItem {
  id                  String          @id @default(uuid())
  receiptId           String          @unique
  warehouseId         String
  poolId              String
  binLocation         String?
  status              InventoryStatus @default(AVAILABLE)
  reservedById        String?
  reservedAt          DateTime?
  reservationExpires  DateTime?

  receipt          ItemReceipt            @relation(fields: [receiptId], references: [id])
  warehouse        Warehouse              @relation(fields: [warehouseId], references: [id])
  pool             Pool                   @relation(fields: [poolId], references: [id])
  reservedBy       User?                  @relation("ReservedBy", fields: [reservedById], references: [id])
  statusEvents     InventoryStatusEvent[]
  claimConsumption ClaimConsumption?
  outboundShipment Shipment?              @relation("OutboundShipments")

  @@index([poolId, status])
  @@index([warehouseId])
  @@map("inventory_items")
}

model InventoryStatusEvent {
  id             String          @id @default(uuid())
  inventoryId    String
  previousStatus InventoryStatus
  newStatus      InventoryStatus
  actorId        String
  evidenceHash   String?
  onChainAnchor  String?
  createdAt      DateTime        @default(now())

  inventory InventoryItem @relation(fields: [inventoryId], references: [id], onDelete: Cascade)
  actor     User          @relation("EventActor", fields: [actorId], references: [id])

  @@index([inventoryId])
  @@map("inventory_status_events")
}

model Claim {
  id              String      @id @default(uuid())
  userId          String
  poolId          String
  receiptId       String
  onChainTokenId  String?
  status          ClaimStatus @default(PENDING)
  activatedAt     DateTime?
  consumedAt      DateTime?

  user         User              @relation(fields: [userId], references: [id])
  pool         Pool              @relation(fields: [poolId], references: [id])
  receipt      ItemReceipt       @relation(fields: [receiptId], references: [id])
  consumption  ClaimConsumption?

  @@index([userId, status])
  @@index([poolId])
  @@map("claims")
}

model ClaimConsumption {
  id             String   @id @default(uuid())
  claimId        String   @unique
  inventoryId    String   @unique
  consumedAt     DateTime @default(now())
  onChainTxHash  String?

  claim     Claim         @relation(fields: [claimId], references: [id])
  inventory InventoryItem @relation(fields: [inventoryId], references: [id])

  @@map("claim_consumptions")
}

model Shipment {
  id                String           @id @default(uuid())
  submissionId      String?
  inventoryId       String?          @unique
  direction         ShipmentDirection
  carrier           String?
  trackingNumber    String?
  status            ShipmentStatus   @default(CREATED)
  estimatedDelivery DateTime?
  deliveredAt       DateTime?
  proofHash         String?
  createdAt         DateTime         @default(now())

  submission     ItemSubmission?  @relation("InboundShipments", fields: [submissionId], references: [id])
  inventory      InventoryItem?   @relation("OutboundShipments", fields: [inventoryId], references: [id])
  trackingEvents TrackingEvent[]
  courierTask    CourierTask?

  @@index([status])
  @@map("shipments")
}

model TrackingEvent {
  id         String   @id @default(uuid())
  shipmentId String
  eventType  String
  location   String?
  timestamp  DateTime
  rawData    Json?

  shipment Shipment @relation(fields: [shipmentId], references: [id], onDelete: Cascade)

  @@index([shipmentId])
  @@map("tracking_events")
}

model QrLabel {
  id                String   @id @default(uuid())
  submissionId      String   @unique
  qrData            String
  humanReadableCode String
  labelPdfKey       String?
  generatedAt       DateTime @default(now())

  submission ItemSubmission @relation(fields: [submissionId], references: [id])

  @@map("qr_labels")
}

model CourierTask {
  id               String            @id @default(uuid())
  shipmentId       String            @unique
  courierId        String?
  pickupLocation   Json
  dropoffLocation  Json
  fee              Decimal           @db.Decimal(10, 2)
  tip              Decimal?          @db.Decimal(10, 2)
  status           CourierTaskStatus @default(POSTED)
  timeWindowStart  DateTime
  timeWindowEnd    DateTime
  createdAt        DateTime          @default(now())

  shipment Shipment       @relation(fields: [shipmentId], references: [id])
  courier  User?          @relation("CourierTasks", fields: [courierId], references: [id])
  events   CourierEvent[]

  @@index([status])
  @@map("courier_tasks")
}

model CourierEvent {
  id        String           @id @default(uuid())
  taskId    String
  eventType CourierEventType
  proofHash String?
  location  Json?
  createdAt DateTime         @default(now())

  task CourierTask @relation(fields: [taskId], references: [id], onDelete: Cascade)

  @@index([taskId])
  @@map("courier_events")
}

model Dispute {
  id           String             @id @default(uuid())
  openedById   String
  objectType   DisputeObjectType
  objectId     String
  reason       String
  status       DisputeStatus      @default(OPEN)
  resolution   DisputeResolution?
  slaDeadline  DateTime
  resolvedAt   DateTime?
  resolvedById String?
  createdAt    DateTime           @default(now())

  openedBy   User              @relation("DisputesOpened", fields: [openedById], references: [id])
  resolvedBy User?             @relation("DisputesResolved", fields: [resolvedById], references: [id])
  evidence   DisputeEvidence[]

  @@index([status])
  @@index([objectType, objectId])
  @@map("disputes")
}

model DisputeEvidence {
  id           String       @id @default(uuid())
  disputeId    String
  submittedById String
  evidenceType EvidenceType
  s3Key        String?
  content      String?
  cidHash      String?
  submittedAt  DateTime     @default(now())

  dispute     Dispute @relation(fields: [disputeId], references: [id], onDelete: Cascade)
  submittedBy User    @relation("EvidenceSubmitter", fields: [submittedById], references: [id])

  @@map("dispute_evidence")
}

model Warehouse {
  id           String          @id @default(uuid())
  name         String
  region       String
  address      Json
  status       WarehouseStatus @default(ACTIVE_WH)
  capacity     Int
  currentCount Int             @default(0)

  inventoryItems InventoryItem[]

  @@map("warehouses")
}

model RestrictedItemRule {
  id              String             @id @default(uuid())
  categoryPattern String
  ruleType        RestrictedRuleType
  description     String
  active          Boolean            @default(true)
  source          String?

  @@map("restricted_item_rules")
}

model OperatorAction {
  id         String   @id @default(uuid())
  operatorId String
  actionType String
  targetType String
  targetId   String
  reason     String
  metadata   Json?
  createdAt  DateTime @default(now())

  operator User @relation(fields: [operatorId], references: [id])

  @@index([operatorId])
  @@index([targetType, targetId])
  @@map("operator_actions")
}

model Notification {
  id        String              @id @default(uuid())
  userId    String
  channel   NotificationChannel
  eventType String
  content   Json
  sentAt    DateTime?
  readAt    DateTime?

  user User @relation(fields: [userId], references: [id])

  @@index([userId, readAt])
  @@map("notifications")
}

model TaxReportingProfile {
  id     String @id @default(uuid())
  userId String @unique
  // Fields TBD based on jurisdictional requirements

  user User @relation(fields: [userId], references: [id])

  @@map("tax_reporting_profiles")
}

model PricingBandSnapshot {
  id        String   @id @default(uuid())
  poolId    String
  bandLow   Decimal  @db.Decimal(10, 2)
  bandHigh  Decimal  @db.Decimal(10, 2)
  capturedAt DateTime @default(now())

  pool Pool @relation(fields: [poolId], references: [id])

  @@index([poolId])
  @@map("pricing_band_snapshots")
}

model BuyOnSiteProgram {
  id              String  @id @default(uuid())
  category        String
  skuPattern      String?
  conditionWindow Json
  payoutPolicy    Json
  active          Boolean @default(true)

  @@map("buy_on_site_programs")
}
```
