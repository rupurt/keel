---
id: 1vv7Yj0Kt
title: Refactor Voyage Transitions to Use Unified Enforcer
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:38:24
scope: 1vv7YWzw2/1vv7YYY0y
index: 3
submitted_at: 2026-02-24T21:38:23
completed_at: 2026-02-24T21:38:24
started_at: 2026-02-24T21:37:02
---

# Refactor Voyage Transitions to Use Unified Enforcer

## Summary

Refactor the `voyage plan` and `voyage start` commands to use the unified enforcement service for validation before updating the voyage status.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Replace manual `evaluate_voyage_transition` calls in `src/commands/voyage/plan.rs` with `enforce_transition`. <!-- verify: manual, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Replace manual `evaluate_voyage_transition` calls in `src/commands/voyage/start.rs` with `enforce_transition`. <!-- verify: manual, SRS-03:end, proof: ac-2.log-->
- [x] [SRS-04/AC-02] Use `format_enforcement_error` to report validation failures. <!-- verify: manual, SRS-04:end, proof: ac-3.log-->
