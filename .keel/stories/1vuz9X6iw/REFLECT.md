# Reflection - Route Doctor Checks Through Gate Evaluators

### L001: Doctor parity is strongest when diagnostics consume enforcement outputs directly
Using `enforce_transition(..., EnforcementPolicy::REPORTING)` in doctor checks keeps transition/completion findings aligned with runtime gate rules while preserving non-blocking reporting semantics.

### L002: Reporting semantics are testable via the blocking subset
Asserting `blocking_problems.is_empty()` in parity tests provides a clear contract that doctor visibility does not inherit runtime blocking behavior.
