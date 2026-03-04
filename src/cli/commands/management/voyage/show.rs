//! Show voyage command

use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::style;
use crate::domain::model::{Board, StoryState, Voyage};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification::parser::{Comparison, parse_ac_references, parse_verify_annotations};

const GOAL_PLACEHOLDER: &str = "(goal not authored yet)";
const SCOPE_PLACEHOLDER: &str = "(scope not authored in SRS.md yet)";
const REQUIREMENTS_PLACEHOLDER: &str = "(no requirements found in SRS.md)";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ScopeSummary {
    in_scope: Vec<String>,
    out_of_scope: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoryRef {
    id: String,
    stage: StoryState,
    index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RequirementRow {
    id: String,
    description: String,
    linked_stories: Vec<StoryRef>,
    completion: String,
    verification: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VoyageShowReport {
    goal: Option<String>,
    scope: ScopeSummary,
    requirements: Vec<RequirementRow>,
    done_stories: usize,
    total_stories: usize,
    done_requirements: usize,
    total_requirements: usize,
}

#[derive(Debug, Clone)]
struct StoryEvidence {
    id: String,
    stage: StoryState,
    index: Option<u32>,
    references: Vec<String>,
    automated_count_by_req: std::collections::BTreeMap<String, usize>,
    manual_count_by_req: std::collections::BTreeMap<String, usize>,
}

/// Show voyage details
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Show voyage details with an explicit board directory.
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let voyage = board.require_voyage(id)?;
    let report = build_voyage_show_report(&board, voyage)?;

    let width = crate::cli::presentation::terminal::get_terminal_width();

    println!("{}", style::heavy_rule(width, None));
    println!(
        "{}",
        style::header(
            voyage.id(),
            &voyage.frontmatter.title,
            style::styled_voyage_id
        )
    );
    println!("{}", style::heavy_rule(width, None));
    println!();

    println!("Epic:     {}", style::styled_epic_id(&voyage.epic_id));
    println!("Status:   {}", style::styled_voyage_stage(&voyage.status()));

    render_goal_scope(&report);
    render_progress(&report);
    render_requirement_matrix(&report);

    println!();
    println!("Path: {}", voyage.path.display().dimmed());

    Ok(())
}

fn build_voyage_show_report(board: &Board, voyage: &Voyage) -> Result<VoyageShowReport> {
    let srs_path = voyage.path.parent().unwrap().join("SRS.md");
    let srs = fs::read_to_string(&srs_path).unwrap_or_default();

    let goal = voyage
        .frontmatter
        .goal
        .as_ref()
        .map(|g| g.trim().to_string())
        .filter(|g| !g.is_empty())
        .or_else(|| extract_goal_from_srs(&srs));

    let scope = parse_scope_summary(&srs);
    let mut requirements = parse_srs_requirements(&srs);
    requirements.sort_by(|a, b| a.0.cmp(&b.0));

    let scope_key = voyage.scope_path();
    let mut story_evidence: Vec<StoryEvidence> = board
        .stories
        .values()
        .filter(|story| story.scope() == Some(scope_key.as_str()))
        .filter_map(|story| {
            let content = fs::read_to_string(&story.path).ok()?;
            let mut references: Vec<String> = parse_ac_references(&content)
                .into_iter()
                .map(|r| r.srs_id)
                .collect();

            let mut automated_count_by_req: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            let mut manual_count_by_req: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();

            for ann in parse_verify_annotations(&content) {
                let req = ann
                    .requirement
                    .as_ref()
                    .map(|r| r.id.clone())
                    .or_else(|| ann.ac_ref.as_ref().map(|r| r.srs_id.clone()));

                let Some(req_id) = req else {
                    continue;
                };

                references.push(req_id.clone());
                if ann.comparison == Comparison::Manual {
                    *manual_count_by_req.entry(req_id).or_insert(0) += 1;
                } else {
                    *automated_count_by_req.entry(req_id).or_insert(0) += 1;
                }
            }

            references.sort();
            references.dedup();

            Some(StoryEvidence {
                id: story.id().to_string(),
                stage: story.stage,
                index: story.index(),
                references,
                automated_count_by_req,
                manual_count_by_req,
            })
        })
        .collect();

    story_evidence.sort_by(story_ordering);

    let total_stories = story_evidence.len();
    let done_stories = story_evidence
        .iter()
        .filter(|story| story.stage == StoryState::Done)
        .count();

    let mut rows = Vec::new();
    for (req_id, description) in requirements {
        let mut linked: Vec<StoryRef> = story_evidence
            .iter()
            .filter(|story| story.references.iter().any(|req| req == &req_id))
            .map(|story| StoryRef {
                id: story.id.clone(),
                stage: story.stage,
                index: story.index,
            })
            .collect();
        linked.sort_by(story_ref_ordering);

        let completion = requirement_completion_label(&linked);

        let automated = story_evidence
            .iter()
            .filter_map(|story| story.automated_count_by_req.get(&req_id))
            .sum::<usize>();
        let manual = story_evidence
            .iter()
            .filter_map(|story| story.manual_count_by_req.get(&req_id))
            .sum::<usize>();
        let verification = requirement_verification_label(automated, manual);

        rows.push(RequirementRow {
            id: req_id,
            description,
            linked_stories: linked,
            completion,
            verification,
        });
    }

    let done_requirements = rows.iter().filter(|row| row.completion == "done").count();
    let total_requirements = rows.len();

    Ok(VoyageShowReport {
        goal,
        scope,
        requirements: rows,
        done_stories,
        total_stories,
        done_requirements,
        total_requirements,
    })
}

fn extract_goal_from_srs(srs: &str) -> Option<String> {
    srs.lines()
        .map(str::trim)
        .find(|line| line.starts_with('>'))
        .map(|line| line.trim_start_matches('>').trim().to_string())
        .filter(|line| !line.is_empty())
}

fn parse_scope_summary(srs: &str) -> ScopeSummary {
    let mut summary = ScopeSummary::default();
    let section = match extract_section(srs, "## Scope") {
        Some(section) => section,
        None => return summary,
    };

    enum Mode {
        None,
        In,
        Out,
    }

    let mut mode = Mode::None;
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("In scope:") {
            mode = Mode::In;
            continue;
        }
        if trimmed.eq_ignore_ascii_case("Out of scope:") {
            mode = Mode::Out;
            continue;
        }
        if let Some(item) = trimmed.strip_prefix("- ") {
            if item.trim().is_empty() {
                continue;
            }
            match mode {
                Mode::In => summary.in_scope.push(item.trim().to_string()),
                Mode::Out => summary.out_of_scope.push(item.trim().to_string()),
                Mode::None => {}
            }
        }
    }

    summary
}

fn parse_srs_requirements(srs: &str) -> Vec<(String, String)> {
    let mut out = parse_requirement_block(
        srs,
        "BEGIN FUNCTIONAL_REQUIREMENTS",
        "END FUNCTIONAL_REQUIREMENTS",
    );
    out.extend(parse_requirement_block(
        srs,
        "BEGIN NON_FUNCTIONAL_REQUIREMENTS",
        "END NON_FUNCTIONAL_REQUIREMENTS",
    ));
    out
}

fn parse_requirement_block(srs: &str, start_marker: &str, end_marker: &str) -> Vec<(String, String)> {
    let mut rows = Vec::new();
    let mut in_block = false;

    for line in srs.lines() {
        if line.contains(start_marker) {
            in_block = true;
            continue;
        }
        if line.contains(end_marker) {
            break;
        }
        if !in_block {
            continue;
        }

        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }

        let cols: Vec<&str> = trimmed
            .split('|')
            .map(str::trim)
            .filter(|col| !col.is_empty())
            .collect();
        if cols.len() < 2 {
            continue;
        }

        let id = cols[0];
        let requirement = cols[1];
        if id.eq_ignore_ascii_case("ID")
            || id.starts_with("---")
            || requirement.eq_ignore_ascii_case("TODO")
        {
            continue;
        }

        rows.push((id.to_string(), requirement.to_string()));
    }

    rows
}

