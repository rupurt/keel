//! Shared HEAD-relative selector parsing and resolution for show commands.

use std::cmp::Ordering;
use std::path::Path;

use crate::domain::model::{Bearing, Board, Voyage};
use crate::infrastructure::bearing_readiness::evaluate_bearing_readiness;
use crate::infrastructure::config::load_config_from;
use crate::infrastructure::scoring::load_bearing_score;

/// Supported entity families for HEAD-relative show selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShowEntityKind {
    Mission,
    Epic,
    Voyage,
    Story,
    Bearing,
    Adr,
    Routine,
}

impl ShowEntityKind {
    fn plural_label(self) -> &'static str {
        match self {
            Self::Mission => "missions",
            Self::Epic => "epics",
            Self::Voyage => "voyages",
            Self::Story => "stories",
            Self::Bearing => "bearings",
            Self::Adr => "ADRs",
            Self::Routine => "routines",
        }
    }
}

/// A normalized show selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShowSelector {
    ExactId(String),
    Head { offset: usize },
}

impl ShowSelector {
    /// Render the normalized selector string.
    pub fn render(&self) -> String {
        match self {
            Self::ExactId(id) => id.clone(),
            Self::Head { offset: 0 } => "HEAD".to_string(),
            Self::Head { offset } => format!("HEAD{}", "~".repeat(*offset)),
        }
    }
}

/// Deterministic selector parse / resolution errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShowSelectorError {
    UnsupportedSyntax {
        input: String,
    },
    EmptyEntitySet {
        entity_kind: ShowEntityKind,
        selector: String,
    },
    OffsetOutOfRange {
        entity_kind: ShowEntityKind,
        selector: String,
        available: usize,
    },
}

impl std::fmt::Display for ShowSelectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedSyntax { input } => write!(
                f,
                "Unsupported HEAD selector syntax `{input}`. Supported forms: exact IDs, HEAD, HEAD~, HEAD~~, HEAD^"
            ),
            Self::EmptyEntitySet {
                entity_kind,
                selector,
            } => write!(
                f,
                "No {} available for selector `{selector}`.",
                entity_kind.plural_label()
            ),
            Self::OffsetOutOfRange {
                entity_kind,
                selector,
                available,
            } => write!(
                f,
                "Selector `{selector}` is out of range for {} (available: {available}).",
                entity_kind.plural_label()
            ),
        }
    }
}

impl std::error::Error for ShowSelectorError {}

/// Parse an exact ID or supported HEAD-relative form into a normalized selector.
pub fn parse_show_selector(input: &str) -> Result<ShowSelector, ShowSelectorError> {
    match input {
        "HEAD" => Ok(ShowSelector::Head { offset: 0 }),
        "HEAD~" | "HEAD^" => Ok(ShowSelector::Head { offset: 1 }),
        "HEAD~~" => Ok(ShowSelector::Head { offset: 2 }),
        _ if input.starts_with("HEAD~") || input.starts_with("HEAD^") => {
            Err(ShowSelectorError::UnsupportedSyntax {
                input: input.to_string(),
            })
        }
        _ => Ok(ShowSelector::ExactId(input.to_string())),
    }
}

/// Resolve a selector against the canonical ordered IDs for the requested entity family.
///
/// `HEAD` and its relatives resolve from the head of the stable list order, meaning the first
/// item in the corresponding list surface is also the `HEAD` item for the show surface.
pub fn resolve_show_selector(
    board_dir: &Path,
    board: &Board,
    entity_kind: ShowEntityKind,
    selector: &str,
) -> Result<String, ShowSelectorError> {
    let selector = parse_show_selector(selector)?;
    match selector {
        ShowSelector::ExactId(id) => Ok(id),
        ShowSelector::Head { offset } => {
            let ordered = ordered_show_ids(board_dir, board, entity_kind);
            let rendered = ShowSelector::Head { offset }.render();
            if ordered.is_empty() {
                return Err(ShowSelectorError::EmptyEntitySet {
                    entity_kind,
                    selector: rendered,
                });
            }

            ordered
                .get(offset)
                .cloned()
                .ok_or(ShowSelectorError::OffsetOutOfRange {
                    entity_kind,
                    selector: rendered,
                    available: ordered.len(),
                })
        }
    }
}

