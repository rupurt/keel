---
id: VDcFgu8NA
title: Business Automation Guide
type: feat
status: needs-human-verification
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T22:05:06
operator-signal: 
scope: VDakmJodq/VDcFd62ny
index: 1
started_at: 2026-03-11T22:01:09
submitted_at: 2026-03-11T22:05:06
---

# Business Automation Guide

## Summary

Author the first canonical `GUIDE.md` that explains how routines, temporal
gating, pulse, and scheduled flow review fit together for business automation.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `GUIDE.md` explains routine authoring, cadence fields, target scope, and blueprint expectations. <!-- verify: llm-judge, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] The guide includes an end-to-end example from routine definition through `keel next`, `keel flow`, and `keel pulse`. <!-- verify: llm-judge, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] The guide documents cron or systemd usage, idempotency expectations, and unsupported automation boundaries. <!-- verify: llm-judge, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-NFR-01/AC-01] Command names and workflow language match supported CLI behavior and hard-cutover semantics. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-4.log-->
