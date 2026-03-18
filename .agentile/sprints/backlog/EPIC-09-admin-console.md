# Epic 9: Admin Console & Governance

**Priority**: Medium
**Size**: L
**Scope Reference**: SCOPE.md — Feature 11 (Admin Console)
**Phase**: Spans multiple sprints (core admin in Sprint 5, full in Phase 4)

## Stories

### Story 9.1: Pool Management
As an admin, I want to configure pools, thresholds, and rules.

**Acceptance Criteria**
- Role-based admin access required
- Create/edit/pause/resume pools
- Pool rules editable with audit logging
- All changes require reason capture

### Story 9.2: Emergency Controls
As a super admin, I want emergency pause controls so I can stop operations if needed.

**Acceptance Criteria**
- Emergency pause by pool (on-chain + off-chain)
- Global emergency pause
- Pause requires reason capture
- Resume requires super admin approval
- All pause/resume actions audit-logged

### Story 9.3: Audit Trail
As an admin, I want to see a complete audit log of all operator actions.

**Acceptance Criteria**
- All operator actions logged with: actor, action, target, reason, timestamp
- Filterable by operator, action type, date range
- Immutable log (append-only)
- Exportable for compliance
