---
id: VDeUNP4rV
title: Move Story Lifecycle Automation To Explicit Reactors
type: feat
status: backlog
created_at: 2026-03-12T04:35:23
updated_at: 2026-03-12T04:40:07
operator-signal: 
scope: VDeRV9CAo/VDeUIiB3Q
index: 2
---

# Move Story Lifecycle Automation To Explicit Reactors

## Summary

Move the live story-started and story-accepted lifecycle automations onto
explicit reactors while preserving today's auto-start and auto-complete
behavior.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `StoryStarted -> StartVoyage` and `StoryAccepted -> CompleteVoyage` run through explicit reactors with current semantics preserved. <!-- verify: cargo test story_started_event story_accepted_event --lib, SRS-02:start:end -->
- [ ] [SRS-NFR-03/AC-01] The refactor leaves one canonical process-manager reaction path for lifecycle automation. <!-- verify: cargo test process_manager --lib, SRS-NFR-03:start:end -->
