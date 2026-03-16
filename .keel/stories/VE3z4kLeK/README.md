---
id: VE3z4kLeK
title: Doctor Warns On Routine Scope Incoherence
type: feat
status: backlog
created_at: 2026-03-16T13:14:07
updated_at: 2026-03-16T13:15:04
operator-signal:
scope: VE3KrOPS/VE3yUoUUy
index: 2
---

# Doctor Warns On Routine Scope Incoherence

## Summary

Add `check_routine_scope_coherence()` to `checks/routines.rs`. For each routine, verify its `target-scope` references an existing epic and a non-terminal voyage. Emit a warning for missing entities and an error for terminal scope targets.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Doctor emits warning when routine target-scope references a missing epic or voyage <!-- verify: test -->
- [ ] [SRS-02/AC-02] Doctor emits error when routine target-scope references a terminal voyage <!-- verify: test -->
