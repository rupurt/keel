//! Shared invariants for keel validation
//!
//! This module defines the core validation rules used by both `keel doctor` and `keel next`.
//! Having a single source of truth eliminates drift between what doctor validates and what
//! commands implicitly require.
//!
//! # Invariants
//!
//! ## Voyage Ready for Work
//! A voyage is ready for work when:
//! - Status is `Planned` or `InProgress`
//! - Has requirements defined in the SRS FUNCTIONAL_REQUIREMENTS section
//!
//! ## Story Workable
//! A story is workable when:
//! - In `backlog` stage
//! - If scoped: voyage is ready for work
//! - If unscoped: always workable (no voyage to block on)
//!
//! # Coherence Rules
//!
//! Coherence rules ensure consistency between parent and child entities.
//! These are implemented as pure functions in [`state_machine::validation`].
//!
//! ## Voyage-Story Coherence
//! - **Draft voyages**: All stories should be in icebox (still being planned)
//! - **Planned voyages**: No stories should be active (not started yet)
//! - **In-progress voyages**: Normal operation (any story state allowed)
//! - **Done voyages**: No constraints (voyage is complete)
//! - **Non-draft voyages**: Must have at least one story
//! - **All stories done**: Voyage should be marked done (auto-fixable)
//!
//! ## Epic-Voyage Coherence
//! - **Done epics**: All voyages should be done
//! - **All voyages done**: Epic should be marked done (auto-fixable)
//!
//! See [`state_machine::validation::validate_voyage_story_coherence`] and
//! [`state_machine::validation::validate_epic_voyage_coherence`] for implementations.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::domain::model::{Board, Epic, Story, StoryState, Voyage, VoyageState};
use crate::infrastructure::validation::{CheckId, Problem};
use std::collections::{BTreeMap, HashSet};
use std::fs;

static REQ_TABLE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\|\s*(SRS-\d+)\s*\|").unwrap());
static AC_REQ_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(SRS-\d+)/AC-\d+\]").unwrap());
static REQ_REF_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bSRS-[A-Z0-9-]+\b").unwrap());
static SRS_REQUIREMENT_ID_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^SRS(?:-NFR)?-\d+$").unwrap());
static PRD_FUNCTIONAL_REQ_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^FR-\d+$").unwrap());
static PRD_NON_FUNCTIONAL_REQ_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^NFR-\d+$").unwrap());
static SOURCE_TOKEN_SPLIT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\s,;/]+").unwrap());

// Re-export coherence validation functions for a unified API
#[allow(unused_imports)] // SuggestedFix exported for public API completeness
pub use crate::domain::state_machine::validation::{
    EpicVoyageViolation, SuggestedFix, VoyageStoryViolation, validate_epic_voyage_coherence,
    validate_voyage_story_coherence,
};

/// Check if a voyage is ready for work.
///
/// A voyage is ready for work when:
/// 1. Status is `Planned` or `InProgress` (not `Draft` or `Done`)
/// 2. Has at least one requirement defined in the SRS
///
/// # Arguments
/// * `voyage` - The voyage to check
/// * `requirements` - List of requirement IDs (e.g., ["SRS-01", "SRS-02"])
#[allow(dead_code)] // Used by tests now, production use in future stories (SRS-05, SRS-06)
pub fn voyage_ready_for_work(voyage: &Voyage, requirements: &[String]) -> bool {
    let status_ok = matches!(
        voyage.status(),
        VoyageState::Planned | VoyageState::InProgress
    );
    let has_requirements = !requirements.is_empty();

    status_ok && has_requirements
}

/// Check if a story is workable.
///
/// A story is workable when:
/// 1. In `backlog` stage
/// 2. If scoped: voyage is ready for work
/// 3. If unscoped: always workable
///
/// Note: Dependency checking was removed. Dependencies will be derived from
/// SRS traceability in a future story.
///
/// # Arguments
/// * `story` - The story to check
/// * `board` - The board containing voyages
/// * `_board_dir` - Path to the board directory (for SRS file access)
#[allow(dead_code)] // Used by tests now, production use in future stories (SRS-05, SRS-06)
pub fn story_workable(story: &Story, board: &Board, _board_dir: &Path) -> bool {
    // Must be in backlog stage
    if story.stage != StoryState::Backlog {
        return false;
    }

    // Check voyage readiness
    match story.scope() {
        None => true, // Unscoped = always workable
        Some(scope) => {
            // Parse scope into epic/voyage
            let parts: Vec<&str> = scope.split('/').collect();
            if parts.len() != 2 {
                return true; // Invalid scope format, allow through
            }

            let voyage_id = parts[1];

            // Check if voyage exists
            let voyage = match board.voyages.get(voyage_id) {
                Some(v) => v,
                None => return false, // Voyage doesn't exist
            };

            // Get requirements from SRS file
            let srs_path = voyage.path.parent().unwrap().join("SRS.md");
            let requirements = parse_requirements(&srs_path);

            voyage_ready_for_work(voyage, &requirements)
        }
    }
}

