# Automation Guide Authoring - SRS

## Summary

This voyage produces the first canonical guide for business automation in Keel,
covering routine authoring, temporal review, pulse execution, and scheduled
automation boundaries.

## Scope

### In Scope

- [SCOPE-01] `GUIDE.md` structure for routine, gating, pulse, and flow concepts
- [SCOPE-02] End-to-end examples of recurring work automation
- [SCOPE-03] Operational safety guidance for cron/systemd and idempotent pulse runs

### Out of Scope

- [SCOPE-90] Implementing the underlying automation features
- [SCOPE-91] Press-release or marketing collateral

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Author `GUIDE.md` that explains routine authoring, cadence fields, target scope, and blueprint expectations. | SCOPE-01 | FR-01 | llm-judge |
| SRS-02 | Include an end-to-end walkthrough that ties routine definition to `keel next`, `keel flow`, and `keel pulse`. | SCOPE-02 | FR-02 | llm-judge |
| SRS-03 | Document operational boundaries for cron/systemd, idempotency expectations, and unsupported automation paths. | SCOPE-03 | FR-03 | llm-judge |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Guide content matches supported CLI names and hard-cutover workflow semantics. | SCOPE-01,SCOPE-03 | NFR-01 | manual review |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
