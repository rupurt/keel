---
# system-managed
id: VEviVV4zQ
index: 4
status: accepted
decided_at: 2026-03-25T17:47:01
supersedes: []
superseded-by: null
# authored
title: Operational Scope Consolidates Into Watch
mission: VE3I9QfPV
context: null
applies-to:
  - VE3IAG4jZ
---

# Operational Scope Consolidates Into Watch

## Status

**Accepted** — This decision records the watch consolidation that retired the legacy project-operations epics.

## Context

Recurring operational routines originally materialized into dedicated project-operations epics. Once the `Standard Operations` watch became the canonical operational scope, those legacy epics stopped carrying unique live pressure and began splitting the same backlog history across dead scopes.

## Decision

Recurring operational work materializes into watch `VE3IAG4jZ` rather than into replacement project-operations epics. When a retired operational epic has no remaining unique live scope, its durable implementation history is consolidated into the watch README or the canonical watch backlog stories, and the obsolete epic plus superseded stories are removed from the board.

## Constraints

- **MUST:** Surface live operational pressure through watch-backed backlog stories.
- **MUST:** Preserve any unique implementation or lineage notes in watch-owned documentation before retiring legacy scopes.
- **MUST NOT:** Keep duplicate routine materializations in retired project-operations epics once a canonical watch story exists for the same routine topic.
- **SHOULD:** Prefer one canonical watch story per recurring routine topic, with legacy materialization history folded into that story when needed.

## Consequences

### Positive

- Strategic capacity accurately reflects operational pressure in the watch bar instead of hiding it behind done epics.
- The board no longer carries dead operational epics solely to preserve routine history.
- Operators review one canonical story per routine topic instead of chasing older epic-scoped materializations.

### Negative

- Some historical story and epic files are removed after their useful content is consolidated elsewhere.

### Neutral

- Mission `VE3I9QfPV` keeps a lightweight architectural record of the consolidation even though its original epics are retired.

## Verification

| Check | Type | Description |
|-------|------|-------------|
| Board health | automated | `just keel doctor --status` reports nominal health after retiring the legacy epics. |
| Flow surface | manual | `just keel flow` shows watch capacity pressure without relying on the retired operational epics. |
| Scope cleanup | manual | No live stories remain scoped to `VDseuzIFg` or `VE3KrOPS`. |

## References

- Watch: `VE3IAG4jZ`
- Mission: `VE3I9QfPV`
