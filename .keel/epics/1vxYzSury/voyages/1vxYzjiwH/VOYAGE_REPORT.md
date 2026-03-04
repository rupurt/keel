# VOYAGE REPORT: Story And Voyage Lifecycle Guidance

## Voyage Metadata
- **ID:** 1vxYzjiwH
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add Canonical Next Guidance To Story Lifecycle Commands
- **ID:** 1vxZ0BXgR
- **Status:** done

#### Summary
Apply canonical deterministic guidance to story lifecycle commands so each successful transition returns one clear next action.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to successful story lifecycle commands (`start`, `reflect`, `record`, `submit`, `accept`) when a deterministic follow-up exists. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::start_action_suggests_submit_for_in_progress, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure suggested commands align with valid story state-machine transitions for the resulting lifecycle state. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::record_action_suggests_start_for_rejected_story, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests for lifecycle guidance output in both human and JSON modes. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::guidance_serializes_with_canonical_next_step_shape, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0BXgR/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0BXgR/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0BXgR/EVIDENCE/ac-2.log)

### Add Canonical Guidance To Voyage Lifecycle Commands
- **ID:** 1vxZ0C8QB
- **Status:** done

#### Summary
Apply canonical deterministic guidance to voyage lifecycle commands so planning and execution transitions expose one clear next command.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to successful voyage lifecycle commands (`plan`, `start`, `done`) where a deterministic follow-up action exists. <!-- verify: cargo test --lib cli::commands::management::voyage::guidance::tests::serializes_plan_guidance_with_canonical_next_step_shape, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure voyage guidance command suggestions align with lifecycle guards and resulting voyage states. <!-- verify: cargo test --lib cli::commands::management::voyage::guidance::tests::start_action_suggests_done_transition, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests covering voyage lifecycle guidance in both human-readable and JSON output. <!-- verify: cargo test --lib voyage::guidance, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0C8QB/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0C8QB/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0C8QB/EVIDENCE/ac-2.log)

### Add Canonical Recovery Guidance To Story Lifecycle Errors
- **ID:** 1vxZ0CBPx
- **Status:** done

#### Summary
Add canonical recovery guidance for story lifecycle failures so blocked transitions return one deterministic remediation command.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Emit canonical `recovery_step` guidance for common story lifecycle failures (for example invalid transition state, unmet preconditions, or missing required artifacts). <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::recovery_not_found_maps_to_story_list, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure recovery guidance commands are executable and directly address the corresponding blocking condition. <!-- verify: cargo test --lib cli::commands::management::story::accept::tests::accept_errors_on_manual_verification_without_human_flag, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests that validate story lifecycle error-to-recovery mapping in human and JSON outputs. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0CBPx/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0CBPx/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0CBPx/EVIDENCE/ac-2.log)


