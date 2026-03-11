//! Flow command - aggregate pull-system diagnostics

use anyhow::Result;

use crate::cli::presentation::flow::display::render_annotated_flow;
use crate::cli::presentation::terminal::get_terminal_width;
use crate::infrastructure::loader::load_board;
use crate::read_model::{flow_status, workflow_lane_flow, workflow_topology};

/// Run the flow command
pub fn run(board_dir: &std::path::Path, no_color: bool) -> Result<()> {
    let output = build_output(board_dir, no_color)?;
    println!("{}", output);

    Ok(())
}

fn build_output(board_dir: &std::path::Path, no_color: bool) -> Result<String> {
    let board = load_board(board_dir)?;
    let width = get_terminal_width();

    let metrics = flow_status::project(&board);
    let topology = workflow_topology::load_for_board(board_dir)?;
    let lane_flow = workflow_lane_flow::project(&board, &topology);

    Ok(render_annotated_flow(
        &board, &metrics, &lane_flow, width, no_color,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBearing, TestBoardBuilder, TestStory};
    use std::fs;

    #[test]
    fn test_flow_run() {
        let temp = TestBoardBuilder::new().build();
        let result = build_output(temp.path(), true);
        assert!(result.is_ok());
    }

    #[test]
    fn build_output_renders_configured_lanes_in_priority_order() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::NeedsHumanVerification))
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .story(TestStory::new("S3").status(StoryState::Done))
            .bearing(TestBearing::new("B1").status("exploring"))
            .build();
        fs::write(
            temp.path().join("keel.toml"),
            r#"[workflow.defaults]
management_role = "reviewer"
delivery_role = "maker"
management_lane = "review"
delivery_lane = "delivery"

[roles.reviewer]
default_lane = "review"
operational_contract = "reviewer-core"

[roles.maker]
default_lane = "delivery"
operational_contract = "maker-core"

[roles.researcher]
default_lane = "research"
operational_contract = "researcher-core"

[lanes.review]
description = "Manual review work"
include = ["story.needs-human-verification"]
exclude = []
parallel = false
manual_accept = true
priority = 300

[lanes.delivery]
description = "Delivery work"
include = ["story.*"]
exclude = ["story.done", "story.icebox", "story.needs-human-verification", "story.rejected"]
parallel = true
manual_accept = false
priority = 200

[lanes.research]
description = "Research work"
include = ["bearing.exploring"]
exclude = []
parallel = false
manual_accept = false
priority = 100
"#,
        )
        .unwrap();

        let output = build_output(temp.path(), true).unwrap();
        let review = output.find("review (1) [p300]").unwrap();
        let delivery = output.find("delivery (1) [p200]").unwrap();
        let research = output.find("research (1) [p100]").unwrap();

        assert!(review < delivery);
        assert!(delivery < research);
        assert!(output.contains("story.needs-human-verification"));
        assert!(output.contains("story.backlog"));
        assert!(output.contains("bearing.exploring"));
        assert!(!output.contains("story.done"));
    }
}
