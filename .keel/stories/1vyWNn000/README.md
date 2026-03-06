---
id: 1vyWNn000
title: Surface Topology Knowledge And Horizon Warnings
type: feat
status: backlog
created_at: 2026-03-06T06:42:27
updated_at: 2026-03-06T06:44:10
scope: 1vyWIF000/1vyWIM000
---

# Surface Topology Knowledge And Horizon Warnings

## Summary

Attach scoped knowledge and forward-looking commentary to the topology output so operators can see what execution has already taught the system and what to watch next.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] The topology view surfaces scoped recent insights and pending unapplied knowledge as annotations relevant to the epic flow. <!-- verify: cargo test --lib topology_annotations_surface_scoped_knowledge, SRS-04:start:end, proof: ac-1.log -->
- [ ] [SRS-05/AC-01] Horizon commentary surfaces verification debt and throughput or ETA risk using deterministic board signals. <!-- verify: cargo test --lib topology_horizon_surfaces_verification_and_eta_risk, SRS-05:start, proof: ac-2.log -->
- [ ] [SRS-05/AC-02] Horizon commentary surfaces tech or process debt heuristics from scoped knowledge signals and labels recommendations as advisory. <!-- verify: llm-judge, SRS-05:continues:end, proof: ac-3.log -->
- [ ] [SRS-05/AC-03] [SRS-NFR-03/AC-02] Knowledge and horizon derivation reuse existing scanner and navigator helpers rather than duplicate parsing paths. <!-- verify: cargo test --lib topology_horizon_reuses_knowledge_helpers, SRS-NFR-03:end, SRS-05:continues, proof: ac-4.log -->
