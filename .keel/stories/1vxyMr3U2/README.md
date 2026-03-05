---
id: 1vxyMr3U2
title: Semantic Conflict Feature Extraction
type: feat
status: backlog
created_at: 2026-03-04T18:23:13
updated_at: 2026-03-04T18:25:50
scope: 1vxyM0hvn/1vxyMT6nz
---

# Semantic Conflict Feature Extraction

## Summary

Implement deterministic semantic feature extraction for candidate story pairs so `keel next --parallel` can reason about difficult-to-resolve merge conflicts.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Pairwise feature vectors are deterministic for identical board and repository inputs. <!-- verify: cargo test --lib next_parallel_feature_vectors_are_deterministic, SRS-01:start:end -->
- [ ] [SRS-01/AC-02] Extractor emits explicit unresolved-context signals when architectural semantics are insufficient. <!-- verify: cargo test --lib next_parallel_feature_vectors_emit_unknown_risk, SRS-01:start:end -->
