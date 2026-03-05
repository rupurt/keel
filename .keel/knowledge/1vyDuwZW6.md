---
source_type: Story
source: stories/1vxqNFaR9/REFLECT.md
scope: 1vxqMtskC/1vxqN5jnA
source_story_id: 1vxqNFaR9
---

### 1vyDuwZW6: Catalog Entries Should Be Declarative And Sorted By ID

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a technique bank that will be extended by config/autodetection stories |
| **Insight** | A stable schema plus ID-sorted built-ins gives deterministic output and a predictable merge base for later override/ranking stages. |
| **Suggested Action** | Keep all built-ins in one constructor and enforce sort-by-ID before returning catalog vectors. |
| **Applies To** | `src/read_model/verification_techniques.rs`, upcoming config merge/recommendation modules |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T18:23:41+00:00 |
| **Score** | 0.81 |
| **Confidence** | 0.91 |
| **Applied** | yes |
