# VOYAGE REPORT: Operational Routine Reliability

## Voyage Metadata
- **ID:** VE3yUoUUy
- **Epic:** VE3KrOPS
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Guard Materialization Against Terminal Voyage Scope
- **ID:** VE3z3roGf
- **Status:** done

#### Summary
Extend `validate_target_scope()` in `routine_materialization.rs` to reject materialization when the target voyage is in a terminal state (done). Pulse should skip the routine with a clear rejection reason instead of creating an orphaned story.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Pulse skips materialization when target voyage status is done <!-- verify: cargo test --bin keel pulse_rejects, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Skipped routine produces a Rejected outcome with reason explaining terminal scope <!-- verify: cargo test --bin keel pulse_rejects, SRS-01:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VE3z3roGf/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VE3z3roGf/EVIDENCE/ac-2.log)

### Doctor Warns On Routine Scope Incoherence
- **ID:** VE3z4kLeK
- **Status:** done

#### Summary
Add `check_routine_scope_coherence()` to `checks/routines.rs`. For each routine, verify its `target-scope` references an existing epic and a non-terminal voyage. Emit a warning for missing entities and an error for terminal scope targets.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Doctor emits warning when routine target-scope references a missing epic or voyage <!-- verify: cargo test --lib scope_coherence, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Doctor emits error when routine target-scope references a terminal voyage <!-- verify: cargo test --lib scope_coherence, SRS-02:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VE3z4kLeK/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VE3z4kLeK/EVIDENCE/ac-2.log)

### Pulse Reports Per Routine Materialization Outcome
- **ID:** VE3z5SB01
- **Status:** done

#### Summary
Ensure the pulse command's non-scene, non-json output prints one structured line per routine showing its materialization outcome (Created, Skipped, Rejected) and reason.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Pulse text output includes one line per evaluated routine with outcome and reason <!-- verify: cargo test --bin keel pulse_human_output, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Created outcome shows the story ID that was materialized <!-- verify: cargo test --bin keel pulse_human_output, SRS-03:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VE3z5SB01/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VE3z5SB01/EVIDENCE/ac-2.log)


