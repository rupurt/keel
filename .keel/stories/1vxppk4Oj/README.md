---
id: 1vxppk4Oj
title: Define Planning Show Output Contracts
type: feat
status: done
created_at: 2026-03-04T09:16:28
updated_at: 2026-03-04T11:49:49
scope: 1vxYzSury/1vxpomgnN
started_at: 2026-03-04T11:31:51
completed_at: 2026-03-04T11:49:49
---

# Define Planning Show Output Contracts

## Summary

Define a shared planning/evidence projection contract that all three `show` commands consume, including deterministic section ordering and missing-data placeholders.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Introduce shared projection types and builders for epic/voyage/story `show` data so command renderers read one canonical contract. <!-- verify: cargo test --lib planning_show_projection_contract, SRS-05:start, proof: ac-1.log -->
- [x] [SRS-05/AC-02] Add parsing utilities that extract authored planning sections (problem, goals/objectives, key requirements, verification strategy) from PRD/SRS content while ignoring scaffold comments. <!-- verify: cargo test --lib planning_doc_extractor, SRS-05:end, proof: ac-2.log -->
- [x] [SRS-05/AC-03] Add deterministic-order tests proving projections emit stable section, requirement, story, and artifact ordering across equivalent board states. <!-- verify: cargo test --lib planning_show_projection_deterministic, SRS-NFR-01:start:end, proof: ac-3.log -->
