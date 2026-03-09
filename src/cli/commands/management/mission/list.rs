//! List missions command

use anyhow::Result;

use crate::cli::table::Table;
use crate::domain::model::Mission;
use crate::infrastructure::loader::load_board;

/// List all missions
pub fn run() -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;

    let mut missions: Vec<&Mission> = board.missions.values().collect();
    missions.sort_by(|a, b| a.id().cmp(b.id()));

    if missions.is_empty() {
        println!("No missions found on this board.");
        return Ok(());
    }

    let mut table = Table::new(&["ID", "TITLE", "STATUS", "CHILDREN"]);
    for mission in missions {
        let child_count = board.mission_child_count(mission.id());
        table.row(&[
            &crate::cli::style::styled_story_id(mission.id()),
            mission.title(),
            &crate::cli::style::styled_mission_status(&mission.status()),
            &child_count.to_string(),
        ]);
    }
    table.print();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBearing, TestBoardBuilder, TestEpic, TestMission};

    #[test]
    fn test_list_missions_displays_table() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One"))
            .mission(TestMission::new("M2").title("Mission Two"))
            .epic(TestEpic::new("E1").mission("M1"))
            .bearing(TestBearing::new("B1").mission("M1"))
            .build();

        // The Table print goes to stdout, so we can't easily capture it in a unit test
        // but we can verify the logic by exposing a collection method.
        let board = load_board(temp.path()).unwrap();
        let mut missions: Vec<_> = board.missions.values().collect();
        missions.sort_by(|a, b| a.id().cmp(b.id()));

        assert_eq!(missions.len(), 2);
        assert_eq!(missions[0].id(), "M1");
        assert_eq!(board.mission_child_count("M1"), 2);
        assert_eq!(board.mission_child_count("M2"), 0);
    }
}
