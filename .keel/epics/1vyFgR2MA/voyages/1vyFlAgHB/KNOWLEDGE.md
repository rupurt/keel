---
created_at: 2026-03-05T17:47:09
---

# Knowledge - 1vyFlAgHB

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Replace Epic Goal CLI With Problem Input (1vyGZd1to)

### 1vyIq5M2c: Verify Annotation Chains Only Materialize One Requirement Token

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | when one acceptance criterion is linked to both a functional SRS requirement and an SRS-NFR requirement |
| **Insight** | The verify-annotation parser keeps only one requirement phase token per AC, so the last `SRS-*:phase` entry controls voyage evidence-chain checks |
| **Suggested Action** | Split evidence-chain phases across separate ACs or put the functional requirement token last when a line carries both SRS and SRS-NFR references |
| **Applies To** | src/infrastructure/verification/parser.rs, .keel/stories/*/README.md |
| **Applied** | yes |



---

## Story: Keep Fresh Epic Scaffolds Doctor Clean (1vyGZet0Z)

### 1vyKD7naI: Keep scaffold templates compliant with day-zero doctor contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Changing epic creation inputs made freshly generated PRDs fail doctor even though the validator logic was correct. |
| **Insight** | If newly scaffolded planning artifacts are expected to be immediately doctor-clean, the fix belongs in the template seed content rather than in weaker validation rules. |
| **Suggested Action** | When creation inputs or placeholder semantics change, regenerate a fresh artifact in tests and run doctor against it before changing any diagnostic gates. |
| **Applies To** | `templates/epic/[name]/PRD.md`, `src/cli/commands/management/epic/new.rs`, doctor scaffold checks |
| **Applied** | yes |



---

## Synthesis

### fjNA0XzGw: Verify Annotation Chains Only Materialize One Requirement Token

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | when one acceptance criterion is linked to both a functional SRS requirement and an SRS-NFR requirement |
| **Insight** | The verify-annotation parser keeps only one requirement phase token per AC, so the last `SRS-*:phase` entry controls voyage evidence-chain checks |
| **Suggested Action** | Split evidence-chain phases across separate ACs or put the functional requirement token last when a line carries both SRS and SRS-NFR references |
| **Applies To** | src/infrastructure/verification/parser.rs, .keel/stories/*/README.md |
| **Linked Knowledge IDs** | 1vyIq5M2c |
| **Score** | 0.75 |
| **Confidence** | 0.90 |
| **Applied** | yes |

### uF16vu0Pp: Keep scaffold templates compliant with day-zero doctor contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Changing epic creation inputs made freshly generated PRDs fail doctor even though the validator logic was correct. |
| **Insight** | If newly scaffolded planning artifacts are expected to be immediately doctor-clean, the fix belongs in the template seed content rather than in weaker validation rules. |
| **Suggested Action** | When creation inputs or placeholder semantics change, regenerate a fresh artifact in tests and run doctor against it before changing any diagnostic gates. |
| **Applies To** | `templates/epic/[name]/PRD.md`, `src/cli/commands/management/epic/new.rs`, doctor scaffold checks |
| **Linked Knowledge IDs** | 1vyKD7naI |
| **Score** | 0.87 |
| **Confidence** | 0.95 |
| **Applied** | yes |

