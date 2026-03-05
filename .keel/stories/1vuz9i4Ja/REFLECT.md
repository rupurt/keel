---
created_at: 2026-02-24T06:23:38
---

# Reflection - Normalize Epic Completion Field to Completed At

### Note 001: Canonical timestamp fields need both writer and validator alignment
Switching epic completion to `completed_at` required updating command writers and doctor field validation together; changing only one side leaves schema drift.

### Note 002: Tightening frontmatter contracts should be backed by regression tests
Adding parser strictness (`deny_unknown_fields`) is safe when paired with targeted tests that lock rejection of legacy fields and acceptance of canonical fields.
