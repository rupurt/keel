# Interactive Inquiry Persona - Product Requirements

## Problem Statement

New developers need a way to test their comprehension of formal rules through play.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Enable interactive comprehension testing of formal rules. | User feedback on Inquiry persona utility | Initial rollout complete |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| New Developer | Just joining the project | Rapidly learning the engine's physics |

## Scope

### In Scope

- [SCOPE-01] Implementation of "Student" persona.
- [SCOPE-02] Implementation of "Interrogator" persona.
- [SCOPE-03] Integration with `keel play --theater --mood inquiry`.

### Out of Scope

- [SCOPE-04] LLM training or non-deterministic persona logic beyond prompt grounding.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Student persona asks clarification questions based on Formal Rules. | GOAL-01 | must | Forces user to articulate rule understanding. |
| FR-02 | Interrogator persona challenges artifact evidence. | GOAL-01 | should | Improves overall data quality and rigor. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Maintain reliability and observability for all new workflow paths introduced by this epic. | GOAL-01 | must | Keeps operations stable and makes regressions detectable during rollout. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Problem outcome | Tests, CLI proofs, or manual review chosen during planning | Story-level verification artifacts linked during execution |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The problem statement reflects a real user or operator need. | The epic may optimize the wrong outcome. | Revisit with planners during decomposition. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which metric best proves the problem above is resolved? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] The team can state a measurable user outcome that resolves the problem above.
<!-- END SUCCESS_CRITERIA -->
