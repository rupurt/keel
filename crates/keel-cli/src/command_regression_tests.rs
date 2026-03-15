//! Command behavior regression suite for migration parity.
//!
//! These tests cover key command-adjacent flows (`next`, `flow`, lifecycle
//! transitions) so refactors preserve observed behavior.

use crate::cli::commands::management::next::NextDecision;
use crate::cli::presentation::drift_surface::{
    render_drift_coverage, render_drift_overview, render_drift_show_section,
};
use crate::cli::presentation::knowledge_graph::{
    KnowledgeGraphZoom, build_knowledge_graph_view, render_knowledge_graph,
};
use crate::cli::presentation::show::ShowDocument;
use crate::cli::presentation::topology::render_topology;
use keel::domain::model::StoryState;
use keel::domain::policy::queue::{FLOW_VERIFY_BLOCK_THRESHOLD, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD};
use keel::read_model::knowledge_graph::{
    build_knowledge_graph_projection, build_structural_drift_summary,
};
use keel::read_model::show_selector::{ShowEntityKind, ordered_show_ids, resolve_show_selector};
use keel::read_model::world_map::{TopologyZoom, WorldMapBuildOptions, build_world_map_projection};
use keel::test_helpers::{TestAdr, TestBearing};
use keel::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};
use std::fs;

fn board_with_verification_and_ready(verify_count: usize, ready_count: usize) -> tempfile::TempDir {
    let mut builder = TestBoardBuilder::new();
    for i in 0..verify_count {
        let id = format!("VERIFY{:02}", i + 1);
        builder = builder.story(TestStory::new(&id).status(StoryState::NeedsHumanVerification));
    }
    for i in 0..ready_count {
        let id = format!("READY{:02}", i + 1);
        builder = builder.story(TestStory::new(&id).status(StoryState::Backlog));
    }
    builder.build()
}

fn graph_drift_fixture() -> tempfile::TempDir {
    let temp = TestBoardBuilder::new()
        .mission(TestMission::new("M1").title("Mission One").status("active"))
        .epic(TestEpic::new("E1").title("Epic One").mission("M1"))
        .voyage(TestVoyage::new("V1", "E1").title("Voyage One").status("planned"))
        .story(
            TestStory::new("S1")
                .title("Story One")
                .scope("E1/V1")
                .status(StoryState::Done)
                .body(
                    "# Summary\n\nGraph drift.\n\n## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] done <!-- verify: manual, SRS-01:start:end -->\n",
                ),
        )
        .build();

    fs::create_dir_all(temp.path().join("knowledge")).unwrap();
    fs::write(
        temp.path().join("knowledge/graph.md"),
        r#"---
source_type: Adhoc
source: knowledge/graph.md
---

### 1AbCdE241: Drift Surfaces Reuse Canonical Projections

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | drift |
| **Insight** | Reuse one drift summary across graph-adjacent surfaces |
| **Suggested Action** | Avoid command-local scanners |
| **Applies To** | src/orphan.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-12T00:00:00Z |
| **Score** | 0.90 |
| **Confidence** | 0.95 |
| **Applied** |  |
"#,
    )
    .unwrap();
    fs::write(temp.path().join("README.md"), "# Project README\n").unwrap();
    fs::write(temp.path().join("ARCHITECTURE.md"), "# Architecture\n").unwrap();
    fs::create_dir_all(temp.path().join("src")).unwrap();
    fs::write(temp.path().join("src/lib.rs"), "pub mod orphan;\n").unwrap();
    fs::write(temp.path().join("src/orphan.rs"), "pub fn orphan() {}\n").unwrap();

    temp
}

fn write_routine(root: &std::path::Path, id: &str, title: &str, target_scope: &str) {
    let routine_dir = root.join("routines").join(id);
    fs::create_dir_all(&routine_dir).unwrap();
    fs::write(
        routine_dir.join("README.md"),
        format!(
            r#"---
id: {id}
title: {title}
cadence:
  cron: 0 9 * * 1
target-scope: {target_scope}
created_at: 2026-03-11T09:00:00
updated_at: 2026-03-11T09:30:00
---

# Blueprint

- Review the open backlog.
"#
        ),
    )
    .unwrap();
}

