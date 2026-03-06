---
id: 1vyWRl000
title: Build Artifact Bundle Materialization
type: feat
status: done
created_at: 2026-03-06T06:46:33
updated_at: 2026-03-06T08:55:48
scope: 1vyWLl000/1vyWNV000
index: 2
started_at: 2026-03-06T08:51:44
completed_at: 2026-03-06T08:55:48
---

# Build Artifact Bundle Materialization

## Summary

Implement bundle materialization so the verification executor can package a story's evidence into the canonical artifact-judge input before invoking any external semantic judge.

## Acceptance Criteria

- [x] [SRS-02/AC-01] The verification executor materializes the artifact bundle from story evidence before `llm-judge` runs. <!-- verify: cargo test -p keel verification_executor_materializes_judge_bundle, SRS-02:start:end, proof: ac-1.log-->
