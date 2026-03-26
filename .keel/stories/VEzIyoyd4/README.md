---
# system-managed
id: VEzIyoyd4
status: done
created_at: 2026-03-26T08:30:42
updated_at: 2026-03-26T09:03:54
# authored
title: Document Speccy Hooks And External Adoption Boundaries
type: feat
operator-signal:
scope: VEzIwU3fh/VEzIxN01G
index: 3
started_at: 2026-03-26T09:01:44
submitted_at: 2026-03-26T09:03:47
completed_at: 2026-03-26T09:03:54
---

# Document Speccy Hooks And External Adoption Boundaries

## Summary

Define and document the public hook surface and the boundary between reusable `speccy` behavior and host-owned project logic so other projects can adopt the crate intentionally, with template inventory remaining host-owned while generic frontmatter mutation lives in `speccy`.

## Acceptance Criteria

- [x] [SRS-04/AC-01] Planning and voyage artifacts record which concerns remain host-owned after the extraction, including the final decision that generic frontmatter mutation lives in `speccy`. <!-- verify: manual, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-03/AC-01] The documented hook model supports embedded or caller-managed template catalogs without forcing filesystem assumptions into `speccy`. <!-- verify: cargo test -p speccy, SRS-NFR-03:start:end, proof: ac-2.log-->
