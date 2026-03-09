# MissionEntity — Brief

## Hypothesis

Autonomous harnesses halt prematurely because keel has no first-class
representation of long-running objectives. The "keep going" contract lives in
AGENTS.md prose which harnesses interpret inconsistently. A Mission entity that
encodes goals, success criteria, and halting rules as machine-readable board
state will let keel reason about completion deterministically and eliminate
false halting.

## Problem Space

Current autonomous workflows (sift, port) span multiple epics over days.
The stopping condition is implicit — an AGENTS.md policy that says "don't stop
until the objective is done." Problems observed:

1. **False halting**: Harnesses stop when the queue is empty even though the
   objective is incomplete. They treat `keel next` returning no work as a
   signal to stop, when they should be creating the next planning unit.
2. **No traceability**: When a multi-day session ends, there's no single
   artifact showing what was accomplished, what's left, and why decisions
   were made. You have to reconstruct intent from scattered epics/voyages.
3. **No verification boundary**: Real-world success (revenue, user metrics,
   latency targets) has no place on the board. The harness can't distinguish
   "board work done" from "objective achieved."
4. **No lineage root**: Epics, bearings, and ADRs spawned during autonomous
   work are disconnected. There's no parent entity linking them to a common
   objective.

## Success Criteria

- [ ] Mission entity with state machine, frontmatter, loader, and Board integration
- [ ] CLI commands for full mission lifecycle (new, refine, activate, show, list, pause, achieve, verify, abandon)
- [ ] Lineage field (`mission`) on epics, bearings, and ADRs with doctor validation
- [ ] Doctor checks that signal mission completion and prevent false halting
- [ ] `keel next` mission-awareness — recommends creating work when queue empty but mission incomplete
- [ ] CHARTER.md with structured goals, constraints, and halting rules
- [ ] Refinement loop (`keel mission refine`) for interactive goal elicitation
- [ ] LOG.md decision journal with digest/rotation for long-running missions

## Open Questions

- What threshold triggers LOG.md digest? Line count, age, or entry count?
- Should `keel flow` show mission-level progress as a top-level summary?
- How does mission-awareness interact with multi-mission boards (future)?
- Should CHARTER.md constraints feed into ADR governance (e.g., auto-create ADR from constraint)?
