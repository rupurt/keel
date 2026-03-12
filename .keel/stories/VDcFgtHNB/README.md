---
id: VDcFgtHNB
title: Pulse Routine Materialization
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:45
operator-signal: 
scope: VDakmG8cH/VDcFd5Sop
index: 1
---

# Pulse Routine Materialization

## Summary

Materialize due routine work safely so recurring automation creates board work
exactly once per eligible window.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Due routines materialize work exactly once per eligible window under their target scope. <!-- verify: test, SRS-02:start -->
- [ ] [SRS-02/AC-02] Already materialized or no-longer-eligible routines are skipped without duplicate work creation. <!-- verify: test, SRS-02:end -->
- [ ] [SRS-NFR-01/AC-01] Repeated pulse runs remain idempotent and safe for frequent cron or systemd execution. <!-- verify: test, SRS-NFR-01:start:end -->
- [ ] [SRS-03/AC-01] Pulse records enough diagnostic state to explain why a routine was created, skipped, or deferred. <!-- verify: test, SRS-03:start:end -->
