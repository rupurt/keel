---
id: 1vzQwk000
title: Cut Over Bearing Lifecycle Commands And Guidance To Research Language
type: feat
status: done
created_at: 2026-03-08T20:06:18
updated_at: 2026-03-08T20:30:09
scope: 1vzQpr000/1vzQtq000
index: 2
started_at: 2026-03-08T20:25:00
completed_at: 2026-03-08T20:30:09
---

# Cut Over Bearing Lifecycle Commands And Guidance To Research Language

## Summary

Cut the CLI and guidance surfaces over from survey semantics to research semantics so bearing operators see one canonical command path and one consistent lifecycle vocabulary.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Bearing lifecycle commands, clap help, and runtime dispatch replace the survey-era command path with one canonical research command path. <!-- verify: cargo test -p keel bearing_research_command_contract, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Generated next-step guidance, command examples, and user-facing docs use research language consistently for the evidence stage. <!-- verify: cargo test -p keel bearing_research_guidance_and_docs_are_consistent, SRS-02:continues, proof: ac-2.log-->
- [x] [SRS-02/AC-03] [SRS-NFR-02/AC-01] Equivalent bearing states produce deterministic research-stage guidance and migration messages across human-readable and JSON command output. <!-- verify: cargo test -p keel bearing_research_guidance_is_deterministic, SRS-NFR-02:start:end, SRS-02:end, proof: ac-3.log-->
