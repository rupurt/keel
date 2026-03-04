---
id: 1vxZ0BinH
title: Implement Shared Guidance Renderer Helpers
type: feat
status: backlog
created_at: 2026-03-03T15:18:07
updated_at: 2026-03-03T15:18:07
scope: 1vxYzSury/1vxYzh8ep
---

# Implement Shared Guidance Renderer Helpers

## Summary

Extract and reuse shared guidance rendering helpers so commands emit canonical next and recovery guidance through one implementation path.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Implement shared helper(s) that build canonical guidance payloads from command decisions without duplicating per-command logic.
- [ ] [SRS-01/AC-02] Refactor `keel next` JSON formatting to use the shared helper path while preserving existing decision semantics.
- [ ] [SRS-01/AC-03] Provide focused tests proving helpers produce stable command strings and do not emit conflicting guidance fields.
