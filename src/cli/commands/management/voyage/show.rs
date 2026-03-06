//! Show voyage command

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::duration::render_completed_with_length;
use crate::cli::presentation::planning_lineage;
use crate::cli::presentation::progress::render_count_bar;
use crate::cli::presentation::requirements::grouped_requirement_lines;
use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::domain::model::{Board, Voyage};
use crate::infrastructure::loader::load_board;
use crate::read_model::planning_show::{self, VoyageShowProjection};

const GOAL_PLACEHOLDER: &str = "(goal not authored yet)";
const SCOPE_PLACEHOLDER: &str = "(scope not authored in SRS.md yet)";
const SCOPE_DRIFT_PLACEHOLDER: &str = "(no scope drift detected)";
const REQUIREMENTS_PLACEHOLDER: &str = "(no requirements found in SRS.md)";

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

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", voyage.frontmatter.title.bold()))
        .row("Epic:", style::styled_epic_id(&voyage.epic_id))
        .row("Status:", style::styled_voyage_stage(&voyage.status()));
    metadata.push_standard_timestamps(
        voyage
            .frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
        voyage
            .frontmatter
            .started_at
            .map(|started_at| format!("{}", started_at.dimmed())),
        voyage
            .frontmatter
            .updated_at
            .map(|updated_at| format!("{}", updated_at.dimmed())),
        voyage.frontmatter.completed_at.map(|completed_at| {
            render_completed_with_length(voyage.frontmatter.started_at, completed_at)
        }),
    );
    let voyage_dir = voyage.path.parent().unwrap_or(&voyage.path);
    let srs_path = voyage_dir.join("SRS.md");
    let sdd_path = voyage_dir.join("SDD.md");
    metadata.push_row("Path:", format!("{}", voyage.path.display().dimmed()));
    metadata.push_row("SRS:", format!("{}", srs_path.display().dimmed()));
    metadata.push_row("SDD:", format!("{}", sdd_path.display().dimmed()));

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    document.push_sections_spaced(voyage_sections(&report));
    document.push_spacer();
    document.push_lines(requirement_matrix_lines(&report));
    document.push_spacer();
    document.push_section(progress_section(&report));
    document.print();

    Ok(())
}

fn build_voyage_show_report(board: &Board, voyage: &Voyage) -> Result<VoyageShowProjection> {
    planning_show::build_voyage_show_projection(board, voyage)
}

fn voyage_sections(report: &VoyageShowProjection) -> Vec<ShowSection> {
    vec![
        goal_section(report),
        scope_section(
            "In Scope",
            report.scope.in_scope.iter().cloned(),
            SCOPE_PLACEHOLDER,
        ),
        scope_section(
            "Out of Scope",
            report.scope.out_of_scope.iter().cloned(),
            SCOPE_PLACEHOLDER,
        ),
        scope_section(
            "Scope Drift",
            report
                .scope_drift
                .iter()
                .map(planning_lineage::format_scope_drift_row),
            SCOPE_DRIFT_PLACEHOLDER,
        ),
    ]
}

fn goal_section(report: &VoyageShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Goal");
    section.push_text_block(report.goal.as_deref().unwrap_or(GOAL_PLACEHOLDER));
    section
}

fn scope_section<I, S>(title: &str, items: I, empty_placeholder: &str) -> ShowSection
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut section = ShowSection::new(title);
    section.push_bullets(items, Some(format!("{}", empty_placeholder.dimmed())));
    section
}

fn progress_section(report: &VoyageShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Progress");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(13);
    fields.push_row(
        "Stories:",
        render_count_bar(report.done_stories, report.total_stories, 15, None),
    );
    fields.push_row(
        "Requirements:",
        render_count_bar(
            report.done_functional_requirements,
            report.total_functional_requirements,
            15,
            Some("(functional)"),
        ),
    );

    if report.total_non_functional_requirements > 0 {
        fields.push_row(
            "NFRs:",
            format!(
                "{}/{} (advisory, not counted toward completion)",
                report.done_non_functional_requirements, report.total_non_functional_requirements
            ),
        );
    }
    section.push_key_values(fields);
    section
}

