---
created_at: 2026-03-06T08:46:32
---

# Knowledge - 1vyWNL000

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Link Tape Evidence Into Verification Manifests (1vyWSD000)

### 1vyYIj000: Dogfood Evidence Needs Its Own Board

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When a dogfood or e2e harness needs a real keel story to own evidence while the exercised workspace must remain disposable. |
| **Insight** | Persisting tape artifacts into the primary `.keel` or the disposable scenario workspace creates contract drift: the primary board stops being immutable, while the resettable workspace loses durable proof ownership. A separate artifact board keeps ownership, manifests, and evidence stable without polluting the runtime board. |
| **Suggested Action** | For future dogfood flows, separate execution state from evidence ownership. Keep the executable workspace resettable and route rendered artifacts into a dedicated keel board whose stories reference the canonical scenario sources. |
| **Applies To** | testdata/dogfood/**, src/infrastructure/dogfood_*, src/infrastructure/verification/** |
| **Applied** |  |



---

## Story: Create Secondary Dogfood Workspace (1vyWSB000)

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

## Story: Author Epic Workflow Dogfood Tapes (1vyWSC000)

### 1vyXcz000: Use hidden setup blocks and dynamic ID discovery in VHS planning flows

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Authoring VHS tapes for keel workflows that create new epics, voyages, and stories on a reset fixture board |
| **Insight** | The readable part of the tape should stay focused on the operator-facing workflow, while markdown authoring and ID plumbing happen in `Hide` blocks using `latest_id` discovery instead of fixed IDs. |
| **Suggested Action** | Keep visible commands to the user journey, generate authored artifacts in hidden heredocs, and derive IDs from the fixture state after each create step to preserve repeatability. |
| **Applies To** | `testdata/dogfood/scenarios/*.tape`, `src/infrastructure/dogfood_runner.rs` |
| **Applied** | `epic-flow.tape` |



---

## Story: Author Bearing Workflow Dogfood Tapes (1vyWRk000)

### 1vyXi6000: Author transition-created bearing artifacts after the lifecycle step

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Building deterministic bearing demos or automation around `bearing survey` and `bearing assess` |
| **Insight** | The bearing lifecycle commands create `SURVEY.md` and `ASSESSMENT.md` themselves, so authoring those files before the transition causes hard failures; the correct flow is transition first, then fill the generated artifact. |
| **Suggested Action** | In tapes or scripts, treat `bearing survey` and `bearing assess` as the scaffold-creation step, then write authored content into the generated files before continuing. |
| **Applies To** | `testdata/dogfood/scenarios/bearing-flow.tape`, `templates/bearings/*.md` |
| **Applied** | `bearing-flow.tape` |



---

## Story: Build Tape Runner And Reset Harness (1vyWRj000)

### 1vyWX1Qh7: Timebox External Verification Runners And Emit Log Paths

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | when keel delegates acceptance proofs to external tools such as VHS or future semantic judges |
| **Insight** | External verifier processes can hang without producing useful stderr, so the runner must enforce a timeout and always persist a log path or the queue stalls without actionable failure context. |
| **Suggested Action** | Wrap external verification tools in an explicit timeout, keep the failing workspace/tape/output paths in the error, and write a run log even on failure. |
| **Applies To** | `src/infrastructure/vhs.rs`, `src/infrastructure/dogfood_runner.rs`, `testdata/dogfood/scenarios/*.tape` |
| **Applied** | yes |



---

## Synthesis

### TmDU0CTXN: Dogfood Evidence Needs Its Own Board

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When a dogfood or e2e harness needs a real keel story to own evidence while the exercised workspace must remain disposable. |
| **Insight** | Persisting tape artifacts into the primary `.keel` or the disposable scenario workspace creates contract drift: the primary board stops being immutable, while the resettable workspace loses durable proof ownership. A separate artifact board keeps ownership, manifests, and evidence stable without polluting the runtime board. |
| **Suggested Action** | For future dogfood flows, separate execution state from evidence ownership. Keep the executable workspace resettable and route rendered artifacts into a dedicated keel board whose stories reference the canonical scenario sources. |
| **Applies To** | testdata/dogfood/**, src/infrastructure/dogfood_*, src/infrastructure/verification/** |
| **Linked Knowledge IDs** | 1vyYIj000 |
| **Score** | 0.89 |
| **Confidence** | 0.90 |
| **Applied** |  |

### pX0vG6XWV: Verify Annotation Chains Only Materialize One Requirement Token

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

### AgIipQAMg: Use hidden setup blocks and dynamic ID discovery in VHS planning flows

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Authoring VHS tapes for keel workflows that create new epics, voyages, and stories on a reset fixture board |
| **Insight** | The readable part of the tape should stay focused on the operator-facing workflow, while markdown authoring and ID plumbing happen in `Hide` blocks using `latest_id` discovery instead of fixed IDs. |
| **Suggested Action** | Keep visible commands to the user journey, generate authored artifacts in hidden heredocs, and derive IDs from the fixture state after each create step to preserve repeatability. |
| **Applies To** | `testdata/dogfood/scenarios/*.tape`, `src/infrastructure/dogfood_runner.rs` |
| **Linked Knowledge IDs** | 1vyXcz000 |
| **Score** | 0.82 |
| **Confidence** | 0.93 |
| **Applied** | `epic-flow.tape` |

### kEfwospLA: Author transition-created bearing artifacts after the lifecycle step

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Building deterministic bearing demos or automation around `bearing survey` and `bearing assess` |
| **Insight** | The bearing lifecycle commands create `SURVEY.md` and `ASSESSMENT.md` themselves, so authoring those files before the transition causes hard failures; the correct flow is transition first, then fill the generated artifact. |
| **Suggested Action** | In tapes or scripts, treat `bearing survey` and `bearing assess` as the scaffold-creation step, then write authored content into the generated files before continuing. |
| **Applies To** | `testdata/dogfood/scenarios/bearing-flow.tape`, `templates/bearings/*.md` |
| **Linked Knowledge IDs** | 1vyXi6000 |
| **Score** | 0.78 |
| **Confidence** | 0.92 |
| **Applied** | `bearing-flow.tape` |

### tcdZH9BjZ: Timebox External Verification Runners And Emit Log Paths

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | when keel delegates acceptance proofs to external tools such as VHS or future semantic judges |
| **Insight** | External verifier processes can hang without producing useful stderr, so the runner must enforce a timeout and always persist a log path or the queue stalls without actionable failure context. |
| **Suggested Action** | Wrap external verification tools in an explicit timeout, keep the failing workspace/tape/output paths in the error, and write a run log even on failure. |
| **Applies To** | `src/infrastructure/vhs.rs`, `src/infrastructure/dogfood_runner.rs`, `testdata/dogfood/scenarios/*.tape` |
| **Linked Knowledge IDs** | 1vyWX1Qh7 |
| **Score** | 0.84 |
| **Confidence** | 0.88 |
| **Applied** | yes |

