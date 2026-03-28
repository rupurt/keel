---
# system-managed
id: VF8hnTVs9
status: done
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:24:00
# authored
title: Drive Help Text And Capability Guidance From Catalog
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkTCk6
index: 2
started_at: 2026-03-27T23:17:37
submitted_at: 2026-03-27T23:23:55
completed_at: 2026-03-27T23:24:00
---

# Drive Help Text And Capability Guidance From Catalog

## Summary

Cut the existing help-group and capability-classification logic over to the canonical catalog so the CLI stops teaching one taxonomy while code uses another, and make the catalog's scene-support metadata queryable for later voyages.

## Acceptance Criteria

- [x] [SRS-02/AC-01] The help-family rendering is generated from the canonical command catalog rather than a separate hard-coded narrative block. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Capability guidance classification is resolved from the canonical command metadata rather than an independent enum map. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] Scene-capable commands are queryable from the catalog without maintaining a separate hard-coded scene list. <!-- verify: manual, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-NFR-01/AC-01] Public command names and family vocabulary remain stable after the cutover. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-4.log-->
