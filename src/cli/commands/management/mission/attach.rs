//! Attach command - assign a bearing to a mission.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use keel::infrastructure::config::find_board_dir;
use keel::infrastructure::frontmatter_mutation::{Mutation, apply};
use keel::infrastructure::loader::load_board;

/// Assign a bearing to a mission.
pub fn run(mission_id: &str, bearing_id: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, mission_id, bearing_id)
}

/// Assign a bearing to a mission with an explicit board directory.
pub fn run_with_dir(board_dir: &Path, mission_id: &str, bearing_id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let mission = board.require_mission(mission_id)?;
    let bearing = board.require_bearing(bearing_id)?;

    if let Some(current_mission) = bearing.frontmatter.mission.as_deref() {
        if current_mission == mission_id {
            return Err(anyhow!(
                "Bearing {} is already attached to mission {}. No change was made.",
                bearing.id(),
                current_mission
            ));
        }
        return Err(anyhow!(
            "Bearing {} is already attached to mission {}. Remove or update `mission:` in {}/README.md first, then retry.",
            bearing.id(),
            current_mission,
            bearing.path.display()
        ));
    }

    let content = fs::read_to_string(&bearing.path)
        .with_context(|| format!("Failed to read bearing: {}", bearing.path.display()))?;

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let updated_content = apply(
        &content,
        &[
            Mutation::set("mission", mission.id()),
            Mutation::set("updated_at", &now),
        ],
    );

    fs::write(&bearing.path, updated_content)
        .with_context(|| format!("Failed to write bearing: {}", bearing.path.display()))?;

    println!(
        "Attached bearing {} to mission {}",
        bearing.id(),
        mission.id()
    );

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestBearing, TestBoardBuilder, TestMission};

    #[test]
    fn attach_adds_mission_to_bearing() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Test Mission"))
            .bearing(TestBearing::new("B1").title("Researching Mission Lineage"))
            .build();
        let board_dir = board.path();

        run_with_dir(board_dir, "M1", "B1").unwrap();

        let readme = fs::read_to_string(board_dir.join("bearings/B1/README.md")).unwrap();
        assert!(readme.contains("mission: M1"));
        assert!(readme.contains("updated_at:"));
    }

    #[test]
    fn attach_fails_if_mission_is_missing() {
        let board = TestBoardBuilder::new()
            .bearing(TestBearing::new("B1"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "MISSING", "B1");

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Mission not found")
        );
    }

    #[test]
    fn attach_fails_if_bearing_is_missing() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "M1", "MISSING");

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Bearing not found")
        );
    }

    #[test]
    fn attach_fails_if_bearing_already_assigned_to_same_mission() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1"))
            .bearing(TestBearing::new("B1").mission("M1"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "M1", "B1");

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("already attached to mission M1")
        );
    }

    #[test]
    fn attach_fails_if_bearing_is_assigned_to_other_mission() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1"))
            .mission(TestMission::new("M2"))
            .bearing(TestBearing::new("B1").mission("M2"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "M1", "B1");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("already attached to mission M2"));
        assert!(err.contains("README.md"));
    }

    #[test]
    fn attach_followed_by_activate_uses_mission_owned_bearing() {
        use std::fs;

        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("defining"))
            .bearing(TestBearing::new("B1"))
            .build();
        let board_dir = board.path();

        run_with_dir(board_dir, "M1", "B1").unwrap();

        let charter_path = board_dir.join("missions/M1/CHARTER.md");
        fs::write(
            charter_path,
            r#"
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Attach mission-bearing lineage | board: B1 |

## Constraints

- Keep all bearing lineage updates in command surfaces.

## Halting Rules

- Do not halt while any MG-* goal has unfinished board work.
- Halt when mission-bearing lineage is explicit and consistent.
- Yield to human when only non-board goals remain.
"#,
        )
        .unwrap();

        keel::application::mission_lifecycle::MissionLifecycleService::activate(board_dir, "M1")
            .unwrap();

        let readme = fs::read_to_string(board_dir.join("missions/M1/README.md")).unwrap();
        assert!(readme.contains("status: active"));
    }
}
