//! Attach command - assign a mission child to a mission.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use keel::infrastructure::config::find_board_dir;
use keel::infrastructure::frontmatter_mutation::{Mutation, apply};
use keel::infrastructure::loader::load_board;

struct ResolvedAttachTarget {
    kind_label: &'static str,
    kind_title: &'static str,
    id: String,
    path: PathBuf,
    current_mission: Option<String>,
    stamp_updated_at: bool,
}

/// Assign a mission child to a mission.
pub fn run(
    mission_id: &str,
    bearing_id: Option<&str>,
    epic_id: Option<&str>,
    adr_id: Option<&str>,
) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, mission_id, bearing_id, epic_id, adr_id)
}

/// Assign a mission child to a mission with an explicit board directory.
pub fn run_with_dir(
    board_dir: &Path,
    mission_id: &str,
    bearing_id: Option<&str>,
    epic_id: Option<&str>,
    adr_id: Option<&str>,
) -> Result<()> {
    let board = load_board(board_dir)?;
    let mission = board.require_mission(mission_id)?;
    let target = resolve_target(&board, bearing_id, epic_id, adr_id)?;

    if let Some(current_mission) = target.current_mission.as_deref() {
        if current_mission == mission_id {
            return Err(anyhow!(
                "{} {} is already attached to mission {}. No change was made.",
                target.kind_title,
                target.id,
                current_mission
            ));
        }
        return Err(anyhow!(
            "{} {} is already attached to mission {}. Remove or update `mission:` in {} first, then retry.",
            target.kind_title,
            target.id,
            current_mission,
            target.path.display()
        ));
    }

    let content = fs::read_to_string(&target.path).with_context(|| {
        format!(
            "Failed to read {}: {}",
            target.kind_label,
            target.path.display()
        )
    })?;

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let mut mutations = vec![Mutation::set("mission", mission.id())];
    if target.stamp_updated_at {
        mutations.push(Mutation::set("updated_at", &now));
    }
    let updated_content = apply(&content, &mutations);

    fs::write(&target.path, updated_content).with_context(|| {
        format!(
            "Failed to write {}: {}",
            target.kind_label,
            target.path.display()
        )
    })?;

    println!(
        "Attached {} {} to mission {}",
        target.kind_label,
        target.id,
        mission.id()
    );

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

fn resolve_target(
    board: &keel::domain::model::Board,
    bearing_id: Option<&str>,
    epic_id: Option<&str>,
    adr_id: Option<&str>,
) -> Result<ResolvedAttachTarget> {
    match (bearing_id, epic_id, adr_id) {
        (Some(bearing_id), None, None) => {
            let bearing = board.require_bearing(bearing_id)?;
            Ok(ResolvedAttachTarget {
                kind_label: "bearing",
                kind_title: "Bearing",
                id: bearing.id().to_string(),
                path: bearing.path.clone(),
                current_mission: bearing.frontmatter.mission.clone(),
                stamp_updated_at: true,
            })
        }
        (None, Some(epic_id), None) => {
            let epic = board.require_epic(epic_id)?;
            Ok(ResolvedAttachTarget {
                kind_label: "epic",
                kind_title: "Epic",
                id: epic.id().to_string(),
                path: epic.path.clone(),
                current_mission: epic.frontmatter.mission.clone(),
                stamp_updated_at: false,
            })
        }
        (None, None, Some(adr_id)) => {
            let adr = board.require_adr(adr_id)?;
            Ok(ResolvedAttachTarget {
                kind_label: "ADR",
                kind_title: "ADR",
                id: adr.id().to_string(),
                path: adr.path.clone(),
                current_mission: adr.frontmatter.mission.clone(),
                stamp_updated_at: false,
            })
        }
        _ => Err(anyhow!(
            "Exactly one of --bearing, --epic, or --adr must be provided."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestAdr, TestBearing, TestBoardBuilder, TestEpic, TestMission};

    #[test]
    fn attach_adds_mission_to_bearing() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Test Mission"))
            .bearing(TestBearing::new("B1").title("Researching Mission Lineage"))
            .build();
        let board_dir = board.path();

        run_with_dir(board_dir, "M1", Some("B1"), None, None).unwrap();

        let readme = fs::read_to_string(board_dir.join("bearings/B1/README.md")).unwrap();
        assert!(readme.contains("mission: M1"));
        assert!(readme.contains("updated_at:"));
    }

    #[test]
    fn attach_adds_mission_to_epic() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission With Epic"))
            .epic(TestEpic::new("E1").title("Attached Epic"))
            .build();
        let board_dir = board.path();

        run_with_dir(board_dir, "M1", None, Some("E1"), None).unwrap();

        let readme = fs::read_to_string(board_dir.join("epics/E1/README.md")).unwrap();
        assert!(readme.contains("mission: M1"));
    }

    #[test]
    fn attach_adds_mission_to_adr() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission With ADR"))
            .adr(TestAdr::new("ADR-0001").title("Attached ADR"))
            .build();
        let board_dir = board.path();

        run_with_dir(board_dir, "M1", None, None, Some("ADR-0001")).unwrap();

        let readme = fs::read_to_string(board_dir.join("adrs/ADR-0001/README.md")).unwrap();
        assert!(readme.contains("mission: M1"));
    }

    #[test]
    fn attach_fails_if_mission_is_missing() {
        let board = TestBoardBuilder::new()
            .bearing(TestBearing::new("B1"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "MISSING", Some("B1"), None, None);

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

        let result = run_with_dir(board_dir, "M1", Some("MISSING"), None, None);

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

        let result = run_with_dir(board_dir, "M1", Some("B1"), None, None);

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

        let result = run_with_dir(board_dir, "M1", Some("B1"), None, None);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("already attached to mission M2"));
        assert!(err.contains("README.md"));
    }

    #[test]
    fn attach_fails_if_epic_is_assigned_to_other_mission() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1"))
            .mission(TestMission::new("M2"))
            .epic(TestEpic::new("E1").mission("M2"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "M1", None, Some("E1"), None);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Epic E1 is already attached to mission M2"));
        assert!(err.contains("README.md"));
    }

    #[test]
    fn attach_fails_if_adr_is_assigned_to_other_mission() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1"))
            .mission(TestMission::new("M2"))
            .adr(TestAdr::new("ADR-0001").mission("M2"))
            .build();
        let board_dir = board.path();

        let result = run_with_dir(board_dir, "M1", None, None, Some("ADR-0001"));

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("ADR ADR-0001 is already attached to mission M2"));
        assert!(err.contains("README.md"));
    }

    #[test]
    fn attach_requires_exactly_one_target_flag() {
        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1"))
            .bearing(TestBearing::new("B1"))
            .epic(TestEpic::new("E1"))
            .build();
        let board_dir = board.path();

        let none = run_with_dir(board_dir, "M1", None, None, None);
        assert!(none.is_err());
        assert!(
            none.unwrap_err()
                .to_string()
                .contains("Exactly one of --bearing, --epic, or --adr")
        );

        let multiple = run_with_dir(board_dir, "M1", Some("B1"), Some("E1"), None);
        assert!(multiple.is_err());
        assert!(
            multiple
                .unwrap_err()
                .to_string()
                .contains("Exactly one of --bearing, --epic, or --adr")
        );
    }

    #[test]
    fn attach_followed_by_activate_uses_mission_owned_bearing() {
        use std::fs;

        let board = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("defining"))
            .bearing(TestBearing::new("B1"))
            .build();
        let board_dir = board.path();

        run_with_dir(board_dir, "M1", Some("B1"), None, None).unwrap();

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