fn requirement_matrix_lines(report: &VoyageShowProjection) -> Vec<String> {
    grouped_requirement_lines(&report.requirements, REQUIREMENTS_PLACEHOLDER)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::domain::state_machine::invariants::{ScopeLineageIssue, ScopeLineageIssueKind};
    use crate::read_model::planning_show::{
        RequirementCompletion, RequirementKind, RequirementRow, ScopeDriftRow, StoryRef,
    };
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::NaiveDate;

    #[test]
    fn voyage_duration_rendering_formats_elapsed_time() {
        let started = NaiveDate::from_ymd_opt(2026, 3, 4)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        let completed = NaiveDate::from_ymd_opt(2026, 3, 4)
            .unwrap()
            .and_hms_opt(10, 15, 0)
            .unwrap();

        let value = render_completed_with_length(Some(started), completed);

        assert!(value.contains("1h 15m"));
    }

    #[test]
    fn test_extract_body() {
        let content = "---\nid: test\n---\nBody content";
        let section = planning_show::extract_section(content, "## Missing");
        assert!(section.is_none());
    }

    #[test]
    fn voyage_show_goal_scope() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
> Ship planning-grade voyage summaries.

## Scope
In scope:
- Render goal and scope summaries.
- Render requirement progress.

Out of scope:
- Lifecycle transition changes.
"#,
            ))
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
| ID | Requirement | Scope | Verification |
|----|-------------|-------|--------------|
| SRS-01 | Render goal summary. | SCOPE-01 | test |
| SRS-02 | Render requirement matrix. | SCOPE-02 | test |
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
        let req_one = report
            .requirements
            .iter()
            .find(|row| row.id == "SRS-01")
            .unwrap();
        assert_eq!(req_one.completion, RequirementCompletion::Done);
        assert_eq!(req_one.verification, "automated (1)");
        assert_eq!(req_one.scope_refs, vec!["SCOPE-01"]);
        assert_eq!(req_one.linked_stories[0].id, "S1");

        let req_two = report
            .requirements
            .iter()
            .find(|row| row.id == "SRS-02")
            .unwrap();
        assert_eq!(req_two.completion, RequirementCompletion::Queued);
        assert_eq!(req_two.verification, "manual (1)");
        assert_eq!(req_two.scope_refs, vec!["SCOPE-02"]);
        assert_eq!(req_two.linked_stories[0].id, "S2");
    }

    #[test]
    fn voyage_show_progress() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Verification |
|----|-------------|-------|--------------|
| SRS-01 | Requirement one. | SCOPE-01 | test |
| SRS-02 | Requirement two. | SCOPE-02 | test |
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
        assert_eq!(report.done_functional_requirements, 1);
        assert_eq!(report.total_functional_requirements, 2);
        assert_eq!(report.done_non_functional_requirements, 0);
        assert_eq!(report.total_non_functional_requirements, 0);
    }

    #[test]
    fn voyage_show_deterministic_ordering() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Verification |
