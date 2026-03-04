# Planning Read Surfaces And Evidence Visibility - Software Requirements Specification

> Make epic/voyage/story show outputs planning-ready, verification-aware, and acceptance-friendly.

**Epic:** [1vxYzSury](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- Upgrade `keel epic show`, `keel voyage show`, and `keel story show` to surface authored planning and verification evidence (not only metadata or lineage labels).
- Add deterministic progress and requirement coverage views so planning status is actionable from the terminal.
- Surface proof artifacts (including media such as `.gif`) in human-review friendly output.

Out of scope:
- Changes to lifecycle transitions (`start`, `submit`, `accept`, etc.).
- New artifact capture workflows beyond existing `verify`/`record` behavior.
- Schema migrations for historical board artifacts.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Epic PRDs and voyage SRS files contain authored sections or structured tables. | Content | Summary extraction quality degrades; output must fall back to explicit placeholders. |
| Story evidence remains in `stories/<id>/EVIDENCE/` and verification annotations stay in story AC comments. | Data contract | Evidence rendering may miss artifacts or misclassify verification state. |
| Throughput data can be derived from board story completion timestamps. | Runtime signal | Epic ETA must degrade to "insufficient throughput data" instead of misleading estimates. |

## Constraints

- Preserve hard-cutover policy: one canonical rendering path per command, no legacy compatibility branches.
- Keep output deterministic (stable ordering and labels) so snapshots and harness parsing are reliable.
- Avoid requiring markdown authors to rewrite all existing artifacts before command rollout.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | `keel epic show` MUST render a planning summary that surfaces the epic problem, goals/objectives, and key requirements from authored planning docs. | FR-01 | parser unit tests + CLI snapshot tests |
| SRS-02 | `keel epic show` MUST render progress and verification readiness, including automated/manual verification coverage, evidence artifact inventory, and a time-to-complete estimate when throughput is available. | FR-01 | read-model tests + CLI snapshot tests |
| SRS-03 | `keel voyage show` MUST render voyage goal/scope and a requirements progress table mapping requirements to linked stories and completion state. | FR-01 | requirements mapping tests + CLI snapshot tests |
| SRS-04 | `keel story show` MUST render concrete evidence details (proof file, metadata, excerpt, and supplemental/media artifacts) for each acceptance criterion instead of only abstract evidence-chain lines. | FR-01 | filesystem fixture tests + CLI snapshot tests |
| SRS-05 | The three `show` commands MUST consume a shared planning-read projection so contracts remain consistent and reusable for future chat summaries. | FR-01 | shared projection unit tests + command integration tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Show output MUST be deterministic (stable section order, stable sorting for requirements/stories/artifacts). | NFR-01 | deterministic regression tests |
| SRS-NFR-02 | Missing authored planning/evidence data MUST render explicit placeholder messages rather than silently omitting sections. | NFR-01 | missing-data fixture tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
