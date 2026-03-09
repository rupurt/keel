---
id: 1vzQwj000
title: Replace Bearing Scaffolds With The Evidence Document Contract
type: feat
status: backlog
created_at: 2026-03-08T20:06:17
updated_at: 2026-03-08T20:09:57
scope: 1vzQpr000/1vzQtq000
index: 1
---

# Replace Bearing Scaffolds With The Evidence Document Contract

## Summary

Replace the bearing scaffolds so the canonical document bundle becomes `BRIEF.md` for framing, `EVIDENCE.md` for cited research capture, and `ASSESSMENT.md` for synthesis, with no remaining survey-era scaffold output.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `keel bearing new` scaffolds `README.md`, `BRIEF.md`, `EVIDENCE.md`, and `ASSESSMENT.md`, and generated document references omit `SURVEY.md`. <!-- verify: cargo test -p keel bearing_scaffold_uses_evidence_document_contract, SRS-01:start, proof: ac-1.log-->
- [ ] [SRS-01/AC-02] `BRIEF.md` is framing-only while `EVIDENCE.md` owns cited research capture structure, so the templates no longer duplicate findings across both documents. <!-- verify: cargo test -p keel bearing_brief_and_evidence_templates_have_distinct_responsibilities, SRS-01:continues, proof: ac-2.log-->
- [ ] [SRS-01/AC-03] [SRS-NFR-01/AC-01] No supported scaffold or generator path in scope continues to emit survey-era document names or compatibility aliases. <!-- verify: cargo test -p keel bearing_scaffold_contract_has_no_survey_aliases, SRS-NFR-01:start:end, SRS-01:end, proof: ac-3.log-->
