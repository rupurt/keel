---
id: VDhu6JN89
title: Add HEAD Selector Parsing And Stable Order Resolution
type: feat
status: backlog
created_at: 2026-03-12T18:36:22
updated_at: 2026-03-12T18:39:50
operator-signal: 
scope: VDhtrxgW6/VDhtzKSNF
index: 1
---

# Add HEAD Selector Parsing And Stable Order Resolution

## Summary

Add the shared HEAD-selector parser and the stable ordering providers that convert HEAD-relative selectors into concrete entity IDs without changing existing exact-ID lookups.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Introduce a shared selector parser that accepts exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ and normalizes unsupported forms into deterministic errors. <!-- verify: cargo test --lib head_selector_parser, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] Expose canonical ordered ID providers for mission, epic, voyage, story, bearing, ADR, and routine entities using the same stable default ordering semantics as their list surfaces. <!-- verify: cargo test --lib head_selector_ordering, SRS-02:start:end -->
- [ ] [SRS-NFR-01/AC-01] Equivalent fixture boards resolve the same HEAD-relative selectors across repeated runs. <!-- verify: cargo test --lib head_selector_determinism, SRS-NFR-01:start:end -->