fn head_show_fixture() -> tempfile::TempDir {
    let temp = TestBoardBuilder::new()
        .mission(TestMission::new("M2").title("Mission Two").status("active"))
        .mission(TestMission::new("M1").title("Mission One").status("active"))
        .epic(TestEpic::new("E2").title("Epic Two").mission("M2"))
        .epic(TestEpic::new("E1").title("Epic One").mission("M1"))
        .voyage(TestVoyage::new("V2", "E2").index(2).status("planned"))
        .voyage(TestVoyage::new("V1", "E1").index(1).status("in-progress"))
        .story(
            TestStory::new("S2")
                .scope("E2/V2")
                .index(2)
                .status(StoryState::Backlog),
        )
        .story(
            TestStory::new("S1")
                .scope("E1/V1")
                .index(1)
                .status(StoryState::InProgress),
        )
        .bearing(TestBearing::new("B2").title("Bearing Two").status("ready"))
        .bearing(
            TestBearing::new("B1")
                .title("Bearing One")
                .status("exploring"),
        )
        .adr(TestAdr::new("ADR-002").status("accepted"))
        .adr(TestAdr::new("ADR-001").status("proposed"))
        .build();

    write_routine(temp.path(), "routine-zeta", "Zeta Review", "E2/V2");
    write_routine(temp.path(), "routine-alpha", "Alpha Review", "E1/V1");

    temp
}

fn command_help(args: &[&str]) -> String {
    crate::cli::build_cli()
        .try_get_matches_from(args.iter().copied())
        .unwrap_err()
        .to_string()
}

#[test]
fn regression_next_and_flow_align_on_human_blocked_boundary() {
    let temp = board_with_verification_and_ready(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD, 1);
    let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();

    let next = crate::cli::commands::management::next::calculate_next(
        &board,
        temp.path(),
        false,
        &crate::cli::commands::management::next::ItemFilter::none(),
    )
    .unwrap();
    assert!(
        matches!(next, NextDecision::Blocked(_)),
        "human next should be blocked at policy threshold"
    );

    let metrics = keel::read_model::flow_metrics::calculate_metrics(&board);
    let health = crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(&metrics);
    assert!(
        health.action_summary.to_lowercase().contains("blocked"),
        "flow summary should indicate blocked human queue at threshold"
    );
}

#[test]
fn regression_next_and_flow_align_on_flow_blocked_boundary() {
    let temp = board_with_verification_and_ready(FLOW_VERIFY_BLOCK_THRESHOLD + 1, 1);
    let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();

    let next = crate::cli::commands::management::next::calculate_next(
        &board,
        temp.path(),
        false,
        &crate::cli::commands::management::next::ItemFilter::none(),
    )
    .unwrap();
    assert!(
        matches!(next, NextDecision::Blocked(_)),
        "human next should be blocked when flow is verify-blocked"
    );

    let metrics = keel::read_model::flow_metrics::calculate_metrics(&board);
    let health = crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(&metrics);
    assert!(
        health
            .action_summary
            .to_lowercase()
            .contains("verification queue is blocked"),
        "flow summary should indicate verification queue blocked"
    );
}

#[test]
fn regression_story_lifecycle_command_chain_reaches_done() {
    let temp = TestBoardBuilder::new()
        .story(
            TestStory::new("REGCHAIN1")
                .status(StoryState::Backlog)
                .body(
                    "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Manual check <!-- verify: manual, SRS-01:start:end -->",
                ),
        )
        .build();

    crate::cli::commands::management::story::start::run(temp.path(), "REGCHAIN1", None).unwrap();

    let evidence_dir = temp.path().join("stories/REGCHAIN1/EVIDENCE");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::write(
        temp.path().join("stories/REGCHAIN1/REFLECT.md"),
        "# Reflection\n\n## Knowledge\n\n## Observations\n\nNo reusable insight captured.\n",
    )
    .unwrap();

    crate::cli::commands::management::story::submit::run(temp.path(), "REGCHAIN1").unwrap();
    crate::cli::commands::management::story::accept::run(temp.path(), "REGCHAIN1", "manager", None)
        .unwrap();

    let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
    let story = board.require_story("REGCHAIN1").unwrap();
    assert_eq!(story.status, StoryState::Done);

    let content = fs::read_to_string(temp.path().join("stories/REGCHAIN1/README.md")).unwrap();
    assert!(content.contains("submitted_at:"));
    assert!(content.contains("completed_at:"));
}

