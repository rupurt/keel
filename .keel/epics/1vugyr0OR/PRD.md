# Institutional Memory & Verification - Product Requirements

> Transform board evidence into a living, high-fidelity audit trail with continuous verification and rich reporting.

## Problem Statement

As Keel matures, we are capturing more artifacts and evidence of work. However, this data is currently passive. There is no automated way to ensure that previously recorded proofs remain valid as the codebase evolves, and there is no high-fidelity way to share this verified progress with stakeholders who don't want to dig through Markdown files.

## Goals & Objectives

Our goal is to turn Keel's evidence into a "living" audit trail that is continuously verified and professionally reported.

| Goal | Success Metric | Target |
|------|----------------|--------|
| Continuous Trust | 100% of automated proofs re-verified on command | Zero "proof drift" |
| High-Fidelity Reporting | Automated generation of stakeholder-ready reports | 1-click reporting |
| Frictionless Capture | Developers can record proofs without leaving their context | < 30s to record |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Developer | Implementing features and fixing bugs | Easy way to prove work is done correctly |
| Auditor/Stakeholder | Reviewing project progress and safety | Verified evidence of requirement satisfaction |
| AI Agent | Automating SDLC tasks | Clear, parseable verification instructions |

## Scope

### In Scope

- Automated re-verification of all linked proofs (`keel verify --all`).
- Rich provenance tracking for all recorded evidence.
- Stakeholder-ready audit and voyage narrative reports.
- Streamlined developer experience for capturing evidence.

### Out of Scope

- Integrating with external CI/CD platforms (initially).
- Non-Markdown based reporting (PDF/HTML generation is a future step, initially we focus on rich Markdown/Terminal).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Top-level `verify --all` command | must | Prevents evidence drift as code changes |
| FR-02 | Provenance metadata in all logs | must | Essential for auditability and institutional memory |
| FR-03 | Voyage Narrative Reports | should | Connects technical artifacts to product goals |
| FR-04 | Editor integration for capture | could | Reduces developer friction |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Comma-separated verify syntax | must | Clean, robust parsing of multiple attributes |
| NFR-02 | Minimal E2E CLI testing | must | Ensures command-line interface reliability |
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

