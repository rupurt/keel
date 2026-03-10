---
id: VDUGfcLQB
title: Update Flow Terminology
type: feat
status: done
created_at: 2026-03-10T10:38:13
updated_at: 2026-03-10T13:58:06
scope: VDTpFlMKc/VDUG60pcX
index: 2
started_at: 2026-03-10T13:56:16
completed_at: 2026-03-10T13:58:06
---

# Update Flow Terminology

## Summary

Update queue names from Human/Agent to Management/Execution.

## Acceptance Criteria

- [x] [SRS-04/AC-01] `keel flow` labels change from "Human Queue" to "Management Queue" and "Agent Queue" to "Execution Queue" <!-- verify: cargo test cli::presentation::flow::display::tests:: -- --nocapture, SRS-04:start, proof: ac-1.log -->
- [x] [SRS-04/AC-02] Update `keel flow` docs and help text <!-- verify: cargo test command_help_docs_describe_role_based_queue_terms -- --nocapture, SRS-04:end, proof: ac-2.log -->