/// Return canonical ordered IDs for a showable entity family using the same stable list semantics
/// as the corresponding list surfaces, with all filters effectively enabled.
pub fn ordered_show_ids(
    board_dir: &Path,
    board: &Board,
    entity_kind: ShowEntityKind,
) -> Vec<String> {
    match entity_kind {
        ShowEntityKind::Mission => ordered_mission_ids(board),
        ShowEntityKind::Epic => ordered_epic_ids(board),
        ShowEntityKind::Voyage => ordered_voyage_ids(board),
        ShowEntityKind::Story => ordered_story_ids(board),
        ShowEntityKind::Bearing => ordered_bearing_ids(board_dir, board),
        ShowEntityKind::Adr => ordered_adr_ids(board),
        ShowEntityKind::Routine => ordered_routine_ids(board),
    }
}

pub fn ordered_mission_ids(board: &Board) -> Vec<String> {
    let mut ids: Vec<_> = board.missions.keys().cloned().collect();
    ids.sort();
    ids
}

pub fn ordered_epic_ids(board: &Board) -> Vec<String> {
    let mut ids: Vec<_> = board.epics.keys().cloned().collect();
    ids.sort();
    ids
}

pub fn ordered_voyage_ids(board: &Board) -> Vec<String> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|left, right| compare_voyages(board, left, right));
    voyages
        .into_iter()
        .map(|voyage| voyage.id().to_string())
        .collect()
}

pub fn ordered_story_ids(board: &Board) -> Vec<String> {
    let mut stories: Vec<_> = board.stories.values().collect();
    stories.sort_by(|left, right| compare_stories(board, left, right));
    stories
        .into_iter()
        .map(|story| story.id().to_string())
        .collect()
}

pub fn ordered_bearing_ids(board_dir: &Path, board: &Board) -> Vec<String> {
    let (config, _source) = load_config_from(board_dir);
    let weights = config.current_weights();
    let mut scored: Vec<_> = board
        .bearings
        .values()
        .map(|bearing| (bearing, bearing_sort_score(board_dir, bearing, &weights)))
        .collect();

    scored.sort_by(|left, right| match (left.1, right.1) {
        (Some(score_left), Some(score_right)) => score_right
            .partial_cmp(&score_left)
            .unwrap_or(Ordering::Equal)
            .then_with(|| left.0.id().cmp(right.0.id())),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left.0.id().cmp(right.0.id()),
    });

    scored
        .into_iter()
        .map(|(bearing, _)| bearing.id().to_string())
        .collect()
}

pub fn ordered_adr_ids(board: &Board) -> Vec<String> {
    let mut ids: Vec<_> = board.adrs.keys().cloned().collect();
    ids.sort();
    ids
}

pub fn ordered_routine_ids(board: &Board) -> Vec<String> {
    let mut ids: Vec<_> = board.routines.keys().cloned().collect();
    ids.sort();
    ids
}

fn compare_voyages(board: &Board, left: &Voyage, right: &Voyage) -> Ordering {
    let left_epic = board.epics.get(&left.epic_id);
    let right_epic = board.epics.get(&right.epic_id);

    let epic_cmp = match (
        left_epic.and_then(|epic| epic.index()),
        right_epic.and_then(|epic| epic.index()),
    ) {
        (Some(left_index), Some(right_index)) => left_index.cmp(&right_index),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left.epic_id.cmp(&right.epic_id),
    };

    if epic_cmp != Ordering::Equal {
        return epic_cmp;
    }

    match (left.index(), right.index()) {
        (Some(left_index), Some(right_index)) => left_index
            .cmp(&right_index)
            .then_with(|| left.id().cmp(right.id())),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => left.id().cmp(right.id()),
    }
}

