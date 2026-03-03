---
id: 1vxH83JcY
title: Canonicalize Template Tokens To Schema Names
type: refactor
status: done
created_at: 2026-03-02T20:13:03
updated_at: 2026-03-02T21:55:02
scope: 1vxGy5tco/1vxGzV3oR
index: 1
started_at: 2026-03-02T21:31:56
submitted_at: 2026-03-02T21:42:18
completed_at: 2026-03-02T21:55:02
---

# Canonicalize Template Tokens To Schema Names

## Summary

Replace non-canonical template token names with canonical schema/frontmatter-mirrored tokens and align creation render inputs with the new token vocabulary.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Replace legacy token names (for example `date`, `datetime`) in active planning templates with canonical schema-mirrored tokens. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Update rendering callsites in creation paths so all canonical tokens are populated correctly without fallback alias handling. <!-- verify: manual, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests asserting deprecated token aliases are absent from embedded templates. <!-- verify: manual, SRS-01:end, proof: ac-3.log-->
