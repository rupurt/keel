---
source_type: Story
source: stories/1vwqCe5T0/REFLECT.md
scope: 1vwq96cpt/1vwq9Zf67
source_story_id: 1vwqCe5T0
---

### 1vyDuwDTq: Thin Command Adapters Preserve Behavior During Refactors

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Extracting orchestration out of CLI command handlers while keeping existing workflow behavior stable |
| **Insight** | Moving orchestration to an application service is low-risk when command handlers become thin pass-through adapters and existing command tests remain the compatibility suite. |
| **Suggested Action** | For future migrations, extract service logic first, then convert command files to wrappers and keep legacy helper behavior behind `#[cfg(test)]` shims only where needed. |
| **Applies To** | src/application/story_lifecycle.rs, src/commands/story/{start,submit,accept,reject,ice,thaw}.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T00:26:18+00:00 |
| **Score** | 0.90 |
| **Confidence** | 0.91 |
| **Applied** | yes |
