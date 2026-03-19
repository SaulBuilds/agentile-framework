# ADR-<NUMBER>: <Title>

> Architecture Decision Record. Once accepted, this document is append-only.
> Place in `.agentile/planset/adr/ADR-<NUMBER>-<slug>.md`.

---

## Metadata

| Field | Value |
|-------|-------|
| **ADR Number** | ADR-<NUMBER> |
| **Date** | YYYY-MM-DD |
| **Status** | PROPOSED / ACCEPTED / DEPRECATED / SUPERSEDED BY ADR-<N> |
| **Author** | <name> |
| **Sprint** | S-<ID> (if applicable) |

---

## Context

<What is the problem or situation that motivates this decision? What constraints exist? What forces are at play?>

---

## Decision

<What is the architectural decision? State it clearly and concisely.>

**We will <decision statement>.**

---

## Rationale

<Why was this decision made? What alternatives were considered and why were they rejected?>

### Alternatives Considered

| Alternative | Pros | Cons | Why Rejected |
|-------------|------|------|--------------|
| <Option A> | <pros> | <cons> | <reason> |
| <Option B> | <pros> | <cons> | <reason> |
| <Chosen option> | <pros> | <cons> | **Selected** |

---

## Consequences

### Positive
- <Positive consequence 1>
- <Positive consequence 2>

### Negative
- <Negative consequence 1 and how it will be mitigated>
- <Negative consequence 2 and how it will be mitigated>

### Neutral
- <Neutral observation>

---

## Affected Components

| Component | Impact |
|-----------|--------|
| `<module or crate>` | <how it is affected> |

---

## Compliance

- [ ] Discussed with team / documented in sprint
- [ ] Consistent with CONFIG.md canonical values
- [ ] Does not violate any CORE_RULES
- [ ] Formal verification updated (if consensus/state machine change)

---

## References

- <Link to related ADR, issue, PR, or external resource>
- <Link to TLA+ spec if applicable>

---

## Revision History

| Date | Author | Change |
|------|--------|--------|
| YYYY-MM-DD | <name> | Initial proposal |
