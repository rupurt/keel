---
id: 1vx8V5VeE
title: Relocate Infrastructure Services Into Src Infrastructure
type: feat
status: done
created_at: 2026-03-02T11:00:15
updated_at: 2026-03-02T11:40:00
scope: 1vwq96cpt/1vx8TLqpp
index: 3
submitted_at: 2026-03-02T11:39:22
completed_at: 2026-03-02T11:40:00
---

# Relocate Infrastructure Services Into Src Infrastructure

## Summary

Relocate filesystem adapters, parsing/loading, template rendering, generation,
and verification modules into `src/infrastructure/**` so external IO concerns
are physically separated from domain and application logic.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Parser/loader/template/generation/verification modules are moved under `src/infrastructure/**` and referenced from normalized paths. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log -->
- [x] [SRS-03/AC-02] Legacy top-level service roots are removed from active module declarations with no duplicate implementations retained. <!-- verify: manual, SRS-03:continues, proof: ac-2.log -->
- [x] [SRS-03/AC-03] `main.rs` and dependent modules compile with infrastructure modules imported from `src/infrastructure/**`. <!-- verify: manual, SRS-03:continues, proof: ac-3.log -->
- [x] [SRS-03/AC-04] Regression and lifecycle tests stay green after infrastructure relocation. <!-- verify: manual, SRS-03:end, proof: ac-4.log -->
