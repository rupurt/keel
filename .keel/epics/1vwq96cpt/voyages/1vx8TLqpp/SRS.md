# Normalize Physical DDD Module Layout - Software Requirements Specification

> Physically align src with DDD layers: cli, application, domain, infrastructure, read_model; remove legacy top-level module duplication.

**Epic:** [1vwq96cpt](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

Deliver a full physical normalization of `src/` into explicit DDD layers so module
placement matches architecture intent and is easy to navigate:

- In scope:
  - Relocate CLI adapters and terminal formatting into `src/cli/**`.
  - Relocate core domain entities/policies/transitions into `src/domain/**`.
  - Relocate infrastructure adapters/services into `src/infrastructure/**`.
  - Keep `src/application/**` and `src/read_model/**` as dedicated layers.
  - Add tests/contracts that fail if legacy top-level module families reappear.
- Out of scope:
  - New product behavior unrelated to structural normalization.
  - Changing command UX semantics except where required for module relocation.

## Assumptions & Dependencies

<!-- What we assume to be true; external systems, services, or conditions we depend on -->

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing regression tests capture current CLI behavior for core flows. | quality gate | Structural moves could silently regress behavior. |
| `just keel doctor`, `just quality`, and `just test` remain authoritative release checks. | process | Normalization may pass local compile but violate board/quality invariants. |
| Story acceptance command date-format issue is handled in a separate follow-up. | dependency | This voyage may report unrelated known issue noise during acceptance workflows. |

## Constraints

- Preserve backward-compatible behavior for supported commands.
- Ship in small, verifiable story slices that can be reviewed independently.
- Keep domain rules independent of CLI rendering concerns.
- Avoid introducing cyclical dependencies between DDD layers.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | All command-line entrypoints, clap routing, and terminal rendering adapters shall live under `src/cli/**`, with no active top-level `src/commands/**`, `src/flow/**`, or `src/next/**` module families. | PRD-01 | architecture contract tests + `rg --files` inspection |
| SRS-02 | Core business entities, policies, transition guards, and invariants shall live under `src/domain/**`, with no active top-level `src/model/**`, `src/policy/**`, `src/state_machine/**`, or `src/transitions/**` module families. | PRD-01 | architecture contract tests + compile + regression tests |
| SRS-03 | Persistence, parsing/loading, template rendering, generation, and verification adapters shall live under `src/infrastructure/**`, with no duplicated legacy top-level service modules. | PRD-02 | architecture contract tests + compile + regression tests |
| SRS-04 | Module declarations in `main.rs` shall expose normalized layer roots (`cli`, `application`, `domain`, `infrastructure`, `read_model`) and remove legacy top-level roots from active dispatch wiring. | PRD-03 | compile + architecture contract tests |
| SRS-05 | Architecture contract tests shall explicitly fail on reintroduction of forbidden top-level legacy module families and cross-layer dependency violations after normalization. | PRD-04 | automated test suite |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Normalization changes shall preserve behavior of existing command regression tests for story, voyage, epic, doctor, and flow operations. | NFR-01 | `just test` |
| SRS-NFR-02 | Each migration slice shall be independently reviewable and verifiable through story-level evidence and reflection artifacts. | NFR-02 | story manifests + review inspection |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
