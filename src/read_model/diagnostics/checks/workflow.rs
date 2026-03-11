use crate::read_model::diagnostics::types::*;
use crate::read_model::workflow_topology;
use std::path::Path;

/// Check workflow topology for integrity (missing defaults, bad references, overlap)
pub fn check_workflow_topology(board_dir: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();
    let project_root = workflow_topology::project_root_for_board(board_dir);
    let config_path = project_root.join("keel.toml");

    match workflow_topology::load_for_board(board_dir) {
        Ok(_) => {}
        Err(e) => {
            problems.push(
                Problem::error(config_path, format!("invalid workflow topology: {}", e))
                    .with_check_id(CheckId::WorkflowTopology),
            );
        }
    }

    problems
}
