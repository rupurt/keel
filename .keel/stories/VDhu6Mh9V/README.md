---
id: VDhu6Mh9V
title: Lock HEAD Show Contracts With Regressions
type: feat
status: done
created_at: 2026-03-12T18:36:23
updated_at: 2026-03-12T19:41:44
operator-signal: 
scope: VDhtrxgW6/VDhtzKSNF
index: 3
started_at: 2026-03-12T19:37:48
completed_at: 2026-03-12T19:41:44
---

# Lock HEAD Show Contracts With Regressions

## Summary

Lock the HEAD-relative selector contract with regression coverage and CLI guidance so the supported syntax and ordering semantics do not drift.

## Acceptance Criteria

- [x] [SRS-04/AC-02] Unsupported selector syntax is rejected with canonical guidance that points users back to exact IDs or supported HEAD forms. <!-- verify: cargo test --bin keel head_show_commands_reject_invalid_syntax, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-02] Regression coverage proves the show-command head target matches the corresponding canonical default list ordering for every supported entity type. <!-- verify: cargo test --bin keel head_show_contract_matches_default_list_order, SRS-NFR-01:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] Help text or user-facing guidance touched by the change stays aligned with the supported selector forms and entity coverage. <!-- verify: cargo test --bin keel head_show_guidance_contract, SRS-NFR-02:start:end, proof: ac-3.log-->
