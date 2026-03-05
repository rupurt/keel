---
created_at: 2026-02-24T17:25:15
---

# Reflection - Route Doctor Checks Through Gate Evaluators

### Note 001: Doctor parity is strongest when diagnostics consume enforcement outputs directly
Using `enforce_transition(..., EnforcementPolicy::REPORTING)` in doctor checks keeps transition/completion findings aligned with runtime gate rules while preserving non-blocking reporting semantics.

### Note 002: Reporting semantics are testable via the blocking subset
Asserting `blocking_problems.is_empty()` in parity tests provides a clear contract that doctor visibility does not inherit runtime blocking behavior.
