# VOYAGE REPORT: Command Classification Drift Guards

## Voyage Metadata
- **ID:** 1vxYzsAxT
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add Drift Tests For Canonical Guidance Contracts
- **ID:** 1vxZ0FGSF
- **Status:** done

#### Summary
Add drift tests that enforce canonical command guidance contracts and prevent actionable/informational classification regressions.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add drift tests asserting actionable management commands emit canonical guidance with the expected contract shape. <!-- verify: cargo test --lib guidance_contracts::actionable_commands_emit_canonical_guidance_shape, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Add drift tests asserting informational management commands omit canonical guidance fields. <!-- verify: cargo test --lib guidance_contracts::informational_commands_omit_guidance_fields, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Ensure drift tests fail on contract-key changes or classification regressions that would break harness automation. <!-- verify: cargo test --lib guidance_contracts, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0FGSF/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0FGSF/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0FGSF/EVIDENCE/ac-2.log)

### Implement Command Capability Classification Map
- **ID:** 1vxZ0FZaN
- **Status:** done

#### Summary
Implement a single command-capability classification map so guidance rendering can consistently distinguish actionable from informational commands.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Introduce a canonical classification map that labels management commands as actionable or informational. <!-- verify: cargo test --lib capability_map::tests::classification_map_labels_representative_commands, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Use the classification map in guidance rendering paths to control when `next_step` or `recovery_step` guidance is emitted. <!-- verify: cargo test --lib capability_map::tests::informational_commands_suppress_guidance_payload, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering representative commands in both categories to ensure deterministic classification behavior. <!-- verify: cargo test --lib capability_map::tests::actionable_commands_emit_canonical_next_or_recovery_payload, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0FZaN/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0FZaN/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0FZaN/EVIDENCE/ac-2.log)

### Document Command Guidance Contract For Harness Consumers
- **ID:** 1vxZ0FtD2
- **Status:** done

#### Summary
Document the canonical command guidance contract for harness consumers so automation can reliably interpret actionable and recovery recommendations.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Document the canonical guidance schema (`next_step`, `recovery_step`, and command string semantics) in CLI-facing documentation. <!-- verify: rg --line-number -e guidance.next_step.command -e guidance.recovery_step.command ../README.md, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Document classification semantics for actionable versus informational commands and the single canonical next-step rule. <!-- verify: rg --line-number -e Actionable: -e Informational: -e next-step ../README.md, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Provide examples for success, blocked recovery, and informational no-guidance cases that harnesses can consume directly. <!-- verify: rg --line-number -e no-action-required -e recovery_step -e 1vxZ0FtD2 ../README.md, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0FtD2/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0FtD2/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0FtD2/EVIDENCE/ac-2.log)


