# Role and Lane Config Contract - Software Requirements Specification

> Define a config-driven role and lane contract that replaces hardcoded manager/engineer routing, supports configurable lane selectors with sensible defaults, and provides a clear implementation decomposition.

**Epic:** [VDVPODBXF](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Config schema and seeded defaults for workflow roles, lanes, and exact role overrides.
- [SCOPE-02] Config-driven lane resolution for `next`, `accept`, role-context selection, and other role-aware command guidance.
- [SCOPE-03] Lane selector globs, include/exclude semantics, and topology validation.
- [SCOPE-04] Dynamic rendering of effective topology in `keel config show` and `keel flow`.

### Out of Scope

- [SCOPE-05] User-authored template body files loaded directly from config.
- [SCOPE-06] Multiple default lanes or shared cross-lane visibility policies.
- [SCOPE-07] New lifecycle states for stories, voyages, bearings, or epics.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The existing taxonomy parser continues to accept subtype-free roles such as `operator` or `manager` | Dependency | Queue access would still require subtype-shaped examples |
| Current flow and next algorithms can be parameterized by topology without changing entity state machines | Assumption | The voyage would need deeper architectural changes than planned |
| Layered TOML config loading remains the canonical configuration mechanism | Dependency | A parallel config path would create drift across CLI surfaces |

## Constraints

- Hard cutover applies: remove literal manager/engineer routing instead of preserving compatibility aliases.
- Seeded defaults must keep the CLI usable even when the new config sections are absent.
- Lane selectors must target canonical board state identifiers and use one shared matcher across validation, routing, and rendering.
- Role subtype remains optional for lane access but must still participate in existing fine-grained entity matching when a story or entity requires it.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The config loader must resolve an effective workflow topology from `workflow.defaults`, `roles`, `lanes`, and `role_overrides`, seeding `manager`/`operator` roles and `management`/`delivery` lanes when absent. | SCOPE-01 | FR-01 | unit test |
| SRS-02 | `keel config show` must render the effective workflow defaults, configured role families, lane definitions, and exact taxonomy overrides, including seeded defaults when the board omits topology config. | SCOPE-01 | FR-03 | integration test |
| SRS-03 | Lane definitions must support `description`, ordered `include`, `exclude`, `parallel`, `manual_accept`, and `priority`, where selectors are glob patterns over canonical board state identifiers such as `story.backlog` and `bearing.exploring`. | SCOPE-02 | FR-04 | unit test |
| SRS-04 | `keel next --role <taxonomy>` must resolve the base role family through config, map it to the configured default lane, and reject unknown families with guidance based on the configured default role examples. | SCOPE-03 | FR-02 | integration test |
| SRS-05 | `keel next --parallel` must be allowed only when the resolved lane has `parallel = true`, and the recovery guidance must reference the configured delivery role example instead of a hardcoded family. | SCOPE-03 | FR-08 | unit test |
| SRS-06 | `keel story accept --role <taxonomy>` must authorize manual verification when the resolved lane has `manual_accept = true`; stories without manual verification criteria remain acceptable for any valid role. | SCOPE-03 | FR-05 | integration test |
| SRS-07 | Role context and actionable guidance must resolve by configured base role family, with exact `role_overrides` entries taking precedence for template selection when the full taxonomy matches. | SCOPE-03 | FR-06 | unit test |
| SRS-08 | `keel flow` must render configured lanes dynamically in deterministic `priority` order and count only the work items selected by each lane's resolved source set. | SCOPE-04 | FR-03 | integration test + snapshot |
| SRS-09 | `keel doctor` must fail on missing workflow defaults, missing role-to-lane references, unknown selectors, and any cross-lane source overlap. | SCOPE-03 | FR-07 | integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | With no topology overrides in `keel.toml`, the effective topology must preserve the seeded `manager`/`operator` roles and `management`/`delivery` lanes. | SCOPE-01 | NFR-02 | unit test |
| SRS-NFR-02 | The same config and taxonomy input must produce identical lane, capability, and template resolution across repeated calls. | SCOPE-03 | NFR-01 | unit test |
| SRS-NFR-03 | Selector compilation and evaluation must fail fast with precise errors and must not silently ignore unknown patterns or sources. | SCOPE-03 | NFR-03 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
