# Automated Verification Techniques Bank - Product Requirements

> Enable a reusable bank of automated verification techniques that keel can configure via keel.toml and infer via project autodetection.

## Problem Statement

Verification planning was fragmented across static assumptions and command-specific heuristics, which made recommendations inconsistent and hard to trust. Teams needed one canonical technique catalog that can be configured, autodetected per project context, and surfaced consistently through command interfaces so verification planning remains evidence-driven and repeatable.

## Goals & Objectives

| Goal | Success Metric | Target |
|------|----------------|--------|
| Establish a reusable technique catalog | Verification technique inventory is defined once and reused across config/verify surfaces | 100% of supported built-in techniques modeled in catalog |
| Support project-aware planning | Detection and recommendation commands reflect project signals deterministically | Stable recommendations for the same workspace signals |
| Enable explicit operator control | `keel.toml` overrides allow enabling/disabling techniques without code changes | Config overrides honored across all surfaced commands |
| Complete command-surface cutover | Verification technique surfaces move to canonical config/verify commands | Legacy/duplicated surfaces removed in scope |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Planner | Defines verification strategy before decomposition closes | Clear recommendation of viable techniques for current project context |
| Implementer Agent | Executes stories and records evidence | Deterministic commands to discover and apply active techniques |
| Maintainer | Owns verification command contracts and config schema | One authoritative model with hard-cutover behavior |

## Scope

### In Scope

- Define a canonical verification-technique catalog model and metadata schema.
- Implement project-signal autodetection and recommendation logic tied to active techniques.
- Add `keel.toml` overrides for technique enable/disable behavior.
- Cut over verification technique command surfaces to config/verify commands with machine-readable output.

### Out of Scope

- Building external SaaS integrations for verification execution orchestration.
- Supporting backward-compatible legacy verify command contracts during cutover.
- Introducing manual evidence workflows unrelated to automated technique selection.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | The system must provide a canonical catalog of verification techniques with metadata required for detection and recommendation. | must | Creates a single source of truth for verification planning. |
| FR-02 | `keel.toml` must allow explicit per-technique enable/disable overrides. | must | Gives operators deterministic control over active techniques. |
| FR-03 | `keel verify detect` must analyze project signals and report detected technique eligibility. | must | Grounds recommendations in workspace reality. |
| FR-04 | `keel verify recommend` must return only techniques that are both detected and active. | must | Keeps recommendations actionable and accurate. |
| FR-05 | `keel config show` must expose a complete technique matrix including detected, disabled, and active status. | should | Improves planning transparency and troubleshooting. |
| FR-06 | Legacy verify command surfaces replaced by canonical subcommands in scope must be removed in the same slice. | must | Enforces hard-cutover policy and avoids dual contracts. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Detection and recommendation must be deterministic for the same project inputs and config. | must | Prevents planning churn and flaky automation behavior. |
| NFR-02 | Command output contracts must be machine-readable and stable across patch releases. | must | Enables robust harness integration. |
| NFR-03 | Technique catalog evolution must remain backward-safe within the new canonical schema. | should | Reduces migration overhead for future additions. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add model and parser tests for technique catalog definitions and `keel.toml` override behavior.
- Add command-level tests for `verify detect`, `verify recommend`, and `config show` matrix parity.
- Add hard-cutover regression tests proving removed legacy verify command shapes fail fast with actionable guidance.
- Validate end-to-end by running `just keel doctor` and `just test` with fixtures covering varied project signal combinations.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Project file- and stack-signals are sufficient to infer useful technique recommendations. | Recommendations may be noisy or incomplete for some repos. | Detection fixture suite across representative project archetypes. |
| Operators prefer declarative config overrides over command-level ad hoc flags. | Additional override surfaces may be requested. | User feedback from planning workflow and command telemetry proxies. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How should conflicting signals be ranked when multiple techniques are plausible? | Verification owner | Monitoring |
| Hard-cutover command changes could break existing automation scripts unexpectedly. | Maintainer | Mitigated with clear migration errors and docs |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A canonical technique catalog powers `config show`, `verify detect`, and `verify recommend` outputs.
- [ ] `keel.toml` overrides deterministically control technique active status.
- [ ] Recommendations include only detected and active techniques across regression fixtures.
- [ ] Legacy verify command surfaces in scope are removed and covered by hard-cutover tests.
<!-- END SUCCESS_CRITERIA -->

