---
created_at: 2026-03-05T16:44:19
---

# Knowledge - 1vyFiQPoH

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Gate Voyage Planning On PRD Lineage (1vyGZEO8S)

### 1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Parsing authored PRD/SRS markdown tables where an empty cell is semantically meaningful |
| **Insight** | Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace |
| **Suggested Action** | Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes |
| **Applies To** | `src/domain/state_machine/*.rs`, planning document table parsers |
| **Applied** |  |



---

## Story: Render Epic Requirement Coverage (1vyGZEZNc)

### 1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Parsing authored PRD/SRS markdown tables where an empty cell is semantically meaningful |
| **Insight** | Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace |
| **Suggested Action** | Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes |
| **Applies To** | `src/domain/state_machine/*.rs`, planning document table parsers |
| **Applied** |  |

### 1vyIA4sQm: Scope Planning Diagnostics To Transition-Relevant Voyages

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | when doctor reuses planning-gate lineage checks |
| **Insight** | reporting the exact same lineage rules across the whole board retroactively fails historical `done` voyages that cannot exercise `voyage plan`, so diagnostic scope must match transition reachability unless migration is explicit work |
| **Suggested Action** | apply planning coherence checks to non-terminal voyages by default, and handle historical migrations in separate board-cleanup stories |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/*.rs`, planning coherence diagnostics |
| **Applied** | yes |



---

## Synthesis

### XulqhuzCa: Preserve Empty Markdown Table Cells In Planning Parsers

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Parsing authored PRD/SRS markdown tables where an empty cell is semantically meaningful |
| **Insight** | Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace |
| **Suggested Action** | Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes |
| **Applies To** | `src/domain/state_machine/*.rs`, planning document table parsers |
| **Linked Knowledge IDs** | 1vyH1gD7p |
| **Score** | 0.84 |
| **Confidence** | 0.93 |
| **Applied** |  |

### 3M8tCLJGF: Preserve Empty Markdown Table Cells In Planning Parsers

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Parsing authored PRD/SRS markdown tables where an empty cell is semantically meaningful |
| **Insight** | Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace |
| **Suggested Action** | Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes |
| **Applies To** | `src/domain/state_machine/*.rs`, planning document table parsers |
| **Linked Knowledge IDs** | 1vyH1gD7p |
| **Score** | 0.84 |
| **Confidence** | 0.93 |
| **Applied** |  |

### wpJ5gpgD6: Scope Planning Diagnostics To Transition-Relevant Voyages

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | when doctor reuses planning-gate lineage checks |
| **Insight** | reporting the exact same lineage rules across the whole board retroactively fails historical `done` voyages that cannot exercise `voyage plan`, so diagnostic scope must match transition reachability unless migration is explicit work |
| **Suggested Action** | apply planning coherence checks to non-terminal voyages by default, and handle historical migrations in separate board-cleanup stories |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/*.rs`, planning coherence diagnostics |
| **Linked Knowledge IDs** | 1vyIA4sQm |
| **Score** | 0.88 |
| **Confidence** | 0.95 |
| **Applied** | yes |

