# Enhanced Governance and Dependency Visibility - System Design Document

> Strengthen ADR blocking feedback and implement visual dependency rendering in flow.

**Epic:** [1vv7YWzw2](../../README.md) | **SRS:** [SRS.md](SRS.md)

## System Overview

This voyage enhances the visibility of governance and dependencies across the CLI.

## Components

### Governance Feedback
- Update `src/next/format.rs` to include detailed ADR blocking messages when in agent mode.
- Ensure the specific ADR ID and title are surfaced.

### Dependency Modeling
- Utilize `src/traceability.rs` to derive dependency chains.
- Create a new module `src/flow/layout.rs` or similar to handle the visual positioning of stories based on their dependencies.

### Flow Rendering
- Update `src/flow/display.rs` to render visual indicators (e.g., arrows or indented chains) for dependencies.
- Highlight "Blocking" stories that are preventing downstream work.

## Constraints & Considerations

- Rendering should be clean and not clutter the existing dashboard.
- Ensure that circular dependencies are handled gracefully (though they should be rare).
