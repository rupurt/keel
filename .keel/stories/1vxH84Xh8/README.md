---
id: 1vxH84Xh8
title: Extend Adr Creation Inputs For Context Ownership
type: feat
status: backlog
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzV3oR
index: 3
---

# Extend Adr Creation Inputs For Context Ownership

## Summary

Add explicit ADR creation inputs for context scope ownership and persist these values directly in ADR frontmatter.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Add optional `--context` to `adr new` and persist the value in the created ADR frontmatter.
- [ ] [SRS-03/AC-02] Add repeatable `--applies-to` to `adr new` and persist all provided values in frontmatter order.
- [ ] [SRS-03/AC-03] Add command tests validating parser behavior and persisted frontmatter for both flags.
