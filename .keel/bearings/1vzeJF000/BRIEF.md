# MissionEntity — Brief

## Hypothesis

Introducing a first-class Mission entity with machine-readable goals, success
criteria, and halting rules will let autonomous harnesses reason about mission
completion deterministically instead of relying on implicit AGENTS.md policy.

## Problem Space

Current autonomous workflows can span multiple epics and days of work, but the
stopping condition is implicit. That creates false halts when the queue is
 empty, weak traceability for what was accomplished, no clean distinction
 between "board work done" and "objective achieved", and no single lineage root
 for mission-scoped entities.

## Success Criteria

- [ ] A mission can encode goals, halting rules, and verification boundaries as
      durable board state.
- [ ] Autonomous loops can distinguish temporary queue exhaustion from true
      mission completion.
- [ ] Mission lineage makes related epics, bearings, and ADRs navigable from a
      single steering artifact.

## Open Questions

- What threshold should trigger `LOG.md` digest: line count, age, or entry
  count?
- Should `keel flow` surface mission-level progress as a top-level summary?
- How should mission-awareness interact with multi-mission boards?
- Should `CHARTER.md` constraints feed directly into ADR governance?
