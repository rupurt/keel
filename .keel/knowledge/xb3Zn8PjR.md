---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9wpT7/KNOWLEDGE.md
scope: null
source_story_id: null
---

### xb3Zn8PjR: Build Typed Command Actions Before Dispatching

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring large CLI dispatch trees while preserving behavior and clap argument contracts |
| **Insight** | Converting `ArgMatches` into typed action enums at the boundary and routing through module `run(action)` functions keeps `main` focused on parsing while pushing interface adaptation into command-group modules |
| **Suggested Action** | Keep adding action enums and single entrypoint adapters per command group so architecture tests can enforce delegation contracts cleanly |
| **Applies To** | src/main.rs; src/commands/*/mod.rs |
| **Linked Knowledge IDs** | 1vyDuwf3P |
| **Observed At** |  |
| **Score** | 0.80 |
| **Confidence** | 0.87 |
| **Applied** | 1vwqCf53S |
