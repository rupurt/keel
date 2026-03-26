---
# system-managed
id: VF0XEmsbm
status: backlog
created_at: 2026-03-26T13:33:37
updated_at: 2026-03-26T13:37:12
# authored
title: Reduce Speccy Public API And Rewire Keel To The Smaller Surface
type: feat
operator-signal:
scope: VF0XAFqlF/VF0XBQxJ5
index: 2
---

# Reduce Speccy Public API And Rewire Keel To The Smaller Surface

## Summary

Reduce `speccy`'s public rendering surface so the crate exposes a smaller, options-driven API and Keel consumes that reduced contract. This pass should remove the helper matrix that currently multiplies top-level render entrypoints.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `speccy` exposes a reduced render API centered on core entrypoints plus options instead of separate top-level helper combinations. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Keel's template rendering adapters and direct callers are updated to use the reduced `speccy` surface. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-NFR-01/AC-01] Automated verification proves the reduced API preserves current supported render and frontmatter mutation behavior. <!-- verify: manual, SRS-NFR-01:start:end -->
