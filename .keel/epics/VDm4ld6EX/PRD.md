# Compact Status And Mission Drilldown - Product Requirements

## Problem Statement

Users need a compact --status snapshot and a deeper mission-next exploration path so they can act on status reports without reading the full board state.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Let users pull a compact status answer without reading a full mission or board report. | `keel mission next --status` returns exactly three short bullets with clear next steps. | Contract agreed and implemented |
| GOAL-02 | Let users expand from status into a richer drilldown without changing commands mentally. | `keel mission next` exposes the next layer of evidence, rationale, and recommended action below the status summary. | Contract agreed and implemented |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator or manager checking progress | Someone who wants a fast answer first, then deeper context only if needed. | A short status summary that immediately points to the next action. |

## Scope

### In Scope

- [SCOPE-01] A compact `--status` surface for mission progress.
- [SCOPE-02] A richer default `mission next` drilldown directly below the compact summary contract.
- [SCOPE-03] Clear guidance that tells users how to make progress before the next status check.

### Out of Scope

- [SCOPE-04] Full dashboard redesign across unrelated commands.
- [SCOPE-05] New persistent background schedulers or hosted automation.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Add a compact status mode that returns exactly three short bullets. | GOAL-01 | must | Establishes the low-friction progress surface users asked for. |
| FR-02 | Ensure each status bullet points at a concrete next action instead of generic narrative. | GOAL-01 | must | Makes the summary operational instead of informational only. |
| FR-03 | Keep the default `mission next` output as the richer exploration surface below the compact mode. | GOAL-02 | must | Preserves a clear expand-from-summary workflow. |
| FR-04 | Surface enough evidence in the richer mode to explain why the recommended next action exists. | GOAL-02 | should | Prevents users from needing to jump across commands for basic context. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Keep compact status output deterministic and snapshot-friendly. | GOAL-01 | must | Status mode should be easy to diff and trust in automation. |
| NFR-02 | Keep line lengths short enough for terminal summaries. | GOAL-01 | must | The mode is intended for low-friction scanning. |
| NFR-03 | Preserve the richer mode's usefulness without forcing users through multiple extra commands. | GOAL-02 | should | The deeper path should still feel direct. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Compact status contract | Snapshot tests or CLI proofs over `mission next --status` | Stable examples showing three short bullets |
| Drilldown usefulness | Manual review and CLI proofs over default `mission next` | Examples showing summary plus deeper guidance |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Users want a two-level "summary then expand" navigation model. | The chosen split between compact and rich modes may be wrong. | Validate against real command walkthroughs during decomposition. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should `--status` live on `mission next` only or be generalized later? | Epic owner | Open |
| How much evidence belongs in rich mode before it becomes noisy again? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Users can get a useful progress answer from `mission next --status` in one screenful.
- [ ] Users can move from compact status to richer drilldown without command confusion.
<!-- END SUCCESS_CRITERIA -->
