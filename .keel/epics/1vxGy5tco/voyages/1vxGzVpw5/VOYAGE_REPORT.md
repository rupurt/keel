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
- **L001: Reuse the structural placeholder detector in runtime gates**
  - Insight: Reusing `first_unfilled_placeholder_pattern` keeps runtime and doctor behavior consistent while avoiding duplicate marker logic.
  - Suggested Action: Add lifecycle gate checks by composing existing structural validators before adding new regex or scanners.
  - Applies To: src/domain/state_machine/gating.rs, src/infrastructure/validation/structural.rs
  - Category: architecture


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
- **L001: Assert check identity and severity for hard-cutover gates**
  - Insight: Message-only assertions can pass even if a hard error silently downgrades to a warning; check-id plus severity assertions prevent this regression class
  - Suggested Action: For each enforcement rule, add at least one integration test that asserts both `check_id` and `severity`
  - Applies To: `src/cli/commands/diagnostics/doctor/mod.rs`, `src/domain/state_machine/gating.rs`
  - Category: testing


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
- **L001: Report Pattern And Severity From One Shared Placeholder Extractor**
  - Insight: A shared unresolved-pattern extractor enables deterministic detection and allows every check to emit the same actionable `pattern: ...` output while enforcing error severity.
  - Suggested Action: Route all new scaffold/default-text checks through the shared extractor and assert severity/message structure in unit tests.
  - Applies To: `src/infrastructure/validation/structural.rs`, `src/cli/commands/diagnostics/doctor/checks/*.rs`
  - Category: code


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
- **L001: Stage-gate scaffold checks to avoid noisy early warnings**
  - Insight: Stage filtering is critical: terminal-only checks avoid penalizing in-progress drafting while still hard-failing review-complete states.
  - Suggested Action: Reuse a shared unresolved-pattern detector and explicitly gate by story stage (`needs-human-verification`, `done`) for terminal coherency rules.
  - Applies To: src/cli/commands/diagnostics/doctor/checks/stories.rs, src/infrastructure/validation/structural.rs
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84nTQ/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84nTQ/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84nTQ/EVIDENCE/ac-2.log)


