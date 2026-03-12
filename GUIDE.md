# Business Automation Guide

This guide explains the current recurring-work automation path in Keel:

1. Author a routine bundle.
2. Review whether it is due with `keel next --role operator` and `keel flow`.
3. Materialize due work with `keel pulse`.
4. Deliver the resulting story through the normal board workflow.

The guide is intentionally strict about what Keel supports today. It does not
describe background daemons, legacy flags, or speculative workflow variants.

## Core Model

- A routine is a recurring-work contract stored as one human-editable file:
  `.keel/routines/<id>/README.md`.
- The routine bundle contains YAML frontmatter plus a `# Blueprint` body.
- The blueprint body is canonical authored content. When `keel pulse`
  materializes work, that blueprint is copied into the new story.
- A routine targets either an epic (`EPIC-ID`) or a voyage inside an epic
  (`EPIC-ID/VOYAGE-ID`) through `target-scope`.

Use the current canonical command names:

- `keel routine new`
- `keel routine list`
- `keel routine show`
- `keel next --role operator`
- `keel flow`
- `keel pulse`

`keel next` now requires `--role`. Use `keel next --role manager` for
management-lane review and `keel next --role operator` for delivery review. Do
not use removed legacy flags such as `--agent` or `--human`.

## Author A Routine

Create the routine bundle with `keel routine new`:

```bash
keel routine new "Weekly Pipeline Review" \
  --target-scope EPIC-ID/VOYAGE-ID \
  --cadence cron="0 9 * * 1" \
  --cadence timezone="America/Los_Angeles"
```

This creates a single bundle at `.keel/routines/<id>/README.md`.

The current authored contract looks like this:

```md
---
id: weekly-pipeline-review
title: Weekly Pipeline Review
cadence:
  cron: 0 9 * * 1
  timezone: America/Los_Angeles
target-scope: EPIC-ID/VOYAGE-ID
created_at: 2026-03-11T00:00:00
updated_at: 2026-03-11T00:00:00
---

# Blueprint

- Describe the recurring trigger or review point.
- Outline the work to perform.
- Capture the expected output or exit criteria.
```

Routine authoring rules:

- `cadence` is stored as a YAML mapping.
- The current scheduling path interprets `cadence.cron` and
  `cadence.timezone`.
- Extra cadence keys may be stored, but they should be treated as opaque unless
  another implemented surface explicitly consumes them.
- `target-scope` must point at existing board scope. Use `EPIC-ID/VOYAGE-ID`
  when you want pulse to create work inside a specific voyage.
- Keep the blueprint body actionable. It becomes the starting content for each
  materialized story.

Use these read surfaces after authoring:

```bash
keel routine list
keel routine show weekly-pipeline-review
```

## Review Schedule State Before Materializing Work

Keel exposes routine schedule state in two places before you run automation.

### `keel next --role operator`

`keel next --role operator` remains the delivery pull surface. When routines are
present, it also prints a `Scheduled routines:` section that marks each routine
as:

- `due now`
- `next run ...`
- `invalid cadence: ...`

Use it when you want a compact answer to: "Should I keep working existing
delivery items, or is recurring work due for this scope?"

### `keel flow`

`keel flow` remains the board dashboard. When routines exist, it adds a
`Scheduled Capacity` section that shows:

- due routines that still need `keel pulse`
- upcoming routines that are not actionable yet
- invalid cadence entries that need repair
- due routines already materialized during the current eligible window

This makes recurring automation demand visible before and after a pulse run.

## End-To-End Example

This example uses one recurring review from definition through automation.

1. Create the routine.

```bash
keel routine new "Weekly Pipeline Review" \
  --target-scope EPIC-ID/VOYAGE-ID \
  --cadence cron="0 9 * * 1" \
  --cadence timezone="America/Los_Angeles"
```

2. Confirm the routine bundle and inspect the blueprint.

```bash
keel routine list
keel routine show weekly-pipeline-review
```

3. Review whether the routine is due before creating work.

```bash
keel next --role operator
keel flow --no-color
```

Expected review signals:

- `keel next --role operator` prints the routine as `due now`, `next run ...`,
  or `invalid cadence: ...`.
- `keel flow --no-color` shows `Scheduled Capacity` when routines exist.
- If the routine is already due and no story was created for the current window,
  flow guidance points you to `keel pulse`.

4. Run one automation cycle.

```bash
keel pulse
```

Or, for scheduler-friendly output:

```bash
keel pulse --json
```

5. Review the result.

- If the routine was due, `keel pulse` creates one story inside the targeted
  scope and reports it as created.
- If the same eligible window already produced a story, `keel pulse` skips it
  instead of duplicating work.
- If cadence is malformed, that routine is reported as invalid/deferred while
  the rest of the cycle continues.

6. Return to normal delivery work.

```bash
keel next --role operator
keel flow --no-color
```

After a successful pulse run, flow can report that the due window was already
materialized, and the resulting story enters the normal scoped delivery queue.

## Running Pulse From Cron Or Systemd

Keel does not ship a daemon, hosted scheduler, or always-on worker. If you want
automation, invoke `keel pulse` from an external scheduler while the working
directory points at the board root.

Example cron entry:

```cron
*/15 * * * * cd /path/to/board && /path/to/keel pulse >> /var/log/keel-pulse.log 2>&1
```

Example systemd service and timer:

```ini
# /etc/systemd/system/keel-pulse.service
[Unit]
Description=Keel pulse run

[Service]
Type=oneshot
WorkingDirectory=/path/to/board
ExecStart=/path/to/keel pulse --json
```

```ini
# /etc/systemd/system/keel-pulse.timer
[Unit]
Description=Run Keel pulse every 15 minutes

[Timer]
OnCalendar=*:0/15
Persistent=true

[Install]
WantedBy=timers.target
```

Operational expectations:

- Repeated runs are safe. Pulse is idempotent per routine eligible window.
- Running pulse more frequently than the routine cadence is allowed; duplicate
  work is skipped.
- Keep scheduler logs. Human and JSON pulse output are both intended to be
  reviewable.
- Invalid routine cadence does not require the whole pulse cycle to fail.
  Repair the affected routine and rerun pulse later.

## Supported Boundaries

What Keel supports today:

- one-file routine bundles under `.keel/routines/<id>/README.md`
- temporal review in `keel next --role operator`
- scheduled automation visibility in `keel flow`
- non-interactive materialization with `keel pulse`
- cron/systemd invoking `keel pulse`

What Keel does not support in this workflow:

- a built-in daemon, hosted scheduler, or background service
- materializing work from `keel next` or `keel flow`
- legacy `keel next --agent` or `keel next --human` flags
- alternate routine lifecycle states beyond the authored bundle itself
- unsupported cadence schemas or undocumented fallback command names

If you need a different automation behavior, plan it as new work instead of
assuming runtime compatibility or hidden fallback paths.
