---
id: 1vxvIaPe8
title: Hard Cutover Verify Command To Subcommands
type: feat
status: done
created_at: 2026-03-04T15:06:36
updated_at: 2026-03-04T15:55:40
scope: 1vxqMtskC/1vxvFrNta
started_at: 2026-03-04T15:44:57
completed_at: 2026-03-04T15:55:40
---

# Hard Cutover Verify Command To Subcommands

## Summary

Perform a hard cutover of verification execution to `keel verify run`, preserving execution semantics while making legacy `keel verify` fail fast with migration guidance.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel verify run` executes the existing verification flow with parity for target selection (`<id>` and `--all`). <!-- verify: cargo test -p keel verify_run_preserves_execution_semantics, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-03] Bare `keel verify` exits non-zero and prints explicit recovery guidance to use `keel verify run`. <!-- verify: cargo test -p keel verify_root_fails_fast_with_run_guidance, SRS-NFR-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-02] `keel verify run --json` returns deterministic machine-readable execution results equivalent to the text path. <!-- verify: cargo test -p keel verify_run_json_contract, SRS-03:end, proof: ac-3.log-->
