//! `keel pulse` command adapter.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;

use keel::domain::model::{AdrStatus, Board, StoryFrontmatter, StoryState, StoryType};
use keel::infrastructure::loader::load_board;
use keel::infrastructure::story_id::generate_story_id;
use keel::read_model::scheduled_routines::{
    RoutineScheduleFilter, ScheduledRoutineGatingReason, ScheduledRoutineProjection,
    ScheduledRoutineState, project_scheduled_routines,
};

const MATERIALIZATION_MARKER_PREFIX: &str = "<!-- keel:pulse-materialization: ";
const MATERIALIZATION_MARKER_SUFFIX: &str = " -->";

/// Run the pulse command.
pub fn run(json: bool) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let output = build_pulse_output_with_dir_at(&board_dir, json, Utc::now())?;
    print!("{output}");
    Ok(())
}

fn build_pulse_output_with_dir_at(
    board_dir: &Path,
    json: bool,
    reference_time: DateTime<Utc>,
) -> Result<String> {
    let cycle = build_pulse_cycle(board_dir, reference_time)?;
    if json {
        Ok(format!("{}\n", serde_json::to_string_pretty(&cycle)?))
    } else {
        Ok(render_pulse_human(&cycle))
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct PulseCycleOutput {
    mode: &'static str,
    evaluated: usize,
    created: usize,
    skipped: usize,
    deferred: usize,
    routines: Vec<PulseRoutineSummary>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct PulseRoutineSummary {
    id: String,
    title: String,
    target_scope: String,
    outcome: PulseRoutineOutcome,
    reason: PulseRoutineReason,
    story_id: Option<String>,
    materialization_key: Option<String>,
    next_eligible_at: Option<DateTime<Utc>>,
    countdown: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PulseRoutineOutcome {
    Created,
    Skipped,
    Deferred,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PulseRoutineReason {
    Materialized,
    AlreadyMaterialized,
    NotDueUntilNextEligible,
    InvalidCadence,
    MaterializationFailed,
}

fn build_pulse_cycle(board_dir: &Path, reference_time: DateTime<Utc>) -> Result<PulseCycleOutput> {
    let board = load_board(board_dir)?;
    let scheduled =
        project_scheduled_routines(&board, reference_time, RoutineScheduleFilter::default());
    let mut materialized_by_key = existing_materializations(&board)?;
    let mut next_scope_indexes = next_scope_indexes(&board);
    let mut routines = Vec::with_capacity(scheduled.len());
    let mut created_any = false;

    for routine in &scheduled {
        let summary = match map_pulse_routine(
            board_dir,
            &board,
            routine,
            reference_time,
            &mut materialized_by_key,
            &mut next_scope_indexes,
        ) {
            Ok(summary) => summary,
            Err(error) => PulseRoutineSummary {
                id: routine.id.clone(),
                title: routine.title.clone(),
                target_scope: routine.target_scope.clone(),
                outcome: PulseRoutineOutcome::Deferred,
                reason: PulseRoutineReason::MaterializationFailed,
                story_id: None,
                materialization_key: materialization_key(routine),
                next_eligible_at: routine.next_eligible_at,
                countdown: routine.countdown.clone(),
                error: Some(error.to_string()),
            },
        };

        created_any |= matches!(summary.outcome, PulseRoutineOutcome::Created);
        routines.push(summary);
    }

    if created_any {
        crate::cli::commands::generate::run(board_dir)?;
    }

    let created = routines
        .iter()
        .filter(|routine| matches!(routine.outcome, PulseRoutineOutcome::Created))
        .count();
    let skipped = routines
        .iter()
        .filter(|routine| matches!(routine.outcome, PulseRoutineOutcome::Skipped))
        .count();
    let deferred = routines
        .iter()
        .filter(|routine| matches!(routine.outcome, PulseRoutineOutcome::Deferred))
        .count();

    Ok(PulseCycleOutput {
        mode: "materialize",
        evaluated: routines.len(),
        created,
        skipped,
        deferred,
        routines,
    })
}

fn map_pulse_routine(
    board_dir: &Path,
    board: &Board,
    routine: &ScheduledRoutineProjection,
    reference_time: DateTime<Utc>,
    materialized_by_key: &mut HashMap<String, String>,
    next_scope_indexes: &mut HashMap<String, u32>,
) -> Result<PulseRoutineSummary> {
    let materialization_key = materialization_key(routine);

    match routine.gating_reason {
        ScheduledRoutineGatingReason::NotDueUntilNextEligible => Ok(PulseRoutineSummary {
            id: routine.id.clone(),
            title: routine.title.clone(),
            target_scope: routine.target_scope.clone(),
            outcome: PulseRoutineOutcome::Deferred,
            reason: PulseRoutineReason::NotDueUntilNextEligible,
            story_id: None,
            materialization_key,
            next_eligible_at: routine.next_eligible_at,
            countdown: routine.countdown.clone(),
            error: None,
        }),
        ScheduledRoutineGatingReason::InvalidCadence => Ok(PulseRoutineSummary {
            id: routine.id.clone(),
            title: routine.title.clone(),
            target_scope: routine.target_scope.clone(),
            outcome: PulseRoutineOutcome::Deferred,
            reason: PulseRoutineReason::InvalidCadence,
            story_id: None,
            materialization_key: None,
            next_eligible_at: routine.next_eligible_at,
            countdown: routine.countdown.clone(),
            error: routine.error.clone(),
        }),
        ScheduledRoutineGatingReason::DueNow => {
            let key = materialization_key.ok_or_else(|| {
                anyhow!("Due routine '{}' is missing next eligible time", routine.id)
            })?;
            if let Some(existing_story_id) = materialized_by_key.get(&key) {
                return Ok(PulseRoutineSummary {
                    id: routine.id.clone(),
                    title: routine.title.clone(),
                    target_scope: routine.target_scope.clone(),
                    outcome: PulseRoutineOutcome::Skipped,
                    reason: PulseRoutineReason::AlreadyMaterialized,
                    story_id: Some(existing_story_id.clone()),
                    materialization_key: Some(key),
                    next_eligible_at: routine.next_eligible_at,
                    countdown: routine.countdown.clone(),
                    error: None,
                });
            }

            let story_id = create_materialized_story(
                board_dir,
                board,
                routine,
                reference_time,
                &key,
                next_scope_indexes,
            )?;
            materialized_by_key.insert(key.clone(), story_id.clone());

            Ok(PulseRoutineSummary {
                id: routine.id.clone(),
                title: routine.title.clone(),
                target_scope: routine.target_scope.clone(),
                outcome: PulseRoutineOutcome::Created,
                reason: PulseRoutineReason::Materialized,
                story_id: Some(story_id),
                materialization_key: Some(key),
                next_eligible_at: routine.next_eligible_at,
                countdown: routine.countdown.clone(),
                error: None,
            })
        }
    }
}

fn materialization_key(routine: &ScheduledRoutineProjection) -> Option<String> {
    matches!(routine.state, ScheduledRoutineState::Due)
        .then(|| {
            routine.next_eligible_at.map(|next_eligible_at| {
                format!("{}@{}", routine.id, format_timestamp(next_eligible_at))
            })
        })
        .flatten()
}

fn existing_materializations(board: &Board) -> Result<HashMap<String, String>> {
    let mut stories: Vec<_> = board.stories.values().collect();
    stories.sort_by_key(|story| story.id());

    let mut materialized = HashMap::new();
    for story in stories {
        let content = match fs::read_to_string(&story.path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        if let Some(key) = extract_materialization_key(&content) {
            materialized
                .entry(key)
                .or_insert_with(|| story.id().to_string());
        }
    }

    Ok(materialized)
}

fn extract_materialization_key(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        let key = line
            .strip_prefix(MATERIALIZATION_MARKER_PREFIX)?
            .strip_suffix(MATERIALIZATION_MARKER_SUFFIX)?;
        Some(key.to_string())
    })
}

fn next_scope_indexes(board: &Board) -> HashMap<String, u32> {
    let mut indexes: HashMap<String, u32> = HashMap::new();
    for story in board.stories.values() {
        if let (Some(scope), Some(index)) = (story.scope(), story.index()) {
            indexes
                .entry(scope.to_string())
                .and_modify(|current| *current = (*current).max(index))
                .or_insert(index);
        }
    }
    indexes
}

fn create_materialized_story(
    board_dir: &Path,
    board: &Board,
    routine: &ScheduledRoutineProjection,
    reference_time: DateTime<Utc>,
    materialization_key: &str,
    next_scope_indexes: &mut HashMap<String, u32>,
) -> Result<String> {
    validate_target_scope(board, &routine.target_scope)?;

    let epic_id = routine
        .target_scope
        .split('/')
        .next()
        .ok_or_else(|| anyhow!("Routine '{}' target scope is empty", routine.id))?;
    let next_index = next_scope_indexes
        .entry(routine.target_scope.clone())
        .and_modify(|index| *index += 1)
        .or_insert(1);
    let story_id = generate_story_id();
    let story_dir = board_dir.join("stories").join(&story_id);
    let story_path = story_dir.join("README.md");
    fs::create_dir_all(&story_dir)
        .with_context(|| format!("Failed to create story directory: {}", story_dir.display()))?;
    fs::create_dir_all(story_dir.join("EVIDENCE")).with_context(|| {
        format!(
            "Failed to create story evidence directory: {}",
            story_dir.join("EVIDENCE").display()
        )
    })?;

    let timestamp = reference_time.naive_utc();
    let frontmatter = StoryFrontmatter {
        id: story_id.clone(),
        title: routine.title.clone(),
        story_type: StoryType::Feat,
        status: StoryState::Backlog,
        scope: Some(routine.target_scope.clone()),
        milestone: None,
        created_at: Some(timestamp),
        updated_at: Some(timestamp),
        started_at: None,
        completed_at: None,
        submitted_at: None,
        index: Some(*next_index),
        governed_by: find_governing_adrs(board, epic_id),
        blocked_by: Vec::new(),
        role: None,
        operator_signal: Some("pulse".to_string()),
    };
    let blueprint = board
        .require_routine(&routine.id)?
        .blueprint_markdown()
        .to_string();
    let content =
        render_materialized_story(&frontmatter, routine, materialization_key, &blueprint)?;
    fs::write(&story_path, content)
        .with_context(|| format!("Failed to write story: {}", story_path.display()))?;

    Ok(story_id)
}

fn validate_target_scope(board: &Board, target_scope: &str) -> Result<()> {
    if let Some((epic_id, voyage_id)) = target_scope.split_once('/') {
        board.require_epic(epic_id)?;
        let voyage = board.require_voyage(voyage_id)?;
        if voyage.epic_id != epic_id {
            return Err(anyhow!(
                "Cannot target scope '{}': voyage '{}' belongs to epic '{}', expected '{}'",
                target_scope,
                voyage_id,
                voyage.epic_id,
                epic_id
            ));
        }
        Ok(())
    } else {
        board.require_epic(target_scope)?;
        Ok(())
    }
}

fn find_governing_adrs(board: &Board, context: &str) -> Vec<String> {
    let mut adr_ids: Vec<String> = board
        .adrs
        .values()
        .filter(|adr| {
            if adr.frontmatter.status != AdrStatus::Accepted {
                return false;
            }
            if adr.frontmatter.context.as_deref() == Some(context) {
                return true;
            }
            adr.frontmatter
                .applies_to
                .iter()
                .any(|scope| scope == context)
        })
        .map(|adr| adr.id().to_string())
        .collect();
    adr_ids.sort();
    adr_ids
}

fn render_materialized_story(
    frontmatter: &StoryFrontmatter,
    routine: &ScheduledRoutineProjection,
    materialization_key: &str,
    blueprint: &str,
) -> Result<String> {
    let yaml = serde_yaml::to_string(frontmatter).context("serialize story frontmatter")?;
    let next_eligible_at = routine
        .next_eligible_at
        .ok_or_else(|| anyhow!("Due routine '{}' is missing next eligible time", routine.id))?;
    let blueprint = render_blueprint_block(blueprint);
    Ok(format!(
        "---\n{yaml}---\n\n{}\n\n# {}\n\n## Summary\n\nMaterialized from routine `{}` for eligible window ending `{}`.\n\n## Acceptance Criteria\n\n- [ ] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window.\n\n## Routine Provenance\n\n- Routine: `{}`\n- Target scope: `{}`\n- Eligible window ends: `{}`\n\n{}\n",
        materialization_marker(materialization_key),
        frontmatter.title,
        routine.id,
        format_timestamp(next_eligible_at),
        routine.id,
        routine.target_scope,
        format_timestamp(next_eligible_at),
        blueprint
    ))
}

fn render_blueprint_block(blueprint: &str) -> String {
    let trimmed = blueprint.trim();
    if let Some(rest) = trimmed.strip_prefix("# Blueprint") {
        format!("## Blueprint{rest}")
    } else if trimmed.is_empty() {
        "## Blueprint".to_string()
    } else {
        format!("## Blueprint\n\n{trimmed}")
    }
}

fn materialization_marker(materialization_key: &str) -> String {
    format!("{MATERIALIZATION_MARKER_PREFIX}{materialization_key}{MATERIALIZATION_MARKER_SUFFIX}")
}

fn render_pulse_human(cycle: &PulseCycleOutput) -> String {
    let mut lines = vec![
        "Pulse cycle".to_string(),
        format!("  Evaluated: {}", cycle.evaluated),
        format!("  Created:  {}", cycle.created),
        format!("  Skipped:  {}", cycle.skipped),
        format!("  Deferred: {}", cycle.deferred),
    ];

    append_outcome_section(
        &mut lines,
        "Created routines:",
        cycle.routines.iter(),
        PulseRoutineOutcome::Created,
    );
    append_outcome_section(
        &mut lines,
        "Skipped routines:",
        cycle.routines.iter(),
        PulseRoutineOutcome::Skipped,
    );
    append_outcome_section(
        &mut lines,
        "Deferred routines:",
        cycle.routines.iter(),
        PulseRoutineOutcome::Deferred,
    );

    lines.push(String::new());
    lines.join("\n")
}

fn append_outcome_section<'a, I>(
    lines: &mut Vec<String>,
    heading: &str,
    routines: I,
    outcome: PulseRoutineOutcome,
) where
    I: Iterator<Item = &'a PulseRoutineSummary>,
{
    let matching: Vec<_> = routines
        .filter(|routine| routine.outcome == outcome)
        .collect();
    if matching.is_empty() {
        return;
    }

    lines.push(String::new());
    lines.push(heading.to_string());
    lines.extend(matching.into_iter().map(render_human_routine_line));
}

fn render_human_routine_line(routine: &PulseRoutineSummary) -> String {
    format!(
        "  - {} | {} | {} | {}",
        routine.id,
        routine.title,
        routine.target_scope,
        human_reason(routine)
    )
}

fn human_reason(routine: &PulseRoutineSummary) -> String {
    match routine.reason {
        PulseRoutineReason::Materialized => format!(
            "created story {} for {}",
            routine.story_id.as_deref().unwrap_or("unknown"),
            routine
                .materialization_key
                .as_deref()
                .unwrap_or("unknown window")
        ),
        PulseRoutineReason::AlreadyMaterialized => format!(
            "already materialized as {} for {}",
            routine.story_id.as_deref().unwrap_or("unknown"),
            routine
                .materialization_key
                .as_deref()
                .unwrap_or("unknown window")
        ),
        PulseRoutineReason::NotDueUntilNextEligible => {
            match (routine.next_eligible_at, routine.countdown.as_deref()) {
                (Some(next_eligible_at), Some(countdown)) => {
                    format!(
                        "not due until {} ({countdown})",
                        format_timestamp(next_eligible_at)
                    )
                }
                (Some(next_eligible_at), None) => {
                    format!("not due until {}", format_timestamp(next_eligible_at))
                }
                (None, Some(countdown)) => format!("not due yet ({countdown})"),
                (None, None) => "not due yet".to_string(),
            }
        }
        PulseRoutineReason::InvalidCadence => format!(
            "invalid cadence: {}",
            routine.error.as_deref().unwrap_or("unknown cadence error")
        ),
        PulseRoutineReason::MaterializationFailed => format!(
            "materialization failed: {}",
            routine
                .error
                .as_deref()
                .unwrap_or("unknown materialization error")
        ),
    }
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use serde_json::json;
    use std::fs;

    fn write_routine(
        root: &Path,
        id: &str,
        title: &str,
        target_scope: &str,
        cadence_block: &str,
        blueprint: &str,
    ) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: {title}
cadence:
{cadence_block}
target-scope: {target_scope}
created_at: 2026-01-01T00:00:00
updated_at: 2026-01-01T00:00:00
---

{blueprint}
"#
            ),
        )
        .unwrap();
    }

    fn story_readmes(root: &Path) -> Vec<String> {
        let mut paths: Vec<_> = fs::read_dir(root.join("stories"))
            .unwrap()
            .flatten()
            .map(|entry| entry.path().join("README.md"))
            .filter(|path| path.exists())
            .collect();
        paths.sort();
        paths
            .into_iter()
            .map(|path| fs::read_to_string(path).unwrap())
            .collect()
    }

    #[test]
    fn pulse_materializes_due_routine_once_per_eligible_window() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1"))
            .build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Review the open backlog.\n",
        );

        let first = build_pulse_output_with_dir_at(
            temp.path(),
            true,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();
        let first_parsed: serde_json::Value = serde_json::from_str(&first).unwrap();
        let first_story_id = first_parsed["routines"][0]["story_id"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(first_parsed["created"], 1);
        assert_eq!(first_parsed["skipped"], 0);
        assert_eq!(first_parsed["deferred"], 0);
        assert_eq!(first_parsed["routines"][0]["outcome"], "created");
        assert_eq!(first_parsed["routines"][0]["reason"], "materialized");
        assert_eq!(
            first_parsed["routines"][0]["materialization_key"],
            "routine-due@2026-01-12T17:00:00Z"
        );

        let stories_after_first = story_readmes(temp.path());
        assert_eq!(stories_after_first.len(), 1);
        assert!(stories_after_first[0].contains("scope: E1/V1"));
        assert!(stories_after_first[0].contains("status: backlog"));
        assert!(
            stories_after_first[0]
                .contains("<!-- keel:pulse-materialization: routine-due@2026-01-12T17:00:00Z -->")
        );
        assert!(stories_after_first[0].contains("operator-signal: pulse"));
        assert!(stories_after_first[0].contains("Review the open backlog."));

        let second = build_pulse_output_with_dir_at(
            temp.path(),
            true,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();
        let second_parsed: serde_json::Value = serde_json::from_str(&second).unwrap();
        assert_eq!(second_parsed["created"], 0);
        assert_eq!(second_parsed["skipped"], 1);
        assert_eq!(second_parsed["deferred"], 0);
        assert_eq!(second_parsed["routines"][0]["outcome"], "skipped");
        assert_eq!(
            second_parsed["routines"][0]["reason"],
            "already_materialized"
        );
        assert_eq!(
            second_parsed["routines"][0]["materialization_key"],
            "routine-due@2026-01-12T17:00:00Z"
        );
        assert_eq!(second_parsed["routines"][0]["story_id"], first_story_id);
        assert_eq!(story_readmes(temp.path()).len(), 1);

        let third = build_pulse_output_with_dir_at(
            temp.path(),
            true,
            Utc.with_ymd_and_hms(2026, 1, 12, 18, 0, 0).unwrap(),
        )
        .unwrap();
        let third_parsed: serde_json::Value = serde_json::from_str(&third).unwrap();
        assert_eq!(third_parsed["created"], 1);
        assert_eq!(third_parsed["skipped"], 0);
        assert_eq!(third_parsed["deferred"], 0);
        assert_eq!(
            third_parsed["routines"][0]["materialization_key"],
            "routine-due@2026-01-19T17:00:00Z"
        );
        assert_ne!(third_parsed["routines"][0]["story_id"], first_story_id);
        assert_eq!(story_readmes(temp.path()).len(), 2);
    }

    #[test]
    fn pulse_human_output_reports_created_skipped_and_deferred_routines() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1"))
            .build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Review the open backlog.\n",
        );
        write_routine(
            temp.path(),
            "routine-upcoming",
            "Friday Review",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Review Friday work.\n",
        );
        write_routine(
            temp.path(),
            "routine-invalid",
            "Broken Review",
            "E1/V1",
            "  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Broken cadence.\n",
        );

        let first = build_pulse_output_with_dir_at(
            temp.path(),
            false,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();
        let second = build_pulse_output_with_dir_at(
            temp.path(),
            false,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        assert!(first.contains("Pulse cycle\n"));
        assert!(first.contains("  Created:  1\n"));
        assert!(first.contains("  Skipped:  0\n"));
        assert!(first.contains("  Deferred: 2\n"));
        assert!(first.contains("Created routines:\n"));
        assert!(first.contains("created story"));
        assert!(first.contains("Deferred routines:\n"));
        assert!(first.contains("not due until 2026-01-05T19:00:00Z (in 1h)"));
        assert!(
            first.contains("invalid cadence: Routine 'routine-invalid' is missing cadence.cron")
        );

        assert!(second.contains("  Created:  0\n"));
        assert!(second.contains("  Skipped:  1\n"));
        assert!(second.contains("Skipped routines:\n"));
        assert!(second.contains("already materialized as"));
    }

    #[test]
    fn pulse_json_output_is_structured_for_created_skipped_and_deferred_state() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1"))
            .build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Review the open backlog.\n",
        );
        write_routine(
            temp.path(),
            "routine-upcoming",
            "Friday Review",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Review Friday work.\n",
        );
        write_routine(
            temp.path(),
            "routine-invalid",
            "Broken Review",
            "E1/V1",
            "  timezone: America/Los_Angeles",
            "# Blueprint\n\n- Broken cadence.\n",
        );

        let rendered = build_pulse_output_with_dir_at(
            temp.path(),
            true,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&rendered).unwrap();
        let created_story_id = parsed["routines"][0]["story_id"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(
            parsed,
            json!({
                "mode": "materialize",
                "evaluated": 3,
                "created": 1,
                "skipped": 0,
                "deferred": 2,
                "routines": [
                    {
                        "id": "routine-due",
                        "title": "Weekly Review",
                        "target_scope": "E1/V1",
                        "outcome": "created",
                        "reason": "materialized",
                        "story_id": created_story_id,
                        "materialization_key": "routine-due@2026-01-12T17:00:00Z",
                        "next_eligible_at": "2026-01-12T17:00:00Z",
                        "countdown": "in 6d 23h",
                        "error": null
                    },
                    {
                        "id": "routine-upcoming",
                        "title": "Friday Review",
                        "target_scope": "E1/V1",
                        "outcome": "deferred",
                        "reason": "not_due_until_next_eligible",
                        "story_id": null,
                        "materialization_key": null,
                        "next_eligible_at": "2026-01-05T19:00:00Z",
                        "countdown": "in 1h",
                        "error": null
                    },
                    {
                        "id": "routine-invalid",
                        "title": "Broken Review",
                        "target_scope": "E1/V1",
                        "outcome": "deferred",
                        "reason": "invalid_cadence",
                        "story_id": null,
                        "materialization_key": null,
                        "next_eligible_at": null,
                        "countdown": null,
                        "error": "Routine 'routine-invalid' is missing cadence.cron"
                    }
                ]
            })
        );
    }

    #[test]
    fn extract_materialization_key_reads_canonical_marker() {
        let content =
            "<!-- keel:pulse-materialization: routine-due@2026-01-12T17:00:00Z -->\n# Story\n";
        assert_eq!(
            extract_materialization_key(content).as_deref(),
            Some("routine-due@2026-01-12T17:00:00Z")
        );
    }

    #[test]
    fn render_blueprint_block_reuses_canonical_heading() {
        assert_eq!(
            render_blueprint_block("# Blueprint\n\n- Review the backlog.\n"),
            "## Blueprint\n\n- Review the backlog."
        );
    }
}