|----|-------------|-------|--------------|
| SRS-10 | Req ten. | SCOPE-03 | test |
| SRS-02 | Req two. | SCOPE-02 | test |
| SRS-01 | Req one. | SCOPE-01 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(TestStory::new("S2").scope("e1/v1").index(2).body(
                r#"## Acceptance Criteria
- [ ] [SRS-01/AC-01] one <!-- verify: manual, SRS-01:start:end -->
"#,
            ))
            .story(TestStory::new("S1").scope("e1/v1").index(1).body(
                r#"## Acceptance Criteria
- [ ] [SRS-01/AC-02] one-b <!-- verify: manual, SRS-01:start:end -->
"#,
            ))
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let report_a = build_voyage_show_report(&board, voyage).unwrap();
        let report_b = build_voyage_show_report(&board, voyage).unwrap();

        let ids_a: Vec<String> = report_a
            .requirements
            .iter()
            .map(|row| row.id.clone())
            .collect();
        let ids_b: Vec<String> = report_b
            .requirements
            .iter()
            .map(|row| row.id.clone())
            .collect();
        assert_eq!(ids_a, ids_b);
        assert_eq!(ids_a, vec!["SRS-01", "SRS-02", "SRS-10"]);

        let linked_ids: Vec<String> = report_a.requirements[0]
            .linked_stories
            .iter()
            .map(|story| story.id.clone())
            .collect();
        assert_eq!(linked_ids, vec!["S1", "S2"]);
    }

    #[test]
    fn voyage_requirement_matrix_renders_functional_before_non_functional() {
        let report = VoyageShowProjection {
            goal: None,
            scope: Default::default(),
            scope_drift: Vec::new(),
            requirements: vec![
                RequirementRow {
                    id: "SRS-NFR-01".to_string(),
                    description: "Meet latency budget".to_string(),
                    kind: RequirementKind::NonFunctional,
                    scope_refs: vec![],
                    linked_stories: vec![],
                    completion: RequirementCompletion::Queued,
                    verification: "manual (1)".to_string(),
                },
                RequirementRow {
                    id: "SRS-01".to_string(),
                    description: "Render grouped requirement output".to_string(),
                    kind: RequirementKind::Functional,
                    scope_refs: vec!["SCOPE-01".to_string()],
                    linked_stories: vec![StoryRef {
                        id: "S1".to_string(),
                        stage: StoryState::Done,
                        index: Some(1),
                    }],
                    completion: RequirementCompletion::Done,
                    verification: "automated (1)".to_string(),
                },
            ],
            done_stories: 1,
            total_stories: 1,
            done_functional_requirements: 1,
            total_functional_requirements: 1,
            done_non_functional_requirements: 0,
            total_non_functional_requirements: 1,
            done_requirements: 1,
            total_requirements: 1,
        };

        let mut document = ShowDocument::new();
        document.push_lines(requirement_matrix_lines(&report));
        let rendered = document.render();

        let functional_idx = rendered.find("Functional Requirements").unwrap();
        let non_functional_idx = rendered.find("Non-Functional Requirements").unwrap();

        assert!(functional_idx < non_functional_idx);
        assert!(rendered.contains("Verification:"));
        assert!(rendered.contains("Linked Stories:"));
        assert!(rendered.contains("Linked scope (1):"));
    }

    #[test]
    fn voyage_progress_uses_projection_functional_non_functional_counts() {
        let report = VoyageShowProjection {
            goal: None,
            scope: Default::default(),
            scope_drift: Vec::new(),
            requirements: vec![
                RequirementRow {
                    id: "SRS-01".to_string(),
                    description: "Functional done".to_string(),
                    kind: RequirementKind::Functional,
                    scope_refs: vec!["SCOPE-01".to_string()],
                    linked_stories: vec![],
                    completion: RequirementCompletion::Done,
                    verification: "automated (1)".to_string(),
                },
                RequirementRow {
                    id: "SRS-02".to_string(),
                    description: "Functional queued".to_string(),
                    kind: RequirementKind::Functional,
                    scope_refs: vec!["SCOPE-02".to_string()],
                    linked_stories: vec![],
                    completion: RequirementCompletion::Queued,
                    verification: "manual (1)".to_string(),
                },
                RequirementRow {
                    id: "SRS-NFR-01".to_string(),
                    description: "NFR queued".to_string(),
                    kind: RequirementKind::NonFunctional,
                    scope_refs: vec!["SCOPE-03".to_string()],
                    linked_stories: vec![],
                    completion: RequirementCompletion::Queued,
                    verification: "manual (1)".to_string(),
                },
            ],
            done_stories: 0,
            total_stories: 0,
            done_functional_requirements: 1,
            total_functional_requirements: 2,
            done_non_functional_requirements: 0,
            total_non_functional_requirements: 1,
            done_requirements: 1,
            total_requirements: 2,
        };

        let section = progress_section(&report);
        let mut document = ShowDocument::new();
        document.push_section(section);
        let rendered = document.render();

        assert!(rendered.contains("Requirements: 1/2"));
        assert!(rendered.contains("NFRs:         0/1"));
    }

    #[test]
    fn voyage_show_renders_top_level_scope_sections_and_hides_scope_lineage() {
        let report = VoyageShowProjection {
            goal: Some("Render planning scope context.".to_string()),
            scope: planning_show::ScopeSummary {
                in_scope: vec!["[SCOPE-01] Render lineage output".to_string()],
                out_of_scope: vec!["[SCOPE-02] Leave contradictions for later".to_string()],
            },
            scope_drift: vec![ScopeDriftRow {
                voyage_id: None,
                issue: ScopeLineageIssue {
                    artifact_path: std::path::PathBuf::from("SRS.md"),
                    scope_id: Some("SCOPE-02".to_string()),
                    line: None,
                    kind: ScopeLineageIssueKind::OutOfScopeContradiction,
                },
            }],
            requirements: Vec::new(),
            done_stories: 0,
            total_stories: 0,
            done_functional_requirements: 0,
            total_functional_requirements: 0,
            done_non_functional_requirements: 0,
            total_non_functional_requirements: 0,
            done_requirements: 0,
            total_requirements: 0,
        };

        let mut document = ShowDocument::new();
        document.push_sections_spaced(voyage_sections(&report));
        let rendered = document.render();

        assert!(rendered.contains("Goal"));
        assert!(rendered.contains("In Scope"));
        assert!(rendered.contains("Out of Scope"));
        assert!(!rendered.contains("Voyage Summary"));
        assert!(!rendered.contains("Scope Lineage"));
        assert!(rendered.contains("SCOPE-01"));
        assert!(rendered.contains("Scope Drift"));
        assert!(rendered.contains("SCOPE-02"));
    }

    #[test]
    fn voyage_show_places_progress_below_requirements() {
        let report = VoyageShowProjection {
            goal: Some("Render voyage summaries.".to_string()),
            scope: Default::default(),
            scope_drift: Vec::new(),
            requirements: vec![RequirementRow {
                id: "SRS-01".to_string(),
                description: "Render grouped requirement output".to_string(),
                kind: RequirementKind::Functional,
                scope_refs: vec!["SCOPE-01".to_string()],
                linked_stories: vec![],
                completion: RequirementCompletion::Queued,
                verification: "manual (1)".to_string(),
            }],
            done_stories: 0,
            total_stories: 1,
            done_functional_requirements: 0,
            total_functional_requirements: 1,
            done_non_functional_requirements: 0,
            total_non_functional_requirements: 0,
            done_requirements: 0,
            total_requirements: 1,
        };

        let mut document = ShowDocument::new();
        document.push_sections_spaced(voyage_sections(&report));
        document.push_spacer();
        document.push_lines(requirement_matrix_lines(&report));
        document.push_spacer();
        document.push_section(progress_section(&report));
        let rendered = document.render();

        let requirements_idx = rendered.find("Functional Requirements").unwrap();
        let progress_idx = rendered.find("Progress").unwrap();
        assert!(requirements_idx < progress_idx);
    }
}
