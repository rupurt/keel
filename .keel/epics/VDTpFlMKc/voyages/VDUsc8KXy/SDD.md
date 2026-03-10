# Role Template Injection - Software Design Description

> Scaffold role-specific management and execution templates for harness guidance and context injection.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds a single role-template registry for the core management and execution roles, then injects the selected template into `keel next` guidance so harnesses receive context and personality hints together with the work decision.

## Context & Boundaries

- In scope: core `manager/*` and `engineer/*` template definitions, deterministic template lookup, and `keel next` guidance injection.
- Out of scope: dynamic template editing, additional role families, or a separate prompt export command.
- External consumer: the outer harness that already reads `keel next` human or JSON output to decide how to continue work.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `RoleTaxonomy` parsing | Internal | Provides the canonical role identity used for template selection. | in-repo |
| `CanonicalGuidance` contract | Internal | Carries next-step commands and the injected role context to harness consumers. | in-repo |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Template transport | Extend `CanonicalGuidance` with an optional role-context payload | Keeps harness consumption on the existing actionable output surface. |
| Template selection | Resolve templates from the base role family (`manager`, `engineer`) and preserve specialization as context metadata | Stable first cut without fragmenting templates by every specialization. |
| Unsupported roles | Fail fast with a deterministic error listing supported families | Avoids silent prompt drift and ambiguous fallbacks. |

## Architecture

- `src/read_model/role_context.rs` projects a parsed `RoleTaxonomy` into a `RoleContextTemplate`.
- `src/cli/commands/management/guidance.rs` grows an optional serialized role-context block alongside the existing next/recovery command fields.
- `src/cli/commands/management/next.rs` resolves role context when `--role` is supplied and attaches it to actionable output paths.

## Components

- `RoleContextTemplate`: immutable data for template id, persona, priorities, workflow hints, and queue-lane expectation.
- `resolve_role_context`: deterministic lookup from parsed role taxonomy to `RoleContextTemplate` or `UnsupportedRole`.
- `Next` output adapters: human-readable and JSON renderers that surface the resolved template only when the caller supplied a role.

## Interfaces

- Internal lookup API: `resolve_role_context(&RoleTaxonomy) -> Result<RoleContextTemplate, UnsupportedRole>`.
- JSON guidance shape:
  - `guidance.role_context.role`
  - `guidance.role_context.template_id`
  - `guidance.role_context.persona`
  - `guidance.role_context.priorities`
  - `guidance.role_context.workflow`

## Data Flow

1. `keel next --role <TAXONOMY>` parses the actor role through the existing taxonomy parser.
2. The parsed role is passed into `resolve_role_context`.
3. `calculate_next` returns the actionable work decision using the same actor role.
4. The command renderer attaches the selected role template to the guidance payload and prints it in human and JSON forms.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unsupported role family | Template lookup returns `UnsupportedRole` | Abort with a deterministic error naming supported families | Re-run with `manager/*` or `engineer/*` |
| Guidance serialization drift | Output regression tests fail | Fix `CanonicalGuidance` and `next` renderers together | Update both JSON and human assertions in one slice |