fn compare_stories(
    board: &Board,
    left: &crate::domain::model::Story,
    right: &crate::domain::model::Story,
) -> Ordering {
    let left_epic_index = left
        .epic()
        .and_then(|id| board.epics.get(id))
        .and_then(|epic| epic.frontmatter.index)
        .unwrap_or(0);
    let right_epic_index = right
        .epic()
        .and_then(|id| board.epics.get(id))
        .and_then(|epic| epic.frontmatter.index)
        .unwrap_or(0);

    let epic_cmp = left_epic_index.cmp(&right_epic_index);
    if epic_cmp != Ordering::Equal {
        return epic_cmp;
    }

    let left_voyage_index = left
        .voyage()
        .and_then(|id| board.voyages.get(id))
        .and_then(|voyage| voyage.frontmatter.index)
        .unwrap_or(0);
    let right_voyage_index = right
        .voyage()
        .and_then(|id| board.voyages.get(id))
        .and_then(|voyage| voyage.frontmatter.index)
        .unwrap_or(0);

    let voyage_cmp = left_voyage_index.cmp(&right_voyage_index);
    if voyage_cmp != Ordering::Equal {
        return voyage_cmp;
    }

    left.index()
        .unwrap_or(0)
        .cmp(&right.index().unwrap_or(0))
        .then_with(|| left.id().cmp(right.id()))
}

