# TLA+ Formal Verification Specs

Formal specifications verifying critical state machine invariants in the Gradient Barter Protocol.

## Specifications

| Spec | What it verifies |
|------|-----------------|
| `ClaimLifecycle` | No skip from PENDING to CONSUMED, no double-consume, CONSUMED only reachable from RESERVED |
| `InventoryLifecycle` | Only valid status transitions, no backward moves from SHIPPED, AVAILABLE only from valid predecessors |
| `ReservationTimeout` | Expired reservations cannot be finalized, finalized items must have been reserved |
| `CourierTaskLifecycle` | Ordered POSTED->ACCEPTED->PICKUP->TRANSIT->DELIVERED->COMPLETED, no backward moves, courier consistency |
| `DisputeLifecycle` | RESOLVED/DENIED are terminal, no evidence on terminal disputes, no duplicate open disputes |
| `ReservationExclusion` | No item reserved by two users simultaneously, available items are unreserved |

## Running with TLC Model Checker

### Prerequisites

Install the TLA+ tools:
```bash
# Option 1: Download TLA+ Toolbox (GUI)
# https://github.com/tlaplus/tlaplus/releases

# Option 2: Install tla2tools.jar for CLI
curl -LO https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar
```

### Run a single spec
```bash
java -jar tla2tools.jar -config ClaimLifecycle.cfg ClaimLifecycle.tla
```

### Run all specs
```bash
for spec in ClaimLifecycle InventoryLifecycle ReservationTimeout CourierTaskLifecycle DisputeLifecycle ReservationExclusion; do
  echo "=== Checking $spec ==="
  java -jar tla2tools.jar -config ${spec}.cfg ${spec}.tla
  echo ""
done
```

### Expected output
Each spec should complete with:
```
Model checking completed. No error has been found.
```

## State Spaces

All specs use small model constants (2-3 items, 2-3 users) to keep model checking tractable while still exercising all reachable states and transitions.
