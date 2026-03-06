---
created_at: 2026-03-05T18:31:09
---

# Knowledge - 1vyFn0OuN

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Parse Canonical Scope Lineage (1vyGZfkjV)

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

## Story: Detect Scope Drift During Planning (1vyGZflfJ)

### 1vyIA4sQm: Scope Planning Diagnostics To Transition-Relevant Voyages

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | when doctor reuses planning-gate lineage checks |
| **Insight** | reporting the exact same lineage rules across the whole board retroactively fails historical `done` voyages that cannot exercise `voyage plan`, so diagnostic scope must match transition reachability unless migration is explicit work |
| **Suggested Action** | apply planning coherence checks to non-terminal voyages by default, and handle historical migrations in separate board-cleanup stories |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/*.rs`, planning coherence diagnostics |
| **Applied** | yes |

### 1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Adding PRD goal-lineage rules that must appear consistently in doctor diagnostics and planning show projections |
| **Insight** | Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render |
| **Suggested Action** | Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks |
| **Applied** |  |

### 1vyKOWjfv: Canonical Scope Contracts Need Explicit Activation Markers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Introducing canonical scope-lineage validation while existing voyages still contain prose-only scope sections |
| **Insight** | New planning contracts are safest when they activate off explicit markers like `SCOPE-*`; otherwise doctor treats historical prose as invalid structure and turns a targeted validator into migration noise. |
| **Suggested Action** | Define an activation marker whenever a new authored planning contract is introduced, and only enforce the stricter validator once that marker appears in the relevant artifacts. |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/cli/commands/diagnostics/doctor/checks/voyages.rs`, PRD and SRS scope sections |
| **Applied** | yes |



---

## Story: Render Scope Lineage In Planning Surfaces (1vyGZgiTK)

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

### 1vyKXvBA1: Lineage Surfaces Need IDs And Prose Together

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering planning lineage for human review while keeping canonical IDs visible for machine-checkable contracts |
| **Insight** | Planning surfaces become much more reviewable when each lineage row carries the canonical ID, the authored prose, and the parent/child disposition context together; token-only output hides meaning, while prose-only output hides the contract. |
| **Suggested Action** | For future lineage read models, project one canonical row format that combines identifiers with authored descriptions and relation context before the CLI renderer touches it. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/epic/show.rs`, `src/cli/commands/management/voyage/show.rs` |
| **Applied** | yes |



---

## Synthesis

### VI81vNZkn: Keep Goal Lineage Parsing On One Canonical Path

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

### t51RzLxYE: Scope Planning Diagnostics To Transition-Relevant Voyages

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

### RhBv18151: Keep Goal Lineage Parsing On One Canonical Path

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

### 4FQ6HqEi3: Canonical Scope Contracts Need Explicit Activation Markers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Introducing canonical scope-lineage validation while existing voyages still contain prose-only scope sections |
| **Insight** | New planning contracts are safest when they activate off explicit markers like `SCOPE-*`; otherwise doctor treats historical prose as invalid structure and turns a targeted validator into migration noise. |
| **Suggested Action** | Define an activation marker whenever a new authored planning contract is introduced, and only enforce the stricter validator once that marker appears in the relevant artifacts. |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/cli/commands/diagnostics/doctor/checks/voyages.rs`, PRD and SRS scope sections |
| **Linked Knowledge IDs** | 1vyKOWjfv |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | yes |

### NrHGiwBLb: Preserve Empty Markdown Table Cells In Planning Parsers

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

### 8q3RNMDp8: Keep Goal Lineage Parsing On One Canonical Path

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

### kzvOierm7: Lineage Surfaces Need IDs And Prose Together

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering planning lineage for human review while keeping canonical IDs visible for machine-checkable contracts |
| **Insight** | Planning surfaces become much more reviewable when each lineage row carries the canonical ID, the authored prose, and the parent/child disposition context together; token-only output hides meaning, while prose-only output hides the contract. |
| **Suggested Action** | For future lineage read models, project one canonical row format that combines identifiers with authored descriptions and relation context before the CLI renderer touches it. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/epic/show.rs`, `src/cli/commands/management/voyage/show.rs` |
| **Linked Knowledge IDs** | 1vyKXvBA1 |
| **Score** | 0.79 |
| **Confidence** | 0.90 |
| **Applied** | yes |

