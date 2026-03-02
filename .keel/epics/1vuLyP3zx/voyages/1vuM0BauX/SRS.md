# foundation-unification - Software Requirements Specification

> Merge Problem types and centralize core story/voyage structural checks.

**Epic:** [1vuLyP3zx](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

This voyage delivers the foundational unification of the validation system. It focuses on merging the disparate error types into a single `Problem` type and ensuring that core structural checks for stories and voyages are centralized and shared between `doctor` and command gates.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Merge `GateProblem` and `Problem` into a single `Problem` type | FR-01 | Inspection/Tests |
| SRS-02 | Centralize story and voyage structural checks in `doctor/checks` | FR-02 | Inspection |
| SRS-03 | Update `evaluate_story_transition` and `evaluate_voyage_transition` to use unified `Problem` type | FR-03 | Tests |
| SRS-04 | Refactor `doctor` to delegate to `gating` or shared check modules for domain rules | FR-03 | Tests |
| SRS-05 | `story submit` uses unified check logic for SRS and Evidence validation | FR-05 | Tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | All validation rules must be centralized in `src/validation/` | NFR-01 | Inspection |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
