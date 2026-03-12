---
id: VDfhzA1l6
title: Stabilize Voyage Artifact Generation Order
type: feat
status: done
created_at: 2026-03-12T09:35:42
updated_at: 2026-03-12T09:44:25
operator-signal:
scope: VDfhxO64S/VDfhz9Hl7
index: 1
started_at: 2026-03-12T09:38:57
completed_at: 2026-03-12T09:44:25
---

# Stabilize Voyage Artifact Generation Order

## Summary

Make the current voyage artifact generators deterministic by sorting unordered traversal points and proving repeated generation is byte-stable.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `VOYAGE_REPORT.md` and `COMPLIANCE_REPORT.md` render proof artifacts in deterministic filename order regardless of filesystem enumeration order. <!-- verify: cargo test --lib voyage_artifacts_render_proofs_deterministically, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `COMPLIANCE_REPORT.md` renders proof links and story coverage in deterministic order for equivalent board inputs. <!-- verify: cargo test --lib compliance_report_renders_deterministically, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] Board artifact sync visits epics and voyages in canonical order before voyage artifact generation. <!-- verify: cargo test --lib sync_board_artifacts_uses_canonical_order, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] Repeated generation on unchanged input and equivalent board layouts yields identical voyage artifact output with no content drift. <!-- verify: cargo test --lib voyage_artifact_generation_is_idempotent, SRS-04:start:end, SRS-NFR-02:start:end, proof: ac-4.log-->
