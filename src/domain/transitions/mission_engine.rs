//! Mission transition engine

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use std::fs;
use std::path::Path;

use super::mission_spec::MissionTransitionSpec;
use crate::domain::model::Mission;
use crate::domain::state_machine::mission::MissionStatus;
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};

pub struct MissionTransitionResult {
    pub mission: Mission,
    pub from: MissionStatus,
    pub to: MissionStatus,
}

pub fn execute(
    board_dir: &Path,
    mission_id: &str,
    spec: &MissionTransitionSpec,
) -> Result<MissionTransitionResult> {
    let board = crate::infrastructure::loader::load_board(board_dir)?;
    let mission = board.require_mission(mission_id)?;
    let from_status = mission.status();

    if !spec.from.contains(&from_status) {
        return Err(anyhow!(
            "Cannot {} mission {} from {} state",
            spec.name,
            mission_id,
            from_status
        ));
    }

    let mut mutations = vec![Mutation::set("status", spec.to.to_string())];
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    if spec.timestamps.updated_at {
        mutations.push(Mutation::set("updated_at", &now));
    }
    if spec.timestamps.activated_at {
        mutations.push(Mutation::set("activated_at", &now));
    }
    if spec.timestamps.achieved_at {
        mutations.push(Mutation::set("achieved_at", &now));
    }
    if spec.timestamps.verified_at {
        mutations.push(Mutation::set("verified_at", &now));
    }

    let readme_path = mission.path.clone();
    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read mission README: {}", readme_path.display()))?;

    let updated_content = apply(&content, &mutations);
    fs::write(&readme_path, updated_content)
        .with_context(|| format!("Failed to write mission README: {}", readme_path.display()))?;

    // Reload mission to return updated state
    let board = crate::infrastructure::loader::load_board(board_dir)?;
    let updated_mission = board.require_mission(mission_id)?.clone();

    Ok(MissionTransitionResult {
        mission: updated_mission,
        from: from_status,
        to: spec.to,
    })
}
