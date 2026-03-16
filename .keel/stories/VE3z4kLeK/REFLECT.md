---
created_at: 2026-03-16T13:23:45
---

# Reflection - Doctor Warns On Routine Scope Incoherence

## Knowledge

## Observations

New check `check_routine_scope_coherence` follows the existing pattern in routines.rs. Warning for missing entities, error for terminal — matches the severity escalation the pulse guard uses. Wired into the diagnostics engine as `routine-scope-coherence`.
