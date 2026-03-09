---
id: 1vzQwm000
title: Model Canonical Evidence Records And Parsing Rules
type: feat
status: backlog
created_at: 2026-03-08T20:06:20
updated_at: 2026-03-08T20:10:04
scope: 1vzQpr000/1vzQu0000
index: 1
---

# Model Canonical Evidence Records And Parsing Rules

## Summary

Define the canonical evidence record schema and parsing rules for `EVIDENCE.md` so every research source carries stable IDs, provenance, dates, and quality metadata that downstream scoring and rendering can trust.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `EVIDENCE.md` supports canonical source records with stable IDs plus required metadata for source class, provenance, publication or observation date, retrieval date, authority, and freshness. <!-- verify: cargo test -p keel evidence_record_schema_is_canonical, SRS-01:start, proof: ac-1.log-->
- [ ] [SRS-01/AC-02] The evidence parser and doctor checks reject malformed or unresolved scaffold entries instead of accepting partially structured research notes. <!-- verify: cargo test -p keel evidence_parser_rejects_malformed_records, SRS-01:end, proof: ac-2.log-->
- [ ] [SRS-01/AC-03] [SRS-NFR-01/AC-01] Equivalent evidence fixtures normalize into deterministic record ordering and metadata output. <!-- verify: cargo test -p keel evidence_record_parsing_is_deterministic, SRS-NFR-01:start:end, SRS-01:end, proof: ac-3.log-->
