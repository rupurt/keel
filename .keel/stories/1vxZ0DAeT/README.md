---
id: 1vxZ0DAeT
title: Add Canonical Guidance To Bearing Transition Commands
type: feat
status: backlog
created_at: 2026-03-03T15:18:09
updated_at: 2026-03-03T15:18:09
scope: 1vxYzSury/1vxYzjVMv
---

# Add Canonical Guidance To Bearing Transition Commands

## Summary

Add canonical guidance output to bearing lifecycle transitions so exploration workflows expose deterministic next or recovery commands when action is required.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add canonical `next_step` guidance to actionable bearing transitions (`survey`, `assess`, `park`, `decline`, `lay`) when a deterministic follow-up exists.
- [ ] [SRS-01/AC-02] Emit canonical `recovery_step` guidance for bearing transition failures that require a concrete remediation command.
- [ ] [SRS-01/AC-03] Add tests covering bearing command guidance behavior in both human-readable and JSON output paths.
