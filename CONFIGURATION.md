# Keel Configuration

Keel is configured via a `keel.toml` file. This document describes the available configuration options and how they drive Keel's behavior.

## Configuration Resolution

Keel resolves configuration in the following order:

1.  **Project-local:** `./keel.toml`
2.  **User-global:** `~/.config/keel.toml`
3.  **Built-in defaults**

## Board Directory

You can specify where Keel stores its state (defaults to `.keel`):

```toml
board_dir = ".keel"
```

## Workflow & Topology

Keel uses a flexible, role-based lane topology to route work. This is configured via the `[workflow]`, `[roles]`, and `[lanes]` sections.

### Workflow Defaults

Defines the default roles and lanes used by commands like `keel next` and `keel flow`.

```toml
[workflow.defaults]
management_role = "manager"
delivery_role = "operator"
management_lane = "management"
delivery_lane = "delivery"
```

### Lanes

Lanes are work queues that aggregate entities based on their status.

```toml
[lanes.delivery]
description = "Active delivery work"
include = ["story.backlog", "story.in-progress"]
exclude = ["story.done"]
parallel = true        # Allows parallel execution of independent stories
manual_accept = false  # Whether stories in this lane require manual verification
priority = 50          # Rendering and selection priority (higher first)
```

**Include/Exclude patterns:**
- `story.backlog`
- `story.in-progress`
- `story.*` (all story statuses)
- `voyage.draft`
- `bearing.exploring`

### Roles

Roles map a "role family" to a default lane and a template.

```toml
[roles.operator]
default_lane = "delivery"
template = "operator-core"
```

### Role Overrides

You can provide specific overrides for full role taxonomy strings.

```toml
[role_overrides."operator/software:infrastructure"]
template = "infrastructure-operator"
```

## Scoring Modes

Keel uses scoring to prioritize work. You can switch between built-in modes or define custom ones.

```toml
[scoring]
mode = "constrained" # Options: constrained, growth, product, or custom
```

### Custom Modes

```toml
[scoring.modes.aggressive]
impact_weight = 3.0
confidence_weight = 2.0
effort_weight = 1.0
risk_weight = 0.5
```

## Doctor Diagnostics

Disable specific doctor checks if they don't apply to your workflow.

```toml
[doctor.checks.voyage-scope-authored-content]
disabled = true
```

## Research Providers

Configure weights and availability for research providers used in Bearings.

```toml
[research.providers.manual]
disabled = false
weight = 1.0

[research.providers.academic]
weight = 1.5
```

## Full Example

```toml
board_dir = ".keel"

[workflow.defaults]
management_role = "manager"
delivery_role = "operator"

[roles.manager]
default_lane = "management"
template = "manager-core"

[roles.operator]
default_lane = "delivery"
template = "operator-core"

[lanes.management]
description = "Planning and verification"
include = ["bearing.*", "voyage.draft", "story.needs-human-verification"]
priority = 100
manual_accept = true

[lanes.delivery]
description = "Implementation"
include = ["story.backlog", "story.in-progress"]
parallel = true
priority = 50

[scoring]
mode = "constrained"

[doctor.checks.story-id-uniqueness]
disabled = false
```
