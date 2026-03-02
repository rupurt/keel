# High Fidelity Reporting - Software Design Description

> Generate rich, stakeholder-ready audit and voyage narrative reports from verified board state.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage implements the reporting engine that transforms passive board data into active narrative reports. It stitches together SRS requirements, story outcomes, and high-fidelity proofs (VHS/LLM) into a cohesive summary for humans.

## Components

### 1. Narrative Synthesizer
- **Purpose**: Combines `PRESS_RELEASE.md`, `SRS.md`, and story `REFLECT.md` files.
- **Behavior**: Uses the Epic's PR as a template, filling in actual story outcomes and "lessons learned" to create a realistic progress report.

### 2. Evidence Embedder
- **Purpose**: Links or embeds rich media proofs into reports.
- **Behavior**: Scans `EVIDENCE/` for artifacts linked in Acceptance Criteria and generates Markdown image/video/link tags.

### 3. PR Release Generator
- **Purpose**: Generates `PR_RELEASE.md` at the voyage root when complete.
- **Behavior**: Produces a summary suitable for copying into GitHub/GitLab PR descriptions.

## Data Flow

```
Board Files (README, SRS, PRD) ──┐
                                 ├─▶ [Reporting Engine] ─▶ VOYAGE_REPORT.md
Story Bundles (README, REFLECT) ─┤
                                 └─▶ [PR Generator] ─▶ PR_RELEASE.md
Evidence Artifacts (VHS, LLM)  ──┘
```

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Format | Markdown-First | Native compatibility with project board and Git hosts |
| Templating | Static Replace | Simple, deterministic, and easy for agents to reason about |
