# Role and Lane Config Contract - Software Design Description

> Define a config-driven role and lane contract that replaces hardcoded manager/engineer routing, supports configurable lane selectors with sensible defaults, and provides a clear implementation decomposition.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a single resolved workflow-topology read model that all queue-sensitive CLI surfaces consume. The topology is sourced from layered TOML config, seeded with sensible defaults, and compiled into canonical role, lane, selector, and capability rules. `keel next`, `keel story accept`, `keel flow`, `keel config show`, and `keel doctor` all depend on that shared resolver instead of hardcoded manager/engineer logic.

## Context & Boundaries

Included in this voyage:
- Config schema for workflow defaults, role families, lanes, and exact role overrides.
- A canonical selector catalog over board states such as `story.backlog`, `story.in-progress`, `story.needs-human-verification`, `voyage.draft`, and `bearing.exploring`.
- Resolver APIs that derive lane membership, lane capabilities, and template selection from parsed taxonomies.
- Dynamic lane rendering for `flow` and effective-topology rendering for `config show`.

Explicitly excluded from this voyage:
- Arbitrary prompt text or template body files authored directly in config.
- Shared cross-lane visibility or multi-lane memberships in v1.
- New lifecycle states beyond the current board model.

```text
keel.toml
    |
    v
Config Loader + Seeded Defaults
    |
    v
Resolved Workflow Topology
    |---------------------------|--------------------------|----------------------|
    v                           v                          v                      v
  next                      story accept                flow                config show / doctor
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Layered `Config` loader | Internal | Reads project/user/default TOML and becomes the source for topology configuration | Current `src/infrastructure/config.rs` |
| Taxonomy parser | Internal | Parses base role plus optional subtype/tags/style/level/context | Current `src/domain/model/taxonomy.rs` |
| Board loader and flow metrics | Internal | Supplies stories, voyages, bearings, and aggregate queue counts for topology projection | Current board read models |
| Doctor framework | Internal | Surfaces hard validation failures for invalid topologies | Current diagnostics framework |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Base queue routing key | Top-level role family only | Lane access should not require a subtype such as `/software` |
| Role subtype semantics | Optional for lane access, still active for entity matching and exact template overrides | Preserves fine-grained filtering without making subtype mandatory everywhere |
| Lane model | Config-defined and unbounded, seeded with `management` and `delivery` defaults | Keeps keel generic across domains while preserving a zero-config baseline |
| Lane capabilities | `parallel` and `manual_accept` live on the lane, not the role family | These behaviors are workflow properties rather than literal role names |
| Lane membership | Ordered `include` plus `exclude` selector globs over canonical board state ids | Supports compact defaults such as `story.*` while remaining auditable |
| Cross-lane overlap | Hard-fail in doctor for v1 | Avoids ambiguous visibility and routing while the topology contract stabilizes |
| Template specialization | Base-role template plus exact `role_overrides.\"<taxonomy>\"` override | Supports subtype-specific guidance without a combinatorial role table |

## Architecture

The change introduces four cooperating layers:

1. Config schema
   - Extend `Config` with workflow-topology sections:

```toml
[workflow.defaults]
management_role = "manager"
delivery_role = "operator"
management_lane = "management"
delivery_lane = "delivery"

[roles.manager]
default_lane = "management"
template = "manager-core"

[roles.operator]
default_lane = "delivery"
template = "operator-core"

[lanes.management]
description = "Planning, triage, calibration, acceptance"
include = ["bearing.*", "voyage.draft", "story.needs-human-verification"]
exclude = []
parallel = false
manual_accept = true
priority = 100

[lanes.delivery]
description = "Work ready for execution"
include = ["story.*"]
exclude = [
  "story.done",
  "story.rejected",
  "story.icebox",
  "story.needs-human-verification",
]
parallel = true
manual_accept = false
priority = 50

[role_overrides."operator/software"]
template = "software-operator-core"
```

2. Resolved workflow topology
   - Build a `ResolvedWorkflowTopology` read model that:
     - seeds default roles and lanes when config sections are missing
     - validates default references and role-to-lane references
     - compiles selector globs into a canonical matcher
     - resolves lane capabilities from parsed actor taxonomies
     - resolves template ids from base roles plus exact overrides

