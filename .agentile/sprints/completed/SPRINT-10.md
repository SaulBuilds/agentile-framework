# Sprint 10 — Third-Party Courier Integration

**Start Date**: 2026-03-06
**End Date**: 2026-03-06
**Status**: COMPLETED

## Sprint Goal
Implement the delivery provider abstraction layer with standard carrier and Uber Direct adapters. Add rate calculation, enhance finalize with carrier shipment creation, and map provider tracking events to internal model.

## Items

### 1. Delivery Provider Abstraction
- **Priority**: High
- **Size**: L
- **Status**: DONE
- **Notes**: DeliveryProvider interface with createShipment, getTrackingEvents, cancelShipment, getRates. DeliveryService provider registry with auto-registration based on env vars. Three adapters: StandardCarrierProvider (ShipEngine), UberDirectProvider (Uber Direct with OAuth2 token caching), LocalDevProvider (no API keys needed).

### 2. Rate Calculation Endpoint
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: POST /shipping/rates — accepts originZip, destinationZip, weight, dimensions. Queries all registered providers in parallel and returns sorted rates. DeliveryController with validation DTOs.

### 3. Enhanced Finalize with Carrier Integration
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: ClaimsService.finalize() now calls DeliveryService.getProviderForMethod() to select provider, creates carrier shipment with tracking number, stores carrier/trackingNumber/estimatedDelivery on Shipment record. Handles provider errors gracefully (falls back to shipment without carrier info).

### 4. Delivery Exception Handling
- **Priority**: Medium
- **Size**: S
- **Status**: DONE
- **Notes**: ShipmentsService.addTrackingEvent() detects EXCEPTION status and auto-notifies the shipment owner via NotificationsService. Finds owner via submission (inbound) or inventory.reservedById (outbound). Added syncProviderTracking() for pulling provider events.

### 5. Tests
- **Priority**: High
- **Size**: M
- **Status**: DONE
- **Notes**: 23 new tests across delivery.service.spec.ts (19 tests covering provider registry, rate calculation, shipment creation, tracking, cancellation, LocalDevProvider), updated claims.service.spec.ts (carrier integration + provider failure fallback), updated shipments.service.spec.ts (exception notification + provider sync).

## Exit Criteria
- [x] DeliveryProvider interface defined with all methods
- [x] Standard carrier adapter implements interface
- [x] Uber Direct adapter implements interface
- [x] Local dev adapter works without API keys
- [x] Rate calculation returns quotes from providers
- [x] Finalize creates carrier shipment with tracking
- [x] Delivery exceptions mapped to internal model
- [x] All tests pass

## Metrics (updated at sprint end)
- Features completed: 5/5
- Tests passing: 166/166
- Blockers encountered: 0
- Blockers resolved: 0