#[test]
fn graph_drift_surfaces_reuse_canonical_projection() {
    let temp = graph_drift_fixture();
    let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
    let mission = board.require_mission("M1").unwrap();
    let epic = board.require_epic("E1").unwrap();
    let voyage = board.require_voyage("V1").unwrap();

    let graph_projection = build_knowledge_graph_projection(&board).unwrap();
    let drift = build_structural_drift_summary(&graph_projection);
    let expected_overview = render_drift_overview("Drift", &drift);
    let expected_radar = render_drift_overview("Drift Radar", &drift);
    let expected_coverage = render_drift_coverage(&drift);

    let graph_render = render_knowledge_graph(
        &build_knowledge_graph_view(&graph_projection, KnowledgeGraphZoom::Source, None),
        120,
    );
    assert!(graph_render.contains(&expected_overview));
    assert!(graph_render.contains(&expected_coverage));

    let topology_projection = build_world_map_projection(
        &board,
        WorldMapBuildOptions {
            zoom: TopologyZoom::Story,
            focus_id: None,
            include_done: true,
            reference_time: None,
        },
    )
    .unwrap();
    let topology_render = render_topology(&topology_projection, 120);
    assert!(topology_render.contains(&expected_radar));
    assert!(topology_render.contains(&expected_coverage));

    let mission_projection =
        keel::read_model::mission_show::build_projection(&board, mission).unwrap();
    let epic_projection =
        keel::read_model::planning_show::build_epic_show_projection(&board, epic).unwrap();
    let voyage_projection =
        keel::read_model::planning_show::build_voyage_show_projection(&board, voyage).unwrap();
    for summary in [
        mission_projection.drift,
        epic_projection.drift,
        voyage_projection.drift,
    ] {
        let mut document = ShowDocument::new();
        document.push_sections_spaced([render_drift_show_section(&summary)]);
        let rendered = document.render();
        assert!(rendered.contains(&format!(
            "{:.2} ({})",
            drift.coefficient,
            drift.severity_label()
        )));
        assert!(rendered.contains(&expected_coverage));
    }
}

#[test]
fn head_show_commands_resolve_management_entities() {
    let temp = head_show_fixture();

    crate::cli::commands::management::mission::show::run_with_dir(temp.path(), "HEAD", false)
        .unwrap();
    crate::cli::commands::management::mission::show::run_with_dir(temp.path(), "M2", false)
        .unwrap();

    crate::cli::commands::management::epic::show::run_with_dir(temp.path(), "HEAD").unwrap();
    crate::cli::commands::management::epic::show::run_with_dir(temp.path(), "E2").unwrap();

    crate::cli::commands::management::voyage::show::run_with_dir(temp.path(), "HEAD").unwrap();
    crate::cli::commands::management::voyage::show::run_with_dir(temp.path(), "V2").unwrap();

    crate::cli::commands::management::story::show::run_with_dir(temp.path(), "HEAD").unwrap();
    crate::cli::commands::management::story::show::run_with_dir(temp.path(), "S2").unwrap();
}

#[test]
fn head_show_commands_resolve_governance_entities() {
    let temp = head_show_fixture();

    crate::cli::commands::management::bearing::show::run_with_dir(temp.path(), "HEAD").unwrap();
    crate::cli::commands::management::bearing::show::run_with_dir(temp.path(), "B2").unwrap();

    crate::cli::commands::management::adr::show::run_with_dir(temp.path(), "HEAD").unwrap();
    crate::cli::commands::management::adr::show::run_with_dir(temp.path(), "ADR-002").unwrap();

    crate::cli::commands::management::routine::show::run_with_dir(temp.path(), "HEAD").unwrap();
    crate::cli::commands::management::routine::show::run_with_dir(temp.path(), "routine-zeta")
        .unwrap();
}

