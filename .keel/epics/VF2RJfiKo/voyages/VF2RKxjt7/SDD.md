# Downstream Adoption And Upgrade Docs - Software Design Description

> Add formal docs that show downstream repositories how to adopt Keel's agent contract and how to upgrade and sync upstream guidance without losing project-specific adaptations.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extends the public docs site with two new workflow pages:

1. A downstream adoption page explaining how `AGENTS.md` and `INSTRUCTIONS.md` become the project-management engine contract inside a repository that uses Keel.
2. An upgrade and sync page explaining how downstream maintainers update Keel and reapply repo-specific instruction adaptations safely.

The design stays inside the existing Docusaurus site, reuses the current visual components and prose style, and cross-links the new pages from adjacent public docs workflows.

## Context & Boundaries

### In Scope

- MDX pages under `website/docs/workflows/`
- sidebar/navigation updates in `website/sidebars.ts`
- light cross-links from adjacent docs pages where they improve discoverability

### Out of Scope

- CLI or runtime changes to Keel
- automated synchronization tooling for downstream instruction files
- edits to downstream repositories beyond using `port` as documentary input

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐ │
│  │         │  │         │  │         │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑
   [External]      [External]
```

## Dependencies

<!-- External systems, libraries, services this design relies on -->

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Docusaurus site in `website/` | local docs platform | Hosts the new MDX workflow pages and navigation | existing repo scaffold |
| `AGENTS.md` and `INSTRUCTIONS.md` in Keel | local source docs | Provide the upstream contract being documented | current repository state |
| `port` repo `AGENTS.md` / `INSTRUCTIONS.md` / `justfile` | local reference docs | Provide a concrete downstream adaptation example | current workspace state |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Docs placement | Add both pages under `website/docs/workflows/` | These pages explain operational practice after basic onboarding, not core ontology. |
| Example strategy | Use `port` as the primary downstream example | The user explicitly pointed to it, and it shows real adaptation seams without requiring internet research. |
| Upgrade guidance style | Provide a checklist-style sync workflow rather than automation claims | Downstream sync is a maintenance practice today, not a built-in Keel migration feature. |

## Architecture

The docs site gains two MDX nodes and updated sidebar wiring:

- `workflows/downstream-project-contracts.mdx`
- `workflows/upgrading-keel-and-syncing-instructions.mdx`
- `sidebars.ts` updated to expose both pages in the workflow section

Existing components such as `SignalGrid` are reused for adaptation matrices and operational callouts.

## Components

- Workflow docs page: downstream contract
  Purpose: explain what `AGENTS.md` and `INSTRUCTIONS.md` do in a downstream repo and how they differ from upstream.
- Workflow docs page: upgrade and sync
  Purpose: document a safe maintenance sequence for upgrading Keel and re-syncing upstream guidance.
- Sidebar integration
  Purpose: keep the new pages discoverable from the current docs IA.

## Interfaces

No external API contracts are introduced. The user-facing interfaces are docs URLs, sidebar labels, and markdown links between pages.

## Data Flow

Source material flows from three local inputs into the public docs:

1. Upstream Keel `AGENTS.md` and `INSTRUCTIONS.md`
2. Downstream `port` adaptations of those files
3. Existing public docs IA and visual components

The authored MDX pages summarize and compare those sources, then link users back into adjacent public docs workflows.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| New pages are not discoverable | Manual sidebar review or broken-link build failure | Update `sidebars.ts` and add cross-links | Rebuild docs site |
| Guidance drifts from actual downstream adaptation seams | Manual review against `port` sources | Revise examples to match current files | Rebuild docs site and record manual proof |
| Upgrade guidance overpromises automation | Manual review of wording and command references | Keep the page checklist-based and explicit about what remains manual | Re-run docs build and review |
