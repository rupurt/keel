use anyhow::Result;
use chrono::Utc;
use owo_colors::OwoColorize;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::io::Write;
use std::path::Path;

use crate::cli::commands::management::next_support::algorithm::{
    MissionWorkSummary, mission_unmet_board_goals, mission_work_summary,
};
use crate::cli::commands::management::next_support::{ItemFilter, calculate_next, format_decision};
use keel::domain::model::{Bearing, Board, Mission, MissionStatus};
use keel::infrastructure::loader::load_board;
use keel::read_model::knowledge::{
    DetectionConfig, Knowledge, RisingPattern, detect_rising_patterns, filter_unapplied,
    scan_all_knowledge,
};
use keel::read_model::workflow_topology;

struct SelectedMission<'a> {
    mission: &'a Mission,
    summary: MissionWorkSummary,
}

/// Run the mission next command
pub fn run(mission_id: Option<&str>) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;

    if let Some(mission_id) = mission_id {
        let mission = board.require_mission(mission_id)?;
        if mission.status() == MissionStatus::Verified {
            flush_and_exit(1);
        }
        return render_mission_next(&board, &board_dir, mission);
    }

    if let Some(selected) = select_mission(&board) {
        print_selected_mission(&selected);
        return render_mission_next(&board, &board_dir, selected.mission);
    }

    let paused = paused_missions(&board);
    let orphaned_bearings = top_orphaned_bearings(&board, 3);
    let knowledge = scan_all_knowledge(&board_dir)?;
    let pending = top_pending_knowledge(&knowledge, 3);
    let patterns = top_rising_patterns(&knowledge, 3);
    println!(
        "{}",
        format_no_actionable_mission_message(&pending, &patterns, &paused, &orphaned_bearings)
    );
    flush_and_exit(1);
}

fn render_mission_next(board: &Board, board_dir: &Path, mission: &Mission) -> Result<()> {
    let topology = workflow_topology::load_for_board(board_dir)?;

    let mut role_families: BTreeSet<String> = topology.roles.keys().cloned().collect();
    role_families.insert(topology.management_role_example().to_string());
    role_families.insert(topology.delivery_role_example().to_string());

    println!("Next steps for mission {}:", mission.id().bold());
    println!();

    let mut found_any = false;

    for role_name in role_families {
        let role_taxonomy = keel::domain::model::taxonomy::parse(&role_name)?;
        let actor_context = topology.resolve_actor_context(&role_taxonomy)?;

        let agent_mode = matches!(
            actor_context.queue_lane,
            keel::read_model::queue_policy::ActorQueueLane::Execution
        );

        let filter = ItemFilter {
            mission_id: Some(mission.id()),
            actor_role: Some(&role_taxonomy),
        };

        let decision = calculate_next(board, board_dir, agent_mode, &filter)?;

        println!("{}:", role_name.bold().blue());
        println!("  {}", format_decision(&decision));
        found_any = true;
    }

    if !found_any {
        println!("No active roles or next steps found for this mission.");
    }

    Ok(())
}

fn select_mission(board: &Board) -> Option<SelectedMission<'_>> {
    let mut candidates: Vec<_> = board
        .missions
        .values()
        .filter_map(|mission| {
            let status_rank = selection_status_rank(mission.status())?;
            let unmet_goals = mission_unmet_board_goals(board, mission);
            let summary = mission_work_summary(board, mission, unmet_goals.len());

            Some((mission, summary, status_rank))
        })
        .collect();

    candidates.sort_by(compare_candidates);

    candidates
        .into_iter()
        .next()
        .map(|(mission, summary, _)| SelectedMission { mission, summary })
}

fn selection_status_rank(status: MissionStatus) -> Option<u8> {
    match status {
        MissionStatus::Active => Some(0),
        MissionStatus::Achieved => Some(1),
        MissionStatus::Defining => Some(2),
        MissionStatus::Paused | MissionStatus::Verified | MissionStatus::Abandoned => None,
    }
}

fn compare_candidates(
    left: &(&Mission, MissionWorkSummary, u8),
    right: &(&Mission, MissionWorkSummary, u8),
) -> Ordering {
    left.2
        .cmp(&right.2)
        .then_with(|| right.1.unmet_goals.cmp(&left.1.unmet_goals))
        .then_with(|| right.1.total_open_items().cmp(&left.1.total_open_items()))
        .then_with(|| right.1.open_epics.cmp(&left.1.open_epics))
        .then_with(|| right.1.open_voyages.cmp(&left.1.open_voyages))
        .then_with(|| right.1.open_stories.cmp(&left.1.open_stories))
        .then_with(|| right.1.open_bearings.cmp(&left.1.open_bearings))
        .then_with(|| right.1.open_adrs.cmp(&left.1.open_adrs))
        .then_with(|| left.0.id().cmp(right.0.id()))
}

