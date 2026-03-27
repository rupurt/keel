# Public Docs Site And Persona Guides - Software Design Description

> Create an onboarding-first MDX documentation site for external OSS users with a product-led narrative, visual components, persona tracks, and absorbed routine automation guidance.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a dedicated public docs site under `website/` using an MDX-native static site framework. The site leads with a product narrative, then steps users through installation, first turn, and mental model before branching into persona-specific workflows. Existing narrow guidance from `GUIDE.md` is absorbed into the new IA rather than maintained as a separate top-level artifact.

## Context & Boundaries

The docs site is a public OSS-facing surface, not a mirror of every internal root markdown document. It should teach the product and common workflows clearly without becoming an unfiltered dump of internal governance docs. The first pass focuses on onboarding, adoption, and key workflows rather than exhaustive reference completeness.

```
┌──────────────────────────────────────────────────────┐
│                    website/                         │
│                                                      │
│  docs/        src/pages/        src/components/      │
│  onboarding   homepage          diagrams/callouts    │
│  personas     mdx pages         visual sections      │
└──────────────────────────┬───────────────────────────┘
                           │
                      static build
                           │
                   S3 + CloudFront hosting
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Docusaurus | Site framework | MDX docs site, routing, theming, static build | current upstream scaffold |
| Nix-provided Node toolchain | Build/runtime dependency | Local docs development and static builds in this repo environment | `nix shell nixpkgs#nodejs_22` |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Site stack | Use Docusaurus in a dedicated `website/` folder. | Strong MDX support, mature static docs workflow, versionable OSS docs ergonomics, and clean static deployment to S3/CloudFront. |
| Tooling path | Use the site through repo-local commands and Nix-provided Node instead of assuming Node is globally installed. | This repo currently has no Node toolchain available by default. |
| Narrative order | Lead with product story and onboarding, then branch into persona tracks and workflow-specific pages. | The user explicitly wants onboarding and adoption first. |
| Terminology model | Translate first, introduce Keel-specific vocabulary gradually. | External OSS users should not need internal jargon to get value on day one. |
| Guide migration | Fold `GUIDE.md` into the new docs IA as routines and pulse documentation. | The docs should feel like one coherent product surface. |

## Architecture

The docs site has three layers:

- `docs/` for structured MDX content and sidebar-driven learning paths.
- `src/pages/` for the custom homepage and other narrative landing pages.
- `src/components/` plus theme-level styling for reusable diagrams, persona cards, and visual callouts.

## Components

- Site scaffold
  Purpose: framework config, build commands, local development path, static export.
  Interface: Docusaurus config, package scripts, repo-level helper commands.
- Narrative homepage
  Purpose: explain what Keel is, why it exists, and how to enter the docs.
  Interface: custom React/MDX page with visual sections and CTA links into the docs paths.
- Core docs IA
  Purpose: installation, quickstart, first turn, core concepts, terminology ramp.
  Interface: MDX docs under structured sidebars.
- Persona tracks
  Purpose: role-specific “how to use Keel” guides after the basics are covered.
  Interface: MDX docs pages grouped by persona.
- Workflow migration pages
  Purpose: absorb routines and pulse docs from `GUIDE.md`.
  Interface: dedicated workflow/reference pages inside the docs site.

## Interfaces

This voyage does not expose a runtime API. Its public interface is the site IA itself:

- Landing page and homepage CTA structure.
- Left-nav docs hierarchy for onboarding, concepts, workflows, and personas.
- Static build output suitable for object storage + CDN hosting.

## Data Flow

1. Source content is authored as MDX plus React-backed visual components.
2. Local development and production builds run through a Node toolchain provided via Nix-backed commands.
3. Docusaurus compiles the site into static assets.
4. The resulting artifact is suitable for publication on `spoke.sh` behind S3 + CloudFront.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Site toolchain unavailable locally | Docs dev/build command fails | Provide Nix-based commands and document the required workflow | Use the repo-supported docs commands |
| Docs IA drifts into jargon-heavy internal language | Manual review against onboarding requirements | Rewrite pages to translate first and defer internal vocabulary | Iterate on prose before closure |
| GUIDE migration leaves duplicate or stale guidance | File review and page review | Remove standalone guide once equivalent docs pages exist | Keep one canonical workflow page in the site |
