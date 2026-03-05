---
id: 1vxyMsepz
title: Conservative Pairwise Conflict Scoring
type: feat
status: backlog
created_at: 2026-03-04T18:23:14
updated_at: 2026-03-04T18:25:50
scope: 1vxyM0hvn/1vxyMT6nz
---

# Conservative Pairwise Conflict Scoring

## Summary

Implement conservative scoring that transforms semantic feature vectors into pairwise conflict risk and confidence, biasing toward blocked outcomes when confidence is low.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Scorer returns pairwise risk and confidence for every evaluated story pair. <!-- verify: cargo test --lib next_parallel_pairwise_scoring_is_conservative, SRS-02:start:end -->
- [ ] [SRS-02/AC-02] Unresolved architectural signals reduce confidence and raise conservative conflict risk. <!-- verify: cargo test --lib next_parallel_pairwise_scoring_penalizes_uncertainty, SRS-02:start:end -->
