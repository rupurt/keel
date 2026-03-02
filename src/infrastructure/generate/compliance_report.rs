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
    let stories = board.stories_for_voyage(voyage);

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
            let story_ids: Vec<_> = entries
                .iter()
                .map(|(id, _)| format!("[{}](../../../../stories/{}/README.md)", id, id))
                .collect();
            let mut all_proofs = Vec::new();
            for (story_id, proofs) in entries {
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
