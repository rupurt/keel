---
id: 1vzQwp000
title: Capture Web Academic Social And Manual Evidence Through One Workflow
type: feat
status: done
created_at: 2026-03-08T20:06:23
updated_at: 2026-03-08T20:57:05
scope: 1vzQpr000/1vzQu0000
index: 3
started_at: 2026-03-08T20:49:01
completed_at: 2026-03-08T20:57:05
---

# Capture Web Academic Social And Manual Evidence Through One Workflow

## Summary

Implement one canonical research ingestion workflow that can capture web, academic, social, and manual evidence into the shared evidence contract while preserving provider provenance for each source.

## Acceptance Criteria

- [x] [SRS-02/AC-01] The research workflow accepts web, academic or prior-art, social or trend, and manual or internal evidence through one canonical command and service path. <!-- verify: cargo test -p keel research_workflow_supports_all_signal_classes, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Evidence captured from each source class persists through the shared canonical source schema with provider provenance attached to every stored record. <!-- verify: cargo test -p keel research_capture_persists_provenance_for_all_signal_classes, SRS-02:end, proof: ac-2.log-->
