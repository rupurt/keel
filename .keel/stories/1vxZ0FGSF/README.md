---
id: 1vxZ0FGSF
title: Add Drift Tests For Canonical Guidance Contracts
type: feat
status: backlog
created_at: 2026-03-03T15:18:11
updated_at: 2026-03-03T15:18:11
scope: 1vxYzSury/1vxYzsAxT
---

# Add Drift Tests For Canonical Guidance Contracts

## Summary

Add drift tests that enforce canonical command guidance contracts and prevent actionable/informational classification regressions.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add drift tests asserting actionable management commands emit canonical guidance with the expected contract shape.
- [ ] [SRS-01/AC-02] Add drift tests asserting informational management commands omit canonical guidance fields.
- [ ] [SRS-01/AC-03] Ensure drift tests fail on contract-key changes or classification regressions that would break harness automation.
