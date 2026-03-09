---
id: 1vzQwr000
title: Render Evidence-Backed Bearing Show And File Surfaces
type: feat
status: backlog
created_at: 2026-03-08T20:06:25
updated_at: 2026-03-08T20:10:04
scope: 1vzQpr000/1vzQu5000
index: 2
---

# Render Evidence-Backed Bearing Show And File Surfaces

## Summary

Render evidence-backed bearing reading surfaces so operators can inspect citations and provenance directly in the terminal without losing access to the underlying evidence document.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] `bearing show` renders compact provenance and citation summaries that support terminal review without dropping the underlying evidence context. <!-- verify: cargo test -p keel bearing_show_renders_compact_evidence_provenance, SRS-04:start, proof: ac-1.log-->
- [ ] [SRS-04/AC-02] `bearing file` and related drill-down affordances make the underlying `EVIDENCE.md` document directly accessible from the terminal workflow. <!-- verify: cargo test -p keel bearing_file_surfaces_evidence_document, SRS-04:continues, proof: ac-2.log-->
- [ ] [SRS-04/AC-03] [SRS-NFR-02/AC-01] Default terminal rendering keeps provenance readable at common terminal widths without forcing raw-file inspection for routine review. <!-- verify: vhs tapes/bearing-evidence-surfaces.tape, SRS-NFR-02:start:end, SRS-04:end, proof: ac-3.gif-->
