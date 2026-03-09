---
id: 1vzQws000
title: Gate Readiness And Board Projections On Evidence Quality
type: feat
status: backlog
created_at: 2026-03-08T20:06:26
updated_at: 2026-03-08T20:10:04
scope: 1vzQpr000/1vzQu5000
index: 3
---

# Gate Readiness And Board Projections On Evidence Quality

## Summary

Gate bearing readiness on evidence quality and expose that state in board projections so incomplete or weakly supported research is visible before a bearing is treated as decision-ready.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `keel doctor` and readiness gates block bearings whose evidence coverage, citation quality, or contradiction handling do not satisfy the decision-ready contract. <!-- verify: cargo test -p keel bearing_readiness_requires_evidence_quality, SRS-03:start, proof: ac-1.log-->
- [ ] [SRS-03/AC-02] Bearing list, flow, and related projections surface evidence-backed readiness and score outputs so weak research is visible in board views. <!-- verify: cargo test -p keel bearing_projections_surface_evidence_quality, SRS-03:continues, proof: ac-2.log-->
- [ ] [SRS-03/AC-03] Recovery guidance points operators toward missing evidence or citation work rather than generic document-presence checks. <!-- verify: cargo test -p keel bearing_readiness_guidance_targets_missing_evidence, SRS-03:end, proof: ac-3.log-->
