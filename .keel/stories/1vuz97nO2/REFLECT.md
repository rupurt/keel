### L001: Decision logic should consume policy intent, not raw counts
Pulling research/planning presence checks and draft-voyage classification into queue policy helpers keeps `next` and flow behavior aligned and reduces drift risk.

### L002: Ordering rules are policy too
Treating backlog ordering as a named policy comparator makes prioritization deterministic across call sites and easier to regression test.
