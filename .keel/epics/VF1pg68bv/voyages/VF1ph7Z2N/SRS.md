# Public Docs Site And Persona Guides - SRS

## Summary

Epic: VF1pg68bv
Goal: Create an onboarding-first MDX documentation site for external OSS users with a product-led narrative, visual components, persona tracks, and absorbed routine automation guidance.

## Scope

### In Scope

- [SCOPE-01] Scaffold a static MDX docs site in `website/` with repo-local build and development commands that work in Keel’s Nix-based environment.
- [SCOPE-02] Author an onboarding-first information architecture covering product narrative, installation, quickstart, first turn, and gradual concept introduction.
- [SCOPE-03] Add persona-specific guides for project managers, programmers, designers, and broader leadership/specialist roles after the basics.
- [SCOPE-04] Migrate the routines and pulse workflow guidance out of `GUIDE.md` and into the public docs site.
- [SCOPE-05] Add reusable diagrams, illustrations, and page-level components that make the docs feel product-quality on first pass.

### Out of Scope

- [SCOPE-06] Full paid-product or enterprise documentation.
- [SCOPE-07] Exhaustive command-by-command reference coverage for every Keel surface.
- [SCOPE-08] Final live deployment infrastructure for `spoke.sh` beyond build-readiness and configuration guidance.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The repo must provide a scaffolded MDX docs site with structured navigation, a product-led homepage, and reproducible local dev/build commands that work through the repo-supported toolchain. | SCOPE-01 SCOPE-05 | FR-01 | docs build + manual review |
| SRS-02 | The docs IA must take a new OSS user from narrative overview to installation, first turn, and core Keel concepts while introducing internal vocabulary gradually. | SCOPE-02 | FR-02 | manual review |
| SRS-03 | The docs must include separate persona tracks after the basics for project managers, programmers, designers, and a broader leadership/specialist audience. | SCOPE-03 | FR-03 | manual review |
| SRS-04 | The routines and pulse workflow guidance currently in `GUIDE.md` must be moved into the public docs site and the standalone guide removed. | SCOPE-04 | FR-04 | manual review + file check |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The docs experience must feel formal and product-led while keeping Keel’s cinematic opinions as controlled accent rather than noise. | SCOPE-02 SCOPE-03 SCOPE-05 | NFR-01 | manual review |
| SRS-NFR-02 | Docs examples must remain AI-vendor-neutral and suitable for multiple harnesses. | SCOPE-02 SCOPE-03 SCOPE-04 | NFR-02 | manual review |
| SRS-NFR-03 | The site must build successfully from this repo through documented tooling compatible with the Nix-based environment and static hosting expectations. | SCOPE-01 | NFR-03 | docs build |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
