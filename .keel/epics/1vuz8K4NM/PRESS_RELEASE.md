# PRESS RELEASE: Flow Coherence Refactor

## FOR IMMEDIATE RELEASE

**Keel introduces Flow Coherence Refactor: a unified queue policy, transition enforcement path, and canonical schema migration for predictable agentic delivery.**

**LUNAR BASE ALPHA** — Keel today announced Flow Coherence Refactor, a major reliability and maintainability upgrade for teams managing work through human and agent collaboration.

### The Problem
Teams were seeing inconsistent operational guidance because queue policy logic, transition enforcement, and schema handling were implemented in multiple places. The result was drift between `next` and `flow`, mismatches between runtime and diagnostics behavior, and continued dependence on legacy terminology and field variants.

### The Solution
Flow Coherence Refactor unifies decision logic and governance paths around canonical rules. Queue thresholds are centralized, transition checks are enforced through one gate-driven path for both runtime and doctor reporting, and a hard migration removes legacy schema compatibility so canonical state and field names are the only supported contract.

### Key Features
- **Unified queue policy module** used by both `next` and `flow`, including a strict human-mode boundary that does not return implementation work.
- **Shared transition enforcement service** used by story/voyage commands and diagnostics to keep strict and reporting behavior coherent.
- **Hard schema migration and cleanup** covering canonical state names, `completed_at` normalization, and removal of compatibility aliases.

### Customer Quote
"Flow Coherence Refactor made keel predictable again. Our planners and agents now see the same rules, and our diagnostics point to real issues instead of workflow ambiguity."

---
**About Keel**: Keel is an agentic SDLC management system designed to minimize drift through planning, execution, and verification.
