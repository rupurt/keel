# Flow Integration - Software Design Description

> keel next mission-awareness, keel flow mission progress, CHARTER.md goal parsing

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage closes the autonomous loop by making `keel next` and `keel flow`
mission-aware. When an active mission exists and the work queue is empty, the
system recommends creating the next work unit instead of returning nothing.
This is the key anti-false-halt mechanism.

## Architecture

### Modified Files

| File | Change |
|------|--------|
| `src/cli/commands/flow/next.rs` | Add mission-aware fallback when no stories ready |
| `src/cli/commands/flow/flow.rs` | Add mission progress section to flow output |
| `src/cli/commands/management/mission/refine.rs` | CHARTER.md completeness analysis |

### New Files

| File | Purpose |
|------|---------|
| `src/infrastructure/validation/charter.rs` | CHARTER.md parsing (if not already created in V3) |

## Components

### Mission-Aware Next

The `keel next --agent` decision tree becomes:

```
1. Check for ready stories → return story (existing behavior)
2. Check for active missions with unmet goals → return mission recommendation
3. Return empty (no work, no mission)
```

The recommendation includes:
- Mission ID and title
- Unmet goal summary
- Suggested action: "create bearing" (if research needed), "create epic" (if planning needed), "create voyage" (if decomposition needed)

### Flow Mission Summary

When active missions exist, `keel flow` prepends a mission section:

```
Mission: Build Search CLI (active)
  Goals: 2/3 board goals met, 1 metric goal (human)
  Epics: 2 active, 1 done
  Bearings: 1 in-progress
```

### CHARTER.md Completeness Analysis

The refine command checks:
1. Goals section exists with at least one MG-XX row
2. Each MG-XX has non-empty Description and Verification columns
3. Constraints section has at least one bullet
4. Halting Rules section has at least one bullet
5. At least one `board:` verification goal exists (machine-checkable baseline)

Missing sections generate targeted questions for the harness to ask.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Mission recommendation in `keel next` returns suggestion, not action | Harness decides what to create | Respects harness autonomy |
| Flow mission section is prepended, not appended | Mission is highest-level context | Scans top-down |
| Refine requires at least one `board:` goal | Must have machine-checkable baseline | Prevents purely manual missions from using auto-halt |
