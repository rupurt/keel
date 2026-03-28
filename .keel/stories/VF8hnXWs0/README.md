---
# system-managed
id: VF8hnXWs0
status: backlog
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:10:55
# authored
title: Explain Next Routing With Canonical Role Context
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkVGjy
index: 2
---

# Explain Next Routing With Canonical Role Context

## Summary

Teach `keel next` to explain its routing decision from canonical topology and role-context data so role-scoped pulls are legible instead of magical.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `keel next --explain` surfaces the resolved lane, queue type, and role-context contract for the selected role. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] The explanation is derived from workflow-topology and role-context projections rather than duplicated local heuristics. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-NFR-01/AC-01] Explanation output does not change the underlying next-decision behavior. <!-- verify: manual, SRS-NFR-01:start:end -->
