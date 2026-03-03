# VOYAGE REPORT: Doctor And Transition Hard Enforcement

## Voyage Metadata
- **ID:** 1vxGzVpw5
- **Epic:** 1vxGy5tco
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Gate Story Submit And Accept On Coherent Artifacts
- **ID:** 1vxH84M8t
- **Status:** done

#### Summary
Enforce submit/accept lifecycle gating so unresolved scaffold/default story and reflection artifacts cannot advance to terminal states.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Story submit is blocked when story README or REFLECT contains unresolved scaffold/default patterns. <!-- verify: cargo test -p keel domain::state_machine::gating::tests::evaluate_story_submit_blocks_unresolved_readme_scaffold, SRS-04:start, proof: ac-1.log -->
- [x] [SRS-04/AC-02] Story accept is blocked on the same coherency violations. <!-- verify: cargo test -p keel domain::state_machine::gating::tests::evaluate_story_accept_blocks_unresolved_reflect_scaffold, SRS-04:end, proof: ac-2.log -->
- [x] [SRS-05/AC-01] Generated report artifacts remain excluded from unresolved-scaffold enforcement scope. <!-- verify: cargo test -p keel domain::state_machine::gating::tests::evaluate_story_accept_ignores_generated_manifest_for_scaffold_gate, SRS-05:start:end, proof: ac-3.log -->

#### Implementation Insights
# Reflection - Gate Story Submit And Accept On Coherent Artifacts

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Reuse the structural placeholder detector in runtime gates

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story submit/accept transitions need coherence checks aligned with doctor enforcement. |
| **Insight** | Reusing `first_unfilled_placeholder_pattern` keeps runtime and doctor behavior consistent while avoiding duplicate marker logic. |
| **Suggested Action** | Add lifecycle gate checks by composing existing structural validators before adding new regex or scanners. |
| **Applies To** | src/domain/state_machine/gating.rs, src/infrastructure/validation/structural.rs |
| **Observed At** | 2026-03-03T18:05:00Z |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** | |

## Observations

The story required both runtime gating and doctor parity. The cleanest path was adding one shared coherence helper in `gating.rs` that inspects only `README.md` and `REFLECT.md`, then proving exclusion behavior with a manifest artifact test.

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84M8t/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84M8t/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84M8t/EVIDENCE/ac-2.log)

### Add Hard Cutover Regression Coverage
- **ID:** 1vxH84jzB
- **Status:** done

#### Summary
Add regression coverage that enforces hard-cutover behavior across doctor and transition gates with no warning-oriented legacy expectations.

#### Acceptance Criteria
- [x] [SRS-06/AC-01] Add regression tests proving doctor and transition paths enforce hard errors for unresolved scaffold/default text. <!-- verify: cargo test -p keel validate_detects_terminal_story_scaffold_text, SRS-06:start, proof: ac-1.log-->
- [x] [SRS-06/AC-02] Replace legacy warning-oriented expectations with hard-failure assertions. <!-- verify: cargo test -p keel evaluate_story_submit_blocks_unresolved_readme_scaffold, SRS-06:continues, proof: ac-2.log-->
- [x] [SRS-06/AC-03] Ensure updated suites remain green under `just quality` and `just test`. <!-- verify: manual, SRS-06:end, proof: ac-3.log -->

#### Implementation Insights
# Reflection - Add Hard Cutover Regression Coverage

## Knowledge

### L001: Assert check identity and severity for hard-cutover gates

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Updating terminal artifact coherence enforcement for doctor and story transitions |
| **Insight** | Message-only assertions can pass even if a hard error silently downgrades to a warning; check-id plus severity assertions prevent this regression class |
| **Suggested Action** | For each enforcement rule, add at least one integration test that asserts both `check_id` and `severity` |
| **Applies To** | `src/cli/commands/diagnostics/doctor/mod.rs`, `src/domain/state_machine/gating.rs` |
| **Observed At** | 2026-03-03T19:30:00Z |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

- Doctor and transition logic already emitted `Severity::Error`; the main gap was regression precision in tests.
- `story record` requires `verify` annotations to exist first, so AC lines without verify comments must be normalized before evidence capture.
- Running `just quality` and `just test` after tightening assertions validated there was no impact to unrelated checks.

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84jzB/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84jzB/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84jzB/EVIDENCE/ac-2.log)

### Escalate Unresolved Scaffold Checks To Doctor Errors
- **ID:** 1vxH84k3U
- **Status:** done

#### Summary
Promote unresolved scaffold/default text findings from warning-level to error-level in covered doctor checks.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Doctor emits errors (not warnings) for unresolved scaffold/default patterns in covered planning/coherency docs. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Doctor error output includes artifact path and offending pattern for remediation. <!-- verify: manual, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Enforcement remains hard-cutover and does not downgrade unresolved scaffold/default findings to warnings. <!-- verify: manual, SRS-01:end, proof: ac-3.log-->

#### Implementation Insights
# Reflection - Escalate Unresolved Scaffold Checks To Doctor Errors

## Knowledge

### L001: Report Pattern And Severity From One Shared Placeholder Extractor

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple doctor checks were independently detecting TODO/tokens and emitting inconsistent warning messages. |
| **Insight** | A shared unresolved-pattern extractor enables deterministic detection and allows every check to emit the same actionable `pattern: ...` output while enforcing error severity. |
| **Suggested Action** | Route all new scaffold/default-text checks through the shared extractor and assert severity/message structure in unit tests. |
| **Applies To** | `src/infrastructure/validation/structural.rs`, `src/cli/commands/diagnostics/doctor/checks/*.rs` |
| **Observed At** | 2026-03-03T05:58:00Z |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

- Converting only severity was not enough; AC-02 needed explicit offending-pattern strings in the rendered doctor message.
- Keeping the old warning-oriented assertions would silently permit rollback, so explicit hard-cutover tests were necessary.

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84k3U/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84k3U/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84k3U/EVIDENCE/ac-2.log)

### Enforce Terminal Story Coherency In Doctor
- **ID:** 1vxH84nTQ
- **Status:** done

#### Summary
Add stage-aware story/reflection coherency checks so unresolved default scaffold text blocks terminal workflow states in diagnostics.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Doctor fails `needs-human-verification` and `done` stories that retain default story scaffold text. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-03/AC-01] Doctor fails `needs-human-verification` and `done` stories that retain default reflection scaffold text. <!-- verify: manual, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-02/AC-02] Non-terminal stories are excluded from these terminal coherency checks. <!-- verify: manual, SRS-02:end, proof: ac-3.log-->

#### Implementation Insights
# Reflection - Enforce Terminal Story Coherency In Doctor

## Knowledge

### L001: Stage-gate scaffold checks to avoid noisy early warnings

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding scaffold/default text diagnostics to doctor checks |
| **Insight** | Stage filtering is critical: terminal-only checks avoid penalizing in-progress drafting while still hard-failing review-complete states. |
| **Suggested Action** | Reuse a shared unresolved-pattern detector and explicitly gate by story stage (`needs-human-verification`, `done`) for terminal coherency rules. |
| **Applies To** | src/cli/commands/diagnostics/doctor/checks/stories.rs, src/infrastructure/validation/structural.rs |
| **Observed At** | 2026-03-03T17:13:33Z |
| **Score** | 0.85 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

The implementation was straightforward once the existing unresolved-pattern helper was reused. The main practical effect is that doctor now surfaces a backlog of previously hidden terminal-story scaffold issues, which matches the hard-enforcement intent and will require a follow-up cleanup pass of existing `.keel` artifacts.

#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84nTQ/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84nTQ/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84nTQ/EVIDENCE/ac-2.log)


