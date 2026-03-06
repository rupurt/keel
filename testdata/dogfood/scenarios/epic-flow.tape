Require bash
Require cargo

Set Shell "bash"
Set Width 1400
Set Height 900
Set FontSize 22
Set TypingSpeed 0ms

Hide
Type "alias keel='cargo run --quiet --manifest-path ../../../Cargo.toml --'"
Enter
Type "latest_id() { find $1 -mindepth 1 -maxdepth 1 -type d -printf '%f\\n' | sort | tail -n 1; }"
Enter
Show

Type "pwd"
Enter
Sleep 500ms

Type "ls .keel"
Enter
Sleep 500ms

Type "keel epic new 'Dogfood Planning Flow' --problem 'Agents need a real end-to-end planning demo that shows how keel creates, decomposes, and steers work on a clean board.'"
Enter
Sleep 800ms

Hide
Type "sleep 1"
Enter
Type "EPIC_ID=$(latest_id .keel/epics)"
Enter
Type "cat > .keel/epics/$EPIC_ID/PRD.md <<'EOF'"
Enter
Type "# Dogfood Planning Flow - Product Requirements"
Enter
Enter
Type "## Problem Statement"
Enter
Enter
Type "Agents need a real end-to-end planning demo that shows how keel creates, decomposes, and steers work on a clean board."
Enter
Enter
Type "## Goals & Objectives"
Enter
Enter
Type "| ID | Goal | Success Metric | Target |"
Enter
Type "|----|------|----------------|--------|"
Enter
Type "| GOAL-01 | Demonstrate epic decomposition on the secondary board. | A planned voyage and a linked story exist on the dogfood board. | One planned voyage with one executable story |"
Enter
Enter
Type "## Users"
Enter
Enter
Type "| Persona | Description | Primary Need |"
Enter
Type "|---------|-------------|--------------|"
Enter
Type "| Agent Operator | A contributor learning or validating the keel planning flow. | A concise, repeatable planning walkthrough. |"
Enter
Enter
Type "## Scope"
Enter
Enter
Type "### In Scope"
Enter
Enter
Type "- [SCOPE-01] Create an epic and author a canonical PRD."
Enter
Type "- [SCOPE-02] Decompose the epic into one voyage and one story."
Enter
Type "- [SCOPE-03] Use `keel next` and `keel flow` to inspect the resulting queue."
Enter
Enter
Type "### Out of Scope"
Enter
Enter
Type "- [SCOPE-04] Execute or accept the planned story."
Enter
Enter
Type "## Requirements"
Enter
Enter
Type "### Functional Requirements"
Enter
Enter
Type "<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->"
Enter
Type "| ID | Requirement | Goals | Priority | Rationale |"
Enter
Type "|----|-------------|-------|----------|-----------|"
Enter
Type "| FR-01 | The workflow creates one epic with authored planning documents. | GOAL-01 | must | Shows the canonical epic authoring path. |"
Enter
Type "| FR-02 | The workflow plans one voyage with one linked story. | GOAL-01 | must | Demonstrates tactical decomposition. |"
Enter
Type "| FR-03 | The workflow surfaces `keel next` and `keel flow` after planning. | GOAL-01 | should | Shows the steering surfaces agents use after planning. |"
Enter
Type "<!-- END FUNCTIONAL_REQUIREMENTS -->"
Enter
Enter
Type "### Non-Functional Requirements"
Enter
Enter
Type "<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->"
Enter
Type "| ID | Requirement | Goals | Priority | Rationale |"
Enter
Type "|----|-------------|-------|----------|-----------|"
Enter
Type "| NFR-01 | The workflow stays repeatable on the reset secondary workspace. | GOAL-01 | must | Keeps the dogfood flow stable enough to rerun. |"
Enter
Type "<!-- END NON_FUNCTIONAL_REQUIREMENTS -->"
Enter
Enter
Type "## Verification Strategy"
Enter
Enter
Type "| Area | Method | Evidence |"
Enter
Type "|------|--------|----------|"
Enter
Type "| Planning flow | CLI walkthrough | `keel epic show`, `keel voyage show`, `keel next`, `keel flow` |"
Enter
Enter
Type "## Assumptions"
Enter
Enter
Type "| Assumption | Impact if Wrong | Validation |"
Enter
Type "|------------|-----------------|------------|"
Enter
Type "| The secondary workspace starts empty before the workflow runs. | The created IDs and steering output may become noisy. | Reset the workspace before each tape. |"
Enter
Enter
Type "## Open Questions & Risks"
Enter
Enter
Type "| Question/Risk | Owner | Status |"
Enter
Type "|---------------|-------|--------|"
Enter
Type "| Which compact flow best demonstrates steering without excessive CLI noise? | Dogfood suite owner | Open |"
Enter
Enter
Type "## Success Criteria"
Enter
Enter
Type "<!-- BEGIN SUCCESS_CRITERIA -->"
Enter
Type "- [ ] The dogfood board contains one planned voyage with one backlog story."
Enter
Type "- [ ] `keel next --agent` recommends the created story."
Enter
Type "<!-- END SUCCESS_CRITERIA -->"
Enter
Type "EOF"
Enter
Show

