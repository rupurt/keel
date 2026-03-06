//! Stakeholder Compliance Report generation

use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::domain::model::{Board, Voyage};
use crate::domain::state_machine::invariants::parse_requirements;
use crate::infrastructure::templates;
use crate::infrastructure::verification::parser::parse_verify_annotations;

/// Generate a stakeholder-ready COMPLIANCE_REPORT.md for the voyage
pub fn generate(_board_dir: &Path, board: &Board, voyage: &Voyage) -> anyhow::Result<()> {
    let content = generate_compliance_report(board, voyage);
    let report_path = voyage.path.parent().unwrap().join("COMPLIANCE_REPORT.md");
    fs::write(report_path, content)?;
    Ok(())
}

/// Generate the content for COMPLIANCE_REPORT.md
pub fn generate_compliance_report(board: &Board, voyage: &Voyage) -> String {
    let srs_path = voyage.path.parent().unwrap().join("SRS.md");
    let requirements = parse_requirements(&srs_path);
    let mut stories = board.stories_for_voyage(voyage);
    stories.sort_by(|a, b| a.id().cmp(b.id()));

    // Map requirements to stories and their proof
    let mut req_map: HashMap<String, Vec<(String, Vec<String>)>> = HashMap::new();
    for story in &stories {
        if let Ok(content) = fs::read_to_string(&story.path) {
            let annotations = parse_verify_annotations(&content);
            for ann in annotations {
                if let Some(req_ref) = ann.requirement {
                    let mut proofs = Vec::new();
                    // Check EVIDENCE/ directory for proof artifacts
                    let story_dir = story.path.parent().unwrap();
                    let evidence_dir = story_dir.join("EVIDENCE");
                    if evidence_dir.exists()
                        && let Ok(entries) = fs::read_dir(evidence_dir)
                    {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() {
                                proofs
                                    .push(path.file_name().unwrap().to_string_lossy().to_string());
                            }
                        }
                        proofs.sort();
                    }

                    req_map
                        .entry(req_ref.id)
                        .or_default()
                        .push((story.id().to_string(), proofs));
                }
            }
        }
    }

    let mut matrix = String::new();
    for req_id in requirements {
        let (status, implemented_by, proof_links) = if let Some(entries) = req_map.get(&req_id) {
            let mut sorted_entries: Vec<_> = entries.iter().collect();
            sorted_entries.sort_by(|(story_id_a, _), (story_id_b, _)| story_id_a.cmp(story_id_b));

            let story_ids: Vec<_> = sorted_entries
                .iter()
                .map(|(id, _)| format!("[{}](../../../../stories/{}/README.md)", id, id))
                .collect();
            let mut all_proofs = Vec::new();
            for (story_id, proofs) in sorted_entries {
                for proof in proofs {
                    all_proofs.push(format!(
                        "[{}](../../../../stories/{}/EVIDENCE/{})",
                        proof, story_id, proof
                    ));
                }
            }
            ("✓ VERIFIED", story_ids.join(", "), all_proofs.join("<br>"))
        } else {
            ("○ PENDING", "-".to_string(), "-".to_string())
        };

        writeln!(
            matrix,
            "| {} | {} | {} | {} |",
            req_id, status, implemented_by, proof_links
        )
        .unwrap();
    }

    templates::voyage::COMPLIANCE
        .replace("{{title}}", voyage.title())
        .replace("{{matrix}}", matrix.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::path::{Path, PathBuf};

    use crate::domain::model::{
        Story, StoryFrontmatter, StoryState, StoryType, VoyageFrontmatter, VoyageState,
    };

    fn write_srs(voyage_dir: &Path) {
        fs::write(
            voyage_dir.join("SRS.md"),
            r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement |
| --- | --- |
| SRS-01 | Deterministic ordering |
| SRS-02 | Pending requirement |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();
    }

    fn make_voyage(root: &Path) -> Voyage {
        let epic_id = "epic-alpha";
        let voyage_id = "01-deterministic-reports";
        let voyage_dir = root
            .join("epics")
            .join(epic_id)
            .join("voyages")
            .join(voyage_id);
        fs::create_dir_all(&voyage_dir).unwrap();
        fs::write(
            voyage_dir.join("README.md"),
            format!(
                "---\nid: {voyage_id}\ntitle: Deterministic Reports\nstatus: in-progress\nepic: {epic_id}\ncreated_at: 2026-01-01T00:00:00\nupdated_at: 2026-01-01T00:00:00\nstarted_at: 2026-01-01T00:00:00\n---\n"
            ),
        )
        .unwrap();
        write_srs(&voyage_dir);

        Voyage {
            frontmatter: VoyageFrontmatter {
                id: voyage_id.to_string(),
                title: "Deterministic Reports".to_string(),
                goal: Some("Keep compliance output stable".to_string()),
                status: VoyageState::InProgress,
                epic: Some(epic_id.to_string()),
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
            },
            path: voyage_dir.join("README.md"),
            epic_id: epic_id.to_string(),
        }
    }

    fn make_story(
        root: &Path,
        id: &str,
        scope: &str,
        requirement: &str,
        evidence_files: &[&str],
    ) -> Story {
        let story_dir = root.join("stories").join(id);
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();
        for evidence in evidence_files {
            fs::write(story_dir.join("EVIDENCE").join(evidence), "proof").unwrap();
        }

        fs::write(
            story_dir.join("README.md"),
            format!(
                "---\nid: {id}\ntitle: Story {id}\ntype: feat\nstatus: done\nscope: {scope}\ncreated_at: 2026-01-01T00:00:00\nupdated_at: 2026-01-01T00:00:00\nstarted_at: 2026-01-01T01:00:00\ncompleted_at: 2026-01-01T02:00:00\nsubmitted_at: 2026-01-01T03:00:00\n---\n\n# Story\n\n- [x] [{requirement}/AC-01] Verify output <!-- verify: manual, {requirement}:start:end -->\n"
            ),
        )
        .unwrap();

        Story::new(
            StoryFrontmatter {
                id: id.to_string(),
                title: format!("Story {id}"),
                story_type: StoryType::Feat,
                status: StoryState::Done,
                scope: Some(scope.to_string()),
                milestone: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
                submitted_at: None,
                index: None,
                governed_by: Vec::new(),
                blocked_by: Vec::new(),
                role: None,
            },
            story_dir.join("README.md"),
        )
    }

    #[test]
    fn compliance_report_is_deterministic_across_equivalent_boards() {
        let temp = tempfile::TempDir::new().unwrap();
        let voyage = make_voyage(temp.path());
        let scope = voyage.scope_path();

        let mut story_fixtures: HashMap<String, Story> = HashMap::new();
        story_fixtures.insert(
            "FEAT0001".to_string(),
            make_story(
                temp.path(),
                "FEAT0001",
                &scope,
                "SRS-01",
                &["zeta.log", "alpha.log"],
            ),
        );
        story_fixtures.insert(
            "FEAT0002".to_string(),
            make_story(
                temp.path(),
                "FEAT0002",
                &scope,
                "SRS-01",
                &["delta.log", "beta.log"],
            ),
        );
        story_fixtures.insert(
            "FEAT0003".to_string(),
            make_story(temp.path(), "FEAT0003", &scope, "SRS-01", &["charlie.log"]),
        );

        let orderings = vec![
            vec!["FEAT0003", "FEAT0001", "FEAT0002"],
            vec!["FEAT0002", "FEAT0003", "FEAT0001"],
            vec!["FEAT0001", "FEAT0002", "FEAT0003"],
            vec!["FEAT0002", "FEAT0001", "FEAT0003"],
            vec!["FEAT0003", "FEAT0002", "FEAT0001"],
            vec!["FEAT0001", "FEAT0003", "FEAT0002"],
        ];

        let mut outputs = HashSet::new();
        for order in orderings {
            let mut board = Board::new(PathBuf::from(temp.path()));
            for story_id in order {
                board.stories.insert(
                    story_id.to_string(),
                    story_fixtures.get(story_id).unwrap().clone(),
                );
            }
            outputs.insert(generate_compliance_report(&board, &voyage));
        }

        assert_eq!(
            outputs.len(),
            1,
            "compliance report should be stable across equivalent story orderings"
        );

        let report = outputs.into_iter().next().unwrap();
        let srs_01_line = report
            .lines()
            .find(|line| line.starts_with("| SRS-01 |"))
            .expect("SRS-01 row should exist");

        let expected_story_links = ["FEAT0001", "FEAT0002", "FEAT0003"]
            .into_iter()
            .map(|id| format!("[{id}](../../../../stories/{id}/README.md)"))
            .collect::<Vec<_>>()
            .join(", ");
        assert!(srs_01_line.contains(&expected_story_links));

        let expected_proof_links = vec![
            ("FEAT0001", "alpha.log"),
            ("FEAT0001", "zeta.log"),
            ("FEAT0002", "beta.log"),
            ("FEAT0002", "delta.log"),
            ("FEAT0003", "charlie.log"),
        ]
        .into_iter()
        .map(|(story_id, proof)| {
            format!("[{proof}](../../../../stories/{story_id}/EVIDENCE/{proof})")
        })
        .collect::<Vec<_>>()
        .join("<br>");
        assert!(srs_01_line.contains(&expected_proof_links));
    }
}
