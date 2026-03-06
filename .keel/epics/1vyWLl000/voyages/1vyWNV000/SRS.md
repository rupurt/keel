# Artifact-Aware Judge Contract - Software Requirements Specification

> Upgrade llm-judge so it evaluates artifact bundles from dogfood runs through a provider-agnostic contract instead of judging only git diff text.

**Epic:** [1vyWLl000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-05] Define the artifact-bundle contract used to judge tape-driven evidence.
- [SCOPE-06] Replace the diff-only `llm-judge` stub with a provider-agnostic external command contract.
- [SCOPE-06] Persist judge transcripts/results as first-class evidence linked to the evaluated acceptance criteria.

Out of scope:
- [SCOPE-02] Additional epic workflow tape authoring.
- [SCOPE-03] Additional bearing workflow tape authoring.
- [SCOPE-07] Mandatory CI enforcement for dogfood runs.
- [SCOPE-09] Hardcoded vendor SDK integration inside keel.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Phase 1 produces stable artifact sets (for example rendered tapes plus transcripts/logs) that can be bundled for judging. | Evidence input | The judge contract will not have enough context to evaluate acceptance criteria. |
| An external `llm-judge` executable or wrapper can be supplied by the environment. | Provider integration | Keel would need to become provider-specific or ship its own remote client. |
| Acceptance criteria text plus artifact references are sufficient to let a semantic judge produce a pass/fail transcript. | Evaluation model | The bundle schema may need richer planning metadata. |

## Constraints

- The judge path must remain provider-agnostic.
- `llm-judge` must evaluate explicit artifact bundles, not raw git diff output.
- Failure paths must preserve intermediate bundle and transcript artifacts for inspection.
- One canonical artifact-bundle schema should serve both `verify run` and `story record --judge`.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Define a machine-readable artifact-bundle schema that includes story metadata, acceptance-criterion text, and references to tape-driven evidence artifacts. | SCOPE-05 | FR-06 | schema tests + fixture bundle assertions |
| SRS-02 | The verification executor MUST materialize the artifact bundle from story evidence before invoking `llm-judge`. | SCOPE-05 | FR-06 | executor tests + bundle creation assertions |
| SRS-03 | `llm-judge` execution MUST call a provider-agnostic external contract that accepts an artifact-bundle path and returns a pass/fail transcript result. | SCOPE-06 | FR-07 | mocked judge command tests + contract integration tests |
| SRS-04 | `keel verify run` and `keel story record --judge` MUST persist judge transcripts/results as evidence and report failures against the evaluated acceptance criterion. | SCOPE-06 | FR-06 | command tests + manifest assertions |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The artifact-bundle schema MUST be deterministic for equivalent inputs. | SCOPE-05 | NFR-01 | deterministic bundle serialization tests |
| SRS-NFR-02 | The judge integration MUST remain provider-agnostic and must not hardcode a vendor SDK or transport. | SCOPE-06 | NFR-04 | dependency audit tests + contract-path assertions |
| SRS-NFR-03 | Judge failures MUST preserve the artifact bundle and transcript/debug outputs for manual inspection. | SCOPE-06 | NFR-04 | failure-path integration tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
