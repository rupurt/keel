# VOYAGE REPORT: Template And CLI Contract Canonicalization

## Voyage Metadata
- **ID:** 1vxGzV3oR
- **Epic:** 1vxGy5tco
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Canonicalize Template Tokens To Schema Names
- **ID:** 1vxH83JcY
- **Status:** done

#### Summary
Replace non-canonical template token names with canonical schema/frontmatter-mirrored tokens and align creation render inputs with the new token vocabulary.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Replace legacy token names (for example `date`, `datetime`) in active planning templates with canonical schema-mirrored tokens. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Update rendering callsites in creation paths so all canonical tokens are populated correctly without fallback alias handling. <!-- verify: manual, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests asserting deprecated token aliases are absent from embedded templates. <!-- verify: manual, SRS-01:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyDuwE2r: Keep Token Names Equal To Frontmatter Keys**
  - Insight: Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward.
  - Suggested Action: For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases.
  - Applies To: `templates/**`, `src/cli/commands/management/*/new.rs`, `src/infrastructure/templates.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH83JcY/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH83JcY/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH83JcY/EVIDENCE/ac-2.log)

### Align CLI Contracts For Creation Commands
- **ID:** 1vxH83MOO
- **Status:** done

#### Summary
Align creation command interfaces with ownership policy so only user-owned inputs are exposed while system-owned fields remain runtime-managed.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Verify `epic new`, `voyage new`, `story new`, `bearing new`, and `adr new` expose only approved user-owned creation inputs. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Confirm no creation command introduces CLI flags for system-owned fields (`id`, `index`, `status`, `*_at`). <!-- verify: manual, SRS-02:continues, proof: ac-2.log-->
- [x] [SRS-02/AC-03] Make `voyage new --goal` required at CLI parse time and keep runtime validation behavior coherent. <!-- verify: manual, SRS-02:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] Confirm `voyage new --goal` is enforced by clap parse requirements and remains coherent with runtime validation. <!-- verify: manual, SRS-04:start:end, proof: ac-4.log-->

#### Implementation Insights
- **1vyDuwuNj: Keep CLI contract updates end-to-end**
  - Insight: Command tree flags, runtime mappers, and user-facing suggestion strings drift unless updated in the same slice.
  - Suggested Action: Pair every CLI contract edit with parser rejection tests for removed flags and updates to generated command hints.
  - Applies To: src/cli/command_tree.rs, src/cli/runtime.rs, src/cli_tests.rs, src/cli/presentation/flow/next_up.rs
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH83MOO/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH83MOO/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH83MOO/EVIDENCE/ac-2.log)

### Codify Token Bucket Contract Tests
- **ID:** 1vxH84K5a
- **Status:** done

#### Summary
Create deterministic contract tests for token bucket policy so unknown tokens or ownership violations fail immediately.

#### Acceptance Criteria
- [x] [SRS-05/AC-02] Define and test the allowed token bucket contract (CLI-owned, system-owned, generated markers). <!-- verify: cargo test -p keel generated_marker_contract_remains_literal, SRS-05:start, proof: ac-1.log -->
- [x] [SRS-05/AC-03] Fail tests when templates contain unknown tokens or out-of-bucket token usage. <!-- verify: cargo test -p keel template_tokens_match_known_bucket_inventory, SRS-05:continues, proof: ac-2.log -->
- [x] [SRS-05/AC-04] Add/adjust drift tests to keep token inventory and CLI contract aligned over time. <!-- verify: cargo test -p keel creation_command_new_surfaces_match_cli_owned_token_contract, SRS-05:end, proof: ac-3.log -->

#### Implementation Insights
- **1vyDuwGh9: Keep token inventories and CLI `new` surfaces coupled by drift tests**
  - Insight: A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries.
  - Suggested Action: When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic.
  - Applies To: src/infrastructure/templates.rs, src/drift_tests.rs, src/cli/command_tree.rs
  - Category: testing


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84K5a/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84K5a/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84K5a/EVIDENCE/ac-2.log)

### Extend Adr Creation Inputs For Context Ownership
- **ID:** 1vxH84Xh8
- **Status:** done

#### Summary
Add explicit ADR creation inputs for context scope ownership and persist these values directly in ADR frontmatter.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Add optional `--context` to `adr new` and persist the value in the created ADR frontmatter. <!-- verify: cargo test -p keel cli::commands::management::adr::tests::new_adr_persists_context_and_applies_to_in_frontmatter, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] Add repeatable `--applies-to` to `adr new` and persist all provided values in frontmatter order. <!-- verify: cargo test -p keel cli::commands::management::adr::tests::new_adr_persists_context_and_applies_to_in_frontmatter, SRS-03:continues, proof: ac-2.log -->
- [x] [SRS-03/AC-03] Add command tests validating parser behavior and persisted frontmatter for both flags. <!-- verify: cargo test -p keel cli_tests::cli_parses_adr_new_with_context_and_applies_to, SRS-03:end, proof: ac-3.log -->

#### Implementation Insights
- **1vyDuwxcg: Keep CLI parser, runtime mapping, and template tokens aligned**
  - Insight: Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set.
  - Suggested Action: For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior.
  - Applies To: src/cli/commands/management/adr/mod.rs, src/cli/runtime.rs, src/cli/command_tree.rs, templates/adrs/ADR.md
  - Category: process


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxH84Xh8/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxH84Xh8/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxH84Xh8/EVIDENCE/ac-2.log)