fn print_selected_mission(selected: &SelectedMission<'_>) {
    println!(
        "Selected mission {} ({}): {}",
        selected.mission.id().bold(),
        selected.mission.status(),
        selected.mission.title().bold()
    );

    if let Some(outstanding) = render_outstanding_summary(&selected.summary) {
        println!("Outstanding: {outstanding}");
    }

    println!();
}

fn render_outstanding_summary(summary: &MissionWorkSummary) -> Option<String> {
    let mut parts = Vec::new();

    if summary.unmet_goals > 0 {
        parts.push(format_goal_count(summary.unmet_goals));
    }
    if summary.open_epics > 0 {
        parts.push(format_count(summary.open_epics, "open epic", "open epics"));
    }
    if summary.open_voyages > 0 {
        parts.push(format_count(
            summary.open_voyages,
            "open voyage",
            "open voyages",
        ));
    }
    if summary.open_stories > 0 {
        parts.push(format_count(
            summary.open_stories,
            "open story",
            "open stories",
        ));
    }
    if summary.open_bearings > 0 {
        parts.push(format_count(
            summary.open_bearings,
            "open bearing",
            "open bearings",
        ));
    }
    if summary.open_adrs > 0 {
        parts.push(format_count(summary.open_adrs, "open ADR", "open ADRs"));
    }

    (!parts.is_empty()).then(|| parts.join(", "))
}

fn format_goal_count(count: usize) -> String {
    format_count(count, "unmet board goal", "unmet board goals")
}

fn format_count(count: usize, singular: &str, plural: &str) -> String {
    format!("{} {}", count, if count == 1 { singular } else { plural })
}

fn paused_missions(board: &Board) -> Vec<Mission> {
    let mut paused: Vec<_> = board
        .missions
        .values()
        .filter(|mission| mission.status() == MissionStatus::Paused)
        .cloned()
        .collect();
    paused.sort_by(|left, right| left.id().cmp(right.id()));
    paused
}

fn top_orphaned_bearings(board: &Board, limit: usize) -> Vec<Bearing> {
    let mut orphaned: Vec<_> = board
        .bearings
        .values()
        .filter(|bearing| bearing.frontmatter.mission.is_none() && !bearing.is_complete())
        .cloned()
        .collect();
    orphaned.sort_by(|left, right| left.priority_key().cmp(&right.priority_key()));
    orphaned.truncate(limit);
    orphaned
}

fn top_pending_knowledge(knowledge: &[Knowledge], limit: usize) -> Vec<Knowledge> {
    let mut pending = filter_unapplied(knowledge.to_vec());
    pending.sort_by(|left, right| {
        right
            .confidence
            .partial_cmp(&left.confidence)
            .unwrap_or(Ordering::Equal)
            .then_with(|| {
                right
                    .score
                    .partial_cmp(&left.score)
                    .unwrap_or(Ordering::Equal)
            })
            .then_with(|| left.id.cmp(&right.id))
    });
    pending.truncate(limit);
    pending
}

fn top_rising_patterns(knowledge: &[Knowledge], limit: usize) -> Vec<RisingPattern> {
    let signals: Vec<_> = knowledge.iter().filter_map(Knowledge::to_signal).collect();
    let mut patterns = detect_rising_patterns(&signals, Utc::now(), &DetectionConfig::default());
    patterns.truncate(limit);
    patterns
}

