---
id: 1vxZ0BinH
title: Implement Shared Guidance Renderer Helpers
type: feat
status: done
created_at: 2026-03-03T15:18:07
updated_at: 2026-03-03T18:20:12
scope: 1vxYzSury/1vxYzh8ep
started_at: 2026-03-03T18:15:19
completed_at: 2026-03-03T18:20:12
---

# Implement Shared Guidance Renderer Helpers

## Summary

Extract and reuse shared guidance rendering helpers so commands emit canonical next and recovery guidance through one implementation path.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Implement shared helper(s) that build canonical guidance payloads from command decisions without duplicating per-command logic. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::render_command_guidance_, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Refactor `keel next` JSON formatting to use the shared helper path while preserving existing decision semantics. <!-- verify: cargo test --lib decision_to_json_, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Provide focused tests proving helpers produce stable command strings and do not emit conflicting guidance fields. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::render_command_guidance_never_emits_conflicting_fields, SRS-01:end, proof: ac-3.log-->
