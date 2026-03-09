---
id: 1vzQwl000
title: Enforce Hard-Cutover Validation And Migrate Bearing Fixtures
type: feat
status: done
created_at: 2026-03-08T20:06:19
updated_at: 2026-03-08T20:39:56
scope: 1vzQpr000/1vzQtq000
index: 3
started_at: 2026-03-08T20:30:27
completed_at: 2026-03-08T20:39:56
---

# Enforce Hard-Cutover Validation And Migrate Bearing Fixtures

## Summary

Fail fast on legacy survey-era bearing artifacts and migrate in-repo fixtures and example boards so the hard cutover leaves no supported workflow or test data on the old contract.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel doctor` and readiness checks fail bearings that still rely on `SURVEY.md` or otherwise violate the new framing/evidence/assessment contract, with actionable recovery guidance. <!-- verify: cargo test -p keel bearing_doctor_rejects_legacy_survey_contract, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Fixture boards, generated examples, and test helpers in scope are migrated so supported tests and workflows no longer depend on survey-era artifacts or commands. <!-- verify: cargo test -p keel bearing_fixture_boards_use_evidence_contract, SRS-04:start, proof: ac-2.log-->
- [x] [SRS-04/AC-02] [SRS-NFR-01/AC-02] Legacy survey-era paths fail hard without compatibility aliases in loaders, validators, or lifecycle transitions. <!-- verify: cargo test -p keel bearing_hard_cutover_rejects_legacy_survey_paths, SRS-NFR-01:start:end, SRS-04:end, proof: ac-3.log-->
