# VOYAGE REPORT: Speccy Foundation And Keel Integration Pilot

## Voyage Metadata
- **ID:** VEzIxN01G
- **Epic:** VEzIwU3fh
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Extract Speccy Template Rendering Primitives Into A Reusable Workspace Crate
- **ID:** VEzIyo8d2
- **Status:** done

#### Summary
Create the new `speccy` workspace crate and move the generic markdown template rendering primitives into it without importing any Keel-specific modules or board concepts.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `speccy` exposes deterministic placeholder rendering and markdown document helper APIs equivalent to the current generic behavior in `template_rendering.rs`. <!-- verify: cargo test -p speccy, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `speccy` exposes a host integration hook surface for template lookup and optional post-render behavior without importing Keel-specific types, file paths, or board concepts. <!-- verify: cargo test -p speccy, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] The new crate remains free of `keel-core` and `keel-cli` dependencies and is covered by crate-level tests for representative placeholder and frontmatter/body cases. <!-- verify: cargo test -p speccy, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VEzIyo8d2/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VEzIyo8d2/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VEzIyo8d2/EVIDENCE/ac-3.log)

### Migrate Keel Markdown Template Rendering Onto Speccy
- **ID:** VEzIyofd3
- **Status:** done

#### Summary
Define the host integration hook surface needed for Keel, then rewire existing template-rendering call sites to consume `speccy` for generic rendering while keeping only host-specific adapter behavior in Keel.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] `speccy` defines the host integration hook surface Keel needs for template lookup and optional post-render behavior without introducing Keel-specific types into the reusable crate. <!-- verify: cargo test -p speccy, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-01] Keel call sites that currently use `template_rendering::{render, render_body, render_with_mutations}` consume `speccy` for the generic rendering path. <!-- verify: cargo test -p keel, SRS-03:start, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] Representative Keel scaffold-generation flows continue to produce behaviorally equivalent output after the cutover. <!-- verify: cargo test -p keel, SRS-NFR-02:start:end, SRS-03:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VEzIyofd3/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VEzIyofd3/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VEzIyofd3/EVIDENCE/ac-3.log)

### Document Speccy Hooks And External Adoption Boundaries
- **ID:** VEzIyoyd4
- **Status:** done

#### Summary
Define and document the public hook surface and the boundary between reusable `speccy` behavior and host-owned project logic so other projects can adopt the crate intentionally, with template inventory remaining host-owned while generic frontmatter mutation lives in `speccy`.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Planning and voyage artifacts record which concerns remain host-owned after the extraction, including the final decision that generic frontmatter mutation lives in `speccy`. <!-- verify: manual, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-03/AC-01] The documented hook model supports embedded or caller-managed template catalogs without forcing filesystem assumptions into `speccy`. <!-- verify: cargo test -p speccy, SRS-NFR-03:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VEzIyoyd4/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VEzIyoyd4/EVIDENCE/ac-2.log)


