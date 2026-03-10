---
created_at: 2026-03-10T14:47:57
---

# Reflection - Define Core Role Templates

## Knowledge

## Observations

Keeping the registry in `src/read_model/role_context.rs` isolated the new template data from queue routing and guidance rendering, which keeps the follow-on injection work small and deterministic.
