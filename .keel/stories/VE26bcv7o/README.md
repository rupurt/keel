---
id: VE26bcv7o
title: Apply URL-Derived Defaults in Research Capture
type: feat
status: backlog
created_at: 2026-03-16T05:31:30
updated_at: 2026-03-16T05:31:48
operator-signal:
scope: VDiHwULir/VE26NBIKc
index: 1
---

# Apply URL-Derived Defaults in Research Capture

## Summary

When `--location` is a URL (`http://` or `https://`), auto-derive `--class web`, `--retrieved-at <today>`, and `--provider manual:<domain>` if those flags are not explicitly provided. Explicit flags override the defaults.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Capture with URL location auto-defaults class to `web` when `--class` is omitted. <!-- verify: cargo test -p keel url_capture_defaults_class_to_web, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] Capture with URL location auto-defaults `retrieved-at` to today when `--retrieved-at` is omitted. <!-- verify: cargo test -p keel url_capture_defaults_retrieved_at_to_today, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Capture with URL location auto-defaults provider to `manual:<domain>` when `--provider` is omitted. <!-- verify: cargo test -p keel url_capture_defaults_provider_from_domain, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] Explicit `--class`, `--retrieved-at`, and `--provider` flags override URL-derived defaults. <!-- verify: cargo test -p keel url_capture_explicit_flags_override_defaults, SRS-04:start:end -->
