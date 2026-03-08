//! Shared voyage artifact synchronization.

use crate::domain::model::{Board, Voyage, VoyageState};

use super::{artifact_io, compliance_report, knowledge_synthesis, voyage_readme, voyage_report};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SyncOptions {
    pub synthesize_knowledge: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncIssue {
    pub artifact: &'static str,
    pub message: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SyncOutcome {
    pub issues: Vec<SyncIssue>,
}

/// Synchronize generated voyage artifacts for the current voyage state.
pub fn sync(board: &Board, voyage: &Voyage, options: SyncOptions) -> SyncOutcome {
    let mut outcome = SyncOutcome::default();

    if let Err(err) = voyage_readme::generate(board, voyage) {
        outcome.issues.push(SyncIssue {
            artifact: "README.md",
            message: err.to_string(),
        });
    }

    if voyage.status() == VoyageState::Done {
        if let Err(err) = voyage_report::generate(board, voyage) {
            outcome.issues.push(SyncIssue {
                artifact: "VOYAGE_REPORT.md",
                message: err.to_string(),
            });
        }
        if let Err(err) = compliance_report::generate(board, voyage) {
            outcome.issues.push(SyncIssue {
                artifact: "COMPLIANCE_REPORT.md",
                message: err.to_string(),
            });
        }
        if options.synthesize_knowledge
            && let Err(err) = knowledge_synthesis::synthesize_voyage_knowledge(board, voyage)
        {
            outcome.issues.push(SyncIssue {
                artifact: "KNOWLEDGE.md",
                message: err.to_string(),
            });
        }
    } else if let Err(err) = remove_done_reports(voyage) {
        outcome.issues.push(SyncIssue {
            artifact: "done-only reports",
            message: err.to_string(),
        });
    }

    outcome
}

fn remove_done_reports(voyage: &Voyage) -> anyhow::Result<()> {
    let Some(voyage_dir) = voyage.path.parent() else {
        return Ok(());
    };

    for artifact in ["VOYAGE_REPORT.md", "COMPLIANCE_REPORT.md"] {
        artifact_io::remove_if_exists(&voyage_dir.join(artifact))?;
    }

    Ok(())
}
