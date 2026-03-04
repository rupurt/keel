---
id: 1vxZ0DAeT
title: Add Canonical Guidance To Bearing Transition Commands
type: feat
status: done
created_at: 2026-03-03T15:18:09
updated_at: 2026-03-03T19:30:15
scope: 1vxYzSury/1vxYzjVMv
started_at: 2026-03-03T19:21:35
completed_at: 2026-03-03T19:30:15
---

# Add Canonical Guidance To Bearing Transition Commands

## Summary

Add canonical guidance output to bearing lifecycle transitions so exploration workflows expose deterministic next or recovery commands when action is required.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to actionable bearing transitions (`survey`, `assess`, `park`, `decline`, `lay`) when a deterministic follow-up exists. <!-- verify: cargo test --lib cli::commands::management::bearing::guidance::tests::terminal_actions_map_to_next_human, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Emit canonical `recovery_step` guidance for bearing transition failures that require a concrete remediation command. <!-- verify: cargo test --lib cli::commands::management::bearing::guidance::tests::lay_epic_exists_recovery_maps_to_epic_show, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering bearing command guidance behavior in both human-readable and JSON output paths. <!-- verify: cargo test --lib cli::commands::management::bearing::guidance::tests, SRS-01:end, proof: ac-3.log-->
