---
id: 1vxyMsvug
title: Parallel Queue Selection With Confidence Threshold
type: feat
status: backlog
created_at: 2026-03-04T18:23:14
updated_at: 2026-03-04T18:25:50
scope: 1vxyM0hvn/1vxyMT6nz
---

# Parallel Queue Selection With Confidence Threshold

## Summary

Integrate confidence thresholding into parallel selection so only high-confidence low-conflict candidates are surfaced as actionable work.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] A single global confidence threshold gates pairwise parallel eligibility. <!-- verify: cargo test --lib next_parallel_threshold_blocks_uncertain_pairs, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] Threshold gating blocks uncertain pairs conservatively by default when confidence is unresolved. <!-- verify: cargo test --lib next_parallel_threshold_blocks_uncertain_pairs, SRS-03:start:end -->
