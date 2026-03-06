---
id: 1vyWRm000
title: Persist Judge Outputs In Verification Evidence
type: feat
status: backlog
created_at: 2026-03-06T06:46:34
updated_at: 2026-03-06T06:50:33
scope: 1vyWLl000/1vyWNV000
index: 4
---

# Persist Judge Outputs In Verification Evidence

## Summary

Persist semantic judge outputs as normal story evidence so `verify run` and `story record --judge` both produce auditable transcripts and leave failed judge runs inspectable.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] [SRS-NFR-03/AC-01] `keel verify run` and `keel story record --judge` persist judge transcripts/results as evidence, report failures against the evaluated acceptance criterion, and preserve failed artifact bundles and transcript/debug outputs for manual inspection. <!-- verify: cargo test -p keel judge_results_persist_as_story_evidence, SRS-04:start:end, SRS-NFR-03:start:end -->
