# Remove File Heartbeat And Align Pacemaker Operations - SRS

## Summary

Epic: VF7Geb3Wa
Goal: Remove the .keel/heartbeat dependency and realign hooks, diagnostics, poke behavior, and documentation around the Git-derived pacemaker model.

## Scope

### In Scope

- [SCOPE-01] Remove file-backed heartbeat loading and fallback behavior from core board, cache, graph, and flow paths.
- [SCOPE-02] Stop hooks and `keel poke` from mutating or staging `.keel/heartbeat`.
- [SCOPE-03] Update doctor, pacemaker messaging, and downstream-facing docs to describe the derived heartbeat model.
- [SCOPE-04] Prove the cutover with regression tests and docs/build verification.

### Out of Scope

- [SCOPE-05] Adding a daemon, watcher process, or background activity stream.
- [SCOPE-06] Reworking inbox/ping/pong behavior beyond removing its heartbeat-file side effect.
- [SCOPE-07] Broad changes to flow capacity or delivery heuristics unrelated to heartbeat semantics.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Core board models, cache invalidation, graph surfaces, and `flow` must stop depending on `.keel/heartbeat` as a required control path or compatibility fallback. | SCOPE-01 | FR-04 | automated tests + code review |
| SRS-02 | The pre-commit hook and `keel poke` must stop mutating or staging `.keel/heartbeat` while preserving their non-heartbeat responsibilities. | SCOPE-02 | FR-04 | command tests + manual review |
| SRS-03 | Doctor and pacemaker messaging must explain the derived heartbeat model and frame commit/hook lifecycle as the governing stabilizers once the file path is removed. | SCOPE-03 | FR-05 | CLI proof + docs review |
| SRS-04 | Foundational docs, MDX docs, and downstream upgrade guidance must teach the new pacemaker model and remove instructions to commit `.keel/heartbeat`. | SCOPE-03 SCOPE-04 | FR-05 | docs build + artifact review |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The final model must leave no hidden file-backed pacemaker dependency in code paths that affect operator state. | SCOPE-01 SCOPE-02 SCOPE-03 | NFR-02 | code review + tests |
| SRS-NFR-02 | Public and downstream docs must remain internally consistent about the new heartbeat semantics after the cutover. | SCOPE-03 SCOPE-04 | NFR-04 | docs review + docs build |
| SRS-NFR-03 | Relevant fmt, clippy, tests, and docs verification must pass before the voyage can close. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-04 | NFR-03 | fmt + clippy + tests + docs build |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