fn bearing_sort_score(
    board_dir: &Path,
    bearing: &Bearing,
    weights: &crate::infrastructure::config::ModeWeights,
) -> Option<f64> {
    let readiness = evaluate_bearing_readiness(board_dir, bearing, Some(weights));
    readiness
        .score
        .as_ref()
        .map(|score| score.weighted_score)
        .or_else(|| {
            if !bearing.has_assessment {
                return None;
            }

            let assessment_path = board_dir
                .join("bearings")
                .join(bearing.id())
                .join("ASSESSMENT.md");
            let evidence_path = board_dir
                .join("bearings")
                .join(bearing.id())
                .join("EVIDENCE.md");

            load_bearing_score(&assessment_path, &evidence_path, weights)
                .ok()
                .map(|score| score.weighted_score)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{
        TestAdr, TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };
    use std::fs;
    use std::path::Path;

    fn write_routine(root: &Path, id: &str, title: &str, target_scope: &str) {
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

    fn strong_evidence_fixture() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | academic | manual:prior-art-review | https://example.com/paper | 2026-02-01 | 2026-03-07 | high | high | Prior art supports the direction. |
| SRC-02 | web | manual:official-doc | https://example.com/docs | 2026-03-01 | 2026-03-08 | high | high | Official docs confirm feasibility. |
"#
    }

    fn weak_evidence_fixture() -> &'static str {
        r#"
## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | social | manual:community-signal | https://example.com/thread | 2020-03-01 | 2026-03-08 | low | low | One anecdotal signal supports the direction. |
| SRC-02 | social | manual:community-signal | https://example.com/thread-2 | 2020-04-01 | 2026-03-08 | low | low | A second anecdotal signal supports the direction. |
"#
    }

    fn cited_assessment_fixture() -> &'static str {
        r#"
# Assessment

| Factor | Score |
|--------|-------|
| Impact | 4 |
| Confidence | 4 |
| Effort | 2 |
| Risk | 2 |

## Analysis

### Findings
- Delivery teams need source-backed recommendations before converting research into roadmap work [SRC-01][SRC-02]

### Opportunity Cost
Deferring this leaves roadmap work underspecified [SRC-02]

### Dependencies
- Evidence capture must produce canonical source records first [SRC-02]

### Alternatives Considered
- Keep factor-only scoring and trust operator judgment alone [SRC-01]

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-02]
[ ] Park → revisit later [SRC-02]
[ ] Decline → document learnings [SRC-01]
"#
    }

    fn seed_readiness_docs(board_dir: &Path, id: &str, evidence: &str, assessment: &str) {
        let bearing_dir = board_dir.join("bearings").join(id);
        fs::write(bearing_dir.join("EVIDENCE.md"), evidence).unwrap();
        fs::write(bearing_dir.join("ASSESSMENT.md"), assessment).unwrap();
    }

    fn build_selector_fixture(reverse_insertion: bool) -> tempfile::TempDir {
        let mut builder = TestBoardBuilder::new();

        if reverse_insertion {
            builder = builder
                .mission(TestMission::new("M3").title("Mission Three"))
                .mission(TestMission::new("M1").title("Mission One"))
                .mission(TestMission::new("M2").title("Mission Two"))
                .epic(TestEpic::new("E9").index(1))
                .epic(TestEpic::new("E1").index(9))
                .epic(TestEpic::new("E-UNINDEXED"))
                .voyage(TestVoyage::new("V-last", "E1").index(2).status("done"))
                .voyage(TestVoyage::new("V-first", "E9").index(1))
                .voyage(TestVoyage::new("V-unindexed", "E-UNINDEXED"))
                .story(
                    TestStory::new("S2")
                        .scope("E9/V-first")
                        .index(1)
                        .status(StoryState::Done),
                )
                .story(
                    TestStory::new("S1")
                        .scope("E-UNINDEXED/V-unindexed")
                        .index(4),
                )
                .story(
                    TestStory::new("S3")
                        .scope("E1/V-last")
                        .index(1)
                        .status(StoryState::InProgress),
                )
                .bearing(
                    TestBearing::new("B-strong")
                        .status("ready")
                        .has_evidence(true)
                        .has_assessment(true),
                )
                .bearing(
                    TestBearing::new("B-terminal")
                        .status("laid")
                        .has_evidence(false)
                        .has_assessment(false),
                )
                .bearing(
                    TestBearing::new("B-weak")
                        .status("ready")
                        .has_evidence(true)
                        .has_assessment(true),
                )
                .adr(TestAdr::new("ADR-010").status("accepted"))
                .adr(TestAdr::new("ADR-002").status("proposed"));
        } else {
            builder = builder
                .mission(TestMission::new("M2").title("Mission Two"))
                .mission(TestMission::new("M3").title("Mission Three"))
                .mission(TestMission::new("M1").title("Mission One"))
                .epic(TestEpic::new("E1").index(9))
                .epic(TestEpic::new("E-UNINDEXED"))
                .epic(TestEpic::new("E9").index(1))
                .voyage(TestVoyage::new("V-unindexed", "E-UNINDEXED"))
                .voyage(TestVoyage::new("V-first", "E9").index(1))
                .voyage(TestVoyage::new("V-last", "E1").index(2).status("done"))
                .story(
                    TestStory::new("S3")
                        .scope("E1/V-last")
                        .index(1)
                        .status(StoryState::InProgress),
                )
                .story(
                    TestStory::new("S1")
                        .scope("E-UNINDEXED/V-unindexed")
                        .index(4),
                )
                .story(
                    TestStory::new("S2")
                        .scope("E9/V-first")
                        .index(1)
                        .status(StoryState::Done),
                )
                .bearing(
                    TestBearing::new("B-weak")
                        .status("ready")
                        .has_evidence(true)
                        .has_assessment(true),
                )
                .bearing(
                    TestBearing::new("B-strong")
                        .status("ready")
                        .has_evidence(true)
                        .has_assessment(true),
                )
                .bearing(
                    TestBearing::new("B-terminal")
                        .status("laid")
                        .has_evidence(false)
                        .has_assessment(false),
                )
                .adr(TestAdr::new("ADR-002").status("proposed"))
                .adr(TestAdr::new("ADR-010").status("accepted"));
        }

        let temp = builder.build();
        seed_readiness_docs(
            temp.path(),
            "B-strong",
            strong_evidence_fixture(),
            cited_assessment_fixture(),
        );
        seed_readiness_docs(
            temp.path(),
            "B-weak",
            weak_evidence_fixture(),
            cited_assessment_fixture(),
        );
        write_routine(temp.path(), "routine-zeta", "Zeta Review", "E1/V-last");
        write_routine(temp.path(), "routine-alpha", "Alpha Review", "E9/V-first");
        temp
    }

    #[test]
    fn head_selector_parser_accepts_supported_forms() {
        assert_eq!(
            parse_show_selector("mission-123").unwrap(),
            ShowSelector::ExactId("mission-123".to_string())
        );
        assert_eq!(
            parse_show_selector("HEAD").unwrap(),
            ShowSelector::Head { offset: 0 }
        );
        assert_eq!(
            parse_show_selector("HEAD~").unwrap(),
            ShowSelector::Head { offset: 1 }
        );
        assert_eq!(
            parse_show_selector("HEAD^").unwrap(),
            ShowSelector::Head { offset: 1 }
        );
        assert_eq!(
            parse_show_selector("HEAD~~").unwrap(),
            ShowSelector::Head { offset: 2 }
        );
        assert_eq!(
            parse_show_selector("HEADROOM").unwrap(),
            ShowSelector::ExactId("HEADROOM".to_string())
        );
    }

    #[test]
    fn head_selector_parser_rejects_unsupported_forms() {
        let err = parse_show_selector("HEAD~~~").unwrap_err().to_string();
        assert_eq!(
            err,
            "Unsupported HEAD selector syntax `HEAD~~~`. Supported forms: exact IDs, HEAD, HEAD~, HEAD~~, HEAD^"
        );

        let err = parse_show_selector("HEAD~3").unwrap_err().to_string();
        assert_eq!(
            err,
            "Unsupported HEAD selector syntax `HEAD~3`. Supported forms: exact IDs, HEAD, HEAD~, HEAD~~, HEAD^"
        );

        let err = parse_show_selector("HEAD^^").unwrap_err().to_string();
        assert_eq!(
            err,
            "Unsupported HEAD selector syntax `HEAD^^`. Supported forms: exact IDs, HEAD, HEAD~, HEAD~~, HEAD^"
        );
    }

    #[test]
    fn head_selector_ordering_uses_canonical_list_semantics_for_each_entity_type() {
        let temp = build_selector_fixture(false);
        let board = load_board(temp.path()).unwrap();

        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Mission),
            vec!["M1", "M2", "M3"]
        );
        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Epic),
            vec!["E-UNINDEXED", "E1", "E9"]
        );
        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Voyage),
            vec!["V-first", "V-last", "V-unindexed"]
        );
        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Story),
            vec!["S1", "S2", "S3"]
        );
        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Bearing),
            vec!["B-strong", "B-weak", "B-terminal"]
        );
        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Adr),
            vec!["ADR-002", "ADR-010"]
        );
        assert_eq!(
            ordered_show_ids(temp.path(), &board, ShowEntityKind::Routine),
            vec!["routine-alpha", "routine-zeta"]
        );

        assert_eq!(
            resolve_show_selector(temp.path(), &board, ShowEntityKind::Bearing, "HEAD").unwrap(),
            "B-strong"
        );
        assert_eq!(
            resolve_show_selector(temp.path(), &board, ShowEntityKind::Story, "HEAD~").unwrap(),
            "S2"
        );
        assert_eq!(
            resolve_show_selector(temp.path(), &board, ShowEntityKind::Voyage, "HEAD^").unwrap(),
            "V-last"
        );
    }

    #[test]
    fn head_selector_ordering_reports_empty_and_out_of_range_errors_deterministically() {
        let temp = TestBoardBuilder::new().build();
        let board = load_board(temp.path()).unwrap();

        let empty_err = resolve_show_selector(temp.path(), &board, ShowEntityKind::Mission, "HEAD")
            .unwrap_err();
        assert_eq!(
            empty_err.to_string(),
            "No missions available for selector `HEAD`."
        );

        let fixture = build_selector_fixture(false);
        let board = load_board(fixture.path()).unwrap();
        let range_err =
            resolve_show_selector(fixture.path(), &board, ShowEntityKind::Adr, "HEAD~~")
                .unwrap_err();
        assert_eq!(
            range_err.to_string(),
            "Selector `HEAD~~` is out of range for ADRs (available: 2)."
        );
    }

    #[test]
    fn head_selector_determinism_resolves_same_targets_on_equivalent_boards() {
        let left = build_selector_fixture(false);
        let right = build_selector_fixture(true);
        let left_board = load_board(left.path()).unwrap();
        let right_board = load_board(right.path()).unwrap();

        let kinds = [
            ShowEntityKind::Mission,
            ShowEntityKind::Epic,
            ShowEntityKind::Voyage,
            ShowEntityKind::Story,
            ShowEntityKind::Bearing,
            ShowEntityKind::Adr,
            ShowEntityKind::Routine,
        ];

        for kind in kinds {
            assert_eq!(
                ordered_show_ids(left.path(), &left_board, kind),
                ordered_show_ids(right.path(), &right_board, kind)
            );
            assert_eq!(
                resolve_show_selector(left.path(), &left_board, kind, "HEAD").unwrap(),
                resolve_show_selector(right.path(), &right_board, kind, "HEAD").unwrap()
            );
            assert_eq!(
                resolve_show_selector(left.path(), &left_board, kind, "HEAD~").unwrap(),
                resolve_show_selector(right.path(), &right_board, kind, "HEAD~").unwrap()
            );
        }
    }
}
