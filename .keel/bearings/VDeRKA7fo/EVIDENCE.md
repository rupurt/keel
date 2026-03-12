---
id: VDeRKA7fo
---

# Simulation Kernel Architecture Research — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | manual | manual:repo-inspection | ARCHITECTURE.md; src/application/process_manager.rs; src/application/domain_events.rs | 2026-03-12 | 2026-03-12 | high | high | The current architecture already formalizes layers and contains an embryonic event/reactor model in the process manager and domain event types. |
| SRC-02 | manual | manual:repo-inspection | src/read_model/routine_due_state.rs; src/read_model/scheduled_routines.rs | 2026-03-12 | 2026-03-12 | high | high | Temporal evaluation is already deterministic and reference-time driven, which makes it a natural candidate for a shared pulse abstraction. |
| SRC-03 | manual | manual:repo-inspection | src/read_model/flow_status.rs; src/read_model/queue_policy.rs; src/cli/commands/management/next_support/algorithm.rs | 2026-03-12 | 2026-03-12 | high | high | Flow and next both derive operational decisions from board projections, suggesting room for a clearer shared projection pipeline. |

## Technical Research

### Feasibility
The evidence supports an incremental architecture extension. Keel already has state machines, domain events, process-manager orchestration, and deterministic time-based evaluation. The missing piece is not a new runtime model but a clearer internal vocabulary and a small number of reusable abstractions that make those patterns explicit.

## Key Findings

1. Keel already behaves like a simulation in bounded areas, especially cross-aggregate lifecycle reactions and time-based routine evaluation [SRC-01][SRC-02]
2. The existing layered architecture is strong enough that a simulation kernel should sit inside it, not replace it [SRC-01]
3. Shared projection inputs would likely help `flow`, `next`, and mission steering stay aligned as more temporal and reactive behavior is added [SRC-03]

## Unknowns

- Whether a `BoardPulse` should be a first-class type shared across read models or just a thin internal helper.
- How many distinct reactor units exist in practice once process-manager logic is decomposed.