fn format_no_actionable_mission_message(
    pending: &[Knowledge],
    patterns: &[RisingPattern],
    paused: &[Mission],
    orphaned_bearings: &[Bearing],
) -> String {
    let mut out = String::new();
    writeln!(out, "No actionable missions found.").unwrap();
    writeln!(
        out,
        "Returning a non-zero exit code so the harness halts until a human chooses the next direction."
    )
    .unwrap();

    if !paused.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Paused missions:").unwrap();
        for mission in paused {
            writeln!(out, "  - {} {}", mission.id(), mission.title()).unwrap();
        }
    }

    if !orphaned_bearings.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Orphaned bearings worth turning into missions:").unwrap();
        for bearing in orphaned_bearings {
            writeln!(out, "  - {} {}", bearing.id(), bearing.title().trim()).unwrap();
            writeln!(
                out,
                "    Suggested command: {}",
                suggested_mission_command(bearing.title())
            )
            .unwrap();
        }
    }

    if !pending.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Pending knowledge worth institutionalizing:").unwrap();
        for knowledge in pending {
            writeln!(out, "  - {} {}", knowledge.id, knowledge.title.trim()).unwrap();
        }
    }

    if !patterns.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "Rising patterns worth a human look:").unwrap();
        for pattern in patterns {
            writeln!(
                out,
                "  - {} (+{:.0}% trend, {} refs)",
                pattern.pattern_id(),
                pattern.trend_delta() * 100.0,
                pattern.evidence_ids().len()
            )
            .unwrap();
        }
    }

    writeln!(out).unwrap();
    writeln!(out, "Suggested human input:").unwrap();
    if !pending.is_empty() {
        writeln!(
            out,
            "  - Review `keel knowledge impact` to institutionalize pending knowledge before continuing."
        )
        .unwrap();
    }
    if !patterns.is_empty() {
        writeln!(
            out,
            "  - Review `keel knowledge explore` to see whether a new bearing or ADR is warranted."
        )
        .unwrap();
    }
    if !paused.is_empty() {
        writeln!(
            out,
            "  - Decide whether the paused missions should stay halted or be replaced by a new mission."
        )
        .unwrap();
    }
    if !orphaned_bearings.is_empty() {
        writeln!(
            out,
            "  - Create a mission for the orphaned bearings above, then assign each bearing's `mission:` field to the new mission ID."
        )
        .unwrap();
    }
    writeln!(
        out,
        "  - Create a fresh mission with `keel mission new \"<Title>\"` if no current mission should continue."
    )
    .unwrap();

    out.trim_end().to_string()
}

fn suggested_mission_command(bearing_title: &str) -> String {
    format!(
        "keel mission new \"{}\"",
        escape_double_quoted_argument(&suggested_mission_title(bearing_title))
    )
}

fn suggested_mission_title(bearing_title: &str) -> String {
    let trimmed = bearing_title.trim();
    for suffix in [
        " Research",
        " Exploration",
        " Investigation",
        " Discovery",
        " Study",
    ] {
        if let Some(stripped) = trimmed.strip_suffix(suffix) {
            let stripped = stripped.trim();
            if !stripped.is_empty() {
                return stripped.to_string();
            }
        }
    }
    trimmed.to_string()
}

