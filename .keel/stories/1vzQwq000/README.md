---
id: 1vzQwq000
title: Make Bearing Assessment Evidence-Aware And Compute EV Scores
type: feat
status: done
created_at: 2026-03-08T20:06:24
updated_at: 2026-03-08T21:19:25
scope: 1vzQpr000/1vzQu5000
index: 1
started_at: 2026-03-08T21:09:03
completed_at: 2026-03-08T21:19:25
---

# Make Bearing Assessment Evidence-Aware And Compute EV Scores

## Summary

Make `ASSESSMENT.md` citation-aware and evolve EV scoring so the recommendation path is grounded in traceable evidence quality signals such as breadth, freshness, authority, and contradiction handling instead of relying only on authored judgment fields.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `ASSESSMENT.md` requires canonical evidence citations for findings, dependencies, alternatives, and recommendations so assessment claims are traceable to stored source IDs. <!-- verify: cargo test -p keel bearing_assessment_requires_evidence_citations, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] EV scoring incorporates evidence breadth, freshness, authority, and contradiction or gap handling alongside authored impact, confidence, effort, and risk factors. <!-- verify: cargo test -p keel bearing_ev_score_uses_evidence_quality_signals, SRS-02:start, proof: ac-2.log-->
- [x] [SRS-02/AC-02] Fixture scenarios prove that stronger or weaker evidence meaningfully changes computed assessment and EV outcomes. <!-- verify: cargo test -p keel bearing_ev_score_changes_with_evidence_quality, SRS-02:continues, proof: ac-3.log-->
- [x] [SRS-02/AC-03] [SRS-NFR-01/AC-01] Equivalent evidence inputs and authored assessment factors always produce identical scores and ordering. <!-- verify: cargo test -p keel bearing_ev_score_is_deterministic, SRS-NFR-01:start:end, SRS-02:end, proof: ac-4.log-->
