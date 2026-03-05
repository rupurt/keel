---
created_at: 2026-02-24T14:08:45
---

### Note 001: Queue boundaries should be enforced in the decision algorithm
Separating human and agent execution paths in `calculate_next` prevents policy drift and ensures human mode cannot accidentally surface implementation work.

### Note 002: Mixed-state regression tests are required for queue policy guarantees
Tests that combine in-progress and backlog implementation items with human queues catch boundary leaks that isolated single-queue tests can miss.
