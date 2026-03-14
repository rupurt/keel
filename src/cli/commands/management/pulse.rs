//! `keel pulse` command adapter.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, SecondsFormat, Utc};
use colored::Color;
use serde::Serialize;
use txtplot::ChartContext;

use keel::domain::model::{AdrStatus, Board, StoryFrontmatter, StoryState, StoryType};
use keel::infrastructure::loader::load_board;
use keel::infrastructure::story_id::generate_story_id;
use keel::read_model::routine_materialization::{
    existing_materializations, materialization_marker, projection_materialization_key,
};
use keel::read_model::scheduled_routines::{
    RoutineScheduleFilter, ScheduledRoutineGatingReason, ScheduledRoutineProjection,
    project_scheduled_routines,
};

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

#[derive(Debug, Clone, Serialize, PartialEq)]
struct PulseCycleOutput {
    mode: &'static str,
    evaluated: usize,
    created: usize,
    skipped: usize,
    deferred: usize,
    routines: Vec<PulseRoutineSummary>,
    /// Simulation health metrics
    health: Option<PulseHealthMetrics>,
    /// Activity metrics (heartrate)
    activity: Option<PulseActivityMetrics>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct PulseHealthMetrics {
    drift_coefficient: f64,
    remediation_hours: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
struct PulseActivityMetrics {
    sparkline: String,
    activity_ekg: String,
    daily_avg: f64,
    weekly_total: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum PulseRoutineOutcome {
    Created,
    Skipped,
    Deferred,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
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
                materialization_key: projection_materialization_key(routine),
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

    // 4. Calculate Health
    let (report, _) = keel::read_model::diagnostics::validate(board_dir)?;
    let health = Some(PulseHealthMetrics {
        drift_coefficient: report.drift_coefficient,
        remediation_hours: report.estimated_remediation_hours,
    });

    // 5. Calculate Activity (EKG)
    let history = keel::read_model::throughput_history::project_default(&board);
    let activity = if history.weekly.iter().all(|w| w.stories_done == 0) {
        None
    } else {
        Some(PulseActivityMetrics {
            sparkline: render_sparkline(&history),
            activity_ekg: render_pulse_ekg(&history),
            daily_avg: history
                .weekly
                .first()
                .map(|w| w.stories_done as f64 / 7.0)
                .unwrap_or(0.0),
            weekly_total: history
                .weekly
                .first()
                .map(|w| w.stories_done)
                .unwrap_or(0),
        })
    };

    Ok(PulseCycleOutput {
        mode: "materialize",
        evaluated: routines.len(),
        created,
        skipped,
        deferred,
        routines,
        health,
        activity,
    })
}

fn render_sparkline(history: &keel::read_model::throughput_history::ThroughputHistory) -> String {
    let mut values: Vec<f64> = history
        .weekly
        .iter()
        .take(12)
        .rev()
        .map(|w| w.stories_done as f64)
        .collect();

    if values.is_empty() {
        return "·".repeat(12);
    }

    // Pad if less than 12 weeks
    while values.len() < 12 {
        values.insert(0, 0.0);
    }

    let max = values.iter().fold(0.0f64, |a, &b| a.max(b));
    let ticks = [' ', ' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    values
        .iter()
        .map(|&v| {
            if max == 0.0 {
                '·'
            } else {
                let i = (v / max * (ticks.len() - 1) as f64).round() as usize;
                ticks[i]
            }
        })
        .collect()
}

fn render_pulse_ekg(history: &keel::read_model::throughput_history::ThroughputHistory) -> String {
    let values: Vec<f64> = history
        .weekly
        .iter()
        .take(12)
        .rev()
        .map(|w| w.stories_done as f64)
        .collect();

    if values.is_empty() {
        return "·".repeat(12);
    }

    let width = 48;
    let height = 10;
    let mut chart = ChartContext::new(width, height);

    let max = values.iter().fold(0.0f64, |a, &b| a.max(b));
    let x_scale = (width - 1) as f64 / (values.len() - 1) as f64;
    let y_scale = if max > 0.0 {
        (height - 1) as f64 / max
    } else {
        1.0
    };

    for i in 0..values.len() - 1 {
        let x1 = (i as f64 * x_scale).round() as isize;
        let y1 = (height as f64 - 1.0 - (values[i] * y_scale)).round() as isize;
        let x2 = ((i + 1) as f64 * x_scale).round() as isize;
        let y2 = (height as f64 - 1.0 - (values[i + 1] * y_scale)).round() as isize;

        chart.canvas.line_screen(x1, y1, x2, y2, Some(Color::Cyan));
    }

    let rendered = chart.canvas.render_with_options(true, None);
    rendered
        .lines()
        .map(|line| format!("  {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn map_pulse_routine(
    board_dir: &Path,
    board: &Board,
    routine: &ScheduledRoutineProjection,
    reference_time: DateTime<Utc>,
    materialized_by_key: &mut HashMap<String, String>,
    next_scope_indexes: &mut HashMap<String, u32>,
) -> Result<PulseRoutineSummary> {
    let materialization_key = projection_materialization_key(routine);

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

fn render_pulse_human(cycle: &PulseCycleOutput) -> String {
    use owo_colors::OwoColorize;
    let mut lines = Vec::new();

    lines.push(String::new());
    lines.push(format!("  {}", "Simulation Heartbeat".bold().underline()));

    // 1. Activity (The EKG)
    if let Some(activity) = &cycle.activity {
        lines.push(format!(
            "  Activity:  {:.1} stories/day avg ({} this week)",
            activity.daily_avg, activity.weekly_total
        ));
        lines.push(activity.activity_ekg.clone());
    }

    // 2. Health (The Physical)
    if let Some(health) = &cycle.health {
        let status = if health.drift_coefficient > 0.5 {
            "Critical".bold().red().to_string()
        } else if health.drift_coefficient > 0.1 {
            "Elevated".bold().yellow().to_string()
        } else {
            "Healthy".bold().green().to_string()
        };

        lines.push(format!(
            "  Health:    {} (Drift: {:.2}, Debt: {:.1}h)",
            status, health.drift_coefficient, health.remediation_hours
        ));
    }

    // 3. Work (The Volume)
    lines.push(format!(
        "  Volume:    {} routines evaluated ({} created, {} skipped)",
        cycle.evaluated, cycle.created, cycle.skipped
    ));

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

        assert!(first.contains("Simulation Heartbeat"));
        assert!(!first.contains("Activity:"));
        assert!(first.contains("Health:"));
        assert!(first.contains("Volume:    3 routines evaluated (1 created, 0 skipped)"));
        assert!(first.contains("Created routines:"));
        assert!(first.contains("created story"));
        assert!(first.contains("Deferred routines:"));
        assert!(first.contains("not due until 2026-01-05T19:00:00Z (in 1h)"));
        assert!(
            first.contains("invalid cadence: Routine 'routine-invalid' is missing cadence.cron")
        );

        assert!(second.contains("Volume:    3 routines evaluated (0 created, 1 skipped)"));
        assert!(second.contains("Skipped routines:"));
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

        assert_eq!(parsed["mode"], "materialize");
        assert_eq!(parsed["evaluated"], 3);
        assert_eq!(parsed["created"], 1);
        assert_eq!(parsed["skipped"], 0);
        assert_eq!(parsed["deferred"], 2);

        assert!(parsed["health"].is_object());
        let drift = parsed["health"]["drift_coefficient"].as_f64().unwrap();
        assert!(drift >= 0.16 && drift <= 0.18, "Drift was {drift}");

        assert!(parsed["activity"].is_null());

        assert_eq!(parsed["routines"][0]["id"], "routine-due");
        assert_eq!(parsed["routines"][0]["outcome"], "created");
        assert_eq!(parsed["routines"][0]["story_id"], created_story_id);

        assert_eq!(parsed["routines"][1]["id"], "routine-upcoming");
        assert_eq!(parsed["routines"][1]["outcome"], "deferred");

        assert_eq!(parsed["routines"][2]["id"], "routine-invalid");
        assert_eq!(parsed["routines"][2]["outcome"], "deferred");
    }

    #[test]
    fn render_blueprint_block_reuses_canonical_heading() {
        assert_eq!(
            render_blueprint_block("# Blueprint\n\n- Review the backlog.\n"),
            "## Blueprint\n\n- Review the backlog."
        );
    }
}
