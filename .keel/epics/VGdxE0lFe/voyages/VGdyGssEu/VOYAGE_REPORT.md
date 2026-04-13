# VOYAGE REPORT: Specify Managed Foreign Worktree Lifecycle

## Voyage Metadata
- **ID:** VGdyGssEu
- **Epic:** VGdxE0lFe
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 1/1 stories complete

## Implementation Narrative
### Enforce Managed Foreign Worktree Lifecycle For Stack Execution
- **ID:** VGdyhbpcW
- **Status:** done

#### Summary
Define the first foreign-reactor isolation contract: outside execution in
another member repo must use a managed git worktree on the stack branch, validate
ownership before work starts, and clean up or report leftovers when the stack
closes.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] The lifecycle requires foreign reactor execution to happen in a managed worktree rather than the member repo's primary checkout. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] The lifecycle validates `stack/<id>` branch or approved stack-derived head state before foreign execution begins. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] The lifecycle defines create, reuse, and inspection behavior for managed foreign worktrees while a stack is open. <!-- verify: manual, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] The lifecycle defines stack-close garbage collection and fail-safe reporting for ambiguous leftovers. <!-- verify: manual, SRS-04:start:end, proof: ac-4.log-->
- [x] [SRS-05/AC-01] The lifecycle names the command and hook enforcement points that reject unsupported foreign execution. <!-- verify: manual, SRS-05:start:end, proof: ac-5.log-->
- [x] [SRS-NFR-01/AC-01] Managed worktree operations avoid perturbing the member repo's primary checkout. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-6.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VGdyhbpcW/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VGdyhbpcW/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VGdyhbpcW/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/VGdyhbpcW/EVIDENCE/ac-4.log)
- [ac-5.log](../../../../stories/VGdyhbpcW/EVIDENCE/ac-5.log)
- [ac-6.log](../../../../stories/VGdyhbpcW/EVIDENCE/ac-6.log)


