---
id: VDUGflIUh
title: Update Next Role Routing
type: feat
status: done
created_at: 2026-03-10T10:38:13
updated_at: 2026-03-10T13:52:56
scope: VDTpFlMKc/VDUG60pcX
index: 3
started_at: 2026-03-10T13:36:24
completed_at: 2026-03-10T13:52:56
---

# Update Next Role Routing

## Summary

Update `keel next` to route based on `--role` instead of `--agent`/`--human`.

## Acceptance Criteria

- [x] [SRS-02/AC-01] `keel next` accepts `--role <TAXONOMY>` <!-- verify: cargo test cli_tests::cli_parses_next_with_ -- --nocapture, SRS-02:start, proof: ac-1.log -->
- [x] [SRS-02/AC-02] `--agent` and `--human` are removed or error gracefully (conflict) <!-- verify: cargo test cli_tests::cli_rejects_legacy_next_ -- --nocapture, SRS-02:end, proof: ac-2.log -->
- [x] [SRS-03/AC-01] `manager/*` role maps to Management queue decisions <!-- verify: cargo test cli::commands::management::next_support::algorithm::tests::manager_roles_route_to_management_queue_decisions -- --nocapture, SRS-03:start, proof: ac-3.log -->
- [x] [SRS-03/AC-02] `engineer/*` role maps to Execution queue work <!-- verify: cargo test cli::commands::management::next_support::algorithm::tests::engineer_roles_route_to_execution_queue_work -- --nocapture, SRS-03:end, proof: ac-4.log -->
