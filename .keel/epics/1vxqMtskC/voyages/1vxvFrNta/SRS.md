# Verification Technique Command Surface Cutover - Software Requirements Specification

> Move verification-technique surfacing to config/verify commands with hard-cutover verify subcommands and machine-readable output

**Epic:** [1vxqMtskC](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- Move verification-technique state surfacing to `keel config show` as a per-technique flag matrix.
- Represent each technique row with a hyphenated-id `label` and flags: `detected`, `disabled`, `active`.
- Ensure `keel config show` lists all techniques (built-in + custom), even when inactive.
- Hard cutover verification execution surface to `keel verify run` and add `keel verify recommend`.
- Add machine-readable `--json` output for `keel config show`, `keel verify run`, and `keel verify recommend`.
- Restrict recommendation output to detected + active techniques and keep it advisory-only.
- Remove recommendation sections from `keel epic show`, `keel voyage show`, and `keel story show`.
- Update planning guidance in `AGENTS.md` to use `keel config show` and `keel verify recommend`.

Out of scope:
- Reworking scoring mode behavior (`keel config mode` remains as-is).
- Automatic execution of recommended techniques.
- Backward-compatibility aliases for legacy `keel verify` invocation.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The verification technique bank in `read_model::verification_techniques` remains the canonical source of technique metadata. | Internal dependency | Command surfaces diverge and flags/recommendations drift. |
| Existing board/story parsing continues to provide enough signal to infer currently used techniques. | Runtime dependency | Active/recommendation filtering becomes incomplete or noisy. |
| CLI command-tree/runtime changes can be landed as hard cutover without compatibility shims. | Product decision | Legacy scripts using old verify syntax will break unexpectedly. |

## Constraints

- Hard cutover policy applies: remove legacy `keel verify` execution path in the same slice.
- Technique row `label` must be the hyphenated technique id.
- `active` is defined as `detected && !disabled`.
- Recommendation command output must include only detected and active techniques.
- JSON outputs must be deterministic and stable for automation use.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | `keel config show` MUST render every technique (built-in + custom) with row fields `label` (hyphenated id), `detected`, `disabled`, and `active`, and MUST support deterministic `--json` output using the same field contract. | User directive | command output tests + parser/unit tests + JSON contract tests |
| SRS-02 | `keel config show` MUST no longer render scoring sections; `keel config mode` behavior and output contract remain unchanged. | User directive | command output tests + mode regression tests |
| SRS-03 | Verification execution MUST move to `keel verify run`, preserve current verification behavior (`id`/`--all` semantics), and support deterministic `--json` output. | User directive | command parsing/runtime tests + verification command tests + JSON contract tests |
| SRS-04 | `keel verify recommend` MUST render recommendation commentary only for techniques where `detected=true` and `active=true`, and MUST support deterministic `--json` output using the same filter contract. | User directive | recommendation filter tests + command output tests + JSON contract tests |
| SRS-05 | `keel epic show`, `keel voyage show`, and `keel story show` MUST not render recommendation sections. | User directive | show-command regression tests |
| SRS-06 | `AGENTS.md` planning workflow MUST explicitly reference `keel config show` and `keel verify recommend` for verification planning. | User directive | documentation tests/review |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Technique matrix and JSON output ordering MUST be deterministic across equivalent repository states. | NFR-01 | determinism regression tests |
| SRS-NFR-02 | Legacy `keel verify` invocation MUST fail fast with explicit recovery guidance to `keel verify run`. | Hard cutover policy | command error-path tests |
| SRS-NFR-03 | `keel verify recommend` MUST be advisory-only and MUST NOT execute verification commands/tools. | Safety policy | no-side-effect behavior tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
