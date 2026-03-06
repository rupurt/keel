---
created_at: 2026-03-05T17:53:17
---

# Knowledge - 1vyFmfjA9

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Validate Goal Links In PRD Requirements (1vyGZeEI7)

### 1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Parsing authored PRD/SRS markdown tables where an empty cell is semantically meaningful |
| **Insight** | Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace |
| **Suggested Action** | Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes |
| **Applies To** | `src/domain/state_machine/*.rs`, planning document table parsers |
| **Applied** |  |

### 1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Adding PRD goal-lineage rules that must appear consistently in doctor diagnostics and planning show projections |
| **Insight** | Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render |
| **Suggested Action** | Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks |
| **Applied** |  |



---

## Story: Parse Canonical Goal Lineage (1vyGZeNMa)

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

## Story: Render Goal Coverage In Epic Planning (1vyGZfiEk)

### 1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Adding PRD goal-lineage rules that must appear consistently in doctor diagnostics and planning show projections |
| **Insight** | Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render |
| **Suggested Action** | Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks |
| **Applied** |  |



---

## Synthesis

### Ae9IPgiPR: Preserve Empty Markdown Table Cells In Planning Parsers

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

### u3TVCi9iv: Keep Goal Lineage Parsing On One Canonical Path

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Adding PRD goal-lineage rules that must appear consistently in doctor diagnostics and planning show projections |
| **Insight** | Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render |
| **Suggested Action** | Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks |
| **Linked Knowledge IDs** | 1vyJXGpcM |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** |  |

### ZdWsVSMN2: Preserve Empty Markdown Table Cells In Planning Parsers

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

### lRYy7tUIu: Keep Goal Lineage Parsing On One Canonical Path

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Adding PRD goal-lineage rules that must appear consistently in doctor diagnostics and planning show projections |
| **Insight** | Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render |
| **Suggested Action** | Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks |
| **Linked Knowledge IDs** | 1vyJXGpcM |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** |  |

