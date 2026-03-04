---
id: 1vxqNFNdN
title: Implement Project Autodetection And Recommendation Engine
type: feat
status: backlog
created_at: 2026-03-04T09:51:05
updated_at: 2026-03-04T09:51:13
scope: 1vxqMtskC/1vxqN5jnA
---

# Implement Project Autodetection And Recommendation Engine

## Summary

Build the autodetection and ranking pipeline that infers project stack signals and recommends the highest-value automated verification techniques with rationale.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Detect project stack signals from repository artifacts (for example Rust CLI and browser test stack markers) and compute confidence scores. <!-- verify: cargo test --lib technique_project_signal_detection, SRS-03:start -->
- [ ] [SRS-03/AC-02] Produce ranked recommendations from the merged catalog with rationale and applicability metadata per recommendation. <!-- verify: cargo test --lib technique_recommendation_ranking, SRS-03:end -->
- [ ] [SRS-NFR-01/AC-02] Recommendation ranking is deterministic for equivalent repository inputs. <!-- verify: cargo test --lib technique_recommendation_deterministic, SRS-NFR-01:start:end -->
