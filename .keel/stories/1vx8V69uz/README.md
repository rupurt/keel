---
id: 1vx8V69uz
title: Relocate Domain Core Modules Into Src Domain
type: feat
status: done
created_at: 2026-03-02T11:00:16
updated_at: 2026-03-02T11:31:03
scope: 1vwq96cpt/1vx8TLqpp
index: 4
submitted_at: 2026-03-02T11:30:47
completed_at: 2026-03-02T11:31:03
---

# Relocate Domain Core Modules Into Src Domain

## Summary

Relocate core entities, policies, transition logic, and domain validation from
legacy top-level modules into `src/domain/**` so domain logic is physically
grouped and independent from adapters.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Core domain families (`model`, `policy`, `state_machine`, `transitions`) are moved into `src/domain/**` and compile from their new paths. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log -->
- [x] [SRS-02/AC-02] No active top-level module declarations remain for `src/model/**`, `src/policy/**`, `src/state_machine/**`, or `src/transitions/**`. <!-- verify: manual, SRS-02:continues, proof: ac-2.log -->
- [x] [SRS-02/AC-03] Layer dependencies continue to flow from `cli/application` into `domain` without cyclical imports. <!-- verify: manual, SRS-02:continues, proof: ac-3.log -->
- [x] [SRS-02/AC-04] Domain move preserves behavior validated by existing test suites. <!-- verify: manual, SRS-02:end, proof: ac-4.log -->