fn escape_double_quoted_argument(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn flush_and_exit(code: i32) -> ! {
    let _ = std::io::stdout().flush();
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};
    use keel::domain::model::StoryState;
    use keel::read_model::knowledge::{KnowledgeSourceType, ReflectionSignal};
    use keel::test_helpers::{
        TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };
    use std::fs;
    use std::path::PathBuf;

    fn srs() -> &'static str {
        "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"
    }

    fn write_charter(board_dir: &Path, mission_id: &str, target: &str) {
        fs::write(
            board_dir
                .join("missions")
                .join(mission_id)
                .join("CHARTER.md"),
            format!(
                r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Goal for {mission_id} | board: {target} |
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn select_mission_prefers_active_mission_with_more_work() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .mission(TestMission::new("M2").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .epic(TestEpic::new("E2").mission("M2"))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .status("planned")
                    .srs_content(srs()),
            )
            .voyage(
                TestVoyage::new("V2", "E2")
                    .status("planned")
                    .srs_content(srs()),
            )
            .voyage(
                TestVoyage::new("V3", "E2")
                    .status("planned")
                    .srs_content(srs()),
            )
            .story(
                TestStory::new("S1")
                    .scope("E1/V1")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .scope("E2/V2")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .scope("E2/V3")
                    .status(StoryState::Backlog),
            )
            .build();

        write_charter(temp.path(), "M1", "E1");
        write_charter(temp.path(), "M2", "E2");

        let board = load_board(temp.path()).unwrap();
        let selected = select_mission(&board).expect("expected selected mission");

        assert_eq!(selected.mission.id(), "M2");
        assert_eq!(selected.summary.open_voyages, 2);
        assert_eq!(selected.summary.open_stories, 2);
    }

    #[test]
    fn select_mission_skips_paused_missions_even_with_more_work() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("paused"))
            .mission(TestMission::new("M2").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .epic(TestEpic::new("E2").mission("M2"))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .status("planned")
                    .srs_content(srs()),
            )
            .voyage(
                TestVoyage::new("V2", "E1")
                    .status("planned")
                    .srs_content(srs()),
            )
            .voyage(
                TestVoyage::new("V3", "E2")
                    .status("planned")
                    .srs_content(srs()),
            )
            .story(
                TestStory::new("S1")
                    .scope("E1/V1")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .scope("E1/V2")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .scope("E2/V3")
                    .status(StoryState::Backlog),
            )
            .build();

        write_charter(temp.path(), "M1", "E1");
        write_charter(temp.path(), "M2", "E2");

        let board = load_board(temp.path()).unwrap();
        let selected = select_mission(&board).expect("expected selected mission");

        assert_eq!(selected.mission.id(), "M2");
    }

    #[test]
    fn no_actionable_mission_message_surfaces_knowledge_and_patterns() {
        let pending = vec![Knowledge {
            id: "1AbCdE234".to_string(),
            source: PathBuf::from("stories/S1/REFLECT.md"),
            source_type: KnowledgeSourceType::Story,
            scope: Some("E1/V1".to_string()),
            source_story_id: Some("S1".to_string()),
            title: "Institutionalize planning guardrail".to_string(),
            category: "process".to_string(),
            context: "when missions stall without planning".to_string(),
            insight: "Unapplied mission guidance tends to cause drift.".to_string(),
            suggested_action: "Fold the rule into the mission workflow.".to_string(),
            applies_to: "AGENTS.md".to_string(),
            applied: String::new(),
            created_at: None,
            observed_at: Some(Utc.with_ymd_and_hms(2026, 1, 3, 12, 0, 0).unwrap()),
            score: 0.8,
            confidence: 0.95,
            linked_ids: Vec::new(),
            similar_to: None,
            similarity_score: None,
        }];

        let now = Utc.with_ymd_and_hms(2026, 1, 5, 12, 0, 0).unwrap();
        let signals = vec![
            ReflectionSignal {
                context_id: None,
                focus_area: Some("process".to_string()),
                score: 0.2,
                confidence: 0.92,
                observed_at: now - Duration::days(3),
                evidence_id: "K1".to_string(),
            },
            ReflectionSignal {
                context_id: None,
                focus_area: Some("process".to_string()),
                score: 0.5,
                confidence: 0.93,
                observed_at: now - Duration::days(2),
                evidence_id: "K2".to_string(),
            },
            ReflectionSignal {
                context_id: None,
                focus_area: Some("process".to_string()),
                score: 0.9,
                confidence: 0.94,
                observed_at: now - Duration::days(1),
                evidence_id: "K3".to_string(),
            },
        ];
        let patterns = detect_rising_patterns(&signals, now, &DetectionConfig::default());

        let message = format_no_actionable_mission_message(&pending, &patterns, &[], &[]);

        assert!(message.contains("No actionable missions found."));
        assert!(message.contains("Pending knowledge worth institutionalizing"));
        assert!(message.contains("1AbCdE234 Institutionalize planning guardrail"));
        assert!(message.contains("Rising patterns worth a human look"));
        assert!(message.contains("focus:process"));
        assert!(message.contains("keel knowledge impact"));
        assert!(message.contains("keel knowledge explore"));
        assert!(message.contains("keel mission new"));
    }

    #[test]
    fn top_orphaned_bearings_prioritizes_open_unmissioned_bearings() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .bearing(
                TestBearing::new("B1")
                    .title("Payments Research")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .bearing(TestBearing::new("B2").status("laid"))
            .bearing(TestBearing::new("B3").status("exploring").mission("M1"))
            .bearing(
                TestBearing::new("B4")
                    .status("evaluating")
                    .has_evidence(true),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let orphaned = top_orphaned_bearings(&board, 10);

        let orphaned_ids: Vec<_> = orphaned.iter().map(|bearing| bearing.id()).collect();
        assert_eq!(orphaned_ids, vec!["B1", "B4"]);
    }

    #[test]
    fn no_actionable_mission_message_recommends_missions_for_orphaned_bearings() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("B1")
                    .title("Payments Research")
                    .status("ready")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let orphaned = top_orphaned_bearings(&board, 3);

        let message = format_no_actionable_mission_message(&[], &[], &[], &orphaned);

        assert!(message.contains("Orphaned bearings worth turning into missions"));
        assert!(message.contains("B1 Payments Research"));
        assert!(message.contains("keel mission new \"Payments\""));
        assert!(message.contains("assign each bearing's `mission:` field"));
    }

    #[test]
    fn suggested_mission_title_strips_research_suffixes() {
        assert_eq!(suggested_mission_title("Payments Research"), "Payments");
        assert_eq!(
            suggested_mission_title("Temporal Automation Exploration"),
            "Temporal Automation"
        );
        assert_eq!(
            suggested_mission_title("Strategic Capacity"),
            "Strategic Capacity"
        );
    }
}
