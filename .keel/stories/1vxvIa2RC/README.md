---
id: 1vxvIa2RC
title: Remove Planning Show Recommendations And Update Planning Guidance
type: feat
status: backlog
created_at: 2026-03-04T15:06:36
updated_at: 2026-03-04T15:16:24
scope: 1vxqMtskC/1vxvFrNta
---

# Remove Planning Show Recommendations And Update Planning Guidance

## Summary

Remove recommendation sections from planning read commands and update architect planning guidance to rely on `config show` and `verify recommend` for verification technique planning.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] `keel epic show`, `keel voyage show`, and `keel story show` no longer render verification-technique recommendation sections. <!-- verify: cargo test -p keel planning_show_omits_verification_recommendations, SRS-05:start -->
- [ ] [SRS-05/AC-02] Existing planning/evidence/progress sections remain intact after recommendation removal. <!-- verify: cargo test -p keel planning_show_preserves_existing_sections, SRS-05:end -->
- [ ] [SRS-06/AC-01] `AGENTS.md` planning workflow explicitly references `just keel config show` (inventory) and `just keel verify recommend` (detected+active options) for verification planning. <!-- verify: manual, SRS-06:start:end, proof: ac-1.log -->
