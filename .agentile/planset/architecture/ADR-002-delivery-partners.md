# ADR-002: Delivery Partner Selection

**Status**: Accepted
**Date**: 2026-03-06
**Decision Makers**: Protocol operator (SaulBuilds)

## Context

The Gradient Barter Protocol needs third-party delivery for v1 (community courier mode is Phase 3). We need a partner that supports:
- On-demand or scheduled pickup from warehouses
- Last-mile delivery to claim holders
- Proof-of-delivery capture
- API integration for status tracking
- Reasonable cost for item-sized packages

## Options Evaluated

### Option A: Uber Direct (Uber Daas)
| Criteria | Assessment |
|----------|-----------|
| API maturity | High — well-documented REST API |
| Coverage | Major US metros, expanding |
| Cost model | Per-delivery fee based on distance |
| POD support | Photo proof, signature available |
| Integration complexity | Medium — webhook-based status updates |
| Package size limits | Standard — not large/heavy items |
| Pros | Fast delivery, real-time tracking, established brand trust |
| Cons | Metro-only coverage, premium pricing, less control over courier |

### Option B: DoorDash Drive
| Criteria | Assessment |
|----------|-----------|
| API maturity | High — Drive API for businesses |
| Coverage | Broad US coverage including suburbs |
| Cost model | Per-delivery fee, volume discounts |
| POD support | Photo proof |
| Integration complexity | Medium — similar to Uber |
| Package size limits | Standard |
| Pros | Broader coverage than Uber Direct, volume pricing |
| Cons | Primarily food-delivery brand (perception), less package handling experience |

### Option C: Standard Carriers (UPS/FedEx/USPS)
| Criteria | Assessment |
|----------|-----------|
| API maturity | High — ShipEngine/EasyPost abstractions available |
| Coverage | Nationwide including rural |
| Cost model | Weight/size-based, zone pricing |
| POD support | Signature, tracking events |
| Integration complexity | Low — mature API aggregators exist |
| Package size limits | Very flexible |
| Pros | Universal coverage, handles any size, established logistics |
| Cons | Slower (1-5 day delivery), no on-demand option, less flexible pickup |

## Decision

### Primary Partner (v1): Standard Carriers via ShipEngine/EasyPost abstraction

**Rationale**:
1. **Coverage**: Nationwide from day one, including non-metro areas where items may ship to/from
2. **Package flexibility**: Handles the diverse item sizes in the protocol (tools, electronics, appliances)
3. **Cost**: More predictable pricing for the variety of items; Uber/DoorDash premium pricing doesn't make sense for non-urgent barter trades
4. **Integration**: ShipEngine or EasyPost provide a single abstraction over UPS, FedEx, USPS — matches our own abstraction layer requirement
5. **POD**: Tracking events and signature-on-delivery satisfy proof requirements
6. **User expectation**: Users understand "ship via UPS" for physical goods; same-day delivery not required for barter

### Fallback: Uber Direct (for local same-day in metros)
- Available as an option for users in supported metros who want faster delivery
- Higher fee passed to user
- Not the default path

### Future: Community Courier (Phase 3, Sprint 11)
- Built after the standard carrier path is stable
- For local, specialized, or lower-density deliveries

## Abstraction Layer Design

```typescript
interface DeliveryProvider {
  createShipment(params: ShipmentParams): Promise<ShipmentResult>;
  getTrackingEvents(shipmentId: string): Promise<TrackingEvent[]>;
  cancelShipment(shipmentId: string): Promise<void>;
  getRate(params: RateParams): Promise<Rate[]>;
}

class ShipEngineProvider implements DeliveryProvider { ... }
class UberDirectProvider implements DeliveryProvider { ... }
class CommunityProvider implements DeliveryProvider { ... } // Phase 3
```

The logistics service selects the provider based on delivery method, region, and user preference. Provider-specific logic is isolated in adapters. Core reservation and claim logic never touches provider details.

## Consequences

- Users will see 1-5 day delivery times (not same-day) as default
- Shipping costs are predictable and carrier-competitive
- Integration effort is low (ShipEngine/EasyPost handles multi-carrier complexity)
- Same-day option available in metros via Uber Direct at premium
- Switching carriers or adding new ones requires only a new adapter

## Supersedes

None (first decision on this topic).
