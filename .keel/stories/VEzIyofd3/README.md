---
# system-managed
id: VEzIyofd3
status: backlog
created_at: 2026-03-26T08:30:42
updated_at: 2026-03-26T08:37:19
# authored
title: Migrate Keel Markdown Template Rendering Onto Speccy
type: feat
operator-signal:
scope: VEzIwU3fh/VEzIxN01G
index: 2
---

# Migrate Keel Markdown Template Rendering Onto Speccy

## Summary

Define the host integration hook surface needed for Keel, then rewire existing template-rendering call sites to consume `speccy` for generic rendering while keeping only host-specific adapter behavior in Keel.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `speccy` defines the host integration hook surface Keel needs for template lookup and optional post-render behavior without introducing Keel-specific types into the reusable crate. <!-- verify: cargo test -p speccy, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Keel call sites that currently use `template_rendering::{render, render_body, render_with_mutations}` consume `speccy` for the generic rendering path. <!-- verify: cargo test -p keel, SRS-03:start -->
- [ ] [SRS-NFR-02/AC-01] Representative Keel scaffold-generation flows continue to produce behaviorally equivalent output after the cutover. <!-- verify: cargo test -p keel, SRS-NFR-02:start:end, SRS-03:end -->