/// Parse requirement IDs from an SRS file's FUNCTIONAL_REQUIREMENTS section.
///
/// # Arguments
/// * `srs_path` - Path to the SRS.md file
///
/// # Returns
/// Vector of requirement IDs (e.g., ["SRS-01", "SRS-02"])
#[allow(dead_code)] // Used by tests now, production use in future stories (SRS-05, SRS-06)
pub fn parse_requirements(srs_path: &Path) -> Vec<String> {
    let srs_content = match std::fs::read_to_string(srs_path) {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let mut requirements = Vec::new();
    let mut in_table = false;

    for line in srs_content.lines() {
        if line.contains("BEGIN FUNCTIONAL_REQUIREMENTS") {
            in_table = true;
            continue;
        }
        if line.contains("END FUNCTIONAL_REQUIREMENTS") {
            break;
        }
        if in_table
            && let Some(cap) = REQ_TABLE_REGEX.captures(line)
            && let Some(req_id) = cap.get(1)
        {
            requirements.push(req_id.as_str().to_string());
        }
    }

    requirements
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrdRequirementKind {
    Functional,
    NonFunctional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrdRequirementEntry {
    pub epic_id: String,
    pub id: String,
    pub description: String,
    pub kind: PrdRequirementKind,
    pub priority: Option<String>,
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrdRequirementLineage {
    pub epic_id: String,
    pub parent_requirements: BTreeMap<String, PrdRequirementEntry>,
}

impl PrdRequirementLineage {
    pub fn get(&self, id: &str) -> Option<&PrdRequirementEntry> {
        self.parent_requirements.get(id)
    }

    pub fn ordered_entries(&self) -> Vec<&PrdRequirementEntry> {
        self.parent_requirements.values().collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SrsRequirementKind {
    Functional,
    NonFunctional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrsRequirementEntry {
    pub id: String,
    pub description: String,
    pub kind: SrsRequirementKind,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrdLineageIssueKind {
    MissingSource,
    MultipleSources,
    NonCanonicalSource,
    UnknownParent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrdLineageIssue {
    pub srs_path: PathBuf,
    pub requirement_id: String,
    pub source_value: Option<String>,
    pub kind: PrdLineageIssueKind,
}

impl PrdLineageIssue {
    pub fn message(&self) -> String {
        match self.kind {
            PrdLineageIssueKind::MissingSource => format!(
                "{} in {} is missing Source (expected exactly one canonical FR-* or NFR-* token)",
                self.requirement_id,
                self.srs_path.display()
            ),
            PrdLineageIssueKind::MultipleSources => format!(
                "{} in {} has invalid Source '{}' (expected exactly one canonical FR-* or NFR-* token)",
                self.requirement_id,
                self.srs_path.display(),
                self.source_value.as_deref().unwrap_or("<empty>")
            ),
            PrdLineageIssueKind::NonCanonicalSource => format!(
                "{} in {} uses non-canonical Source '{}' (expected FR-* or NFR-*)",
                self.requirement_id,
                self.srs_path.display(),
                self.source_value.as_deref().unwrap_or("<empty>")
            ),
            PrdLineageIssueKind::UnknownParent => format!(
                "{} in {} references unknown parent Source '{}' (expected an FR-* or NFR-* defined in the epic PRD)",
                self.requirement_id,
                self.srs_path.display(),
                self.source_value.as_deref().unwrap_or("<empty>")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrdCoverageChild {
    pub voyage_id: String,
    pub voyage_index: Option<u32>,
    pub requirement_id: String,
    pub kind: SrsRequirementKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrdCoverageRow {
    pub parent: PrdRequirementEntry,
    pub linked_children: Vec<PrdCoverageChild>,
}

impl PrdCoverageRow {
    pub fn is_covered(&self) -> bool {
        !self.linked_children.is_empty()
    }
}

pub fn parse_prd_requirement_lineage(epic_id: &str, prd_path: &Path) -> PrdRequirementLineage {
    let prd_content = match fs::read_to_string(prd_path) {
        Ok(content) => content,
        Err(_) => {
            return PrdRequirementLineage {
                epic_id: epic_id.to_string(),
                parent_requirements: BTreeMap::new(),
            };
        }
    };

    let mut parent_requirements = BTreeMap::new();
    parse_prd_requirement_block(
        &prd_content,
        epic_id,
        "BEGIN FUNCTIONAL_REQUIREMENTS",
        "END FUNCTIONAL_REQUIREMENTS",
        PrdRequirementKind::Functional,
        &PRD_FUNCTIONAL_REQ_RE,
        &mut parent_requirements,
    );
    parse_prd_requirement_block(
        &prd_content,
        epic_id,
        "BEGIN NON_FUNCTIONAL_REQUIREMENTS",
        "END NON_FUNCTIONAL_REQUIREMENTS",
        PrdRequirementKind::NonFunctional,
        &PRD_NON_FUNCTIONAL_REQ_RE,
        &mut parent_requirements,
    );

    PrdRequirementLineage {
        epic_id: epic_id.to_string(),
        parent_requirements,
    }
}

pub fn parse_srs_requirement_entries(srs_path: &Path) -> Vec<SrsRequirementEntry> {
    let srs_content = match fs::read_to_string(srs_path) {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };

    let mut entries = Vec::new();
    parse_srs_requirement_block(
        &srs_content,
        "BEGIN FUNCTIONAL_REQUIREMENTS",
        "END FUNCTIONAL_REQUIREMENTS",
        SrsRequirementKind::Functional,
        &mut entries,
    );
    parse_srs_requirement_block(
        &srs_content,
        "BEGIN NON_FUNCTIONAL_REQUIREMENTS",
        "END NON_FUNCTIONAL_REQUIREMENTS",
        SrsRequirementKind::NonFunctional,
        &mut entries,
    );

    entries
}

pub fn evaluate_prd_srs_lineage(voyage: &Voyage, board: &Board) -> Vec<PrdLineageIssue> {
    let srs_path = voyage.path.parent().unwrap().join("SRS.md");
    let Some(epic) = board.epics.get(&voyage.epic_id) else {
        return Vec::new();
    };
    let prd_path = epic.path.parent().unwrap().join("PRD.md");
    let parent_lineage = parse_prd_requirement_lineage(epic.id(), &prd_path);
    let requirements = parse_srs_requirement_entries(&srs_path);

    let mut issues = Vec::new();
    for requirement in requirements {
        if let Err(issue) = canonical_prd_source_id(&requirement, &srs_path, &parent_lineage) {
            issues.push(issue);
        }
    }

    issues
}

pub fn prd_srs_lineage_problems(voyage: &Voyage, board: &Board, check_id: CheckId) -> Vec<Problem> {
    evaluate_prd_srs_lineage(voyage, board)
        .into_iter()
        .map(|issue| {
            Problem::error(issue.srs_path.clone(), issue.message())
                .with_scope(voyage.scope_path())
                .with_check_id(check_id)
        })
        .collect()
}

pub fn build_epic_prd_requirement_coverage(epic: &Epic, board: &Board) -> Vec<PrdCoverageRow> {
    let prd_path = epic.path.parent().unwrap_or(&epic.path).join("PRD.md");
    let parent_lineage = parse_prd_requirement_lineage(epic.id(), &prd_path);

    let mut linked_children_by_parent: BTreeMap<String, Vec<PrdCoverageChild>> = BTreeMap::new();
    let mut voyages = board.voyages_for_epic_id(epic.id());
    voyages.sort_by(|a, b| a.index().cmp(&b.index()).then_with(|| a.id().cmp(b.id())));

    for voyage in voyages {
        let srs_path = voyage.path.parent().unwrap_or(&voyage.path).join("SRS.md");
        for requirement in parse_srs_requirement_entries(&srs_path) {
            let Ok(parent_id) = canonical_prd_source_id(&requirement, &srs_path, &parent_lineage)
            else {
                continue;
            };

            linked_children_by_parent
                .entry(parent_id)
                .or_default()
                .push(PrdCoverageChild {
                    voyage_id: voyage.id().to_string(),
                    voyage_index: voyage.index(),
                    requirement_id: requirement.id,
                    kind: requirement.kind,
                });
        }
    }

    parent_lineage
        .ordered_entries()
        .into_iter()
        .map(|parent| {
            let mut linked_children = linked_children_by_parent
                .remove(&parent.id)
                .unwrap_or_default();
            linked_children.sort_by(|a, b| {
                a.voyage_index
                    .cmp(&b.voyage_index)
                    .then_with(|| a.voyage_id.cmp(&b.voyage_id))
                    .then_with(|| a.requirement_id.cmp(&b.requirement_id))
            });

            PrdCoverageRow {
                parent: parent.clone(),
                linked_children,
            }
        })
        .collect()
}

fn canonical_prd_source_id(
    requirement: &SrsRequirementEntry,
    srs_path: &Path,
    parent_lineage: &PrdRequirementLineage,
) -> Result<String, PrdLineageIssue> {
    let Some(raw_source) = requirement
        .source
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Err(PrdLineageIssue {
            srs_path: srs_path.to_path_buf(),
            requirement_id: requirement.id.clone(),
            source_value: None,
            kind: PrdLineageIssueKind::MissingSource,
        });
    };

    let source_tokens: Vec<&str> = SOURCE_TOKEN_SPLIT_RE
        .split(raw_source)
        .filter(|token| !token.is_empty())
        .collect();

    if source_tokens.len() != 1 {
        return Err(PrdLineageIssue {
            srs_path: srs_path.to_path_buf(),
            requirement_id: requirement.id.clone(),
            source_value: Some(raw_source.to_string()),
            kind: PrdLineageIssueKind::MultipleSources,
        });
    }

    let source_token = source_tokens[0];
    if !PRD_FUNCTIONAL_REQ_RE.is_match(source_token)
        && !PRD_NON_FUNCTIONAL_REQ_RE.is_match(source_token)
    {
        return Err(PrdLineageIssue {
            srs_path: srs_path.to_path_buf(),
            requirement_id: requirement.id.clone(),
            source_value: Some(source_token.to_string()),
            kind: PrdLineageIssueKind::NonCanonicalSource,
        });
    }

    if !parent_lineage
        .parent_requirements
        .contains_key(source_token)
    {
        return Err(PrdLineageIssue {
            srs_path: srs_path.to_path_buf(),
            requirement_id: requirement.id.clone(),
            source_value: Some(source_token.to_string()),
            kind: PrdLineageIssueKind::UnknownParent,
        });
    }

    Ok(source_token.to_string())
}

fn parse_prd_requirement_block(
    content: &str,
    epic_id: &str,
    start_marker: &str,
    end_marker: &str,
    kind: PrdRequirementKind,
    id_pattern: &Regex,
    parent_requirements: &mut BTreeMap<String, PrdRequirementEntry>,
) {
    let mut in_block = false;
    let mut priority_column_index: Option<usize> = None;
    let mut rationale_column_index: Option<usize> = None;

    for line in content.lines() {
        if line.contains(start_marker) {
            in_block = true;
            priority_column_index = None;
            rationale_column_index = None;
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

        let cols = markdown_row_columns(trimmed);
        if cols.len() < 2 {
            continue;
        }

        let id = cols[0];
        if id.eq_ignore_ascii_case("ID") {
            priority_column_index = cols
                .iter()
                .position(|col| col.eq_ignore_ascii_case("Priority"));
            rationale_column_index = cols
                .iter()
                .position(|col| col.eq_ignore_ascii_case("Rationale"));
            continue;
        }

        let description = cols[1];
        if id.starts_with("---") || description.is_empty() || !id_pattern.is_match(id) {
            continue;
        }

        let priority = priority_column_index
            .and_then(|idx| cols.get(idx))
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);
        let rationale = rationale_column_index
            .and_then(|idx| cols.get(idx))
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        parent_requirements
            .entry(id.to_string())
            .or_insert_with(|| PrdRequirementEntry {
                epic_id: epic_id.to_string(),
                id: id.to_string(),
                description: description.to_string(),
                kind,
                priority,
                rationale,
            });
    }
}

fn parse_srs_requirement_block(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    kind: SrsRequirementKind,
    entries: &mut Vec<SrsRequirementEntry>,
) {
    let mut in_block = false;
    let mut source_column_index: Option<usize> = None;

    for line in content.lines() {
        if line.contains(start_marker) {
            in_block = true;
            source_column_index = None;
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

        let cols = markdown_row_columns(trimmed);
        if cols.len() < 2 {
            continue;
        }

        let id = cols[0];
        if id.eq_ignore_ascii_case("ID") {
            source_column_index = cols
                .iter()
                .position(|col| col.eq_ignore_ascii_case("Source"));
            continue;
        }

        let description = cols[1];
        if id.starts_with("---") || description.is_empty() || !SRS_REQUIREMENT_ID_RE.is_match(id) {
            continue;
        }

        let source = source_column_index
            .and_then(|idx| cols.get(idx))
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        entries.push(SrsRequirementEntry {
            id: id.to_string(),
            description: description.to_string(),
            kind,
            source,
        });
    }
}

fn markdown_row_columns(row: &str) -> Vec<&str> {
    row.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(str::trim)
        .collect()
}

/// Return SRS requirements for a voyage that are not covered by any story
/// annotations in acceptance criteria.
///
/// This function is the core traceability check used by both planning and doctor
/// validation so behavior stays consistent.
#[allow(dead_code)] // Used by multiple command modules and tests
pub fn uncovered_requirements_for_voyage(voyage: &Voyage, board: &Board) -> Vec<String> {
    let srs_path = voyage.path.parent().unwrap().join("SRS.md");
    let all_requirements: HashSet<String> = parse_requirements(&srs_path).into_iter().collect();

    if all_requirements.is_empty() {
        return Vec::new();
    }

    let voyage_scope = format!("{}/{}", voyage.epic_id, voyage.id());
    let mut covered: HashSet<String> = HashSet::new();

    for story in board.stories.values() {
        if story.scope() != Some(&voyage_scope) {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for caps in AC_REQ_RE.captures_iter(&content) {
            covered.insert(caps[1].to_string());
        }

        for caps in REQ_REF_RE.captures_iter(&content) {
            covered.insert(caps[0].to_string());
        }
    }

    let mut missing: Vec<String> = all_requirements.difference(&covered).cloned().collect();
    missing.sort();

    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    use crate::domain::model::{StoryFrontmatter, VoyageFrontmatter};
    fn make_story(id: &str, stage: StoryState, scope: Option<&str>) -> Story {
        Story {
            frontmatter: StoryFrontmatter {
                id: id.to_string(),
                title: format!("Story {}", id),
                story_type: crate::domain::model::StoryType::Feat,
                status: stage,
                scope: scope.map(|s| s.to_string()),
                milestone: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                submitted_at: None,
                completed_at: None,
                index: None,
                governed_by: vec![],
                blocked_by: vec![],
                role: None,
            },
            path: std::path::PathBuf::from(format!("{}.md", id)),
            stage,
        }
    }

    fn make_voyage(id: &str, status: VoyageState, path: &Path) -> Voyage {
        Voyage {
            frontmatter: VoyageFrontmatter {
                id: id.to_string(),
                title: format!("Voyage {}", id),
                goal: None,
                status,
                epic: Some("test-epic".to_string()),
                index: None,
                created_at: None,
                updated_at: None,
                started_at: None,
                completed_at: None,
            },
            path: path.to_path_buf(),
            epic_id: "test-epic".to_string(),
        }
    }

    fn make_board(stories: Vec<Story>) -> Board {
        let mut board = Board::new(std::path::PathBuf::new());
        for story in stories {
            board.stories.insert(story.id().to_string(), story);
        }
        board
    }

    fn create_voyage_with_srs(root: &Path, epic: &str, voyage: &str, req_count: usize) {
        let voyage_path = root.join("epics").join(epic).join("voyages").join(voyage);
        fs::create_dir_all(&voyage_path).unwrap();

        // Create SRS with requirements
        let mut srs = String::from("# SRS\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n");
        srs.push_str("| ID | Requirement | Verification |\n");
        srs.push_str("|----|-------------|--------------|\n");
        for i in 1..=req_count {
            srs.push_str(&format!("| SRS-{:02} | Requirement {} | test |\n", i, i));
        }
        srs.push_str("<!-- END FUNCTIONAL_REQUIREMENTS -->\n");

        fs::write(voyage_path.join("SRS.md"), srs).unwrap();
        fs::write(voyage_path.join("README.md"), "---\nid: voyage\n---\n").unwrap();
    }

    // ==================== voyage_ready_for_work tests ====================

    #[test]
    fn voyage_ready_for_work_returns_true_for_planned_with_requirements() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::Planned,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string(), "SRS-02".to_string()];

        assert!(voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_true_for_in_progress_with_requirements() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::InProgress,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string()];

        assert!(voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_false_for_draft() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::Draft,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string()];

        assert!(!voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_false_for_done() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::Done,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements = vec!["SRS-01".to_string()];

        assert!(!voyage_ready_for_work(&voyage, &requirements));
    }

    #[test]
    fn voyage_ready_for_work_returns_false_without_requirements() {
        let temp = TempDir::new().unwrap();
        let voyage = make_voyage(
            "01-voyage",
            VoyageState::InProgress,
            &temp.path().join("voyages/01-voyage/README.md"),
        );
        let requirements: Vec<String> = vec![];

        assert!(!voyage_ready_for_work(&voyage, &requirements));
    }

    // ==================== story_workable tests ====================

    #[test]
    fn story_workable_returns_false_for_in_progress_story() {
        let temp = TempDir::new().unwrap();
        let story = make_story("S1", StoryState::InProgress, None);
        let board = make_board(vec![story.clone()]);

        assert!(!story_workable(&story, &board, temp.path()));
    }

    #[test]
    fn story_workable_returns_true_for_unscoped_backlog_story() {
        let temp = TempDir::new().unwrap();
        let story = make_story("S1", StoryState::Backlog, None);
        let board = make_board(vec![story.clone()]);

        assert!(story_workable(&story, &board, temp.path()));
    }

    #[test]
    fn story_workable_returns_true_for_scoped_with_ready_voyage() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create voyage with requirements
        create_voyage_with_srs(root, "test-epic", "01-voyage", 3);

        let story = make_story("S1", StoryState::Backlog, Some("test-epic/01-voyage"));
        let mut board = make_board(vec![story.clone()]);

        // Add voyage to board
        board.voyages.insert(
            "01-voyage".to_string(),
            make_voyage(
                "01-voyage",
                VoyageState::InProgress,
                &root.join("epics/test-epic/voyages/01-voyage/README.md"),
            ),
        );

        assert!(story_workable(&story, &board, root));
    }

    #[test]
    fn story_workable_returns_false_for_scoped_with_draft_voyage() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create voyage with requirements
        create_voyage_with_srs(root, "test-epic", "01-voyage", 3);

        let story = make_story("S1", StoryState::Backlog, Some("test-epic/01-voyage"));
        let mut board = make_board(vec![story.clone()]);

        // Add draft voyage to board
        board.voyages.insert(
            "01-voyage".to_string(),
            make_voyage(
                "01-voyage",
                VoyageState::Draft,
                &root.join("epics/test-epic/voyages/01-voyage/README.md"),
            ),
        );

        assert!(!story_workable(&story, &board, root));
    }

    // ==================== parse_requirements tests ====================

    #[test]
    fn parse_requirements_extracts_ids_from_srs() {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        create_voyage_with_srs(root, "test-epic", "01-voyage", 3);

        let srs_path = root.join("epics/test-epic/voyages/01-voyage/SRS.md");
        let requirements = parse_requirements(&srs_path);

        assert_eq!(requirements.len(), 3);
        assert!(requirements.contains(&"SRS-01".to_string()));
        assert!(requirements.contains(&"SRS-02".to_string()));
        assert!(requirements.contains(&"SRS-03".to_string()));
    }

    #[test]
    fn parse_requirements_returns_empty_for_missing_file() {
        let temp = TempDir::new().unwrap();
        let srs_path = temp.path().join("nonexistent/SRS.md");

        let requirements = parse_requirements(&srs_path);

        assert!(requirements.is_empty());
    }

    #[test]
    fn parse_requirements_returns_empty_when_no_requirements_section() {
        let temp = TempDir::new().unwrap();
        let srs_path = temp.path().join("SRS.md");

        fs::write(&srs_path, "# SRS\n\nNo requirements here.").unwrap();

        let requirements = parse_requirements(&srs_path);

        assert!(requirements.is_empty());
    }

    #[test]
    fn srs_source_requires_exactly_one_canonical_prd_parent() {
        use crate::infrastructure::loader::load_board;
        use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Valid parent source | FR-01 | test |
| SRS-02 | Missing source |  | test |
| SRS-03 | Multiple sources | FR-01, FR-02 | test |
| SRS-04 | Legacy alias | PRD-01 | test |
| SRS-05 | Unknown parent | FR-99 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Valid NFR source | NFR-01 | test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("PLAN01")
                    .scope("test-epic/01-draft")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] test"),
            )
            .build();
        fs::write(
            temp.path().join("epics/test-epic/PRD.md"),
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Valid source. | must | test |
| FR-02 | Extra valid source. | should | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Valid NFR source. | must | test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-draft").unwrap();
        let issues = evaluate_prd_srs_lineage(voyage, &board);

        assert_eq!(issues.len(), 4);
        assert!(issues.iter().any(|issue| {
            issue.requirement_id == "SRS-02" && issue.kind == PrdLineageIssueKind::MissingSource
        }));
        assert!(issues.iter().any(|issue| {
            issue.requirement_id == "SRS-03" && issue.kind == PrdLineageIssueKind::MultipleSources
        }));
        assert!(issues.iter().any(|issue| {
            issue.requirement_id == "SRS-04"
                && issue.kind == PrdLineageIssueKind::NonCanonicalSource
                && issue.source_value.as_deref() == Some("PRD-01")
        }));
        assert!(issues.iter().any(|issue| {
            issue.requirement_id == "SRS-05"
                && issue.kind == PrdLineageIssueKind::UnknownParent
                && issue.source_value.as_deref() == Some("FR-99")
        }));
    }

    #[test]
    fn prd_lineage_parser_builds_canonical_parent_map() {
        let temp = TempDir::new().unwrap();
        let prd_path = temp.path().join("PRD.md");
        fs::write(
            &prd_path,
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-02 | Render epic coverage. | should | Coverage needs a parent contract. |
| PRD-99 | legacy alias should be ignored | must | hard-cutover |
| FR-01 | Parse canonical lineage. | must | Shared consumers need one parser. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-02 | Keep ordering deterministic. | must | Stable planning output. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let lineage = parse_prd_requirement_lineage("epic-123", &prd_path);

        assert_eq!(lineage.epic_id, "epic-123");
        assert_eq!(lineage.parent_requirements.len(), 3);
        assert!(!lineage.parent_requirements.contains_key("PRD-99"));

        let functional = lineage.get("FR-01").unwrap();
        assert_eq!(functional.epic_id, "epic-123");
        assert_eq!(functional.kind, PrdRequirementKind::Functional);
        assert_eq!(functional.description, "Parse canonical lineage.");
        assert_eq!(functional.priority.as_deref(), Some("must"));
        assert_eq!(
            functional.rationale.as_deref(),
            Some("Shared consumers need one parser.")
        );

        let non_functional = lineage.get("NFR-02").unwrap();
        assert_eq!(non_functional.kind, PrdRequirementKind::NonFunctional);
        assert_eq!(non_functional.description, "Keep ordering deterministic.");
    }

    #[test]
    fn prd_lineage_model_exposes_reusable_parent_metadata() {
        let temp = TempDir::new().unwrap();
        let prd_path = temp.path().join("PRD.md");
        fs::write(
            &prd_path,
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-03 | Block invalid source lineage. | must | Gates and doctor must share one parent record. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Reject aliases. | must | Hard cutover only. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let lineage = parse_prd_requirement_lineage("epic-abc", &prd_path);
        let ordered = lineage.ordered_entries();

        assert_eq!(ordered.len(), 2);
        assert_eq!(ordered[0].id, "FR-03");
        assert_eq!(ordered[0].epic_id, "epic-abc");
        assert_eq!(ordered[0].priority.as_deref(), Some("must"));
        assert_eq!(
            ordered[0].rationale.as_deref(),
            Some("Gates and doctor must share one parent record.")
        );
        assert_eq!(ordered[1].id, "NFR-01");
        assert_eq!(ordered[1].kind, PrdRequirementKind::NonFunctional);
    }

    #[test]
    fn prd_lineage_parser_is_deterministic() {
        let temp = TempDir::new().unwrap();
        let prd_path_a = temp.path().join("PRD-a.md");
        let prd_path_b = temp.path().join("PRD-b.md");
        fs::write(
            &prd_path_a,
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-10 | Render planning coverage. | should | Coverage needs parent summaries. |
| FR-02 | Parse parent lineage. | must | Shared parsing. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-03 | Emit actionable errors. | must | Diagnostics must stay useful. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();
        fs::write(
            &prd_path_b,
            r#"# PRD

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-03 | Emit actionable errors. | must | Diagnostics must stay useful. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-02 | Parse parent lineage. | must | Shared parsing. |
| FR-10 | Render planning coverage. | should | Coverage needs parent summaries. |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let lineage_a = parse_prd_requirement_lineage("epic-det", &prd_path_a);
        let lineage_b = parse_prd_requirement_lineage("epic-det", &prd_path_b);

        assert_eq!(lineage_a, lineage_b);
        let ordered_ids: Vec<_> = lineage_a
            .ordered_entries()
            .into_iter()
            .map(|entry| entry.id.clone())
            .collect();
        assert_eq!(ordered_ids, vec!["FR-02", "FR-10", "NFR-03"]);
    }

    #[test]
    fn prd_requirement_coverage_preserves_one_to_many_parent_fanout() {
        let temp = crate::test_helpers::TestBoardBuilder::new()
            .epic(crate::test_helpers::TestEpic::new("epic-1"))
            .voyage(
                crate::test_helpers::TestVoyage::new("v1", "epic-1")
                    .index(2)
                    .srs_content(
                        r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-02 | Second child. | FR-01 | cargo test |
| SRS-01 | First child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
                    ),
            )
            .voyage(
                crate::test_helpers::TestVoyage::new("v2", "epic-1")
                    .index(1)
                    .srs_content(
                        r#"# SRS

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Fanout child. | NFR-01 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
                    ),
            )
            .build();
        fs::write(
            temp.path().join("epics/epic-1/PRD.md"),
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Shared parent. | must | fanout |
| FR-02 | Uncovered parent. | should | uncovered |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Non-functional parent. | must | coverage |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic-1").unwrap();

        let coverage = build_epic_prd_requirement_coverage(epic, &board);

        assert_eq!(coverage.len(), 3);
        let fr01 = coverage
            .iter()
            .find(|row| row.parent.id == "FR-01")
            .unwrap();
        assert!(fr01.is_covered());
        let fr01_children: Vec<_> = fr01
            .linked_children
            .iter()
            .map(|child| format!("{}/{}", child.voyage_id, child.requirement_id))
            .collect();
        assert_eq!(fr01_children, vec!["v1/SRS-01", "v1/SRS-02"]);

        let fr02 = coverage
            .iter()
            .find(|row| row.parent.id == "FR-02")
            .unwrap();
        assert!(!fr02.is_covered());
        assert!(fr02.linked_children.is_empty());

        let nfr01 = coverage
            .iter()
            .find(|row| row.parent.id == "NFR-01")
            .unwrap();
        assert_eq!(nfr01.linked_children.len(), 1);
        assert_eq!(nfr01.linked_children[0].voyage_id, "v2");
        assert_eq!(nfr01.linked_children[0].requirement_id, "SRS-NFR-01");
    }

    #[test]
    fn prd_requirement_coverage_ignores_invalid_or_unknown_sources() {
        let temp = crate::test_helpers::TestBoardBuilder::new()
            .epic(crate::test_helpers::TestEpic::new("epic-1"))
            .voyage(
                crate::test_helpers::TestVoyage::new("v1", "epic-1").srs_content(
                    r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Missing source. |  | cargo test |
| SRS-02 | Alias source. | PRD-01 | cargo test |
| SRS-03 | Unknown source. | FR-99 | cargo test |
| SRS-04 | Valid source. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
            "#,
                ),
            )
            .build();
        fs::write(
            temp.path().join("epics/epic-1/PRD.md"),
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Canonical parent. | must | coverage |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic-1").unwrap();

        let coverage = build_epic_prd_requirement_coverage(epic, &board);

        assert_eq!(coverage.len(), 1);
        let parent = &coverage[0];
        let child_ids: Vec<_> = parent
            .linked_children
            .iter()
            .map(|child| child.requirement_id.as_str())
            .collect();
        assert_eq!(child_ids, vec!["SRS-04"]);
    }

    #[test]
    fn uncovered_requirements_for_voyage_reports_missing_ids() {
        use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
| SRS-02 | Requirement 2 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-voyage", "test-epic").srs_content(srs))
            .story(
                TestStory::new("1")
                    .scope("test-epic/01-voyage")
                    .body("- [ ] [SRS-01/AC-01] coverage point"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-voyage").unwrap();

        let missing = uncovered_requirements_for_voyage(voyage, &board);

        assert_eq!(missing, vec!["SRS-02".to_string()]);
    }

    #[test]
    fn uncovered_requirements_for_voyage_returns_none_when_all_covered() {
        use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
| SRS-02 | Requirement 2 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-voyage", "test-epic").srs_content(srs))
            .story(
                TestStory::new("1")
                    .scope("test-epic/01-voyage")
                    .body("- [ ] [SRS-01/AC-01] coverage point"),
            )
            .story(
                TestStory::new("2")
                    .scope("test-epic/01-voyage")
                    .body("- [ ] [SRS-02/AC-01] second point"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-voyage").unwrap();

        let missing = uncovered_requirements_for_voyage(voyage, &board);

        assert!(missing.is_empty());
    }
}
