# Observational Knowledge Synthesis - Software Requirements Specification

> Automate the aggregation of story reflections into voyage knowledge artifacts.

**Epic:** [1vv7YWzw2](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

- Implement automated aggregation of `REFLECT.md` files from stories into a voyage-level `KNOWLEDGE.md`.
- Trigger synthesis when a voyage is transitioned to the `done` state.
- Ensure the synthesis process is observational and non-destructive.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Implement reflection aggregation logic to collect insights from all stories in a voyage. | PRD-02 | automated test |
| SRS-02 | Integrate knowledge synthesis into the `voyage done` command. | PRD-02 | automated test |
| SRS-03 | Define a clear schema for aggregated knowledge artifacts in `KNOWLEDGE.md`. | PRD-02 | manual inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Knowledge synthesis must be idempotent and not overwrite user modifications in KNOWLEDGE.md (if any). | NFR-01 | manual verification |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
