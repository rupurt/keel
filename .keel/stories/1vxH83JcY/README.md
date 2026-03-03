---
id: 1vxH83JcY
title: Canonicalize Template Tokens To Schema Names
type: refactor
status: backlog
created_at: 2026-03-02T20:13:03
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzV3oR
index: 1
---

# Canonicalize Template Tokens To Schema Names

## Summary

Replace non-canonical template token names with canonical schema/frontmatter-mirrored tokens and align creation render inputs with the new token vocabulary.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Replace legacy token names (for example `{{date}}`, `{{datetime}}`) in active planning templates with canonical schema-mirrored tokens.
- [ ] [SRS-01/AC-02] Update rendering callsites in creation paths so all canonical tokens are populated correctly without fallback alias handling.
- [ ] [SRS-01/AC-03] Add regression tests asserting deprecated token aliases are absent from embedded templates.
