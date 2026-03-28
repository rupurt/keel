---
# system-managed
id: VF8hnXtrx
status: done
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:41:32
# authored
title: Add Scene And Routing Contract Guards
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkVhkI
index: 2
started_at: 2026-03-27T23:39:52
submitted_at: 2026-03-27T23:41:28
completed_at: 2026-03-27T23:41:32
---

# Add Scene And Routing Contract Guards

## Summary

Add the remaining contract guards so scene and routing claims in the docs stay locked to the new canonical scene registry and role explainability surfaces.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Contract tests fail when documented scene surfaces or dependency claims diverge from the central scene contracts. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Routing drift tests fail when roles-and-lanes docs examples diverge from `keel roles` and `keel next --explain`. <!-- verify: manual, SRS-04:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The scene and routing guards are readable enough to support intentional product-contract updates. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-3.log-->
