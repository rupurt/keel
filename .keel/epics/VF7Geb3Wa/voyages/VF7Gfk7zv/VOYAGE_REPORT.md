# VOYAGE REPORT: Introduce Derived Heartbeat Surface And Flow Fallback

## Voyage Metadata
- **ID:** VF7Gfk7zv
- **Epic:** VF7Geb3Wa
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Project Heartbeat From Git And Worktree Activity
- **ID:** VF7GiUM4f
- **Status:** done

#### Summary
Add the core read-model projection that derives heartbeat activity from repository state so later CLI and flow surfaces can stop treating `.keel/heartbeat` as the primary pacemaker input.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] A reusable heartbeat projection derives the latest activity timestamp from dirty tracked files first and otherwise from reachable commit activity. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] The projection exposes which signal source won so downstream consumers do not need to re-run repository heuristics independently. <!-- verify: manual, SRS-01:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Deterministic tests cover dirty, clean, and unavailable repository-state cases without surfacing inode-level details as the user-facing contract. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF7GiUM4f/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF7GiUM4f/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF7GiUM4f/EVIDENCE/ac-3.log)

### Add Keel Heartbeat Command Surface
- **ID:** VF7GiVC4g
- **Status:** done

#### Summary
Expose the new derived heartbeat projection through `keel heartbeat` so operators can inspect the exact signal that will govern energized versus unplugged flow behavior.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] `keel heartbeat` reports the latest activity timestamp, age, and source from the derived heartbeat projection. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] The command renders idle or unavailable states without requiring `.keel/heartbeat` to exist. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] The operator-facing command output stays platform-stable and does not make inode behavior part of the documented semantics. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF7GiVC4g/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF7GiVC4g/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF7GiVC4g/EVIDENCE/ac-3.log)

### Use Derived Heartbeat In Flow With Compatibility Fallback
- **ID:** VF7GiVp4h
- **Status:** done

#### Summary
Cut `keel flow --scene` over to the derived heartbeat signal so flow behavior no longer depends on the legacy heartbeat file and pass 2 can remove the old path cleanly.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel flow --scene` uses the derived heartbeat as its primary energization input when deciding whether to render powered or unplugged state. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] `keel flow --scene` no longer depends on the legacy file-backed heartbeat path, allowing the migration to remove that path without changing flow behavior. <!-- verify: manual, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-04/AC-01] Regression coverage proves energized and unplugged scenarios across the derived heartbeat model so the file-backed path can be deleted safely. <!-- verify: manual, SRS-04:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF7GiVp4h/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF7GiVp4h/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF7GiVp4h/EVIDENCE/ac-3.log)


