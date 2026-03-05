---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxqN5jnA/KNOWLEDGE.md
created_at: 2026-03-04T13:06:23
---

### 0MWoLPhDL: Catalog Entries Should Be Declarative And Sorted By ID

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a technique bank that will be extended by config/autodetection stories |
| **Insight** | A stable schema plus ID-sorted built-ins gives deterministic output and a predictable merge base for later override/ranking stages. |
| **Suggested Action** | Keep all built-ins in one constructor and enforce sort-by-ID before returning catalog vectors. |
| **Applies To** | `src/read_model/verification_techniques.rs`, upcoming config merge/recommendation modules |
| **Linked Knowledge IDs** | 1vyDuwZW6 |
| **Observed At** |  |
| **Score** | 0.81 |
| **Confidence** | 0.91 |
| **Applied** | yes |
