# VOYAGE REPORT: Implement Mission Stack Context Surfaces

## Voyage Metadata
- **ID:** VGe8Ad6Jy
- **Epic:** VGe7mCcFW
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Surface Mission Stack In Turn Next And Mission Status
- **ID:** VGe8Mf0Jg
- **Status:** done

#### Summary
Thread the new Mission Stack projection through the canonical operator surfaces.
`turn`, `next`, and `mission next --status` should explain local stack context,
current gating, and cross-repo dependencies while staying unchanged when no
stack is active.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] `keel turn` renders Mission Stack id, branch, member role, mode, checkpoint, and foreign-execution state in both text and JSON surfaces. <!-- verify: cargo test -p keel turn_surfaces_mission_stack_context_in_text_and_json, SRS-02:start:end, proof: ac-1.log -->
- [x] [SRS-03/AC-01] `keel next` emits stack-aware block or yield decisions when local execution is forbidden by the active Mission Stack state. <!-- verify: cargo test -p keel next_emits_stack_aware_decisions, SRS-03:start:end, proof: ac-2.log -->
- [x] [SRS-04/AC-01] `keel mission next --status` reports linked member missions, pending negotiations, and waiting receipts for the local stack. <!-- verify: cargo test -p keel mission_next_status_surfaces_stack_dependencies, SRS-04:start:end, proof: ac-3.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/VGe8Mf0Jg/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VGe8Mf0Jg/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VGe8Mf0Jg/EVIDENCE/ac-3.log)

### Load Mission Stack Projection From Local Manifest
- **ID:** VGe8MfYJW
- **Status:** done

#### Summary
Add the first repo-local Mission Stack read model. Keel should be able to load
optional stack metadata from `.keel/stacks/<id>/manifest.yaml`, combine it with
current git/worktree state, and produce a deterministic local projection for
other adapters to consume without modifying the core board model.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Keel loads a Mission Stack projection from repo-local manifest metadata and derives local member role, stack mode, checkpoint, linked missions, and receipt state. <!-- verify: cargo test mission_stack_loads_projection_from_manifest_and_git_state --lib, SRS-01:start, proof: ac-1.log -->
- [x] [SRS-01/AC-02] The projection derives current branch and checkout/worktree metadata needed for later guardrails. <!-- verify: cargo test mission_stack_derives_branch_and_worktree_state --lib, SRS-01:end, proof: ac-2.log -->
- [x] [SRS-NFR-01/AC-01] Repos without stack metadata remain a no-op and preserve current single-repo behavior. <!-- verify: cargo test mission_stack_absent_repo_is_noop --lib, SRS-NFR-01:start:end, proof: ac-3.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/VGe8MfYJW/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VGe8MfYJW/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VGe8MfYJW/EVIDENCE/ac-3.log)

### Enforce Mission Stack Diagnostics And Foreign Worktree Guards
- **ID:** VGe8Mg4Jj
- **Status:** done

#### Summary
Add the first enforcement layer for Mission Stack protocol rules. `doctor`
should diagnose wrong-branch, checkpoint, foreign-worktree, and stack-close
leftover violations, and execution surfaces should refuse unsupported foreign
checkout paths instead of silently proceeding.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] `keel doctor` reports Mission Stack violations for wrong branch, missing checkpoint acknowledgment, and unsupported foreign execution state. <!-- verify: cargo test -p keel doctor_reports_mission_stack_violations, SRS-05:start, proof: ac-1.log -->
- [x] [SRS-05/AC-02] Closed stacks with leftover managed foreign worktrees are reported conservatively instead of being deleted automatically. <!-- verify: cargo test -p keel doctor_reports_closed_stack_worktree_leftovers, SRS-05:end, proof: ac-2.log -->
- [x] [SRS-06/AC-01] Stack-aware adapter output exposes deterministic machine-readable fields for Mission Stack context and gating decisions. <!-- verify: cargo test -p keel mission_stack_surfaces_expose_deterministic_json, SRS-06:start:end, proof: ac-3.log -->
- [x] [SRS-NFR-02/AC-01] Stack-aware surfaces preserve repo-local heartbeat semantics and do not redefine pacemaker state. <!-- verify: cargo test -p keel mission_stack_surfaces_preserve_heartbeat_semantics, SRS-NFR-02:start:end, proof: ac-4.log -->
- [x] [SRS-NFR-03/AC-01] Foreign-worktree guardrails fail safe by blocking unsupported execution without mutating uncertain checkouts. <!-- verify: cargo test -p keel mission_stack_guardrails_fail_safe, SRS-NFR-03:start:end, proof: ac-5.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/VGe8Mg4Jj/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VGe8Mg4Jj/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VGe8Mg4Jj/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/VGe8Mg4Jj/EVIDENCE/ac-4.log)
- [ac-5.log](../../../../stories/VGe8Mg4Jj/EVIDENCE/ac-5.log)


