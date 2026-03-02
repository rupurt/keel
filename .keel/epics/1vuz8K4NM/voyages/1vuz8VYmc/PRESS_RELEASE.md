# PRESS RELEASE: Policy Module and Queue Semantics

## Overview

## Narrative Summary
### Update Architecture and Command Docs for Queue Policy
Update architecture and command documentation so queue policy thresholds, derivation order, and human/agent boundaries are explicitly consistent with implemented behavior.

### Align Flow Derivation Bottleneck Messaging and Tests
Align flow-state derivation and bottleneck messaging with the same queue policy used by `next` so thresholds and state labels do not drift.

### Refactor Next and Flow to Use Queue Policy
Refactor queue-selection and flow-health paths to consume the shared policy APIs instead of local literals and ad-hoc ordering logic.

### Define Queue Policy Module and Threshold Constants
Create the canonical queue policy module that defines threshold constants, queue categories, and derivation helpers used by both `keel next` and `keel flow`.

### Enforce Human Next Queue Boundary
Enforce the actor boundary that human-mode `keel next` never returns implementation work and only surfaces human-queue actions.

## Key Insights
### Insights from Update Architecture and Command Docs for Queue Policy
### L001: Documentation should encode policy names and boundaries, not just numbers
Using canonical policy constants (`HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD`, `FLOW_VERIFY_BLOCK_THRESHOLD`) in architecture docs keeps behavior and narrative aligned through future threshold changes.

### L002: Drift tests should pin command docs to mode contracts
Asserting README/help text for `keel next` human vs agent semantics prevents subtle messaging drift that can reintroduce queue-boundary confusion.

### Insights from Align Flow Derivation Bottleneck Messaging and Tests
### L001: Policy categories should drive messaging, not only hard-stop thresholds
Using `VerificationQueueCategory` for flow summaries captures `attention`, `human-blocked`, and `flow-blocked` states explicitly, which keeps user-facing guidance aligned with `next` behavior.

### L002: Cross-module boundary tests catch semantic drift early
Asserting `next` human decisions and flow summaries on threshold boundaries (`<5`, `>=5`, `>20`) prevents silent divergence when queue-policy semantics evolve.

### Insights from Refactor Next and Flow to Use Queue Policy
### L001: Decision logic should consume policy intent, not raw counts
Pulling research/planning presence checks and draft-voyage classification into queue policy helpers keeps `next` and flow behavior aligned and reduces drift risk.

### L002: Ordering rules are policy too
Treating backlog ordering as a named policy comparator makes prioritization deterministic across call sites and easier to regression test.

### Insights from Define Queue Policy Module and Threshold Constants
### L001: Queue policy drift prevention
Centralizing queue thresholds and classification helpers into one module removes duplicated literals and keeps `next`, flow bottleneck summaries, and flow state derivation coherent.

### L002: Threshold semantics must be explicit
A small naming mismatch (`>=` vs `>`) can change behavior materially. Encoding policy through category helpers makes boundary semantics clear and testable.

### Insights from Enforce Human Next Queue Boundary
### L001: Queue boundaries should be enforced in the decision algorithm
Separating human and agent execution paths in `calculate_next` prevents policy drift and ensures human mode cannot accidentally surface implementation work.

### L002: Mixed-state regression tests are required for queue policy guarantees
Tests that combine in-progress and backlog implementation items with human queues catch boundary leaks that isolated single-queue tests can miss.

## Verification Proof
### Proof for Define Queue Policy Module and Threshold Constants
- [ac-01-03.txt](../../../../stories/1vuz963fU/EVIDENCE/ac-01-03.txt)

