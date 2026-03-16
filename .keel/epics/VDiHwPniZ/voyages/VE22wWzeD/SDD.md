# Evidence and Risk Carry-Through - Software Design Description

> Preserve evidence provenance and risks in PRD generation

**SRS:** [SRS.md](SRS.md)

## Overview

Extend `create_prd_from_bearing` in `crates/keel-cli/src/cli/commands/management/bearing/mod.rs` to read two additional bearing artifacts (EVIDENCE.md source table and BRIEF.md open questions) and include them in the generated PRD during `bearing lay`.

## Context & Boundaries

```
┌──────────────────────────────────────────────┐
│          create_prd_from_bearing             │
│                                              │
│  ┌────────────┐  ┌─────────┐  ┌──────────┐  │
│  │ BRIEF.md   │  │EVIDENCE │  │ASSESSMENT│  │
│  │ (existing) │  │  .md    │  │  .md     │  │
│  │ +questions │  │ (new)   │  │(existing)│  │
│  └────────────┘  └─────────┘  └──────────┘  │
│                                              │
│              → PRD.md                        │
└──────────────────────────────────────────────┘
```

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Source table format | Copy verbatim from `## Sources` section | Preserves original markdown table without lossy transformation; PRD readers see exact same evidence |
| Open questions format | Convert `- Question text` bullets to `\| Question text \| Planner \| Open \|` rows | Matches existing PRD risks table schema |
| Section placement | Research Provenance after Research Analysis; Risks replace boilerplate | Natural reading order: assessment → sources → remaining risks |

## Components

### Evidence extraction

Read EVIDENCE.md, locate `## Sources` heading, extract everything until the next `##` heading or EOF. If the section exists and contains a markdown table, include it verbatim under `## Research Provenance` in the PRD.

### Open questions extraction

Read BRIEF.md, locate `## Open Questions` heading, extract bullet items (`- ` lines). For each non-empty bullet, emit a row in the PRD risks table with owner "Planner" and status "Open". If no bullets are found, fall back to existing boilerplate.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| EVIDENCE.md missing | `!assessment_path.exists()` pattern already used | Skip Research Provenance section | PRD still valid |
| Sources section empty | Extracted text is empty after trim | Skip Research Provenance section | PRD still valid |
| Open Questions section missing | `extract_section` returns None | Use existing boilerplate risk row | PRD still valid |
