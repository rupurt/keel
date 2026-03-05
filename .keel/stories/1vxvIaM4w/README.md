---
id: 1vxvIaM4w
title: Implement Verify Recommend For Active Detected Techniques
type: feat
status: done
created_at: 2026-03-04T15:06:36
updated_at: 2026-03-04T16:13:11
scope: 1vxqMtskC/1vxvFrNta
started_at: 2026-03-04T15:56:25
completed_at: 2026-03-04T16:13:11
---

# Implement Verify Recommend For Active Detected Techniques

## Summary

Introduce `keel verify recommend` as the recommendation surface, filtered to detected and active techniques only, with advisory-only behavior and machine-readable output.

## Acceptance Criteria

- [x] [SRS-04/AC-01] `keel verify recommend` renders only techniques where `detected=true` and `active=true` (where `active = detected && !disabled`). <!-- verify: cargo test -p keel verify_recommend_filters_detected_active, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-03] `keel verify recommend` is advisory-only and does not execute verification tools/commands. <!-- verify: cargo test -p keel verify_recommend_has_no_execution_side_effects, SRS-NFR-03:start:end, proof: ac-2.log-->
- [x] [SRS-04/AC-02] `keel verify recommend --json` emits deterministic machine-readable recommendations using the same filter contract. <!-- verify: cargo test -p keel verify_recommend_json_contract, SRS-04:end, proof: ac-3.log-->
