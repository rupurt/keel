# PRESS RELEASE: Template And CLI Contract Canonicalization

## Overview

## Narrative Summary
### Canonicalize Template Tokens To Schema Names
Replace non-canonical template token names with canonical schema/frontmatter-mirrored tokens and align creation render inputs with the new token vocabulary.

### Align CLI Contracts For Creation Commands
Align creation command interfaces with ownership policy so only user-owned inputs are exposed while system-owned fields remain runtime-managed.

### Extend Adr Creation Inputs For Context Ownership
Add explicit ADR creation inputs for context scope ownership and persist these values directly in ADR frontmatter.

### Codify Token Bucket Contract Tests
Create deterministic contract tests for token bucket policy so unknown tokens or ownership violations fail immediately.

## Key Insights
### Insights from Canonicalize Template Tokens To Schema Names
- **L001: Keep Token Names Equal To Frontmatter Keys**
  - Insight: Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward.
  - Suggested Action: For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases.


### Insights from Align CLI Contracts For Creation Commands
- **L001: Keep CLI contract updates end-to-end**
  - Insight: Command tree flags, runtime mappers, and user-facing suggestion strings drift unless updated in the same slice.
  - Suggested Action: Pair every CLI contract edit with parser rejection tests for removed flags and updates to generated command hints.


### Insights from Extend Adr Creation Inputs For Context Ownership
- **L001: Keep CLI parser, runtime mapping, and template tokens aligned**
  - Insight: Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set.
  - Suggested Action: For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior.


### Insights from Codify Token Bucket Contract Tests
- **L001: Keep token inventories and CLI `new` surfaces coupled by drift tests**
  - Insight: A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries.
  - Suggested Action: When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic.


## Verification Proof
### Proof for Canonicalize Template Tokens To Schema Names
- [ac-1.log](../../../../stories/1vxH83JcY/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH83JcY/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH83JcY/EVIDENCE/ac-2.log)

### Proof for Align CLI Contracts For Creation Commands
- [ac-1.log](../../../../stories/1vxH83MOO/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH83MOO/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH83MOO/EVIDENCE/ac-2.log)

### Proof for Extend Adr Creation Inputs For Context Ownership
- [ac-1.log](../../../../stories/1vxH84Xh8/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84Xh8/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84Xh8/EVIDENCE/ac-2.log)

### Proof for Codify Token Bucket Contract Tests
- [ac-1.log](../../../../stories/1vxH84K5a/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84K5a/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84K5a/EVIDENCE/ac-2.log)

