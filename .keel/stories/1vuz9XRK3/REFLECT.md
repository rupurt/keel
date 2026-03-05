---
created_at: 2026-02-24T19:07:55
---

# Reflection - Add Regression Tests for Gate Runtime and Reporting Modes

### L-01: Keep parity assertions focused on normalized findings

Runtime and reporting flows can differ in blocking classification while still sharing identical gate findings. Comparing normalized severity/message fingerprints catches rule-source drift without coupling tests to board file paths.