3. Queue-source catalog
   - Define one canonical selector namespace over board states:
     - `story.backlog`
     - `story.in-progress`
     - `story.needs-human-verification`
     - `story.done`
     - `story.rejected`
     - `story.icebox`
     - `voyage.draft`
     - `voyage.in-progress`
     - `voyage.done`
     - `bearing.exploring`
     - `bearing.surveying`
     - `bearing.assessing`
   - `story.*` and other globs expand against this catalog, then `exclude` removes unwanted sources.

4. CLI integrations
   - `next`: resolve actor role -> lane -> capabilities -> allowed queue surface
   - `story accept`: resolve actor role -> lane -> `manual_accept`
   - `flow`: render one lane card per configured lane in descending `priority`
   - `config show`: print the effective seeded topology instead of raw partial config
   - `doctor`: validate references, selectors, and cross-lane overlap using the same resolver

## Components

| Component | Purpose | Key Behavior |
|-----------|---------|--------------|
| `WorkflowDefaultsConfig` | Stores seeded example roles/lanes used for zero-config behavior and guidance text | Provides canonical management and delivery defaults |
| `RoleFamilyConfig` | Declares base role families | Maps each family to a default lane and template id |
| `LaneConfig` | Declares lane behavior and source selectors | Controls visibility, `parallel`, `manual_accept`, and render order |
| `RoleOverrideConfig` | Declares exact taxonomy overrides | Replaces the base template when the full taxonomy matches |
| `ResolvedWorkflowTopology` | Shared topology resolver | Exposes `resolve_lane`, `supports_parallel`, `allows_manual_accept`, and `resolve_template` |
| `QueueSelectorCatalog` | Canonical selector namespace | Prevents drift between validation, routing, and rendering |

## Interfaces

Primary internal interfaces:

- `ResolvedWorkflowTopology::from_config(&Config) -> Result<Self>`
- `ResolvedWorkflowTopology::resolve_actor_lane(&RoleTaxonomy) -> Result<&ResolvedLane>`
- `ResolvedWorkflowTopology::resolve_template(&RoleTaxonomy) -> Result<&str>`
- `ResolvedWorkflowTopology::allows_manual_accept(&RoleTaxonomy) -> Result<bool>`
- `ResolvedWorkflowTopology::supports_parallel(&RoleTaxonomy) -> Result<bool>`
- `ResolvedWorkflowTopology::lane_sources(&str) -> &[QueueSourceId]`

Primary CLI contracts:

- `keel next --role <family[/specialization][:...]>`
- `keel story accept <id> --role <family[/specialization][:...]>`
- `keel config show` prints the effective topology, not just user-authored fragments
- `keel flow` renders configured lanes dynamically, ordered by `priority`

## Data Flow

1. Load layered config from project, user, and defaults.
2. Seed missing workflow defaults, role entries, and lane entries for the zero-config topology.
3. Compile lane selector globs against the canonical queue-source catalog.
4. Validate references and overlap before command-specific routing proceeds.
5. Parse the actor taxonomy from CLI input.
6. Resolve the actor's base role family to a lane, capabilities, and template.
7. Project the board into lane-specific queue views for `next`, `accept`, `flow`, or `config show`.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing workflow default references | Topology build | Return config error and fail doctor | Define the missing default role or lane in config |
| Role references nonexistent lane | Topology build | Return config error and fail doctor | Point the role family at a declared lane |
| Unknown selector token or invalid glob | Selector compilation | Return config error and fail doctor | Replace the selector with a canonical queue-source id or valid glob |
| Cross-lane overlap | Resolved lane source comparison | Fail doctor and block planning/execution until fixed | Narrow selectors or split responsibilities across lanes |
| Unknown actor role family | Command resolution | Return explicit CLI error with configured default-role guidance | Use a configured base role or update config |
| `--parallel` on a non-parallel lane | Command resolution | Reject with lane-capability guidance | Use the configured delivery role or omit `--parallel` |
| Manual accept attempted from a non-authorized lane | Story accept transition gate | Reject with recovery guidance naming the configured management role example | Retry with a role whose lane has `manual_accept = true` |
