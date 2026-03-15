---
id: VDyhMtYBO
title: Implement Circuit Overload Queue Constraints
type: feat
status: done
created_at: 2026-03-15T15:32:26
updated_at: 2026-03-15T15:38:44
operator-signal: 
scope: VDseuzIFg
index: 8
started_at: 2026-03-15T15:33:39
submitted_at: 2026-03-15T15:38:18
completed_at: 2026-03-15T15:38:44
---

# Implement Circuit Overload Queue Constraints

## Summary

Implement the "Circuit Overload" and "Battery Pack" metaphors. A battery pack represents a queue of ready work. If too many are plugged in simultaneously, it creates a risk of Circuit Overload. We need to measure the ready queue size against a configurable `max_battery_packs` limit and report an overload condition via `keel doctor`.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel::infrastructure::config::WorkflowConfig` includes a new `max_battery_packs` field (default: 5) representing the maximum number of ready backlog items. <!-- verify: manual, SRS-01:start, SRS-01:end -->
- [x] [SRS-01/AC-02] `keel config show` renders `max_battery_packs` in the `[workflow]` section. <!-- verify: manual, SRS-01:start, SRS-01:end -->
- [x] [SRS-01/AC-03] A new diagnostic check `check_circuit_overload` verifies that the number of ready backlog stories does not exceed `max_battery_packs`. <!-- verify: manual, SRS-01:start, SRS-01:end -->
- [x] [SRS-01/AC-04] `keel doctor` incorporates the circuit overload check into the workflow/story diagnostic suite. <!-- verify: manual, SRS-01:start, SRS-01:end -->
