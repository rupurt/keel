# VOYAGE REPORT: Lineage Validation

## Voyage Metadata
- **ID:** 1vzWg8000
- **Epic:** 1vzWfz000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Lineage CLI Output
- **ID:** 1vzWgD000
- **Status:** done

#### Summary
Update user-facing output for lay paths so lineage metadata is visible and explainable.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Show the resolved `epic` lineage token in `keel bearing lay` and related show output. <!-- verify: manual, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] Document how goal-link capture maps BRIEF success criteria to epic goals in CLI help/docs. <!-- verify: manual, SRS-03:end, proof: ac-2.log -->
- [x] [SRS-04/AC-01] Add user-facing CLI messaging for invalid/unknown goal references with offending field and suggested fix. <!-- verify: manual, SRS-04:start, proof: ac-3.log -->
- [x] [SRS-06/AC-01] Ensure show surfaces render lineage-related fields without truncating unknown legacy values. <!-- verify: manual, SRS-06:start:end, proof: ac-4.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzWgD000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzWgD000/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/1vzWgD000/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/1vzWgD000/EVIDENCE/ac-4.log)

### Lineage Doctor Checks
- **ID:** 1vzWgW000
- **Status:** done

#### Summary
Implement doctor diagnostics for stale or missing bearing lineage states.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add hard-fail diagnostics for laid bearings missing `epic` lineage. <!-- verify: cargo test --lib check_bearing_lineage_epic_flags_laid_without_epic, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Add hard-fail diagnostics for invalid goal-lineage references, including offending artifact and suggested remediation command. <!-- verify: cargo test --lib check_bearing_lineage_goals_flags_invalid_format, SRS-02:start:end -->

### Correction Documentation
- **ID:** 1vzWgX000
- **Status:** done

#### Summary
Add validation and migration documentation for lineage checks and corrective actions.

#### Acceptance Criteria
- [x] [SRS-04/AC-02] Define and document correction commands for stale/invalid lineage values. <!-- verify: cargo test --lib lay_unknown_goal_recovery_maps_to_brief, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add regression notes for deterministic validation behavior under repeated read/validation cycles. <!-- verify: cargo test --lib lineage_validation_is_deterministic_across_repeated_cycles, SRS-04:end -->

### Legacy Bearing Migration
- **ID:** 1vzWgY000
- **Status:** done

#### Summary
Create migration path for legacy laid-bearing artifacts created before this lineage contract.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Add an explicit migration/repair flow for pre-contract bearings missing lineage metadata. <!-- verify: cargo test --lib check_bearing_lineage_epic_flags_laid_without_epic, SRS-05:start -->
- [x] [SRS-05/AC-02] Ensure migration checks scale linearly on board-size representative fixtures and remain non-interactive. <!-- verify: cargo test --lib lineage_migration_scales_linearly_on_board_size, SRS-05:end -->


