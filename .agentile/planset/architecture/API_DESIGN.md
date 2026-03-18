# API_DESIGN.md — Gradient Barter Protocol

## API Overview

RESTful API served by NestJS. All endpoints require authentication unless marked `[public]`. Role-based access control enforced via NestJS guards.

Base path: `/api/v1`

---

## Identity & Auth

### POST /auth/register
Create account with email or phone.
- Body: `{ email?, phone?, password }`
- Response: `{ userId, token }`

### POST /auth/login
- Body: `{ email | phone, password }`
- Response: `{ userId, token }`

### POST /auth/wallet/link
Link EVM wallet to user profile.
- Body: `{ address, chainId }`
- Response: `{ walletId, nonce }` (nonce for verification)

### POST /auth/wallet/verify
Verify wallet ownership via signed nonce.
- Body: `{ walletId, signature }`
- Response: `{ verified: true }`

### POST /auth/kyc/initiate
Start KYC flow for higher-value pool access.
- Response: `{ kycSessionId, redirectUrl }`

---

## Pools

### GET /pools `[public]`
List pools with filtering.
- Query: `?hue=&band=&region=&quality_tier=&status=`
- Response: `{ pools: [{ id, hue, band, region, qualityTier, status, availableCount }] }`

### GET /pools/:id `[public]`
Pool details including rules, inventory count, and restrictions.

### GET /pools/:id/inventory
List claimable items in a pool (requires active claim in this pool).
- Query: `?page=&limit=&sort=`
- Response: `{ items: [{ id, condition, photos, addedAt }], total, page }`

---

## Submissions

### POST /submissions
Create a new item submission.
- Body: `{ category, estimatedBand, conditionDescription }`
- Response: `{ submissionId, status: "draft" }`

### POST /submissions/:id/media
Upload item photos/video.
- Body: multipart/form-data (files)
- Response: `{ mediaIds: [...] }`

### POST /submissions/:id/submit
Finalize submission — runs restricted item screening.
- Response: `{ status: "submitted" | "blocked", blockReason? }`

### GET /submissions/:id
Get submission details and status.

### GET /submissions
List user's submissions.
- Query: `?status=&page=&limit=`

---

## Labels & QR

### GET /submissions/:id/label
Generate and return QR label.
- Response: `{ qrData, humanReadableCode, labelPdfUrl, packingInstructions }`

### GET /submissions/:id/label/pdf
Download printable PDF label.
- Response: application/pdf

---

## Warehouse (Operator role required)

### POST /warehouse/intake/scan
Scan inbound item QR.
- Body: `{ qrCode | humanReadableCode }`
- Response: `{ submissionId, submitterInfo, category, estimatedBand, media }`

### POST /warehouse/intake/:submissionId/grade
Complete intake grading.
- Body: `{ decision: "accept" | "reject" | "quarantine", finalBand, conditionGrade, notes, evidencePhotos[], binLocation }`
- Response: `{ receiptId, inventoryId?, status }`

### GET /warehouse/intake/queue
List pending intake items.
- Query: `?warehouseId=&status=&page=`

### POST /warehouse/inventory/:id/quarantine
Quarantine an inventory item.
- Body: `{ reason }`

---

## Claims

### GET /claims
List user's claims.
- Query: `?status=&poolId=&page=`
- Response: `{ claims: [{ id, poolId, status, activatedAt, poolHue, poolBand }] }`

### GET /claims/:id
Claim details including source receipt and available actions.

### POST /claims/:claimId/reserve
Reserve an inventory item against this claim.
- Body: `{ inventoryId }`
- Response: `{ reservationId, expiresAt }`

### POST /claims/:claimId/reserve/cancel
Cancel an active reservation (releases item and claim).

### POST /claims/:claimId/reserve/finalize
Finalize reservation — consumes claim, triggers outbound logistics.
- Body: `{ deliveryMethod, shippingAddress? }`
- Response: `{ shipmentId, estimatedDelivery }`

---

## Logistics

### GET /shipments
List user's shipments (inbound and outbound).
- Query: `?direction=&status=&page=`

### GET /shipments/:id
Shipment details with tracking timeline.

### GET /shipments/:id/tracking
Full tracking event history.

---

## Courier (Courier role required)

### GET /courier/tasks
Available delivery tasks.
- Query: `?region=&status=posted&page=`

### POST /courier/tasks/:id/accept
Accept a delivery task.

### POST /courier/tasks/:id/pickup
Confirm pickup with QR scan or PIN.
- Body: `{ qrCode | pin, photoProof? }`

### POST /courier/tasks/:id/milestone
Report in-transit milestone.
- Body: `{ location, notes? }`

### POST /courier/tasks/:id/deliver
Confirm delivery with proof.
- Body: `{ proofMethod: "qr" | "signature" | "photo" | "pin", proofData }`

### GET /courier/earnings
Courier earnings and payout history.

---

## Disputes

### POST /disputes
Open a dispute.
- Body: `{ objectType, objectId, reason }`
- Response: `{ disputeId, slaDeadline }`

### POST /disputes/:id/evidence
Submit evidence for a dispute.
- Body: multipart/form-data (files + text)

### GET /disputes
List user's disputes.
- Query: `?status=&page=`

### GET /disputes/:id
Dispute details with evidence and timeline.

---

## Disputes — Admin (Admin role required)

### GET /admin/disputes/queue
Review queue for open disputes.

### POST /admin/disputes/:id/resolve
Resolve a dispute.
- Body: `{ resolution: "refund_claim" | "replacement" | "deny" | "goodwill_credit", reason }`

---

## Admin (Admin role required)

### GET /admin/pools
List all pools with management data.

### POST /admin/pools
Create a new pool.
- Body: `{ hue, band, region, qualityTier, rules }`

### PATCH /admin/pools/:id
Update pool configuration.

### POST /admin/pools/:id/pause
Pause a pool.

### POST /admin/pools/:id/resume
Resume a paused pool.

### POST /admin/emergency/pause-all
Global emergency pause.
- Body: `{ reason }`

### GET /admin/audit-log
Operator action audit trail.
- Query: `?operatorId=&actionType=&from=&to=&page=`

---

## Analytics `[public for pool metrics, auth for user metrics]`

### GET /analytics/pools/:id
Pool health metrics: available inventory, fill rate, dwell time.

### GET /analytics/user/dashboard
User's contribution history, claims, reservations, deliveries.

---

## Notifications

### GET /notifications
User's notifications.
- Query: `?read=false&page=`

### PATCH /notifications/:id/read
Mark notification as read.

---

## API Conventions

- All responses wrapped in `{ data, meta?, error? }`
- Pagination: `?page=1&limit=20` → `meta: { total, page, limit, pages }`
- Errors: `{ error: { code, message, details? } }`
- Auth: Bearer token in Authorization header
- Rate limiting: Redis-backed, per-user, per-endpoint
- File uploads: Signed URL pattern (POST to get URL, PUT to upload)
- All timestamps: ISO 8601 UTC
- IDs: UUIDs
