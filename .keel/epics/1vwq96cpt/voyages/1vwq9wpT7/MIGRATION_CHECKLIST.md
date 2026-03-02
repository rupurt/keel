# Migration Completion Checklist

> Maintainer checklist for completing and rolling out CLI interface migration work in voyage `1vwq9wpT7`.

## Completion Criteria

- [ ] Story [`1vwqCf53S`](../../../../stories/1vwqCf53S/README.md) is `done` and main dispatch routes through interface adapters.
- [ ] Story [`1vwqCfdUl`](../../../../stories/1vwqCfdUl/README.md) is `done` with architecture contract checks enforcing adapter boundaries.
- [ ] Story [`1vwqCffzr`](../../../../stories/1vwqCffzr/README.md) is `done` with command regression coverage for `next`, `flow`, and lifecycle transitions.
- [ ] Story [`1vwqCfeFP`](../../../../stories/1vwqCfeFP/README.md) is `done` and this checklist is published for maintainers.
- [ ] Voyage [`1vwq9wpT7`](README.md) reports all stories complete and can be promoted to `done`.

## Verification Gates

Run all gates from repository root before final acceptance:

1. `just quality`
2. `just test`
3. `just keel doctor`

Additional targeted migration checks:

1. `cargo test architecture_contract_tests::`
2. `cargo test command_regression_tests::`
3. `just keel next --agent` and `just keel flow` produce consistent queue guidance for current board state

## Rollout Order

1. Deploy adapter and orchestration refactors (stories `1vwqCf53S`, `1vwqCfdUl`).
2. Validate parity using regression suite (story `1vwqCffzr`).
3. Run full gates (`quality`, `test`, `doctor`) on the release candidate branch.
4. Submit and accept remaining voyage stories, then mark voyage `done`.
5. Regenerate board summaries with `just keel generate` and verify updated status output.

## Maintainer Sign-Off

- [ ] Engineering owner approved migration parity and adapter boundary constraints.
- [ ] Human reviewer confirmed manual verification evidence for all `needs-human-verification` stories.
- [ ] Voyage marked `done` and epic progress updated.

## Deferred Follow-Up (Post-Epic)

- [ ] Normalize `story accept` date formatting behavior (tracked after this epic per current execution plan).
