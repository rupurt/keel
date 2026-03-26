---
# system-managed
id: VEzIyo8d2
status: done
created_at: 2026-03-26T08:30:42
updated_at: 2026-03-26T08:56:29
# authored
title: Extract Speccy Template Rendering Primitives Into A Reusable Workspace Crate
type: feat
operator-signal:
scope: VEzIwU3fh/VEzIxN01G
index: 1
started_at: 2026-03-26T08:50:15
completed_at: 2026-03-26T08:56:29
---

# Extract Speccy Template Rendering Primitives Into A Reusable Workspace Crate

## Summary

Create the new `speccy` workspace crate and move the generic markdown template rendering primitives into it without importing any Keel-specific modules or board concepts.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `speccy` exposes deterministic placeholder rendering and markdown document helper APIs equivalent to the current generic behavior in `template_rendering.rs`. <!-- verify: cargo test -p speccy, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `speccy` exposes a host integration hook surface for template lookup and optional post-render behavior without importing Keel-specific types, file paths, or board concepts. <!-- verify: cargo test -p speccy, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] The new crate remains free of `keel-core` and `keel-cli` dependencies and is covered by crate-level tests for representative placeholder and frontmatter/body cases. <!-- verify: cargo test -p speccy, SRS-NFR-01:start:end, proof: ac-3.log-->
