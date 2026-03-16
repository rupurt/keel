---
id: VE23HpS3U
title: Include Evidence Source Table in Generated PRD
type: feat
status: done
created_at: 2026-03-16T05:18:18
updated_at: 2026-03-16T05:23:34
operator-signal:
scope: VDiHwPniZ/VE22wWzeD
index: 1
started_at: 2026-03-16T05:20:00
completed_at: 2026-03-16T05:23:34
---

# Include Evidence Source Table in Generated PRD

## Summary

Extend `create_prd_from_bearing` to read EVIDENCE.md, extract the `## Sources` table, and include it in the generated PRD under a "Research Provenance" heading. When EVIDENCE.md is absent or has no source table, the section is omitted.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Generated PRD includes EVIDENCE.md source table under "## Research Provenance" when evidence exists. <!-- verify: cargo test -p keel bearing_lay_prd_includes_evidence_sources, SRS-01:start:end -->
- [x] [SRS-03/AC-01] Generated PRD omits "Research Provenance" section when EVIDENCE.md is absent. <!-- verify: cargo test -p keel bearing_lay_prd_omits_provenance_without_evidence, SRS-03:start:end -->
