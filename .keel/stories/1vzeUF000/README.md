---
id: 1vzeUF000
title: Mission Domain Model And State Machine
type: feat
status: backlog
created_at: 2026-03-09T10:33:47
updated_at: 2026-03-09T13:22:51
scope: 1vzeJF000/1vzeMk000
index: 1
---

# Mission Domain Model And State Machine

## Summary

Define the Mission domain model and state machine. Create MissionFrontmatter
struct with id, title, status, and timestamp fields. Implement MissionStatus
enum with all lifecycle states and typed transition validation. Implement
Mission struct with Entity trait.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] MissionFrontmatter has id, title, status, created_at, updated_at, activated_at, achieved_at, verified_at fields <!-- verify: test --> <!-- SRS-01:start:end -->
- [ ] [SRS-02/AC-01] MissionStatus enum has Defining, Active, Achieved, Verified, Paused, Abandoned variants <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-03/AC-01] State machine validates activate: Defining→Active <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-03/AC-02] State machine validates achieve: Active→Achieved <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-03/AC-03] State machine validates verify: Achieved→Verified <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-03/AC-04] State machine validates pause: Active→Paused, resume: Paused→Active <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-03/AC-05] State machine validates abandon: Active/Paused→Abandoned <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-03/AC-06] State machine rejects invalid transitions with descriptive error <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-04/AC-01] Mission struct implements Entity trait (id, title, path) with has_charter and has_log fields <!-- verify: test --> <!-- SRS-04:start:end -->
