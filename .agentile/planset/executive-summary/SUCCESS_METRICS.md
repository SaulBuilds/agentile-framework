# Success Metrics & KPIs

## Pilot Targets

These KPIs define success for the v1 pilot launch. They are measured continuously and reported per sprint.

| KPI | Formula | Pilot Target | Frequency |
|-----|---------|-------------|-----------|
| Intake acceptance rate | Accepted items / total graded items | >= 80% | Weekly |
| Average time to claim unlock | Time from item received at warehouse to claim active | <= 48 hours | Weekly |
| Pool fill rate | Items added per pool per week | >= 5 items/pool/week | Weekly |
| Average dwell time | Time from item available to item reserved | <= 14 days | Weekly |
| Reservation-to-delivery success | Successfully delivered / total finalized reservations | >= 95% | Weekly |
| Dispute rate | Disputes opened / total completed trades | <= 5% | Monthly |
| Courier completion rate | Successfully completed tasks / total accepted tasks | >= 95% | Weekly |
| Damaged-in-transit rate | Items damaged during delivery / total deliveries | <= 2% | Monthly |
| Shrinkage rate | Items unaccounted for in warehouse / total items received | <= 1% | Monthly |
| Support tickets per 100 trades | Support tickets / (completed trades / 100) | <= 10 | Monthly |

## Health Indicators

### Green (Healthy)
- All KPIs at or better than target
- Quarantine queue < 5% of total inventory
- No unresolved high-severity incidents
- Pool saturation < 80% across all pools

### Yellow (Watch)
- 1-2 KPIs below target
- Quarantine queue 5-10% of total inventory
- Dwell time trending upward
- Any KPI declining for 2+ consecutive weeks

### Red (Action Required)
- 3+ KPIs below target
- Quarantine queue > 10%
- Shrinkage rate above 2%
- Unresolved safety incidents
- Pool saturation > 90% with no expansion plan

## Measurement Infrastructure

- **On-chain**: Claim mints, burns, reservations, settlements (indexed by event indexer)
- **Warehouse**: Intake events, grading decisions, bin assignments, pick/pack events
- **Logistics**: Pickup, transit, delivery proof events from carrier APIs
- **Backend**: User actions, support tickets, dispute filings, notification delivery rates

## Reporting Cadence

| Report | Audience | Frequency |
|--------|----------|-----------|
| Daily ops dashboard | Warehouse supervisors | Daily |
| Weekly metrics summary | Operations team | Weekly |
| Sprint metrics | Product/engineering (Agentile sprint report) | Per sprint |
| Monthly business review | Leadership | Monthly |
| Pilot assessment | Stakeholders | End of Phase 2 (Sprint 9) |
