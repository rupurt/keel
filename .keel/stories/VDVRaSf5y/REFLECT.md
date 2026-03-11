# Reflection - Validate Topology Selectors And Overlap

## Observations

The config-driven topology needed a robust validation layer to prevent ambiguous work routing. Overlapping lane sources would cause items to appear in multiple lanes, confusing both humans and agents.

## Knowledge

### T001: Workflow Topology Integrity

| Field | Value |
|-------|-------|
| **Insight** | Cross-lane overlap in work selectors leads to non-deterministic queue rendering and pull behavior. |
| **Suggested Action** | Always use unique selectors or explicit exclusions to ensure each work status maps to exactly one lane. |
