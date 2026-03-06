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

Type "keel bearing new 'Dogfood Research Flow'"
Enter
Sleep 800ms

Hide
Type "sleep 1"
Enter
Type "BEARING_ID=$(latest_id .keel/bearings)"
Enter
Type "cat > .keel/bearings/$BEARING_ID/BRIEF.md <<'EOF'"
Enter
Type "# Dogfood Research Flow — Brief"
Enter
Enter
Type "## Hypothesis"
Enter
Enter
Type "A concise tape can prove the bearing lifecycle without needing custom harness code beyond the shared dogfood runner."
Enter
Enter
Type "## Problem Space"
Enter
Enter
Type "Keel needs a research-workflow demo that shows how bearings move from open questions to a graduated epic on a clean board."
Enter
Enter
Type "## Success Criteria"
Enter
Enter
Type "- [x] The bearing can move through survey, assessment, and lay without doctor warnings."
Enter
Type "- [x] The workflow creates a derived epic from the bearing."
Enter
Enter
Type "## Open Questions"
Enter
Enter
Type "- Which minimal authored content is enough to keep the tape readable while satisfying the lifecycle gates?"
Enter
Type "EOF"
Enter
Show

Type "keel bearing survey $BEARING_ID"
Enter
Sleep 800ms

Hide
Type "cat > .keel/bearings/$BEARING_ID/SURVEY.md <<'EOF'"
Enter
Type "---"
Enter
Type "id: SURVEY"
Enter
Type "---"
Enter
Enter
Type "# Dogfood Research Flow — Survey"
Enter
Enter
Type "## Market Research"
Enter
Enter
Type "### Existing Solutions"
Enter
Enter
Type "Most CLI demos stop at screenshots and do not exercise a real research lifecycle."
Enter
Enter
Type "### Competitive Landscape"
Enter
Enter
Type "Terminal demo tooling exists, but it is rarely connected back into a planning board."
Enter
Enter
Type "### Market Size"
Enter
Enter
Type "The relevant need is internal: keep agent workflows inspectable and repeatable."
Enter
Enter
Type "## Technical Research"
Enter
Enter
Type "### Feasibility"
Enter
Enter
Type "The bearing workflow can reuse the same resettable secondary workspace as the epic workflow tape."
Enter
Enter
Type "### Prior Art"
Enter
Enter
Type "The existing dogfood runner and epic tape show the shared harness path already works."
Enter
Enter
Type "### Proof of Concepts"
Enter
Enter
Type "A manual dry run confirmed the lifecycle can be driven entirely through CLI commands."
Enter
Enter
Type "## User Research"
Enter
Enter
Type "### Target Users"
Enter
Enter
Type "Maintainers validating the research workflow and contributors learning it."
Enter
Enter
Type "### Pain Points"
Enter
Enter
Type "The research lifecycle is harder to trust when it is only described in docs."
Enter
Enter
Type "### Validation"
Enter
Enter
Type "A tape-backed walkthrough gives a concrete reproducible example."
Enter
Enter
Type "## Key Findings"
Enter
Enter
Type "1. The shared workspace reset is enough for research flows too."
Enter
Type "2. Minimal authored markdown is sufficient for bearing transitions."
Enter
Type "3. A derived epic is a strong end condition for the demo."
Enter
Enter
Type "## Unknowns"
Enter
Enter
Type "- Whether later artifact-aware judging should inspect bearing tapes differently from execution tapes."
Enter
Type "EOF"
Enter
Show

Type "keel bearing assess $BEARING_ID"
Enter
Sleep 800ms

Hide
Type "cat > .keel/bearings/$BEARING_ID/ASSESSMENT.md <<'EOF'"
Enter
Type "---"
Enter
Type "id: ASSESS"
Enter
Type "---"
Enter
Enter
Type "# Dogfood Research Flow — Assessment"
Enter
Enter
Type "## Scoring Factors"
Enter
Enter
Type "| Factor | Score | Rationale |"
Enter
Type "|--------|-------|-----------|"
Enter
Type "| Impact | 4 | The tape makes the research lifecycle concrete for future contributors. |"
Enter
Type "| Confidence | 4 | The shared runner and reset path already exist. |"
Enter
Type "| Effort | 2 | The tape is narrow and reuses the existing dogfood harness. |"
Enter
Type "| Risk | 2 | The workflow stays local and opt-in. |"
Enter
Enter
Type "## Analysis"
Enter
Enter
Type "### Opportunity Cost"
Enter
Enter
Type "Without this flow, the research lifecycle remains under-demonstrated compared with planning and execution."
Enter
Enter
Type "### Dependencies"
Enter
Enter
Type "The secondary workspace and bearing commands must stay stable."
Enter
Enter
Type "### Alternatives Considered"
Enter
Enter
Type "Document-only guidance was considered, but it does not prove the lifecycle end to end."
Enter
Enter
Type "## Recommendation"
Enter
Enter
Type "[x] Proceed → convert to epic"
Enter
Type "[ ] Park → revisit later"
Enter
Type "[ ] Decline → document learnings"
Enter
Type "EOF"
Enter
Show

Type "keel bearing lay $BEARING_ID"
Enter
Sleep 900ms

Type "exit"
Enter
