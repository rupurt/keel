---
id: VDhu6JN89
title: Add HEAD Selector Parsing And Stable Order Resolution
type: feat
status: done
created_at: 2026-03-12T18:36:22
updated_at: 2026-03-12T19:21:11
operator-signal: 
scope: VDhtrxgW6/VDhtzKSNF
index: 1
started_at: 2026-03-12T19:13:20
completed_at: 2026-03-12T19:21:11
---

# Add HEAD Selector Parsing And Stable Order Resolution

## Summary

Add the shared HEAD-selector parser and the stable ordering providers that convert HEAD-relative selectors into concrete entity IDs without changing existing exact-ID lookups.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Introduce a shared selector parser that accepts exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ and normalizes unsupported forms into deterministic errors. <!-- verify: cargo test --lib head_selector_parser, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Expose canonical ordered ID providers for mission, epic, voyage, story, bearing, ADR, and routine entities using the same stable default ordering semantics as their list surfaces. <!-- verify: cargo test --lib head_selector_ordering, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Equivalent fixture boards resolve the same HEAD-relative selectors across repeated runs. <!-- verify: cargo test --lib head_selector_determinism, SRS-NFR-01:start:end, proof: ac-3.log-->
