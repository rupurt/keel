# Bearing Dependency Primitives - Software Design Description

> Introduce depends_on field, doctor validation, and dependency-aware sort order for bearings

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds a lightweight dependency graph to bearings. The `depends_on` field in bearing frontmatter declares edges. The doctor validates the graph. The sort order in `bearing list` and `next` uses dependency resolution state as a sort tier above EV score.

## Architecture

1. **Model Extension**: Add `depends_on: Option<Vec<String>>` to `BearingFrontmatter` in `crates/keel-core/src/domain/model/bearing.rs`. The loader already deserializes all frontmatter fields via serde, so this is a single struct change.

2. **Doctor Validation**: New check function `check_bearing_dependencies` in `crates/keel-core/src/infrastructure/validation/bearings.rs`:
   - Iterate all bearings; for each `depends_on` entry, verify the target ID exists in `board.bearings`.
   - Build an adjacency list and run a DFS-based cycle detection (standard topological sort attempt).
   - Flag self-references as a special case of cycles.

3. **Sort Order**: Extend `priority_key()` on `Bearing` (or add a separate sort comparator in the CLI) to include a dependency-resolution tier:
   - A bearing's dependencies are "resolved" if every target is in a terminal state (laid, declined, parked).
   - Unresolved bearings sort after resolved ones within the same status tier.

## Components

### BearingFrontmatter (model change)
- File: `crates/keel-core/src/domain/model/bearing.rs`
- Add: `pub depends_on: Option<Vec<String>>`
- Default: `None` (backward compatible)

### Dependency Validation (doctor check)
- File: `crates/keel-core/src/infrastructure/validation/bearings.rs`
- New function: `check_bearing_dependencies(board: &Board) -> Vec<Problem>`
- Registered in diagnostics module under the Sensory subsystem.

### Sort Order (CLI)
- File: `crates/keel-cli/src/cli/commands/management/bearing/mod.rs` (bearing list sort)
- File: `crates/keel-cli/src/cli/presentation/flow/next_up.rs` (next command sort)
- Modify sort comparator to include dependency resolution state.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Dependency storage | Frontmatter field, not ASSESSMENT.md content | Enables deterministic validation and sort without content parsing. The existing `## Dependencies` section in ASSESSMENT.md remains informational. |
| Cycle detection | DFS with coloring | Standard O(V+E) algorithm; bearing count is small enough that simplicity wins over optimization. |
| Terminal states | laid, declined, parked | These represent bearings whose research outcome is settled. |

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Dangling dependency reference | Doctor check | Error-severity problem | Operator fixes frontmatter |
| Cyclic dependency | Doctor check (DFS) | Error-severity problem listing cycle path | Operator removes one edge |
| Self-reference | Doctor check | Error-severity problem | Operator removes self-ref |
