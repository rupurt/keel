# Launch Keel Public MDX Documentation - Product Requirements

## Problem Statement

Keel has strong internal source documents, but it lacks a formal public documentation experience that onboards external OSS users, translates the model gradually, and guides different personas through adoption and day-to-day use.

Today the public surface is fragmented across root markdown files and a narrow automation guide. That material contains real signal, but it is not arranged as a product-led learning journey. New users do not get a narrative homepage, a clear quickstart, a gradual mental-model ramp, or persona-specific paths that show how Keel fits project managers, programmers, designers, and adjacent business roles.

We need an onboarding-first MDX documentation site that tells a coherent story about Keel as a turn-based board operating engine for human/AI delivery teams, while preserving technical rigor and absorbing the existing `GUIDE.md` content into the broader docs IA.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Launch a formal public docs surface for Keel. | A static MDX site exists in-repo with a product-led homepage, structured nav, and repo-local build commands. | Public docs scaffold landed |
| GOAL-02 | Optimize the docs for onboarding and adoption first. | New users can move from product narrative to installation, first turn, and core model without prior Keel vocabulary. | Core onboarding path authored |
| GOAL-03 | Show how Keel serves different team roles. | Persona tracks exist for the three main working roles plus broader leadership/specialist adoption. | Persona docs authored |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| OSS Evaluator | A new external user assessing whether Keel is worth adopting. | A clear product narrative, quickstart, and mental model that make the system understandable fast. |
| Project Manager | Plans and coordinates work across human/AI teams. | A path that explains how Keel structures delivery, verification, and queue discipline. |
| Programmer | Delivers implementation work inside Keel’s workflow. | A path from installation to the first mission/story workflow and day-to-day operations. |
| Designer | Contributes design and research work inside the same board model. | A path that explains how exploratory and delivery work fit together without assuming only software engineers use the system. |
| Leadership / Specialist Roles | Marketers, lawyers, general managers, CEOs, and similar stakeholders. | A concise explanation of how Keel supports planning, governance, review, and adoption across non-programming roles. |

## Scope

### In Scope

- [SCOPE-01] Choose and scaffold an MDX-based docs site in-repo with a static build suitable for S3/CloudFront deployment and local development through repo-supported tooling.
- [SCOPE-02] Create an onboarding-first docs IA with a product-led homepage, quickstart, basics, and a gradual introduction to Keel terminology and concepts.
- [SCOPE-03] Author separate persona tracks after the basics for project managers, programmers, designers, and broader leadership/specialist readers.
- [SCOPE-04] Absorb `GUIDE.md` into the new docs IA by moving routines and pulse material into the public docs site and removing the standalone guide.
- [SCOPE-05] Add visual components, diagrams, and page-level design treatments that make the first pass feel intentional and product-quality rather than purely textual.

### Out of Scope

- [SCOPE-06] Paid-only, hosted-product, or enterprise documentation beyond light future-facing mention.
- [SCOPE-07] Exhaustive reference coverage for every command and internal root document in the first docs pass.
- [SCOPE-08] Final production deployment wiring for S3/CloudFront outside of build-readiness and configuration guidance.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The repo must include a formal MDX docs site scaffold with static build output, structured navigation, and repo-local commands for development and production builds. | GOAL-01 | must | Without the site scaffold there is no durable public docs surface to build on. |
| FR-02 | The docs must lead with product narrative and then guide users through installation, first-turn workflow, and Keel’s core operating model using gradual terminology introduction. | GOAL-02 | must | Onboarding and adoption are the primary goals of this effort. |
| FR-03 | The docs must include separate persona guidance after the basics for project managers, programmers, designers, and a broader leadership/specialist audience. | GOAL-03 | must | The user explicitly wants role-based pathways rather than one undifferentiated reference manual. |
| FR-04 | The current `GUIDE.md` content must be absorbed into the new docs IA as routines and pulse workflow documentation. | GOAL-01 GOAL-02 | must | The narrow guide should become part of the formal public docs experience rather than living beside it. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The first-pass docs must feel formal and product-like, with a touch of Keel’s cinematic point of view, without becoming novelty-first or obscuring practical usage. | GOAL-01 GOAL-02 GOAL-03 | must | The docs are both a product surface and a teaching surface. |
| NFR-02 | Examples and workflows must remain AI-vendor-neutral so the docs work for different harnesses and human/AI collaboration setups. | GOAL-02 GOAL-03 | must | Public OSS docs cannot anchor themselves to one vendor or runtime. |
| NFR-03 | The docs site must build from this repo in the local environment through documented, reproducible tooling compatible with the repo’s Nix-based workflow. | GOAL-01 | must | There is no existing Node toolchain in the repo environment today. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Site scaffold and build | Static build command plus manual inspection of generated IA/components | Story-level evidence logs |
| Onboarding and core narrative | Manual review of authored docs pages and navigation | Story-level evidence logs |
| Persona and routines migration | Manual review of persona pages plus routines/pulse docs migration | Story-level evidence logs |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A static MDX site is the right first public documentation format for Keel. | The team may need a different publishing/runtime model later. | Validate by shipping a credible first public docs pass in-repo. |
| The first public docs pass can focus on onboarding, adoption, and selective workflows without duplicating every internal source document. | The first pass may feel incomplete to deeply technical readers. | Make the IA explicit and leave internal deep references accessible where needed. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much of the internal governance/reference corpus should later be promoted into the public site after the onboarding-first pass? | Epic owner | Open |
| Will the final hosted `spoke.sh` URL structure require different defaults than the local site base configuration? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Keel has a public MDX docs site scaffolded in-repo with a static build path and product-led homepage.
- [ ] New OSS users can move through onboarding, installation, first turn, and core concepts without needing prior Keel vocabulary.
- [ ] Separate persona guides exist after the basics, and the routines/pulse guide has been absorbed into the new docs IA.
<!-- END SUCCESS_CRITERIA -->
