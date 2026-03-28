# Explain Roles Lanes And Next Routing - Software Design Description

> Expose the configured workflow topology as an inspectable product surface and make role-scoped routing legible to humans and agents.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage exposes workflow topology as a first-class product surface. `keel roles` becomes the direct inspection command for configured role and lane behavior, and `keel next --explain` makes one routing decision legible without changing the decision algorithm itself.

## Context & Boundaries

In scope: a read-only roles surface and richer next explanations. Out of scope: topology configuration format changes or algorithm changes to what `next` selects. The voyage stands on the existing `workflow_topology` and `role_context` projections.

```
workflow_topology + role_context
          |             |
          +------+------+ 
                 |
      +----------+-----------+
      |                      |
      v                      v
   keel roles         keel next --explain
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `workflow_topology` projection | Internal | Canonical source of configured roles, lanes, queue posture, and examples. | current read model |
| `role_context` projection | Internal | Supplies contract/persona/priorities for explain surfaces. | current read model |
| Existing `next` algorithm | Internal | Maintains the actual selection behavior while explanations become richer. | current CLI command |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| New inspection surface | Add `keel roles` instead of burying topology inspection in `config show` | Roles and lanes are a first-class product concept in the docs. |
| Explain boundary | Add `--explain` to `keel next` rather than changing default output | The current pull surface stays compact unless the operator asks for the reasoning. |
| Data source | Reuse topology and role-context projections directly | The mission is specifically about removing duplicated implied routing rules. |

## Architecture

Add a roles command module plus a small presentation layer for text/JSON output. Extend `next` argument parsing and output rendering to optionally append a canonical explanation block derived from the resolved actor context and role-context contract.

## Components

- Roles projection adapter: turns topology and role-context data into display-ready structures.
- `keel roles` command: lists roles and optionally focuses one role in text/JSON.
- `next --explain` presentation: augments existing output with routing rationale and role-context hints.

## Interfaces

Key interface expectations:

- `keel roles`
- `keel roles --json`
- `keel next --role ... --explain`
- `keel next --role ... --json` including explanation payload when requested

## Data Flow

1. Load configured workflow topology.
2. Resolve role families, lanes, and role-context contracts.
3. Render them in `keel roles`.
4. On `keel next --explain`, resolve the actor context used for the actual decision and append its explanation.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Roles surface drifts from configured examples | command test failure | read examples from topology rather than hard-coding | keep projection-driven output only |
| `next --explain` misstates the lane or queue type | regression test failure | resolve from actor context after parsing the role | repair explanation plumbing without touching selection |
| JSON explanation becomes unstable | serialization test failure | keep a dedicated payload type | version changes deliberately with docs/test updates |
