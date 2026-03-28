---
# system-managed
id: VF8hnQirj
status: done
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:24:00
# authored
title: Define Canonical CLI Command Catalog
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkTCk6
index: 1
started_at: 2026-03-27T23:11:37
submitted_at: 2026-03-27T23:23:55
completed_at: 2026-03-27T23:24:00
---

# Define Canonical CLI Command Catalog

## Summary

Define the static catalog that describes Keel's public command surface so later help, turn, scene, and routing features can depend on one authoritative taxonomy.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A canonical command descriptor set covers the public CLI commands with family, capability, turn-phase, docs-slug, and scene-support metadata. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-02/AC-01] Catalog ordering and descriptor values are covered by deterministic tests. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-2.log-->
