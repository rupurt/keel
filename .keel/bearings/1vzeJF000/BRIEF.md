# MissionEntity — Brief

## Context
Current autonomous workflows (sift, port) span multiple epics over days.
The stopping condition is implicit — an AGENTS.md policy that says "don't stop
until the objective is done." Problems observed:

1. **False halting**: Harnesses stop when the queue is empty even though the
   objective is incomplete.
2. **No traceability**: No single artifact showing what was accomplished and why.
3. **No verification boundary**: The harness can't distinguish "board work done" from "objective achieved."
4. **No lineage root**: Disconnected entities spawned during autonomous work.

## Objectives
Introduce a Mission entity that encodes goals, success criteria, and halting rules as machine-readable board state to let keel reason about completion deterministically.

## Scope
- Mission entity with state machine, frontmatter, loader, and Board integration
- CLI commands for full mission lifecycle
- Lineage field (`mission`) on epics, bearings, and ADRs with doctor validation
- Doctor checks that signal mission completion
- `keel next` mission-awareness
- CHARTER.md with structured goals, constraints, and halting rules
- Refinement loop (`keel mission refine`)
- LOG.md decision journal

## Research Questions
- What threshold triggers LOG.md digest? Line count, age, or entry count?
- Should `keel flow` show mission-level progress as a top-level summary?
- How does mission-awareness interact with multi-mission boards?
- Should CHARTER.md constraints feed into ADR governance?

## Open Questions
- (none yet)
