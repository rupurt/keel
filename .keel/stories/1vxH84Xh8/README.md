---
id: 1vxH84Xh8
title: Extend Adr Creation Inputs For Context Ownership
type: feat
status: done
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-03T10:41:12
scope: 1vxGy5tco/1vxGzV3oR
index: 3
started_at: 2026-03-03T10:02:42
completed_at: 2026-03-03T10:41:12
---

# Extend Adr Creation Inputs For Context Ownership

## Summary

Add explicit ADR creation inputs for context scope ownership and persist these values directly in ADR frontmatter.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Add optional `--context` to `adr new` and persist the value in the created ADR frontmatter. <!-- verify: cargo test -p keel cli::commands::management::adr::tests::new_adr_persists_context_and_applies_to_in_frontmatter, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] Add repeatable `--applies-to` to `adr new` and persist all provided values in frontmatter order. <!-- verify: cargo test -p keel cli::commands::management::adr::tests::new_adr_persists_context_and_applies_to_in_frontmatter, SRS-03:continues, proof: ac-2.log -->
- [x] [SRS-03/AC-03] Add command tests validating parser behavior and persisted frontmatter for both flags. <!-- verify: cargo test -p keel cli_tests::cli_parses_adr_new_with_context_and_applies_to, SRS-03:end, proof: ac-3.log -->