fn extract_section(content: &str, heading: &str) -> Option<String> {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|c| *c == '#').count();

    for line in content.lines() {
        if line.starts_with(heading) {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with('#') {
                let level = line.chars().take_while(|c| *c == '#').count();
                if level <= heading_level {
                    break;
                }
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    if result.trim().is_empty() {
        None
    } else {
        Some(result)
    }
}

fn story_ordering(a: &StoryEvidence, b: &StoryEvidence) -> Ordering {
    match (a.index, b.index) {
        (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        _ => a.id.cmp(&b.id),
    }
}

fn story_ref_ordering(a: &StoryRef, b: &StoryRef) -> Ordering {
    match (a.index, b.index) {
        (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        _ => a.id.cmp(&b.id),
    }
}

fn requirement_completion_label(linked: &[StoryRef]) -> String {
    if linked.is_empty() {
        "unmapped".to_string()
    } else if linked.iter().all(|story| story.stage == StoryState::Done) {
        "done".to_string()
    } else if linked
        .iter()
        .any(|story| story.stage == StoryState::InProgress || story.stage == StoryState::NeedsHumanVerification)
    {
        "in-progress".to_string()
    } else {
        "queued".to_string()
    }
}

fn requirement_verification_label(automated: usize, manual: usize) -> String {
    match (automated, manual) {
        (0, 0) => "none".to_string(),
        (a, 0) => format!("automated ({a})"),
        (0, m) => format!("manual ({m})"),
        (a, m) => format!("mixed (a:{a}/m:{m})"),
    }
}

fn render_goal_scope(report: &VoyageShowReport) {
    println!();
    println!("{}", "Voyage Summary".bold());
    println!(
        "  Goal: {}",
        report.goal.as_deref().unwrap_or(GOAL_PLACEHOLDER)
    );

    if report.scope.in_scope.is_empty() {
        println!("  In scope:  {}", SCOPE_PLACEHOLDER.dimmed());
    } else {
        println!("  In scope:");
        for item in &report.scope.in_scope {
            println!("    - {}", item);
        }
    }

    if report.scope.out_of_scope.is_empty() {
        println!("  Out of scope: {}", SCOPE_PLACEHOLDER.dimmed());
    } else {
        println!("  Out of scope:");
        for item in &report.scope.out_of_scope {
            println!("    - {}", item);
        }
    }
}

fn render_progress(report: &VoyageShowReport) {
    println!();
    println!("{}", "Progress".bold());
    if report.total_stories > 0 {
        println!(
            "  Stories:      {}/{} {}",
            report.done_stories,
            report.total_stories,
            style::progress_bar(report.done_stories, report.total_stories, 15, None)
        );
    } else {
        println!("  Stories:      0/0");
    }

    if report.total_requirements > 0 {
        println!(
            "  Requirements: {}/{} {}",
            report.done_requirements,
            report.total_requirements,
            style::progress_bar(report.done_requirements, report.total_requirements, 15, None)
        );
    } else {
        println!("  Requirements: 0/0");
    }
}

fn render_requirement_matrix(report: &VoyageShowReport) {
    println!();
    println!("{}", "Requirements Matrix".bold());

    if report.requirements.is_empty() {
        println!("  {}", REQUIREMENTS_PLACEHOLDER.dimmed());
        return;
    }

    println!("  | Requirement | Completion | Verification | Linked Stories |");
    println!("  |-------------|------------|--------------|----------------|");
    for row in &report.requirements {
        let linked = if row.linked_stories.is_empty() {
            "none".to_string()
        } else {
            row.linked_stories
                .iter()
                .map(|story| format!("{}({})", story.id, story.stage))
                .collect::<Vec<_>>()
                .join(", ")
        };

        println!(
            "  | {}: {} | {} | {} | {} |",
            row.id, row.description, row.completion, row.verification, linked
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn test_extract_body() {
        let content = "---\nid: test\n---\nBody content";
        let section = extract_section(content, "## Missing");
        assert!(section.is_none());
    }

    #[test]
    fn voyage_show_goal_scope() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1").srs_content(
                    r#"# SRS
> Ship planning-grade voyage summaries.

## Scope
In scope:
- Render goal and scope summaries.
- Render requirement progress.

Out of scope:
- Lifecycle transition changes.
"#,
                ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let report = build_voyage_show_report(&board, voyage).unwrap();

        assert_eq!(
            report.goal.as_deref(),
            Some("Ship planning-grade voyage summaries.")
        );
        assert_eq!(report.scope.in_scope.len(), 2);
        assert_eq!(report.scope.out_of_scope.len(), 1);
    }

    #[test]
    fn voyage_show_requirement_matrix() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Render goal summary. | test |
| SRS-02 | Render requirement matrix. | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .index(2)
                    .stage(StoryState::Done)
                    .body(
                        r#"## Acceptance Criteria
- [x] [SRS-01/AC-01] Goal summary present <!-- verify: cargo test --lib voyage_show_goal_scope, SRS-01:start:end -->
"#,
                    ),
            )
            .story(
                TestStory::new("S2")
                    .scope("e1/v1")
                    .index(1)
                    .stage(StoryState::Backlog)
                    .body(
                        r#"## Acceptance Criteria
- [ ] [SRS-02/AC-01] Matrix present <!-- verify: manual, SRS-02:start:end -->
"#,
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let report = build_voyage_show_report(&board, voyage).unwrap();

        assert_eq!(report.requirements.len(), 2);
        let r1 = report
            .requirements
            .iter()
            .find(|row| row.id == "SRS-01")
            .unwrap();
        assert_eq!(r1.completion, "done");
        assert_eq!(r1.verification, "automated (1)");
        assert_eq!(r1.linked_stories[0].id, "S1");

        let r2 = report
            .requirements
            .iter()
            .find(|row| row.id == "SRS-02")
            .unwrap();
        assert_eq!(r2.completion, "queued");
        assert_eq!(r2.verification, "manual (1)");
        assert_eq!(r2.linked_stories[0].id, "S2");
    }

    #[test]
    fn voyage_show_progress() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement one. | test |
| SRS-02 | Requirement two. | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .stage(StoryState::Done)
                    .body(
                        r#"## Acceptance Criteria
- [x] [SRS-01/AC-01] done <!-- verify: cargo test --lib x, SRS-01:start:end -->
"#,
                    ),
            )
            .story(
                TestStory::new("S2")
                    .scope("e1/v1")
                    .stage(StoryState::Backlog)
                    .body(
                        r#"## Acceptance Criteria
- [ ] [SRS-02/AC-01] todo <!-- verify: manual, SRS-02:start:end -->
"#,
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let report = build_voyage_show_report(&board, voyage).unwrap();

        assert_eq!(report.done_stories, 1);
        assert_eq!(report.total_stories, 2);
        assert_eq!(report.done_requirements, 1);
        assert_eq!(report.total_requirements, 2);
    }

    #[test]
    fn voyage_show_deterministic_ordering() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-10 | Req ten. | test |
| SRS-02 | Req two. | test |
| SRS-01 | Req one. | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(
                TestStory::new("S2")
                    .scope("e1/v1")
                    .index(2)
                    .body(
                        r#"## Acceptance Criteria
- [ ] [SRS-01/AC-01] one <!-- verify: manual, SRS-01:start:end -->
"#,
                    ),
            )
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .index(1)
                    .body(
                        r#"## Acceptance Criteria
- [ ] [SRS-01/AC-02] one-b <!-- verify: manual, SRS-01:start:end -->
"#,
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let report_a = build_voyage_show_report(&board, voyage).unwrap();
        let report_b = build_voyage_show_report(&board, voyage).unwrap();

        let ids_a: Vec<String> = report_a.requirements.iter().map(|row| row.id.clone()).collect();
        let ids_b: Vec<String> = report_b.requirements.iter().map(|row| row.id.clone()).collect();
        assert_eq!(ids_a, ids_b);
        assert_eq!(ids_a, vec!["SRS-01", "SRS-02", "SRS-10"]);

        let linked_ids: Vec<String> = report_a.requirements[0]
            .linked_stories
            .iter()
            .map(|story| story.id.clone())
            .collect();
        assert_eq!(linked_ids, vec!["S1", "S2"]);
    }
}
