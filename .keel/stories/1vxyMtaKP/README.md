---
id: 1vxyMtaKP
title: Doctor Check For Parallel Conflict Coherence
type: feat
status: done
created_at: 2026-03-04T18:23:15
updated_at: 2026-03-04T19:05:55
scope: 1vxyM0hvn/1vxyMT6nz
started_at: 2026-03-04T19:02:49
completed_at: 2026-03-04T19:05:55
---

# Doctor Check For Parallel Conflict Coherence

## Summary

Add doctor checks that validate explicit and inferred parallel conflict signals for coherence and actionable remediation.

## Acceptance Criteria

- [x] [SRS-07/AC-01] Doctor reports invalid `blocked_by` references and contradictory pair constraints as errors. <!-- verify: cargo test --lib doctor_parallel_conflict_coherence_checks, SRS-07:start:end, proof: ac-1.log-->
- [x] [SRS-07/AC-02] Doctor output includes specific story pairs and remediation guidance. <!-- verify: cargo test --lib doctor_parallel_conflict_reports_actionable_pairs, SRS-07:start:end, proof: ac-2.log-->
