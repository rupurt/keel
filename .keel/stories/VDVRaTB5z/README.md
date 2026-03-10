---
id: VDVRaTB5z
title: Render Dynamic Lanes In Flow
type: feat
status: backlog
created_at: 2026-03-10T15:27:51
updated_at: 2026-03-10T15:29:38
scope: VDVPODBXF/VDVPUCtqS
index: 5
---

# Render Dynamic Lanes In Flow

## Summary

Expose the effective topology in `flow` by rendering configured lane cards and ordering from the resolved lane definitions.

## Acceptance Criteria

- [ ] [SRS-08/AC-01] `keel flow` renders configured lanes in deterministic `priority` order and counts only work selected by each lane's resolved sources. <!-- verify: vhs tapes/workflow-topology-flow.tape, SRS-08:start:end, proof: ac-1.log-->
