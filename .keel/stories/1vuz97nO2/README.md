---
id: 1vuz97nO2
title: Refactor Next and Flow to Use Queue Policy
type: feat
status: done
created_at: 2026-02-24T12:36:41
updated_at: 2026-02-24T13:39:14
scope: 1vuz8K4NM/1vuz8VYmc
index: 2
submitted_at: 2026-02-24T13:39:14
completed_at: 2026-02-24T13:39:14
---

# Refactor Next and Flow to Use Queue Policy

## Summary

Refactor queue-selection and flow-health paths to consume the shared policy APIs instead of local literals and ad-hoc ordering logic.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Refactor `src/next/algorithm.rs` to use policy helpers for blocked, accept, research, planning, and work ordering decisions. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-02/AC-02] Refactor relevant `src/flow/*` decision points to consume policy constants instead of inline literals. <!-- verify: manual, SRS-02:continues -->
- [x] [SRS-02/AC-03] Add regression tests proving policy-driven behavior and removal of inline threshold usage in decision paths. <!-- verify: manual, SRS-02:end -->
