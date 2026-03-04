# VOYAGE REPORT: Governance And Research Guidance

## Voyage Metadata
- **ID:** 1vxYzjVMv
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add Canonical Guidance To ADR Transition Commands
- **ID:** 1vxZ0C7OF
- **Status:** done

#### Summary
Add canonical guidance output to ADR lifecycle transitions so successful and recoverable outcomes expose one deterministic follow-up command.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to actionable ADR transition outputs (`accept`, `reject`, `deprecate`, `supersede`) where a deterministic follow-up exists. <!-- verify: cargo test --lib cli::commands::management::adr::guidance::tests::success_guidance_for_transitions_is_canonical_next_step, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Emit canonical `recovery_step` guidance for ADR transition failures that require explicit user remediation. <!-- verify: cargo test --lib cli::commands::management::adr::guidance::tests::error_with_recovery_embeds_recovery_command_block, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering ADR command output guidance in both human-readable and JSON modes. <!-- verify: cargo test --lib adr::guidance, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0C7OF/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0C7OF/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0C7OF/EVIDENCE/ac-2.log)

### Add Canonical Guidance To Bearing Transition Commands
- **ID:** 1vxZ0DAeT
- **Status:** done

#### Summary
Add canonical guidance output to bearing lifecycle transitions so exploration workflows expose deterministic next or recovery commands when action is required.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to actionable bearing transitions (`survey`, `assess`, `park`, `decline`, `lay`) when a deterministic follow-up exists. <!-- verify: cargo test --lib cli::commands::management::bearing::guidance::tests::terminal_actions_map_to_next_human, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Emit canonical `recovery_step` guidance for bearing transition failures that require a concrete remediation command. <!-- verify: cargo test --lib cli::commands::management::bearing::guidance::tests::lay_epic_exists_recovery_maps_to_epic_show, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering bearing command guidance behavior in both human-readable and JSON output paths. <!-- verify: cargo test --lib cli::commands::management::bearing::guidance::tests, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0DAeT/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0DAeT/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0DAeT/EVIDENCE/ac-2.log)

### Keep Informational Governance Commands Non Prescriptive
- **ID:** 1vxZ0Dvw9
- **Status:** done

#### Summary
Ensure informational governance commands remain non-prescriptive by omitting canonical next-step guidance when no deterministic action is required.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Identify governance read-only commands (for example `adr list/show` and `bearing list/show`) and ensure they do not emit canonical guidance. <!-- verify: cargo test --lib informational_commands_emit_no_guidance, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Preserve prescriptive guidance behavior for actionable governance transitions while keeping informational outputs non-prescriptive. <!-- verify: cargo test --lib guidance::tests, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests asserting informational command outputs omit guidance fields in JSON and avoid imperative next-step text in human output. <!-- verify: cargo test --lib informational_, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0Dvw9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0Dvw9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0Dvw9/EVIDENCE/ac-2.log)


