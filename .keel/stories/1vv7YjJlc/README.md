---
id: 1vv7YjJlc
title: Refactor Story Submit and Accept to Use Unified Enforcer
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:38:04
scope: 1vv7YWzw2/1vv7YYY0y
index: 2
submitted_at: 2026-02-24T21:38:04
completed_at: 2026-02-24T21:38:04
---

# Refactor Story Submit and Accept to Use Unified Enforcer

## Summary

Refactor the `story submit` and `story accept` commands to use the unified enforcement service for validation before updating the story stage.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Replace manual `evaluate_story_transition` calls in `src/commands/story/submit.rs` with `enforce_transition`. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Replace manual `evaluate_story_transition` calls in `src/commands/story/accept.rs` with `enforce_transition`. <!-- verify: manual, SRS-02:end, proof: ac-2.log-->
- [x] [SRS-04/AC-01] Use `format_enforcement_error` to report validation failures. <!-- verify: manual, SRS-04:start, proof: ac-3.log-->
