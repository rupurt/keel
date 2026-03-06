# Institutional Memory & Verification - Product Requirements


## Problem Statement

As Keel matures, we are capturing more artifacts and evidence of work. However, this data is currently passive. There is no automated way to ensure that previously recorded proofs remain valid as the codebase evolves, and there is no high-fidelity way to share this verified progress with stakeholders who don't want to dig through Markdown files.

## Goals & Objectives

Our goal is to turn Keel's evidence into a "living" audit trail that is continuously verified and professionally reported.

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Continuous Trust | 100% of automated proofs re-verified on command | Zero "proof drift" |
| GOAL-02 | High-Fidelity Reporting | Automated generation of stakeholder-ready reports | 1-click reporting |
| GOAL-03 | Frictionless Capture | Developers can record proofs without leaving their context | < 30s to record |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Developer | Implementing features and fixing bugs | Easy way to prove work is done correctly |
| Auditor/Stakeholder | Reviewing project progress and safety | Verified evidence of requirement satisfaction |
| AI Agent | Automating SDLC tasks | Clear, parseable verification instructions |

## Scope

### In Scope

- [SCOPE-01] Automated re-verification of all linked proofs (`keel verify --all`).
- [SCOPE-02] Rich provenance tracking for all recorded evidence.
- [SCOPE-03] Stakeholder-ready audit and voyage narrative reports.
- [SCOPE-04] Streamlined developer experience for capturing evidence.

### Out of Scope

- [SCOPE-05] Integrating with external CI/CD platforms (initially).
- [SCOPE-06] Non-Markdown based reporting (PDF/HTML generation is a future step, initially we focus on rich Markdown/Terminal).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Top-level `verify --all` command | GOAL-01 GOAL-02 GOAL-03 | must | Prevents evidence drift as code changes |
| FR-02 | Provenance metadata in all logs | GOAL-01 GOAL-02 GOAL-03 | must | Essential for auditability and institutional memory |
| FR-03 | Voyage Narrative Reports | GOAL-01 GOAL-02 GOAL-03 | should | Connects technical artifacts to product goals |
| FR-04 | Editor integration for capture | GOAL-01 GOAL-02 GOAL-03 | could | Reduces developer friction |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Comma-separated verify syntax | GOAL-01 GOAL-02 GOAL-03 | must | Clean, robust parsing of multiple attributes |
| NFR-02 | Minimal E2E CLI testing | GOAL-01 GOAL-02 GOAL-03 | must | Ensures command-line interface reliability |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Maintain command-level tests for `verify detect`, `verify recommend`, and `verify --all` with fixture boards that include mixed manual and automated evidence.
- Add regression tests that assert provenance metadata is persisted and surfaced in report output.
- Use `just keel doctor`, `just test`, and at least one generated voyage report as release gates for this epic.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Comma-separation is standard | Parsing logic breaks | Standardized in parser and doctor |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How to handle long-running proofs? | Engineering | Pending |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel doctor` passes with zero warnings or errors
- [ ] `keel verify --all` successfully re-runs all automated proofs
- [ ] Stakeholder report generated for at least one completed voyage
<!-- END SUCCESS_CRITERIA -->

