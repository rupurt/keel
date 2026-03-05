---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9rycE/KNOWLEDGE.md
created_at: 2026-03-02T10:34:57
---

### qUF0j5GEl: Queue-policy facades prevent decision/rendering drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Queue classifications were being consumed directly by multiple modules (`next`, `flow/bottleneck`, and `state_machine/flow`) with repeated policy calls. |
| **Insight** | A small read-model facade (`read_model::queue_policy`) creates one consumption surface for policy outputs while keeping source-of-truth thresholds in `policy::queue`. |
| **Suggested Action** | Add architecture contracts for policy-facade usage whenever policy semantics are consumed by multiple adapters or decision paths. |
| **Applies To** | `src/read_model/queue_policy.rs`, `src/next/algorithm.rs`, `src/flow/bottleneck.rs`, `src/state_machine/flow.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuwSPf |
| **Observed At** |  |
| **Score** | 0.86 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfgC4` |