#[test]
fn head_show_commands_report_selector_errors() {
    let empty = TestBoardBuilder::new().build();
    let err =
        crate::cli::commands::management::mission::show::run_with_dir(empty.path(), "HEAD", false)
            .unwrap_err();
    assert_eq!(
        err.to_string(),
        "No missions available for selector `HEAD`."
    );

    let fixture = TestBoardBuilder::new()
        .epic(TestEpic::new("E1"))
        .voyage(TestVoyage::new("V1", "E1").status("planned"))
        .story(TestStory::new("S1").scope("E1/V1").index(1))
        .build();
    let err = crate::cli::commands::management::story::show::run_with_dir(fixture.path(), "HEAD~")
        .unwrap_err();
    assert_eq!(
        err.to_string(),
        "Selector `HEAD~` is out of range for stories (available: 1)."
    );

    let fixture = head_show_fixture();
    let err =
        crate::cli::commands::management::routine::show::run_with_dir(fixture.path(), "HEAD~3")
            .unwrap_err();
    assert_eq!(
        err.to_string(),
        "Unsupported HEAD selector syntax `HEAD~3`. Supported forms: exact IDs, HEAD, HEAD~, HEAD~~, HEAD^"
    );
}

#[test]
fn head_show_commands_reject_invalid_syntax() {
    let temp = head_show_fixture();
    let expected = "Unsupported HEAD selector syntax `HEAD~3`. Supported forms: exact IDs, HEAD, HEAD~, HEAD~~, HEAD^";

    let cases = [
        crate::cli::commands::management::mission::show::run_with_dir(temp.path(), "HEAD~3", false)
            .unwrap_err()
            .to_string(),
        crate::cli::commands::management::epic::show::run_with_dir(temp.path(), "HEAD~3")
            .unwrap_err()
            .to_string(),
        crate::cli::commands::management::voyage::show::run_with_dir(temp.path(), "HEAD~3")
            .unwrap_err()
            .to_string(),
        crate::cli::commands::management::story::show::run_with_dir(temp.path(), "HEAD~3")
            .unwrap_err()
            .to_string(),
        crate::cli::commands::management::bearing::show::run_with_dir(temp.path(), "HEAD~3")
            .unwrap_err()
            .to_string(),
        crate::cli::commands::management::adr::show::run_with_dir(temp.path(), "HEAD~3")
            .unwrap_err()
            .to_string(),
        crate::cli::commands::management::routine::show::run_with_dir(temp.path(), "HEAD~3")
            .unwrap_err()
            .to_string(),
    ];

    for actual in cases {
        assert_eq!(actual, expected);
    }
}

#[test]
fn head_show_contract_matches_default_list_order() {
    let temp = head_show_fixture();
    let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();

    let expectations = [
        (ShowEntityKind::Mission, vec!["M1", "M2"]),
        (ShowEntityKind::Epic, vec!["E1", "E2"]),
        (ShowEntityKind::Voyage, vec!["V1", "V2"]),
        (ShowEntityKind::Story, vec!["S1", "S2"]),
        (ShowEntityKind::Bearing, vec!["B1", "B2"]),
        (ShowEntityKind::Adr, vec!["ADR-001", "ADR-002"]),
        (
            ShowEntityKind::Routine,
            vec!["routine-alpha", "routine-zeta"],
        ),
    ];

    for (kind, expected) in expectations {
        let expected_ids = expected.into_iter().map(str::to_string).collect::<Vec<_>>();
        assert_eq!(ordered_show_ids(temp.path(), &board, kind), expected_ids);
        assert_eq!(
            resolve_show_selector(temp.path(), &board, kind, "HEAD").unwrap(),
            expected_ids[0]
        );
        assert_eq!(
            resolve_show_selector(temp.path(), &board, kind, "HEAD~").unwrap(),
            expected_ids[1]
        );
        assert_eq!(
            resolve_show_selector(temp.path(), &board, kind, "HEAD^").unwrap(),
            expected_ids[1]
        );
    }
}

#[test]
fn head_show_guidance_contract() {
    let expected = "ID or HEAD selector (HEAD, HEAD~, HEAD~~, HEAD^)";

    for help in [
        command_help(&["keel", "mission", "show", "--help"]),
        command_help(&["keel", "epic", "show", "--help"]),
        command_help(&["keel", "voyage", "show", "--help"]),
        command_help(&["keel", "story", "show", "--help"]),
        command_help(&["keel", "bearing", "show", "--help"]),
        command_help(&["keel", "adr", "show", "--help"]),
        command_help(&["keel", "routine", "show", "--help"]),
    ] {
        assert!(
            help.contains(expected),
            "show help should advertise the supported HEAD selector forms: {help}"
        );
    }
}
