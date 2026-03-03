---
id: 1vxH84K5a
title: Codify Token Bucket Contract Tests
type: chore
status: done
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-03T10:53:41
scope: 1vxGy5tco/1vxGzV3oR
index: 4
started_at: 2026-03-03T10:46:37
completed_at: 2026-03-03T10:53:41
---

# Codify Token Bucket Contract Tests

## Summary

Create deterministic contract tests for token bucket policy so unknown tokens or ownership violations fail immediately.

## Acceptance Criteria

- [x] [SRS-05/AC-02] Define and test the allowed token bucket contract (CLI-owned, system-owned, generated markers). <!-- verify: cargo test -p keel generated_marker_contract_remains_literal, SRS-05:start, proof: ac-1.log -->
- [x] [SRS-05/AC-03] Fail tests when templates contain unknown tokens or out-of-bucket token usage. <!-- verify: cargo test -p keel template_tokens_match_known_bucket_inventory, SRS-05:continues, proof: ac-2.log -->
- [x] [SRS-05/AC-04] Add/adjust drift tests to keep token inventory and CLI contract aligned over time. <!-- verify: cargo test -p keel creation_command_new_surfaces_match_cli_owned_token_contract, SRS-05:end, proof: ac-3.log -->
