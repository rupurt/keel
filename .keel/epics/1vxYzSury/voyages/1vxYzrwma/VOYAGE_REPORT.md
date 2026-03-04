# VOYAGE REPORT: Decision And Verification Guidance Parity

## Voyage Metadata
- **ID:** 1vxYzrwma
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Align Next Command Guidance In Human And Json Output
- **ID:** 1vxZ0DxDl
- **Status:** done

#### Summary
Align `keel next` guidance rendering across human-readable and JSON outputs so both surfaces expose the same canonical next or recovery command.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Ensure every actionable `keel next` decision renders the same canonical command in human and JSON outputs. <!-- verify: cargo test --lib cli::commands::management::next::tests::actionable_decisions_keep_human_and_json_guidance_in_sync, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure blocked `keel next` decisions expose canonical recovery guidance consistently across output modes. <!-- verify: cargo test --lib cli::commands::management::next::tests::blocked_and_empty_decisions_keep_human_and_json_guidance_in_sync, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add parity tests that fail if human formatter and JSON payload diverge for next/recovery guidance. <!-- verify: cargo test --lib cli::commands::management::next::tests, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0DxDl/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0DxDl/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0DxDl/EVIDENCE/ac-2.log)

### Add Canonical Guidance To Verify And Audit Commands
- **ID:** 1vxZ0EXHC
- **Status:** done

#### Summary
Add canonical guidance output to verification and audit command outcomes so success and failure paths provide deterministic next or recovery actions.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add canonical `next_step` guidance for successful `verify` and `story audit` outcomes when a deterministic follow-up action exists. <!-- verify: cargo test --lib verification_guidance::tests::verify_success_guidance_maps_to_story_lifecycle_next_step, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Add canonical `recovery_step` guidance for failed verification and audit outcomes with actionable remediation commands. <!-- verify: cargo test --lib verification_guidance::tests::verify_failed_report_maps_to_story_audit_recovery, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering verify/audit guidance parity across human-readable and JSON command outputs. <!-- verify: cargo test --lib verification_guidance::tests::verify_and_audit_guidance_preserve_human_json_parity, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0EXHC/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0EXHC/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0EXHC/EVIDENCE/ac-2.log)

### Add Canonical Guidance To Play Command Outcomes
- **ID:** 1vxZ0Eoj9
- **Status:** done

#### Summary
Add canonical guidance handling to play command outcomes so deterministic results emit one next action while exploratory outputs remain non-prescriptive.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Emit canonical `next_step` guidance for play outcomes that have a deterministic follow-up command. <!-- verify: cargo test --lib play_guidance::tests::suggest_outcome_maps_to_canonical_next_step, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Keep exploratory play outputs non-prescriptive when no deterministic command can be recommended. <!-- verify: cargo test --lib play_guidance::tests::exploratory_outcomes_emit_no_guidance, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests that cover both prescriptive and non-prescriptive play outcomes across human and JSON output modes. <!-- verify: cargo test --lib play_guidance::tests::play_outcomes_keep_human_and_json_guidance_in_sync, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0Eoj9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0Eoj9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0Eoj9/EVIDENCE/ac-2.log)


