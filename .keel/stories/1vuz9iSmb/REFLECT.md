# Reflection - Purge Legacy Terminology from CLI and Documentation

### L001: Canonical vocabulary is strongest when enforced at parse boundaries
Using explicit CLI value parsers for story/epic/voyage status filters prevents legacy terms from slipping through and keeps user-facing language aligned with canonical state machines.

### L002: Drift guards should target user-facing surfaces directly
A focused drift test over `README.md`, `ARCHITECTURE.md`, and CLI help source catches deprecated terms reappearing in documentation/help snapshots without over-constraining migration internals.
