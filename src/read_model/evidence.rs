//! Evidence chain formatting for display across keel commands.
//!
//! Provides the `EvidenceEntry` data type and display formatting with
//! phase-aware indentation: `:start`/`:end` flush, `:continues` indented.

use crate::infrastructure::verification::parse_verify_annotations;
use crate::infrastructure::verification::parser::RequirementPhase;

/// A single evidence chain entry linking a story AC to a requirement phase.
#[derive(Debug, PartialEq, Clone)]
pub struct EvidenceEntry {
    pub requirement_id: String,
    pub story_id: String,
    pub story_title: String,
    pub criterion: String,
    pub phase: String,
    pub proof: Option<String>,
}

/// Collect evidence chain entries from a story's content.
pub fn collect_story_evidence(
    story_id: &str,
    story_title: &str,
    content: &str,
) -> Vec<EvidenceEntry> {
    let mut entries = Vec::new();

    for ann in parse_verify_annotations(content) {
        for req_ref in ann.requirements {
            let phase = match req_ref.phase {
                RequirementPhase::Start => "start",
                RequirementPhase::Continues => "continues",
                RequirementPhase::End => "end",
                RequirementPhase::StartEnd => "start:end",
            };
            entries.push(EvidenceEntry {
                requirement_id: req_ref.id,
                story_id: story_id.to_string(),
                story_title: story_title.to_string(),
                criterion: ann.criterion.clone(),
                phase: phase.to_string(),
                proof: ann.proof.clone(),
            });
        }
    }

    entries
}

/// Format a single evidence entry with phase-aware indentation (plain text, no color).
/// Used by tests to verify structural formatting without ANSI escapes.
#[cfg(test)]
fn format_evidence_entry(entry: &EvidenceEntry) -> String {
    let indent = if entry.phase == "continues" {
        "      "
    } else {
        "    "
    };
    format!(
        "{}:{} [{}] {} - \"{}\"",
        indent, entry.phase, entry.story_id, entry.story_title, entry.criterion
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_entry_is_flush() {
        let entry = EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "ABC".into(),
            story_title: "My Story".into(),
            criterion: "it works".into(),
            phase: "start".into(),
            proof: None,
        };
        assert_eq!(
            format_evidence_entry(&entry),
            "    :start [ABC] My Story - \"it works\""
        );
    }

    #[test]
    fn continues_entry_is_indented() {
        let entry = EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "ABC".into(),
            story_title: "My Story".into(),
            criterion: "more work".into(),
            phase: "continues".into(),
            proof: None,
        };
        assert_eq!(
            format_evidence_entry(&entry),
            "      :continues [ABC] My Story - \"more work\""
        );
    }

    #[test]
    fn end_entry_is_flush() {
        let entry = EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "XYZ".into(),
            story_title: "Final".into(),
            criterion: "done".into(),
            phase: "end".into(),
            proof: None,
        };
        assert_eq!(
            format_evidence_entry(&entry),
            "    :end [XYZ] Final - \"done\""
        );
    }

    #[test]
    fn start_end_entry_is_flush() {
        let entry = EvidenceEntry {
            requirement_id: "SRS-01".into(),
            story_id: "ONE".into(),
            story_title: "One Shot".into(),
            criterion: "complete".into(),
            phase: "start:end".into(),
            proof: None,
        };
        assert_eq!(
            format_evidence_entry(&entry),
            "    :start:end [ONE] One Shot - \"complete\""
        );
    }

    #[test]
    fn collect_finds_evidence_entries() {
        let content = r#"---
id: TEST1
title: Test
type: feat
status: in-progress
---

## Acceptance Criteria

- [x] First thing <!-- verify: cargo test, proof: first.log, SRS-01:start -->
- [x] Second thing <!-- verify: cargo test, SRS-01:continues -->
- [x] Last thing <!-- verify: cargo test, SRS-01:end -->
"#;
        let entries = collect_story_evidence("TEST1", "Test", content);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].requirement_id, "SRS-01");
        assert_eq!(entries[0].phase, "start");
        assert_eq!(entries[0].criterion, "First thing");
        assert_eq!(entries[0].proof, Some("first.log".to_string()));
        assert_eq!(entries[1].phase, "continues");
        assert_eq!(entries[1].proof, None);
        assert_eq!(entries[2].phase, "end");
    }

    #[test]
    fn collect_returns_empty_when_no_evidence() {
        let content = r#"---
id: TEST2
title: Test
type: feat
status: backlog
---

## Acceptance Criteria

- [ ] No evidence annotation
"#;
        let entries = collect_story_evidence("TEST2", "Test", content);
        assert!(entries.is_empty());
    }

    #[test]
    fn collect_duplicates_entry_for_each_requirement_ref() {
        let content = r#"---
id: TEST3
title: Test
type: feat
status: done
---

## Acceptance Criteria

- [x] Shared proof <!-- verify: manual, SRS-01:end, SRS-02:start:end, proof: shared.log -->
"#;
        let entries = collect_story_evidence("TEST3", "Test", content);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].requirement_id, "SRS-01");
        assert_eq!(entries[0].phase, "end");
        assert_eq!(entries[0].proof, Some("shared.log".to_string()));
        assert_eq!(entries[1].requirement_id, "SRS-02");
        assert_eq!(entries[1].phase, "start:end");
        assert_eq!(entries[1].proof, Some("shared.log".to_string()));
    }
}
