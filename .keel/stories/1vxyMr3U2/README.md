---
id: 1vxyMr3U2
title: Semantic Conflict Feature Extraction
type: feat
status: done
created_at: 2026-03-04T18:23:13
updated_at: 2026-03-04T18:41:18
scope: 1vxyM0hvn/1vxyMT6nz
started_at: 2026-03-04T18:35:51
completed_at: 2026-03-04T18:41:18
---

# Semantic Conflict Feature Extraction

## Summary

Implement deterministic semantic feature extraction for candidate story pairs so `keel next --parallel` can reason about difficult-to-resolve merge conflicts.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Pairwise feature vectors are deterministic for identical board and repository inputs. <!-- verify: cargo test --lib next_parallel_feature_vectors_are_deterministic, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Extractor emits explicit unresolved-context signals when architectural semantics are insufficient. <!-- verify: cargo test --lib next_parallel_feature_vectors_emit_unknown_risk, SRS-01:start:end, proof: ac-2.log-->
