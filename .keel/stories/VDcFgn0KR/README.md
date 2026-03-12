---
id: VDcFgn0KR
title: Routine Board Integration
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:44
operator-signal: 
scope: VDakm8eVW/VDcFd11nc
index: 2
---

# Routine Board Integration

## Summary

Teach the board model and filesystem adapter to discover and persist routine
bundles alongside the existing entity graph.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Board loading discovers routine bundles and exposes them through canonical board structures. <!-- verify: test, SRS-02:start -->
- [ ] [SRS-02/AC-02] Filesystem persistence writes and reloads routine bundles alongside existing entities. <!-- verify: test, SRS-02:end -->
- [ ] [SRS-NFR-01/AC-01] Routine loading and listing remain deterministic and succeed when the board contains zero routines. <!-- verify: test, SRS-NFR-01:start:end -->
