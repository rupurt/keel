# Configurable Role and Lane Topology - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Replace hardcoded manager/engineer queue routing with config-defined roles and lanes, seeded with manager/operator and management/delivery defaults, plus selector-based lane topology and validation. | board: VDVPODBXF |

## Constraints

- Preserve a sensible zero-config topology seeded as `manager`/`operator` roles and `management`/`delivery` lanes.
- Allow unbounded role families and lane names; queue routing cannot depend on literal `manager` or `engineer` strings.
- Role subtypes remain optional and cannot be required just to access a lane.
- Lane source selection must use canonical board-selector globs with doctor validation; intentional cross-lane overlap is out of scope for this rollout.
- Follow hard cutover policy: replace literal manager/engineer routing rather than adding compatibility aliases.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
