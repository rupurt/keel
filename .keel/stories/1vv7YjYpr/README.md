---
id: 1vv7YjYpr
title: Refactor Story Start Command to Use Unified Enforcer
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:37:37
scope: 1vv7YWzw2/1vv7YYY0y
index: 1
submitted_at: 2026-02-24T21:37:35
completed_at: 2026-02-24T21:37:37
started_at: 2026-02-24T21:36:38
---

# Refactor Story Start Command to Use Unified Enforcer

## Summary

Refactor the `story start` command to use the unified enforcement service for validation before updating the story stage.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Replace manual `evaluate_story_transition` calls in `src/commands/story/start.rs` with `enforce_transition`. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Use `format_enforcement_error` to report validation failures. <!-- verify: manual, SRS-01:end, proof: ac-2.log-->
