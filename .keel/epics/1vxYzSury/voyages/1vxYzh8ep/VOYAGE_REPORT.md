# VOYAGE REPORT: Output Contract And Shared Renderer

## Voyage Metadata
- **ID:** 1vxYzh8ep
- **Epic:** 1vxYzSury
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Define Canonical Guidance Output Contract
- **ID:** 1vxZ0AFJK
- **Status:** done

#### Summary
Define a canonical command-guidance contract that can represent one deterministic next step or one deterministic recovery step, and wire it into `keel next` JSON output as the baseline for broader command adoption.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Introduce a shared canonical guidance contract type with explicit `next_step` and `recovery_step` fields for machine-readable command output. <!-- verify: cargo test --lib serializes_next_step_only, SRS-01:start, proof: ac-1.log -->
- [x] [SRS-01/AC-02] Make `keel next --json` emit the canonical guidance contract for actionable decisions (next step on success paths, recovery step on blocked paths). <!-- verify: cargo test --lib decision_to_json_work_includes_next_step_guidance, SRS-01:continues, proof: ac-2.log -->
- [x] [SRS-01/AC-03] Add regression tests covering guidance contract serialization and decision-to-guidance mapping behavior. <!-- verify: cargo test --lib decision_to_json_blocked_includes_recovery_guidance, SRS-01:end, proof: ac-3.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0AFJK/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0AFJK/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0AFJK/EVIDENCE/ac-2.log)

### Add Contract Tests For Canonical Guidance Fields
- **ID:** 1vxZ0Bh0v
- **Status:** done

#### Summary
Add regression tests that lock the canonical guidance payload shape so downstream harnesses can rely on stable `next_step` and `recovery_step` semantics.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add unit tests for canonical guidance serialization covering `next_step`-only, `recovery_step`-only, and omitted guidance cases. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Add mapping tests that validate deterministic command strings for both actionable and blocked decisions. <!-- verify: cargo test --lib decision_to_json_, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Ensure tests fail on contract drift in field names or object shape expected by harness consumers. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::serializes_omitted_guidance_as_empty_object, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0Bh0v/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0Bh0v/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0Bh0v/EVIDENCE/ac-2.log)

### Implement Shared Guidance Renderer Helpers
- **ID:** 1vxZ0BinH
- **Status:** done

#### Summary
Extract and reuse shared guidance rendering helpers so commands emit canonical next and recovery guidance through one implementation path.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Implement shared helper(s) that build canonical guidance payloads from command decisions without duplicating per-command logic. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::render_command_guidance_, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Refactor `keel next` JSON formatting to use the shared helper path while preserving existing decision semantics. <!-- verify: cargo test --lib decision_to_json_, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Provide focused tests proving helpers produce stable command strings and do not emit conflicting guidance fields. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::render_command_guidance_never_emits_conflicting_fields, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxZ0BinH/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxZ0BinH/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxZ0BinH/EVIDENCE/ac-2.log)


