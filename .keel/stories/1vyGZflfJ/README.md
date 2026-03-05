---
id: 1vyGZflfJ
title: Detect Scope Drift During Planning
type: feat
status: backlog
created_at: 2026-03-05T13:49:39
updated_at: 2026-03-05T14:10:02
scope: 1vyFgR2MA/1vyFn0OuN
---

# Detect Scope Drift During Planning

## Summary

Detect scope drift during planning so voyages cannot quietly claim work outside the PRD’s approved boundary or omit required scope mappings.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Doctor diagnostics report unknown scope refs, missing scope mappings, and direct contradictions with PRD out-of-scope definitions. <!-- verify: cargo test -p keel doctor_reports_scope_drift_and_contradictions, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] [SRS-NFR-02/AC-01] Scope drift failures identify the artifact, offending scope ID, and contradiction type. <!-- verify: cargo test -p keel scope_drift_errors_are_actionable, SRS-NFR-02:start:end -->
- [ ] [SRS-03/AC-03] [SRS-NFR-03/AC-01] Scope validation rejects legacy untagged compatibility paths instead of keeping fallback parsing. <!-- verify: cargo test -p keel scope_lineage_rejects_legacy_untagged_paths, SRS-NFR-03:start:end -->
