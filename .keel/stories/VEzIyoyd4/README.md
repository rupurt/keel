---
# system-managed
id: VEzIyoyd4
status: backlog
created_at: 2026-03-26T08:30:42
updated_at: 2026-03-26T08:37:19
# authored
title: Document Speccy Hooks And External Adoption Boundaries
type: feat
operator-signal:
scope: VEzIwU3fh/VEzIxN01G
index: 3
---

# Document Speccy Hooks And External Adoption Boundaries

## Summary

Define and document the public hook surface and the boundary between reusable `speccy` behavior and host-owned project logic so other projects can adopt the crate intentionally.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Planning and voyage artifacts record which concerns remain host-owned after the extraction, including any deferred treatment of generic frontmatter mutation. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-NFR-03/AC-01] The documented hook model supports embedded or caller-managed template catalogs without forcing filesystem assumptions into `speccy`. <!-- verify: cargo test -p speccy, SRS-NFR-03:start:end -->
