---
id: 1vxvIZRXy
title: Refactor Config Show Into Technique Flag Matrix
type: feat
status: in-progress
created_at: 2026-03-04T15:06:35
updated_at: 2026-03-04T15:30:33
scope: 1vxqMtskC/1vxvFrNta
started_at: 2026-03-04T15:30:33
---

# Refactor Config Show Into Technique Flag Matrix

## Summary

Refactor `keel config show` to present verification techniques as a canonical flag matrix and machine-readable payload, while keeping `keel config mode` unchanged.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `keel config show` renders one row per built-in/custom technique with `label`, `detected`, `disabled`, and `active`, where `label` is the hyphenated technique id. <!-- verify: cargo test -p keel config_show_renders_technique_flag_matrix, SRS-01:start, proof: ac-1.log-->
- [ ] [SRS-01/AC-02] The matrix includes all techniques regardless of active/disabled state, with deterministic ordering. <!-- verify: cargo test -p keel config_show_lists_all_techniques_deterministically, SRS-01:end, proof: ac-2.log-->
- [ ] [SRS-02/AC-01] `keel config show` no longer prints scoring output, and `keel config mode` remains behaviorally unchanged. <!-- verify: cargo test -p keel config_show_omits_scoring_and_config_mode_regression, SRS-02:start:end, proof: ac-3.log-->
- [ ] [SRS-01/AC-03] `keel config show --json` emits deterministic machine-readable rows using the same `label/detected/disabled/active` contract. <!-- verify: cargo test -p keel config_show_json_contract, SRS-01:end, proof: ac-4.log-->