Type "keel epic show $EPIC_ID"
Enter
Sleep 900ms

Type "keel voyage new 'Plan One Slice' --epic $EPIC_ID --goal 'Turn the epic into a single planned voyage and story that agents can pick up next.'"
Enter
Sleep 800ms

Hide
Type "sleep 1"
Enter
Type "VOYAGE_ID=$(latest_id .keel/epics/$EPIC_ID/voyages)"
Enter
Type "cat > .keel/epics/$EPIC_ID/voyages/$VOYAGE_ID/SRS.md <<'EOF'"
Enter
Type "# Plan One Slice - Software Requirements Specification"
Enter
Enter
Type "> Turn the epic into a single planned voyage and story that agents can pick up next."
Enter
Enter
Type "**Epic:** [Epic](../../README.md) | **SDD:** [SDD.md](SDD.md)"
Enter
Enter
Type "## Scope"
Enter
Enter
Type "In scope:"
Enter
Type "- [SCOPE-01] Author the voyage requirements and design."
Enter
Type "- [SCOPE-02] Create one linked implementation story."
Enter
Type "- [SCOPE-03] Surface queue guidance with `keel next` and `keel flow`."
Enter
Enter
Type "Out of scope:"
Enter
Type "- [SCOPE-04] Execute the linked story."
Enter
Enter
Type "## Assumptions & Dependencies"
Enter
Enter
Type "| Assumption/Dependency | Type | Impact if Invalid |"
Enter
Type "|-----------------------|------|-------------------|"
Enter
Type "| The epic PRD already defines canonical `FR-*`, `NFR-*`, and scope IDs. | Planning | Voyage lineage will fail if the PRD stays scaffolded. |"
Enter
Enter
Type "## Constraints"
Enter
Enter
Type "- Keep the dogfood planning slice compact enough to render cleanly in one tape."
Enter
Type "- Use canonical lineage fields so `voyage plan` can validate the decomposition."
Enter
Enter
Type "## Requirements"
Enter
Enter
Type "### Functional Requirements"
Enter
Enter
Type "<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->"
Enter
Type "| ID | Requirement | Scope | Source | Verification |"
Enter
Type "|----|-------------|-------|--------|--------------|"
Enter
Type "| SRS-01 | Author the voyage artifacts with canonical lineage back to the epic PRD. | SCOPE-01 | FR-01 | `keel voyage show` |"
Enter
Type "| SRS-02 | Create one linked story with acceptance criteria that trace to this SRS. | SCOPE-02 | FR-02 | `keel story show` |"
Enter
Type "| SRS-03 | Show `keel next` and `keel flow` after the voyage is planned. | SCOPE-03 | FR-03 | `keel next`, `keel flow` |"
Enter
Type "<!-- END FUNCTIONAL_REQUIREMENTS -->"
Enter
Enter
Type "### Non-Functional Requirements"
Enter
Enter
Type "<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->"
Enter
Type "| ID | Requirement | Scope | Source | Verification |"
Enter
Type "|----|-------------|-------|--------|--------------|"
Enter
Type "| SRS-NFR-01 | The planning flow must be repeatable after resetting the secondary workspace. | SCOPE-03 | NFR-01 | repeat the tape on a reset workspace |"
Enter
Type "<!-- END NON_FUNCTIONAL_REQUIREMENTS -->"
Enter
Type "EOF"
Enter
Type "cat > .keel/epics/$EPIC_ID/voyages/$VOYAGE_ID/SDD.md <<'EOF'"
Enter
Type "# Plan One Slice - Software Design Description"
Enter
Enter
Type "> Turn the epic into a single planned voyage and story that agents can pick up next."
Enter
Enter
Type "**SRS:** [SRS.md](SRS.md)"
Enter
Enter
Type "## Overview"
Enter
Enter
Type "This voyage keeps the dogfood planning path minimal: author the voyage docs, create one story, and plan the voyage so queue surfaces can recommend the next slice."
Enter
Enter
Type "## Context & Boundaries"
Enter
Enter
Type "In scope:"
Enter
Type "- One voyage with one linked story."
Enter
Type "- Queue guidance via `keel next` and `keel flow`."
Enter
Enter
Type "Out of scope:"
Enter
Type "- Executing the planned story."
Enter
Enter
Type "## Dependencies"
Enter
Enter
Type "| Dependency | Type | Purpose | Version/API |"
Enter
Type "|------------|------|---------|-------------|"
Enter
Type "| Epic PRD | Internal | Supplies canonical requirement and scope lineage | current board contract |"
Enter
Enter
Type "## Key Decisions"
Enter
Enter
Type "| Decision | Choice | Rationale |"
Enter
Type "|----------|--------|-----------|"
Enter
Type "| Scope size | Plan exactly one story. | Keeps the tape short and the queue output obvious. |"
Enter
Enter
Type "## Architecture"
Enter
Enter
Type "The workflow updates markdown planning artifacts on the secondary board and relies on keel's existing planning read models for steering output."
Enter
Enter
Type "## Components"
Enter
Enter
Type "- Voyage planning artifacts: provide the canonical requirement contract."
Enter
Type "- Story bundle: gives `keel next` a concrete implementation target."
Enter
Type "- Flow surfaces: render the recommended next step and board state summary."
Enter
Enter
Type "## Interfaces"
Enter
Enter
Type "- `keel voyage plan <id>`"
Enter
Type "- `keel next --agent`"
Enter
Type "- `keel flow`"
Enter
Enter
Type "## Data Flow"
Enter
Enter
Type "1. Author SRS and SDD."
Enter
Type "2. Create a story."
Enter
Type "3. Link the story to the voyage."
Enter
Type "4. Author acceptance criteria on the story."
Enter
Type "5. Plan the voyage."
Enter
Type "6. Inspect `keel next` and `keel flow`."
Enter
Enter
Type "## Error Handling"
Enter
Enter
Type "| Error Condition | Detection | Response | Recovery |"
Enter
Type "|-----------------|-----------|----------|----------|"
Enter
Type "| Missing lineage in the voyage docs | `keel voyage plan` fails | Stop and report the missing field | Fix the authored markdown and rerun |"
Enter
Type "EOF"
Enter
Show

