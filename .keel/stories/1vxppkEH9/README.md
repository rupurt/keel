---
id: 1vxppkEH9
title: Render Concrete Evidence In Story Show
type: feat
status: done
created_at: 2026-03-04T09:16:28
updated_at: 2026-03-04T11:30:01
scope: 1vxYzSury/1vxpomgnN
started_at: 2026-03-04T11:18:57
completed_at: 2026-03-04T11:30:01
---

# Render Concrete Evidence In Story Show

## Summary

Rework `keel story show` evidence output to display real proof details (metadata, excerpts, artifact files, and media assets) so human acceptance can happen directly from command output.

## Acceptance Criteria

- [x] [SRS-04/AC-01] For each AC with verify annotations, `keel story show <id>` renders command/manual mode, proof filename, and parsed proof metadata (`recorded_at`, command/mode) when available. <!-- verify: cargo test --lib story_show_proof_metadata, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] `keel story show <id>` surfaces concrete artifact lists, explicitly separating annotation-linked proofs from supplementary artifacts and media files (for example `.gif`, `.png`, `.mp4`, `.webm`) and includes whole-asset playback guidance. <!-- verify: cargo test --lib story_show_artifact_inventory, SRS-04:continues, proof: ac-2.log-->
- [x] [SRS-04/AC-03] `keel story show <id>` includes readable proof excerpts capped at 10 lines for text proofs, plus missing-proof warnings so acceptance decisions do not require separate file navigation. <!-- verify: cargo test --lib story_show_proof_excerpt_10_lines_and_warnings, SRS-04:end, proof: ac-3.log-->
- [x] [SRS-04/AC-04] [SRS-NFR-02/AC-02] Evidence sections render explicit placeholder text when no proof artifacts exist or when evidence directories are absent. <!-- verify: cargo test --lib story_show_missing_evidence_placeholders, SRS-NFR-02:start:end, proof: ac-4.log-->
