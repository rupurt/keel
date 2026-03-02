---
id: 1vugz2cYw
title: Automate Narrative PR Release Generation
type: feat
status: done
created_at: 2026-02-23T17:13:04
updated_at: 2026-02-24T10:44:46
scope: 1vugyr0OR/1vugyujor
index: 3
completed_at: 2026-02-24T00:00:00
---

# Automate Narrative PR Release Generation

## Summary

When a block of work (Voyage) is completed, Keel should generate a high-fidelity Pull Request description. This PR should tell the story of the feature by stitching together story summaries, `REFLECT.md` insights, and embedding the `vhs` recordings/LLM transcripts as proof. The result should be stored in `PRESS_RELEASE.md` within the voyage directory.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel voyage done` generates a `PRESS_RELEASE.md` containing the narrative summary and evidence links <!-- verify: llm-judge, SRS-03:start:end -->