Type "keel story new 'Build The Demonstration Story'"
Enter
Sleep 800ms

Hide
Type "sleep 1"
Enter
Type "STORY_ID=$(latest_id .keel/stories)"
Enter
Type "cat > .keel/stories/$STORY_ID/README.md <<EOF"
Enter
Type "---"
Enter
Type "id: $STORY_ID"
Enter
Type "title: Build The Demonstration Story"
Enter
Type "type: feat"
Enter
Type "status: icebox"
Enter
Type "created_at: 2026-03-06T00:00:00"
Enter
Type "updated_at: 2026-03-06T00:00:00"
Enter
Type "---"
Enter
Enter
Type "# Build The Demonstration Story"
Enter
Enter
Type "## Summary"
Enter
Enter
Type "Create the single executable slice that proves the dogfood planning flow can hand work to an implementer."
Enter
Enter
Type "## Acceptance Criteria"
Enter
Enter
Type "- [ ] [SRS-01/AC-01] The story references the planned voyage scope and requirement lineage. <!-- verify: cargo test --quiet, SRS-01:start -->"
Enter
Type "- [ ] [SRS-02/AC-01] The story is linked to SRS-02 with a concrete implementer-facing outcome. <!-- verify: cargo test --quiet, SRS-02:start -->"
Enter
Type "- [ ] [SRS-03/AC-01] The planned story gives keel next and keel flow a concrete implementation target to render. <!-- verify: cargo test --quiet, SRS-03:start -->"
Enter
Type "EOF"
Enter
Type "keel story link $STORY_ID $VOYAGE_ID"
Enter
Show

Type "keel voyage plan $VOYAGE_ID"
Enter
Sleep 900ms

Type "keel voyage show $VOYAGE_ID"
Enter
Sleep 900ms

Type "keel next --agent"
Enter
Sleep 700ms

Type "keel flow"
Enter
Sleep 900ms

Type "exit"
Enter
