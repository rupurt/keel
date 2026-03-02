---
id: 1vuz9ij1L
title: Remove Stage and Status Compatibility Aliases
type: feat
status: done
created_at: 2026-02-24T12:37:18
updated_at: 2026-02-24T16:08:03
scope: 1vuz8K4NM/1vuz8jNo3
index: 3
submitted_at: 2026-02-24T00:00:00
completed_at: 2026-02-24T00:00:00
---

# Remove Stage and Status Compatibility Aliases

## Summary

Remove compatibility aliases for story and voyage states so the codebase depends directly on canonical state-machine types.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Replace usage of compatibility aliases with direct canonical state types in model, commands, and flow modules. <!-- verify: manual, SRS-03:start -->
- [x] [SRS-03/AC-02] Remove alias definitions and related compatibility comments/tests that are no longer valid after migration. <!-- verify: manual, SRS-03:continues -->
- [x] [SRS-03/AC-03] Ensure the project compiles and test coverage reflects canonical type usage only. <!-- verify: manual, SRS-03:end -->
