//! Canonical planning-show projection contracts shared by epic/voyage/story show commands.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime};

use crate::domain::model::{Board, Epic, EpicState, Story, StoryState, Voyage};
use crate::domain::state_machine::invariants;
use crate::infrastructure::verification::parser::{
    Comparison, parse_ac_references, parse_verify_annotations,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlanningDocSummary {
    pub problem_statement: Option<String>,
    pub goals: Vec<String>,
    pub verification_strategy: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EtaSummary {
    pub throughput_stories_per_week: f64,
    pub remaining_stories: usize,
    pub eta_weeks: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VerificationRollup {
    pub automated_requirements: BTreeSet<String>,
    pub manual_requirements: BTreeSet<String>,
    pub automated_criteria: usize,
    pub manual_criteria: usize,
    pub used_techniques: BTreeSet<String>,
    pub linked_artifacts: BTreeSet<String>,
    pub all_artifacts: BTreeSet<String>,
    pub missing_linked_proofs: usize,
}

impl VerificationRollup {
    pub fn artifact_counts(&self) -> (usize, usize, usize) {
        let mut text = 0;
        let mut media = 0;
        let mut other = 0;

        for artifact in &self.all_artifacts {
            if is_media_artifact(artifact) {
                media += 1;
            } else if is_text_artifact(artifact) {
                text += 1;
            } else {
                other += 1;
            }
        }

        (text, media, other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpicShowProjection {
    pub doc: PlanningDocSummary,
    pub scope_drift: Vec<ScopeDriftRow>,
    pub requirement_coverage: Vec<EpicRequirementCoverageRow>,
    pub total_voyages: usize,
    pub done_voyages: usize,
    pub total_stories: usize,
    pub done_stories: usize,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub eta: EtaSummary,
    pub verification: VerificationRollup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeLineageRow {
    pub scope_id: String,
    pub voyage_description: String,
    pub voyage_disposition: invariants::ScopeDisposition,
    pub epic_description: Option<String>,
    pub epic_disposition: Option<invariants::ScopeDisposition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeDriftRow {
    pub voyage_id: Option<String>,
    pub issue: invariants::ScopeLineageIssue,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScopeSummary {
    pub in_scope: Vec<String>,
    pub out_of_scope: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryRef {
    pub id: String,
    pub stage: StoryState,
    pub index: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpicRequirementCoverageChild {
    pub voyage_id: String,
    pub requirement_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpicRequirementCoverageRow {
    pub id: String,
    pub description: String,
    pub linked_goals: Vec<String>,
    pub linked_children: Vec<EpicRequirementCoverageChild>,
}

impl EpicRequirementCoverageRow {
    pub fn linked_child_count(&self) -> usize {
        self.linked_children.len()
    }

    pub fn is_covered(&self) -> bool {
        !self.linked_children.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementRow {
    pub id: String,
    pub description: String,
    pub kind: RequirementKind,
    pub linked_stories: Vec<StoryRef>,
    pub completion: RequirementCompletion,
    pub verification: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoyageShowProjection {
    pub goal: Option<String>,
    pub scope: ScopeSummary,
    pub scope_lineage: Vec<ScopeLineageRow>,
    pub scope_drift: Vec<ScopeDriftRow>,
    pub requirements: Vec<RequirementRow>,
    pub done_stories: usize,
    pub total_stories: usize,
    pub done_functional_requirements: usize,
    pub total_functional_requirements: usize,
    pub done_non_functional_requirements: usize,
    pub total_non_functional_requirements: usize,
    pub done_requirements: usize,
    pub total_requirements: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProofMetadata {
    pub recorded_at: Option<String>,
    pub mode: Option<String>,
    pub command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceItem {
    pub ac_label: Option<String>,
    pub criterion: String,
    pub requirement: Option<String>,
    pub mode: String,
    pub command: Option<String>,
    pub proof_filename: Option<String>,
    pub proof_metadata: ProofMetadata,
    pub excerpt_lines: Vec<String>,
    pub missing_proof: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvidenceReport {
    pub items: Vec<EvidenceItem>,
    pub evidence_dir_missing: bool,
    pub linked_proofs: Vec<String>,
    pub supplementary_artifacts: Vec<String>,
    pub media_artifacts: Vec<String>,
    pub missing_proofs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryShowProjection {
    pub body: Option<String>,
    pub checked_criteria: usize,
    pub total_criteria: usize,
    pub evidence: EvidenceReport,
}

#[derive(Debug, Clone)]
struct StoryEvidence {
    id: String,
    stage: StoryState,
    index: Option<u32>,
    references: Vec<String>,
    automated_count_by_req: BTreeMap<String, usize>,
    manual_count_by_req: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RequirementEntry {
    id: String,
    description: String,
    kind: RequirementKind,
    verification: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RequirementKind {
    #[default]
    Functional,
    NonFunctional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementCompletion {
    Unmapped,
    Queued,
    InProgress,
    Done,
}

impl RequirementCompletion {
    pub fn is_done(self) -> bool {
        self == Self::Done
    }
}

pub fn build_epic_show_projection(board: &Board, epic: &Epic) -> Result<EpicShowProjection> {
    let prd_path = epic.path.parent().unwrap().join("PRD.md");
    let prd_content = fs::read_to_string(&prd_path).unwrap_or_default();
    let doc = extract_planning_doc_summary(&prd_content);
    let scope_drift = build_epic_scope_drift(board, epic);
    let requirement_coverage = invariants::build_epic_prd_requirement_coverage(epic, board)
        .into_iter()
        .map(|row| {
            let mut linked_goals = row.parent.goal_refs;
            linked_goals.sort();
            linked_goals.dedup();

            EpicRequirementCoverageRow {
                id: row.parent.id,
                description: row.parent.description,
                linked_goals,
                linked_children: row
                    .linked_children
                    .into_iter()
                    .map(|child| EpicRequirementCoverageChild {
                        voyage_id: child.voyage_id,
                        requirement_id: child.requirement_id,
                    })
                    .collect(),
            }
        })
        .collect();

    let voyages = board.voyages_for_epic_id(epic.id());
    let done_voyages = voyages
        .iter()
        .filter(|voyage| voyage.status().to_string() == "done")
        .count();

    let mut stories: Vec<_> = board
        .stories
        .values()
        .filter(|story| story.epic() == Some(epic.id()))
        .collect();
    stories.sort_by(|a, b| a.id().cmp(b.id()));

    let done_stories = stories
        .iter()
        .filter(|story| story.stage.to_string() == "done")
        .count();
    let total_stories = stories.len();
    let remaining_stories = total_stories.saturating_sub(done_stories);

    let started_at = voyages
        .iter()
        .filter_map(|voyage| voyage.frontmatter.started_at)
        .min();
    let completed_at = if epic.status() == EpicState::Done {
        voyages
            .iter()
            .filter_map(|voyage| voyage.frontmatter.completed_at)
            .max()
    } else {
        None
    };

    let voyage_updated_at = voyages
        .iter()
        .flat_map(|voyage| {
            [
                voyage.frontmatter.updated_at,
                voyage.frontmatter.completed_at,
                voyage.frontmatter.started_at,
                voyage.frontmatter.created_at,
            ]
        })
        .flatten()
        .max();
    let story_updated_at = stories
        .iter()
        .flat_map(|story| {
            [
                story.frontmatter.updated_at,
                story.frontmatter.completed_at,
                story.frontmatter.submitted_at,
                story.frontmatter.started_at,
                story.frontmatter.created_at,
            ]
        })
        .flatten()
        .max();
    let updated_at = [voyage_updated_at, story_updated_at]
        .into_iter()
        .flatten()
        .max();

    let throughput_stories_per_week = average_story_throughput_per_week(board, 4);
    let eta_weeks = if remaining_stories > 0 && throughput_stories_per_week > 0.0 {
        Some(remaining_stories as f64 / throughput_stories_per_week)
    } else {
        None
    };

    let mut verification = VerificationRollup::default();
    for story in stories {
        let story_content = fs::read_to_string(&story.path).unwrap_or_default();
        for ann in parse_verify_annotations(&story_content) {
            if let Some(command) = ann.command.as_deref() {
                let command = command.trim();
                if command.starts_with("vhs ") || command == "vhs" {
                    verification.used_techniques.insert("vhs".to_string());
                }
                if command.starts_with("llm-judge") || command == "llm-judge" {
                    verification.used_techniques.insert("llm-judge".to_string());
                }
                if command.contains("playwright") {
                    verification
                        .used_techniques
                        .insert("playwright".to_string());
                }
            }

            if ann.comparison == Comparison::Manual {
                verification.manual_criteria += 1;
                if let Some(req) = ann.requirement {
                    verification.manual_requirements.insert(req.id);
                }
            } else {
                verification.automated_criteria += 1;
                if let Some(req) = ann.requirement {
                    verification.automated_requirements.insert(req.id);
                }
            }

            if let Some(proof) = ann.proof {
                let proof_path = story.path.parent().unwrap().join("EVIDENCE").join(&proof);
                let rel = format!("stories/{}/EVIDENCE/{}", story.id(), proof);
                if proof_path.exists() {
                    verification.linked_artifacts.insert(rel);
                } else {
                    verification.missing_linked_proofs += 1;
                }
            }
        }

        let evidence_dir = story.path.parent().unwrap().join("EVIDENCE");
        if evidence_dir.exists()
            && let Ok(entries) = fs::read_dir(evidence_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    verification.all_artifacts.insert(format!(
                        "stories/{}/EVIDENCE/{}",
                        story.id(),
                        name
                    ));
                }
            }
        }
    }

    Ok(EpicShowProjection {
        doc,
        scope_drift,
        requirement_coverage,
        total_voyages: voyages.len(),
        done_voyages,
        total_stories,
        done_stories,
        started_at,
        completed_at,
        updated_at,
        eta: EtaSummary {
            throughput_stories_per_week,
            remaining_stories,
            eta_weeks,
        },
        verification,
    })
}

pub fn build_voyage_show_projection(
    board: &Board,
    voyage: &Voyage,
) -> Result<VoyageShowProjection> {
    let srs_path = voyage.path.parent().unwrap().join("SRS.md");
    let srs = fs::read_to_string(&srs_path).unwrap_or_default();

    let goal = voyage
        .frontmatter
        .goal
        .as_ref()
        .map(|goal| goal.trim().to_string())
        .filter(|goal| !goal.is_empty())
        .or_else(|| extract_goal_from_srs(&srs));

    let scope = parse_scope_summary(&srs);
    let (scope_lineage, scope_drift) = build_voyage_scope_projection(board, voyage, &srs);
    let mut requirements = parse_srs_requirement_rows(&srs);
    requirements.sort_by(|a, b| a.id.cmp(&b.id));

    let scope_key = voyage.scope_path();
    let mut story_evidence: Vec<StoryEvidence> = board
        .stories
        .values()
        .filter(|story| story.scope() == Some(scope_key.as_str()))
        .filter_map(|story| {
            let content = fs::read_to_string(&story.path).ok()?;
            let mut references: Vec<String> = parse_ac_references(&content)
                .into_iter()
                .map(|reference| reference.srs_id)
                .collect();

            let mut automated_count_by_req: BTreeMap<String, usize> = BTreeMap::new();
            let mut manual_count_by_req: BTreeMap<String, usize> = BTreeMap::new();

            for ann in parse_verify_annotations(&content) {
                let req_id = ann
                    .requirement
                    .as_ref()
                    .map(|req| req.id.clone())
                    .or_else(|| ann.ac_ref.as_ref().map(|ac| ac.srs_id.clone()));
                let Some(req_id) = req_id else {
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
    story_evidence.sort_by(story_evidence_ordering);

    let total_stories = story_evidence.len();
    let done_stories = story_evidence
        .iter()
        .filter(|story| story.stage == StoryState::Done)
        .count();

    let mut rows = Vec::new();
    for requirement in requirements {
        let mut linked_stories: Vec<StoryRef> = story_evidence
            .iter()
            .filter(|story| story.references.iter().any(|req| req == &requirement.id))
            .map(|story| StoryRef {
                id: story.id.clone(),
                stage: story.stage,
                index: story.index,
            })
            .collect();
        linked_stories.sort_by(story_ref_ordering);

        let completion = requirement_completion_state(&linked_stories);
        let automated = story_evidence
            .iter()
            .filter_map(|story| story.automated_count_by_req.get(&requirement.id))
            .sum::<usize>();
        let manual = story_evidence
            .iter()
            .filter_map(|story| story.manual_count_by_req.get(&requirement.id))
            .sum::<usize>();

        rows.push(RequirementRow {
            id: requirement.id,
            description: requirement.description,
            kind: requirement.kind,
            linked_stories,
            completion,
            verification: requirement_verification_label(automated, manual),
        });
    }

    let functional_rows: Vec<&RequirementRow> = rows
        .iter()
        .filter(|row| row.kind == RequirementKind::Functional)
        .collect();
    let non_functional_rows: Vec<&RequirementRow> = rows
        .iter()
        .filter(|row| row.kind == RequirementKind::NonFunctional)
        .collect();

    let done_functional_requirements = functional_rows
        .iter()
        .filter(|row| row.completion.is_done())
        .count();
    let done_non_functional_requirements = non_functional_rows
        .iter()
        .filter(|row| row.completion.is_done())
        .count();
    let total_functional_requirements = functional_rows.len();
    let total_non_functional_requirements = non_functional_rows.len();

    Ok(VoyageShowProjection {
        goal,
        scope,
        scope_lineage,
        scope_drift,
        done_stories,
        total_stories,
        done_functional_requirements,
        total_functional_requirements,
        done_non_functional_requirements,
        total_non_functional_requirements,
        // Canonical completion rollup tracks functional requirements only.
        done_requirements: done_functional_requirements,
        total_requirements: total_functional_requirements,
        requirements: rows,
    })
}

pub fn build_story_show_projection(story: &Story) -> Result<StoryShowProjection> {
    let content = fs::read_to_string(&story.path)?;
    let body = extract_story_body(&content);
    let (checked_criteria, total_criteria) = body
        .as_deref()
        .map(count_story_acceptance_criteria)
        .unwrap_or((0, 0));

    Ok(StoryShowProjection {
        body,
        checked_criteria,
        total_criteria,
        evidence: build_story_evidence_projection(&story.path, &content),
    })
}

pub fn build_story_evidence_projection(story_path: &Path, content: &str) -> EvidenceReport {
    let evidence_dir = story_path.parent().unwrap().join("EVIDENCE");
    let evidence_dir_missing = !evidence_dir.exists();

    let mut all_artifacts = Vec::new();
    if !evidence_dir_missing && let Ok(entries) = fs::read_dir(&evidence_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                all_artifacts.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    all_artifacts.sort();

    let mut linked = BTreeSet::new();
    let mut missing = BTreeSet::new();
    let mut items = Vec::new();

    for ann in parse_verify_annotations(content) {
        let ac_label = ann.ac_ref.as_ref().map(|ac| format!("AC-{:02}", ac.ac_num));
        let proof_filename = ann.proof.clone();
        let requirement = ann
            .requirement
            .as_ref()
            .map(|req| req.id.clone())
            .or_else(|| ann.ac_ref.as_ref().map(|ac| ac.srs_id.clone()));

        let mut proof_metadata = ProofMetadata::default();
        let mut excerpt_lines = Vec::new();
        let mut missing_proof = false;

        if let Some(proof) = &proof_filename {
            let proof_path = evidence_dir.join(proof);
            if proof_path.exists() {
                linked.insert(proof.clone());
                if is_text_artifact(proof) {
                    let (metadata, excerpt) = parse_proof_metadata_and_excerpt(&proof_path);
                    proof_metadata = metadata;
                    excerpt_lines = excerpt;
                } else {
                    proof_metadata = parse_proof_metadata_only(&proof_path);
                }
            } else {
                missing_proof = true;
                missing.insert(proof.clone());
            }
        }

        let mode = if ann.comparison == Comparison::Manual {
            "manual".to_string()
        } else {
            "command".to_string()
        };

        items.push(EvidenceItem {
            ac_label,
            criterion: ann.criterion,
            requirement,
            mode,
            command: ann.command,
            proof_filename,
            proof_metadata,
            excerpt_lines,
            missing_proof,
        });
    }

    let linked_proofs: Vec<String> = linked.into_iter().collect();
    let media_artifacts: Vec<String> = all_artifacts
        .iter()
        .filter(|name| is_media_artifact(name))
        .cloned()
        .collect();
    let supplementary_artifacts: Vec<String> = all_artifacts
        .iter()
        .filter(|name| !linked_proofs.contains(*name))
        .filter(|name| !is_media_artifact(name))
        .cloned()
        .collect();

    EvidenceReport {
        items,
        evidence_dir_missing,
        linked_proofs,
        supplementary_artifacts,
        media_artifacts,
        missing_proofs: missing.into_iter().collect(),
    }
}

pub fn extract_story_body(content: &str) -> Option<String> {
    let mut delimiter_count = 0;

    for (idx, line) in content.lines().enumerate() {
        if line == "---" {
            delimiter_count += 1;
            if delimiter_count == 2 {
                let lines: Vec<&str> = content.lines().collect();
                if idx + 1 < lines.len() {
                    let prefix_len: usize = lines[..=idx].iter().map(|line| line.len() + 1).sum();
                    return Some(content[prefix_len..].to_string());
                }
            }
        }
    }

    None
}

pub fn count_story_acceptance_criteria(body: &str) -> (usize, usize) {
    let mut checked = 0;
    let mut total = 0;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            checked += 1;
            total += 1;
        } else if trimmed.starts_with("- [ ]") {
            total += 1;
        }
    }

    (checked, total)
}

pub fn extract_planning_doc_summary(content: &str) -> PlanningDocSummary {
    let problem_statement = extract_section(content, "## Problem Statement")
        .and_then(|section| authored_section_text(&section));

    let goals = invariants::parse_prd_goal_entries(content)
        .into_iter()
        .map(|entry| format_goal_entry(&entry))
        .collect();

    let mut requirements = parse_requirement_entries(
        content,
        "BEGIN FUNCTIONAL_REQUIREMENTS",
        "END FUNCTIONAL_REQUIREMENTS",
        RequirementKind::Functional,
    );
    requirements.extend(parse_requirement_entries(
        content,
        "BEGIN NON_FUNCTIONAL_REQUIREMENTS",
        "END NON_FUNCTIONAL_REQUIREMENTS",
        RequirementKind::NonFunctional,
    ));
    requirements.sort_by(|a, b| a.id.cmp(&b.id));

    let mut verification_strategy = extract_section(content, "## Verification Strategy")
        .map(|section| parse_verification_strategy(&section))
        .unwrap_or_default();

    for requirement in &requirements {
        if let Some(verification) = &requirement.verification {
            verification_strategy.push(format!("{}: {}", requirement.id, verification));
        }
    }
    verification_strategy.sort();
    verification_strategy.dedup();

    PlanningDocSummary {
        problem_statement,
        goals,
        verification_strategy,
    }
}

pub fn extract_goal_from_srs(srs: &str) -> Option<String> {
    srs.lines()
        .map(str::trim)
        .find(|line| line.starts_with('>'))
        .map(|line| line.trim_start_matches('>').trim().to_string())
        .filter(|line| !line.is_empty())
}

pub fn parse_scope_summary(srs: &str) -> ScopeSummary {
    let mut summary = ScopeSummary::default();
    let Some(section) = extract_section(srs, "## Scope") else {
        return summary;
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

        let Some(item) = trimmed.strip_prefix("- ") else {
            continue;
        };
        let item = item.trim();
        if item.is_empty() || is_scaffold_text(item) {
            continue;
        }

        match mode {
            Mode::In => summary.in_scope.push(item.to_string()),
            Mode::Out => summary.out_of_scope.push(item.to_string()),
            Mode::None => {}
        }
    }

    summary
}

fn build_voyage_scope_projection(
    board: &Board,
    voyage: &Voyage,
    srs_content: &str,
) -> (Vec<ScopeLineageRow>, Vec<ScopeDriftRow>) {
    let Some(epic) = board.epics.get(&voyage.epic_id) else {
        return (Vec::new(), Vec::new());
    };
    let prd_path = epic.path.parent().unwrap().join("PRD.md");
    let prd_content = fs::read_to_string(&prd_path).unwrap_or_default();

    let parent_scope: BTreeMap<String, invariants::PrdScopeEntry> =
        invariants::parse_prd_scope_entries(&prd_content)
            .into_iter()
            .map(|entry| (entry.id.clone(), entry))
            .collect();

    let mut scope_lineage = invariants::parse_srs_scope_links(srs_content)
        .into_iter()
        .map(|link| ScopeLineageRow {
            scope_id: link.parent_id.clone(),
            voyage_description: link.description.clone(),
            voyage_disposition: link.disposition,
            epic_description: parent_scope
                .get(&link.parent_id)
                .map(|parent| parent.description.clone()),
            epic_disposition: parent_scope
                .get(&link.parent_id)
                .map(|parent| parent.disposition),
        })
        .collect::<Vec<_>>();
    scope_lineage.sort_by(|left, right| {
        left.scope_id
            .cmp(&right.scope_id)
            .then_with(|| left.voyage_disposition.cmp(&right.voyage_disposition))
            .then_with(|| left.voyage_description.cmp(&right.voyage_description))
    });

    let mut scope_drift = invariants::evaluate_voyage_scope_lineage(voyage, board)
        .into_iter()
        .map(|issue| ScopeDriftRow {
            voyage_id: None,
            issue,
        })
        .collect::<Vec<_>>();
    sort_scope_drift_rows(&mut scope_drift);

    (scope_lineage, scope_drift)
}

fn build_epic_scope_drift(board: &Board, epic: &Epic) -> Vec<ScopeDriftRow> {
    let prd_path = epic.path.parent().unwrap().join("PRD.md");
    let prd_content = fs::read_to_string(&prd_path).unwrap_or_default();
    let mut uncovered_in_scope: BTreeSet<String> =
        invariants::parse_prd_scope_entries(&prd_content)
            .into_iter()
            .filter(|entry| entry.disposition == invariants::ScopeDisposition::In)
            .map(|entry| entry.id)
            .collect();

    let mut voyages = board.voyages_for_epic_id(epic.id());
    voyages.sort_by(|left, right| {
        crate::infrastructure::utils::cmp_optional_index_then_id(
            left.index(),
            left.id(),
            right.index(),
            right.id(),
        )
    });

    let mut scope_drift = Vec::new();

    for voyage in voyages {
        let srs_path = voyage.path.parent().unwrap().join("SRS.md");
        let srs_content = fs::read_to_string(&srs_path).unwrap_or_default();
        for link in invariants::parse_srs_scope_links(&srs_content) {
            if link.disposition == invariants::ScopeDisposition::In {
                uncovered_in_scope.remove(&link.parent_id);
            }
        }
        scope_drift.extend(
            invariants::evaluate_voyage_scope_lineage(voyage, board)
                .into_iter()
                .map(|issue| ScopeDriftRow {
                    voyage_id: Some(voyage.id().to_string()),
                    issue,
                }),
        );
    }

    for scope_id in uncovered_in_scope {
        scope_drift.push(ScopeDriftRow {
            voyage_id: None,
            issue: invariants::ScopeLineageIssue {
                artifact_path: prd_path.clone(),
                scope_id: Some(scope_id),
                line: None,
                kind: invariants::ScopeLineageIssueKind::MissingScopeMapping,
            },
        });
    }
    sort_scope_drift_rows(&mut scope_drift);

    scope_drift
}

fn sort_scope_drift_rows(rows: &mut [ScopeDriftRow]) {
    rows.sort_by(|left, right| {
        left.voyage_id
            .cmp(&right.voyage_id)
            .then_with(|| left.issue.scope_id.cmp(&right.issue.scope_id))
            .then_with(|| left.issue.line.cmp(&right.issue.line))
            .then_with(|| format!("{:?}", left.issue.kind).cmp(&format!("{:?}", right.issue.kind)))
    });
}

fn parse_srs_requirement_rows(srs: &str) -> Vec<RequirementEntry> {
    let mut rows = parse_requirement_entries(
        srs,
        "BEGIN FUNCTIONAL_REQUIREMENTS",
        "END FUNCTIONAL_REQUIREMENTS",
        RequirementKind::Functional,
    );
    rows.extend(parse_requirement_entries(
        srs,
        "BEGIN NON_FUNCTIONAL_REQUIREMENTS",
        "END NON_FUNCTIONAL_REQUIREMENTS",
        RequirementKind::NonFunctional,
    ));
    rows
}

pub fn extract_section(content: &str, heading: &str) -> Option<String> {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|ch| *ch == '#').count();

    for line in content.lines() {
        if line.starts_with(heading) {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with('#') {
                let level = line.chars().take_while(|ch| *ch == '#').count();
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

pub fn authored_section_text(section: &str) -> Option<String> {
    let mut authored = Vec::new();
    let mut pending_blank_line = false;

    for line in section.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("<!--") {
            continue;
        }

        if trimmed.is_empty() {
            pending_blank_line = !authored.is_empty();
            continue;
        }

        if is_scaffold_text(trimmed) {
            continue;
        }

        if pending_blank_line {
            authored.push(String::new());
            pending_blank_line = false;
        }

        authored.push(trimmed.to_string());
    }

    if authored.is_empty() {
        None
    } else {
        Some(authored.join("\n"))
    }
}

pub fn parse_goals(content: &str) -> Vec<String> {
    invariants::parse_prd_goal_entries(content)
        .into_iter()
        .map(|entry| format_goal_entry(&entry))
        .collect()
}

fn parse_verification_strategy(section: &str) -> Vec<String> {
    let mut strategy = Vec::new();

    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("<!--") {
            continue;
        }

        if trimmed.starts_with('|') {
            let cols = split_markdown_table_row(trimmed);
            if cols.len() >= 2
                && !cols[0].eq_ignore_ascii_case("Requirement")
                && !cols[0].starts_with("---")
                && !is_scaffold_text(&cols[0])
                && !is_scaffold_text(&cols[1])
            {
                strategy.push(format!("{}: {}", cols[0], cols[1]));
            }
            continue;
        }

        if let Some(item) = trimmed.strip_prefix("- ") {
            if !item.is_empty() && !is_scaffold_text(item) {
                strategy.push(item.to_string());
            }
            continue;
        }

        if !is_scaffold_text(trimmed) {
            strategy.push(trimmed.to_string());
        }
    }

    strategy
}

fn format_goal_entry(entry: &invariants::GoalEntry) -> String {
    format!(
        "{}: {} ({} -> {})",
        entry.id, entry.goal, entry.success_metric, entry.target
    )
}

fn split_markdown_table_row(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);

    trimmed
        .split('|')
        .map(|col| col.trim().to_string())
        .collect()
}

fn parse_requirement_entries(
    content: &str,
    start_marker: &str,
    end_marker: &str,
    kind: RequirementKind,
) -> Vec<RequirementEntry> {
    let mut entries = Vec::new();
    let mut in_block = false;
    let mut verification_column_index: Option<usize> = None;

    for line in content.lines() {
        if line.contains(start_marker) {
            in_block = true;
            verification_column_index = None;
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

        let cols = split_markdown_table_row(trimmed);
        if cols.len() < 2 {
            continue;
        }

        let id = cols[0].as_str();
        let requirement = cols[1].as_str();

        if id.eq_ignore_ascii_case("ID") {
            verification_column_index = cols
                .iter()
                .position(|col| col.eq_ignore_ascii_case("Verification"));
            continue;
        }

        if id.starts_with("---")
            || requirement.eq_ignore_ascii_case("TODO")
            || is_scaffold_text(id)
            || is_scaffold_text(requirement)
        {
            continue;
        }

        let verification = verification_column_index
            .and_then(|idx| cols.get(idx))
            .map(|value| value.trim())
            .filter(|value| !value.is_empty() && !is_scaffold_text(value))
            .map(ToOwned::to_owned);

        entries.push(RequirementEntry {
            id: id.to_string(),
            description: requirement.to_string(),
            kind,
            verification,
        });
    }

    entries
}

fn average_story_throughput_per_week(board: &Board, weeks: usize) -> f64 {
    if weeks == 0 {
        return 0.0;
    }

    let today = Local::now().date_naive();
    let current_week_start = start_of_week_monday(today);
    let mut total = 0usize;
    let mut week_start = current_week_start;

    for _ in 0..weeks {
        let week_end = week_start + Duration::days(7);
        total += board
            .stories
            .values()
            .filter(|story| {
                story
                    .frontmatter
                    .completed_at
                    .map(|dt| {
                        let date = dt.date();
                        date >= week_start && date < week_end
                    })
                    .unwrap_or(false)
            })
            .count();
        week_start -= Duration::days(7);
    }

    let denominator = effective_story_weeks(board, weeks, current_week_start);
    if denominator == 0 {
        return 0.0;
    }

    total as f64 / denominator as f64
}

fn effective_story_weeks(
    board: &Board,
    requested_weeks: usize,
    current_week_start: NaiveDate,
) -> usize {
    if requested_weeks == 0 {
        return 0;
    }

    let oldest_completion_week = board
        .stories
        .values()
        .filter_map(|story| {
            story
                .frontmatter
                .completed_at
                .map(|dt| start_of_week_monday(dt.date()))
        })
        .min();

    match oldest_completion_week {
        Some(oldest_week) => {
            let elapsed_weeks = ((current_week_start - oldest_week).num_days() / 7) + 1;
            (elapsed_weeks.max(1) as usize).min(requested_weeks)
        }
        None => requested_weeks,
    }
}

fn start_of_week_monday(date: NaiveDate) -> NaiveDate {
    date - Duration::days(date.weekday().num_days_from_monday() as i64)
}

fn is_scaffold_text(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("todo")
        || lower.contains("what user problem")
        || lower.contains("primary user workflow")
        || lower.contains("operational reliability")
        || lower.contains("add goals")
        || lower.contains("placeholder")
        || lower.contains("example")
}

fn story_evidence_ordering(a: &StoryEvidence, b: &StoryEvidence) -> Ordering {
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

fn requirement_completion_state(linked: &[StoryRef]) -> RequirementCompletion {
    if linked.is_empty() {
        RequirementCompletion::Unmapped
    } else if linked.iter().all(|story| story.stage == StoryState::Done) {
        RequirementCompletion::Done
    } else if linked.iter().any(|story| {
        story.stage == StoryState::InProgress || story.stage == StoryState::NeedsHumanVerification
    }) {
        RequirementCompletion::InProgress
    } else {
        RequirementCompletion::Queued
    }
}

fn requirement_verification_label(automated: usize, manual: usize) -> String {
    match (automated, manual) {
        (0, 0) => "none".to_string(),
        (count, 0) => format!("automated ({count})"),
        (0, count) => format!("manual ({count})"),
        (automated, manual) => format!("mixed (a:{automated}/m:{manual})"),
    }
}

fn parse_proof_metadata_and_excerpt(path: &Path) -> (ProofMetadata, Vec<String>) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return (ProofMetadata::default(), Vec::new()),
    };

    let (metadata, body) = split_frontmatter(&content);
    let excerpt_lines = body.lines().take(10).map(ToOwned::to_owned).collect();
    (metadata, excerpt_lines)
}

fn parse_proof_metadata_only(path: &Path) -> ProofMetadata {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return ProofMetadata::default(),
    };

    let (metadata, _) = split_frontmatter(&content);
    metadata
}

fn split_frontmatter(content: &str) -> (ProofMetadata, String) {
    let mut metadata = ProofMetadata::default();
    if !content.starts_with("---\n") {
        return (metadata, content.to_string());
    }

    let mut lines = content.lines();
    let _ = lines.next();

    let mut header = Vec::new();
    for line in lines.by_ref() {
        if line.trim() == "---" {
            break;
        }
        header.push(line.to_string());
    }

    for line in header {
        let mut parts = line.splitn(2, ':');
        let Some(key) = parts.next() else {
            continue;
        };
        let Some(value) = parts.next() else {
            continue;
        };
        let value = value.trim().to_string();
        match key.trim() {
            "recorded_at" => metadata.recorded_at = Some(value),
            "mode" => metadata.mode = Some(value),
            "command" => metadata.command = Some(value),
            _ => {}
        }
    }

    let body = lines.collect::<Vec<_>>().join("\n");
    (metadata, body)
}

pub fn is_text_artifact(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".log", ".txt", ".md", ".json", ".yaml", ".yml", ".toml"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

pub fn is_media_artifact(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".gif", ".png", ".jpg", ".jpeg", ".webm", ".mp4", ".mov"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::state_machine::invariants::{ScopeDisposition, ScopeLineageIssueKind};
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use tempfile::TempDir;

    fn load_scope_reports(temp: &TempDir) -> (EpicShowProjection, VoyageShowProjection) {
        let board = load_board(temp.path()).unwrap();
        let epic = build_epic_show_projection(&board, board.require_epic("e1").unwrap()).unwrap();
        let voyage =
            build_voyage_show_projection(&board, board.require_voyage("v1").unwrap()).unwrap();
        (epic, voyage)
    }

    #[test]
    fn planning_show_projection_contract() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
> Improve show output quality.

## Scope
In scope:
- Goal and scope summaries.

Out of scope:
- Lifecycle changes.

## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Render a requirement matrix. | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(TestStory::new("S1").scope("e1/v1").body(
                r#"## Acceptance Criteria
- [x] [SRS-01/AC-01] matrix shown <!-- verify: cargo test --lib planning_show_projection_contract, SRS-01:start, proof: ac-1.log -->
"#,
            ))
            .build();

        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Problem Statement
Operators cannot quickly evaluate planning state.

They also need the PRD summary to preserve authored narrative context.

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Improve readability | acceptance speed | 2x |

## Verification Strategy
- Use automated command proofs.

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| FR-01 | Surface planning summaries. | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        std::fs::write(temp.path().join("stories/S1/EVIDENCE/ac-1.log"), "ok").unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let story = board.require_story("S1").unwrap();

        let epic_projection = build_epic_show_projection(&board, epic).unwrap();
        let voyage_projection = build_voyage_show_projection(&board, voyage).unwrap();
        let story_projection = build_story_show_projection(story).unwrap();

        assert_eq!(
            epic_projection.doc.problem_statement.as_deref(),
            Some(
                "Operators cannot quickly evaluate planning state.\n\nThey also need the PRD summary to preserve authored narrative context."
            )
        );
        assert!(
            epic_projection
                .requirement_coverage
                .iter()
                .any(|row| row.id == "FR-01" && row.description == "Surface planning summaries.")
        );

        assert_eq!(
            voyage_projection.goal.as_deref(),
            Some("Improve show output quality.")
        );
        assert_eq!(voyage_projection.requirements.len(), 1);
        assert_eq!(voyage_projection.requirements[0].id, "SRS-01");

        assert_eq!(story_projection.total_criteria, 1);
        assert_eq!(story_projection.evidence.items.len(), 1);
    }

    #[test]
    fn planning_doc_extractor() {
        let doc = r#"# Planning Doc

## Problem Statement
<!-- What user problem does this solve? -->
Planners cannot quickly inspect requirement readiness.

Teams also need to see missing lineage before tactical work starts.

## Goals & Objectives
<!-- TODO: Add goals -->
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-02 | Improve signal quality | operator confidence | 90% |
| GOAL-01 | Reduce planning ambiguity | epic review success | 95% |

## Verification Strategy
<!-- TODO: Add strategy -->
- capture command proof logs
- include artifact inventory with media links

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| FR-01 | Render planning summary. | cargo test |
| FR-02 | TODO | TODO |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-NFR-01 | Deterministic ordering. | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#;

        let summary = extract_planning_doc_summary(doc);

        assert_eq!(
            summary.problem_statement.as_deref(),
            Some(
                "Planners cannot quickly inspect requirement readiness.\n\nTeams also need to see missing lineage before tactical work starts."
            )
        );
        assert_eq!(summary.goals.len(), 2);
        assert!(summary.goals[0].contains("GOAL-01"));
        assert!(summary.goals[1].contains("GOAL-02"));
        assert!(
            summary
                .verification_strategy
                .iter()
                .any(|entry| entry.contains("capture command proof logs"))
        );
        assert!(
            summary
                .verification_strategy
                .iter()
                .any(|entry| entry.contains("FR-01: cargo test"))
        );
        assert!(
            !summary
                .verification_strategy
                .iter()
                .any(|entry| entry.to_ascii_lowercase().contains("todo"))
        );
    }

    #[test]
    fn planning_doc_extractor_does_not_treat_priority_as_verification() {
        let doc = r#"# PRD

## Problem Statement
Operators need reliable planning summaries.

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Improve planning readability. | review speed | 2x |

## Verification Strategy
- Validate with command-level and doctor checks.

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Render canonical requirement summaries. | must | Keeps planning output actionable. |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let summary = extract_planning_doc_summary(doc);

        assert!(
            summary
                .verification_strategy
                .iter()
                .any(|entry| entry.contains("Validate with command-level and doctor checks."))
        );
        assert!(
            !summary
                .verification_strategy
                .iter()
                .any(|entry| entry.contains("FR-01: must"))
        );
    }

    #[test]
    fn goal_lineage_parser_reads_canonical_goal_rows() {
        let prd = r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-02 | Reduce planning ambiguity | approval confidence | 90% |
| GOAL-01 | Improve strategic traceability | linked requirements | 100% |
"#;

        assert_eq!(
            invariants::parse_prd_goal_entries(prd),
            vec![
                invariants::GoalEntry {
                    id: "GOAL-01".to_string(),
                    goal: "Improve strategic traceability".to_string(),
                    success_metric: "linked requirements".to_string(),
                    target: "100%".to_string(),
                },
                invariants::GoalEntry {
                    id: "GOAL-02".to_string(),
                    goal: "Reduce planning ambiguity".to_string(),
                    success_metric: "approval confidence".to_string(),
                    target: "90%".to_string(),
                },
            ]
        );
    }

    #[test]
    fn goal_lineage_parser_is_deterministic() {
        let prd_a = r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-02 | Reduce planning ambiguity | approval confidence | 90% |
| GOAL-01 | Improve strategic traceability | linked requirements | 100% |
"#;

        let prd_b = r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Improve strategic traceability | linked requirements | 100% |
| GOAL-02 | Reduce planning ambiguity | approval confidence | 90% |
"#;

        assert_eq!(
            invariants::parse_prd_goal_entries(prd_a),
            invariants::parse_prd_goal_entries(prd_b)
        );
    }

    #[test]
    fn planning_show_projection_deterministic() {
        let board_a = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
> Deterministic projections.

## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-10 | Ten. | manual |
| SRS-01 | One. | command |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(TestStory::new("S2").scope("e1/v1").index(2).body(
                r#"## Acceptance Criteria
- [ ] [SRS-01/AC-01] queued <!-- verify: manual, SRS-01:start:end, proof: b.log -->
"#,
            ))
            .story(TestStory::new("S1").scope("e1/v1").index(1).body(
                r#"## Acceptance Criteria
- [x] [SRS-01/AC-02] done A <!-- verify: cargo test, SRS-01:start, proof: z.log -->
- [x] [SRS-01/AC-03] done B <!-- verify: cargo test, SRS-01:end, proof: a.log -->
"#,
            ))
            .build();

        let board_b = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
> Deterministic projections.

## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-10 | Ten. | manual |
| SRS-01 | One. | command |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(TestStory::new("S1").scope("e1/v1").index(1).body(
                r#"## Acceptance Criteria
- [x] [SRS-01/AC-02] done A <!-- verify: cargo test, SRS-01:start, proof: z.log -->
- [x] [SRS-01/AC-03] done B <!-- verify: cargo test, SRS-01:end, proof: a.log -->
"#,
            ))
            .story(TestStory::new("S2").scope("e1/v1").index(2).body(
                r#"## Acceptance Criteria
- [ ] [SRS-01/AC-01] queued <!-- verify: manual, SRS-01:start:end, proof: b.log -->
"#,
            ))
            .build();

        std::fs::write(
            board_a.path().join("epics/e1/PRD.md"),
            "# PRD\n\n## Problem Statement\nA\n\n## Goals & Objectives\n| ID | Goal | Success Metric | Target |\n|----|------|----------------|--------|\n| GOAL-01 | G1 | metric | target |\n",
        )
        .unwrap();
        std::fs::write(
            board_b.path().join("epics/e1/PRD.md"),
            "# PRD\n\n## Problem Statement\nA\n\n## Goals & Objectives\n| ID | Goal | Success Metric | Target |\n|----|------|----------------|--------|\n| GOAL-01 | G1 | metric | target |\n",
        )
        .unwrap();

        std::fs::write(board_a.path().join("stories/S1/EVIDENCE/a.log"), "a").unwrap();
        std::fs::write(board_a.path().join("stories/S1/EVIDENCE/z.log"), "z").unwrap();
        std::fs::write(
            board_a.path().join("stories/S1/EVIDENCE/notes.txt"),
            "notes",
        )
        .unwrap();
        std::fs::write(board_a.path().join("stories/S1/EVIDENCE/clip.mp4"), "vid").unwrap();
        std::fs::write(board_a.path().join("stories/S2/EVIDENCE/b.log"), "b").unwrap();

        std::fs::write(board_b.path().join("stories/S1/EVIDENCE/z.log"), "z").unwrap();
        std::fs::write(board_b.path().join("stories/S1/EVIDENCE/a.log"), "a").unwrap();
        std::fs::write(board_b.path().join("stories/S1/EVIDENCE/clip.mp4"), "vid").unwrap();
        std::fs::write(
            board_b.path().join("stories/S1/EVIDENCE/notes.txt"),
            "notes",
        )
        .unwrap();
        std::fs::write(board_b.path().join("stories/S2/EVIDENCE/b.log"), "b").unwrap();

        let loaded_a = load_board(board_a.path()).unwrap();
        let loaded_b = load_board(board_b.path()).unwrap();

        let epic_a =
            build_epic_show_projection(&loaded_a, loaded_a.require_epic("e1").unwrap()).unwrap();
        let epic_b =
            build_epic_show_projection(&loaded_b, loaded_b.require_epic("e1").unwrap()).unwrap();

        let voyage_a =
            build_voyage_show_projection(&loaded_a, loaded_a.require_voyage("v1").unwrap())
                .unwrap();
        let voyage_b =
            build_voyage_show_projection(&loaded_b, loaded_b.require_voyage("v1").unwrap())
                .unwrap();

        let story_a = build_story_show_projection(loaded_a.require_story("S1").unwrap()).unwrap();
        let story_b = build_story_show_projection(loaded_b.require_story("S1").unwrap()).unwrap();

        let req_ids_a: Vec<String> = voyage_a
            .requirements
            .iter()
            .map(|row| row.id.clone())
            .collect();
        assert_eq!(req_ids_a, vec!["SRS-01", "SRS-10"]);
        assert_eq!(voyage_a.requirements, voyage_b.requirements);

        let linked_story_ids: Vec<String> = voyage_a.requirements[0]
            .linked_stories
            .iter()
            .map(|story| story.id.clone())
            .collect();
        assert_eq!(linked_story_ids, vec!["S1", "S2"]);

        let epic_linked_a: Vec<String> = epic_a
            .verification
            .linked_artifacts
            .iter()
            .cloned()
            .collect();
        let epic_linked_b: Vec<String> = epic_b
            .verification
            .linked_artifacts
            .iter()
            .cloned()
            .collect();
        assert_eq!(epic_linked_a, epic_linked_b);

        assert_eq!(story_a.evidence.linked_proofs, vec!["a.log", "z.log"]);
        assert_eq!(
            story_a.evidence.linked_proofs,
            story_b.evidence.linked_proofs
        );
        assert_eq!(story_a.evidence.supplementary_artifacts, vec!["notes.txt"]);
        assert_eq!(story_a.evidence.media_artifacts, vec!["clip.mp4"]);

        // Stable section ordering/contents from authored planning docs.
        assert_eq!(epic_a.doc.problem_statement, epic_b.doc.problem_statement);
        assert_eq!(epic_a.doc.goals, epic_b.doc.goals);
    }

    #[test]
    fn epic_requirement_goal_lineage_preserves_one_to_many_fanout() {
        let temp = TestBoardBuilder::new().epic(TestEpic::new("e1")).build();
        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-02 | Reduce manual review | reviewer minutes | -50% |
| GOAL-01 | Improve strategic traceability | linked requirements | 100% |

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-02 | Expose uncovered goals. | GOAL-01 | should | reviewability |
| FR-01 | Render coverage. | GOAL-02 GOAL-01 | must | visibility |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = build_epic_show_projection(&board, board.require_epic("e1").unwrap()).unwrap();

        let fr01 = epic
            .requirement_coverage
            .iter()
            .find(|row| row.id == "FR-01")
            .unwrap();
        assert_eq!(fr01.linked_goals, vec!["GOAL-01", "GOAL-02"]);

        let fr02 = epic
            .requirement_coverage
            .iter()
            .find(|row| row.id == "FR-02")
            .unwrap();
        assert_eq!(fr02.linked_goals, vec!["GOAL-01"]);
    }

    #[test]
    fn epic_requirement_goal_lineage_projection_is_deterministic() {
        let board_a = TestBoardBuilder::new().epic(TestEpic::new("e1")).build();
        let board_b = TestBoardBuilder::new().epic(TestEpic::new("e1")).build();

        let prd_a = r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-02 | Reduce manual review | reviewer minutes | -50% |
| GOAL-01 | Improve strategic traceability | linked requirements | 100% |

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-02 | Expose uncovered goals. | GOAL-01 | should | reviewability |
| FR-01 | Render coverage. | GOAL-02 GOAL-01 | must | visibility |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        let prd_b = r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Improve strategic traceability | linked requirements | 100% |
| GOAL-02 | Reduce manual review | reviewer minutes | -50% |

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Render coverage. | GOAL-01 GOAL-02 | must | visibility |
| FR-02 | Expose uncovered goals. | GOAL-01 | should | reviewability |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        std::fs::write(board_a.path().join("epics/e1/PRD.md"), prd_a).unwrap();
        std::fs::write(board_b.path().join("epics/e1/PRD.md"), prd_b).unwrap();

        let loaded_a = load_board(board_a.path()).unwrap();
        let loaded_b = load_board(board_b.path()).unwrap();
        let epic_a =
            build_epic_show_projection(&loaded_a, loaded_a.require_epic("e1").unwrap()).unwrap();
        let epic_b =
            build_epic_show_projection(&loaded_b, loaded_b.require_epic("e1").unwrap()).unwrap();

        assert_eq!(epic_a.doc.goals, epic_b.doc.goals);
        assert_eq!(epic_a.requirement_coverage, epic_b.requirement_coverage);
    }

    #[test]
    fn epic_show_aggregates_prd_requirement_coverage_across_voyages() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").index(2).srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-02 | Second child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .voyage(TestVoyage::new("v2", "e1").index(1).srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Quality child. | NFR-01 | cargo test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .build();

        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Shared parent. | GOAL-02 GOAL-01 | must | fanout |
| FR-02 | Uncovered parent. | GOAL-01 | should | uncovered |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Quality parent. | GOAL-03 | must | coverage |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = build_epic_show_projection(&board, board.require_epic("e1").unwrap()).unwrap();

        let coverage_ids: Vec<_> = epic
            .requirement_coverage
            .iter()
            .map(|row| row.id.clone())
            .collect();
        assert_eq!(coverage_ids, vec!["FR-01", "FR-02", "NFR-01"]);

        let fr01 = epic
            .requirement_coverage
            .iter()
            .find(|row| row.id == "FR-01")
            .unwrap();
        assert_eq!(fr01.linked_goals, vec!["GOAL-01", "GOAL-02"]);
        assert_eq!(fr01.linked_child_count(), 2);
        let fr01_children: Vec<_> = fr01
            .linked_children
            .iter()
            .map(|child| format!("{}/{}", child.voyage_id, child.requirement_id))
            .collect();
        assert_eq!(fr01_children, vec!["v2/SRS-01", "v1/SRS-02"]);

        let fr02 = epic
            .requirement_coverage
            .iter()
            .find(|row| row.id == "FR-02")
            .unwrap();
        assert_eq!(fr02.linked_goals, vec!["GOAL-01"]);
        assert!(!fr02.is_covered());
        assert_eq!(fr02.linked_child_count(), 0);

        let nfr01 = epic
            .requirement_coverage
            .iter()
            .find(|row| row.id == "NFR-01")
            .unwrap();
        assert_eq!(nfr01.linked_goals, vec!["GOAL-03"]);
        assert_eq!(nfr01.linked_child_count(), 1);
        assert_eq!(nfr01.linked_children[0].voyage_id, "v2");
        assert_eq!(nfr01.linked_children[0].requirement_id, "SRS-NFR-01");
    }

    #[test]
    fn epic_prd_coverage_projection_is_deterministic() {
        let board_a = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v2", "e1").index(2).srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-02 | Second child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .voyage(TestVoyage::new("v1", "e1").index(1).srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .build();
        let board_b = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").index(1).srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .voyage(TestVoyage::new("v2", "e1").index(2).srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-02 | Second child. | FR-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .build();

        let prd = r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-02 | Uncovered parent. | should | uncovered |
| FR-01 | Shared parent. | must | fanout |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        std::fs::write(board_a.path().join("epics/e1/PRD.md"), prd).unwrap();
        std::fs::write(board_b.path().join("epics/e1/PRD.md"), prd).unwrap();

        let loaded_a = load_board(board_a.path()).unwrap();
        let loaded_b = load_board(board_b.path()).unwrap();
        let epic_a =
            build_epic_show_projection(&loaded_a, loaded_a.require_epic("e1").unwrap()).unwrap();
        let epic_b =
            build_epic_show_projection(&loaded_b, loaded_b.require_epic("e1").unwrap()).unwrap();

        assert_eq!(epic_a.requirement_coverage, epic_b.requirement_coverage);
        let coverage_ids: Vec<_> = epic_a
            .requirement_coverage
            .iter()
            .map(|row| row.id.clone())
            .collect();
        assert_eq!(coverage_ids, vec!["FR-01", "FR-02"]);
        let linked_children: Vec<_> = epic_a.requirement_coverage[0]
            .linked_children
            .iter()
            .map(|child| format!("{}/{}", child.voyage_id, child.requirement_id))
            .collect();
        assert_eq!(linked_children, vec!["v1/SRS-01", "v2/SRS-02"]);
    }

    #[test]
    fn planning_show_renders_scope_lineage_and_drift() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").index(1).srs_content(
                r#"# SRS
> Surface scope lineage for review.

## Scope
In scope:
- [SCOPE-03] Pull a forbidden item into this voyage.
- [SCOPE-99] Reference a missing parent scope item.

Out of scope:
- [SCOPE-02] Defer a valid in-scope item for a later slice.
"#,
            ))
            .build();

        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Scope

### In Scope
- [SCOPE-01] Parse canonical scope IDs.
- [SCOPE-02] Render scope drift findings.

### Out of Scope
- [SCOPE-03] Story-level runtime enforcement.
"#,
        )
        .unwrap();

        let (epic, voyage) = load_scope_reports(&temp);

        let deferred = voyage
            .scope_lineage
            .iter()
            .find(|row| row.scope_id == "SCOPE-02")
            .unwrap();
        assert_eq!(deferred.voyage_disposition, ScopeDisposition::Out);
        assert_eq!(deferred.epic_disposition, Some(ScopeDisposition::In));

        let contradiction = voyage
            .scope_lineage
            .iter()
            .find(|row| row.scope_id == "SCOPE-03")
            .unwrap();
        assert_eq!(contradiction.voyage_disposition, ScopeDisposition::In);
        assert_eq!(contradiction.epic_disposition, Some(ScopeDisposition::Out));

        let unknown = voyage
            .scope_drift
            .iter()
            .find(|row| row.issue.scope_id.as_deref() == Some("SCOPE-99"))
            .unwrap();
        assert_eq!(unknown.issue.kind, ScopeLineageIssueKind::UnknownScopeRef);

        let epic_missing = epic
            .scope_drift
            .iter()
            .find(|row| {
                row.issue.scope_id.as_deref() == Some("SCOPE-01") && row.voyage_id.is_none()
            })
            .unwrap();
        assert_eq!(
            epic_missing.issue.kind,
            ScopeLineageIssueKind::MissingScopeMapping
        );

        let epic_unknown = epic
            .scope_drift
            .iter()
            .find(|row| row.issue.scope_id.as_deref() == Some("SCOPE-99"))
            .unwrap();
        assert_eq!(epic_unknown.voyage_id.as_deref(), Some("v1"));
    }

    #[test]
    fn planning_show_scope_drift_output_is_reviewable() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").index(1).srs_content(
                r#"# SRS
> Review scope drift without opening multiple planning docs.

## Scope
In scope:
- [SCOPE-03] Pull runtime enforcement into this voyage.

Out of scope:
- [SCOPE-02] Defer drift rendering until later.
"#,
            ))
            .build();

        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Scope

### In Scope
- [SCOPE-02] Render scope drift findings.

### Out of Scope
- [SCOPE-03] Story-level runtime enforcement.
"#,
        )
        .unwrap();

        let (epic, voyage) = load_scope_reports(&temp);

        let deferred = voyage
            .scope_lineage
            .iter()
            .find(|row| row.scope_id == "SCOPE-02")
            .unwrap();
        assert_eq!(
            deferred.voyage_description,
            "Defer drift rendering until later."
        );
        assert_eq!(deferred.voyage_disposition, ScopeDisposition::Out);
        assert_eq!(
            deferred.epic_description.as_deref(),
            Some("Render scope drift findings.")
        );
        assert_eq!(deferred.epic_disposition, Some(ScopeDisposition::In));

        let contradiction = voyage
            .scope_lineage
            .iter()
            .find(|row| row.scope_id == "SCOPE-03")
            .unwrap();
        assert_eq!(
            contradiction.voyage_description,
            "Pull runtime enforcement into this voyage."
        );
        assert_eq!(contradiction.voyage_disposition, ScopeDisposition::In);
        assert_eq!(
            contradiction.epic_description.as_deref(),
            Some("Story-level runtime enforcement.")
        );
        assert_eq!(contradiction.epic_disposition, Some(ScopeDisposition::Out));

        let epic_contradiction = epic
            .scope_drift
            .iter()
            .find(|row| row.issue.scope_id.as_deref() == Some("SCOPE-03"))
            .unwrap();
        assert_eq!(epic_contradiction.voyage_id.as_deref(), Some("v1"));
        assert_eq!(
            epic_contradiction.issue.kind,
            ScopeLineageIssueKind::OutOfScopeContradiction
        );

        let epic_missing = epic
            .scope_drift
            .iter()
            .find(|row| {
                row.issue.scope_id.as_deref() == Some("SCOPE-02") && row.voyage_id.is_none()
            })
            .unwrap();
        assert_eq!(
            epic_missing.issue.kind,
            ScopeLineageIssueKind::MissingScopeMapping
        );
    }

    #[test]
    fn planning_show_preserves_scope_text_with_ids() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
> Preserve authored scope prose.

## Scope
In scope:
- [SCOPE-01] Render scope lineage inside the voyage summary.

Out of scope:
- [SCOPE-02] Leave contradiction repair for a later slice.
"#,
            ))
            .build();

        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Scope

### In Scope
- [SCOPE-01] Render scope lineage inside the voyage summary.

### Out of Scope
- [SCOPE-02] Leave contradiction repair for a later slice.
"#,
        )
        .unwrap();

        let (_epic, voyage) = load_scope_reports(&temp);

        assert_eq!(
            voyage.scope.in_scope,
            vec!["[SCOPE-01] Render scope lineage inside the voyage summary.".to_string()]
        );
        assert_eq!(
            voyage.scope.out_of_scope,
            vec!["[SCOPE-02] Leave contradiction repair for a later slice.".to_string()]
        );
        let in_scope_lineage = voyage
            .scope_lineage
            .iter()
            .find(|row| row.scope_id == "SCOPE-01")
            .unwrap();
        assert_eq!(
            in_scope_lineage.voyage_description,
            "Render scope lineage inside the voyage summary."
        );

        let out_of_scope_lineage = voyage
            .scope_lineage
            .iter()
            .find(|row| row.scope_id == "SCOPE-02")
            .unwrap();
        assert_eq!(
            out_of_scope_lineage.voyage_description,
            "Leave contradiction repair for a later slice."
        );
    }

    #[test]
    fn planning_show_omits_verification_recommendations() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1"))
            .story(TestStory::new("S1").scope("e1/v1").body(
                r#"## Acceptance Criteria
- [x] [SRS-01/AC-01] has verification <!-- verify: cargo test -p keel planning_show_omits_verification_recommendations, SRS-01:start:end -->
"#,
            ))
            .build();

        let board = load_board(temp.path()).unwrap();
        let epic = build_epic_show_projection(&board, board.require_epic("e1").unwrap()).unwrap();
        let voyage =
            build_voyage_show_projection(&board, board.require_voyage("v1").unwrap()).unwrap();
        let story = build_story_show_projection(board.require_story("S1").unwrap()).unwrap();

        let EpicShowProjection {
            doc,
            scope_drift,
            requirement_coverage,
            total_voyages,
            done_voyages,
            total_stories,
            done_stories,
            started_at,
            completed_at,
            updated_at,
            eta,
            verification,
        } = epic;
        let _ = (
            doc,
            scope_drift,
            requirement_coverage,
            total_voyages,
            done_voyages,
            total_stories,
            done_stories,
            started_at,
            completed_at,
            updated_at,
            eta,
            verification,
        );

        let VoyageShowProjection {
            goal,
            scope,
            scope_lineage,
            scope_drift,
            requirements,
            done_stories,
            total_stories,
            done_functional_requirements,
            total_functional_requirements,
            done_non_functional_requirements,
            total_non_functional_requirements,
            done_requirements,
            total_requirements,
        } = voyage;
        let _ = (
            goal,
            scope,
            scope_lineage,
            scope_drift,
            requirements,
            done_stories,
            total_stories,
            done_functional_requirements,
            total_functional_requirements,
            done_non_functional_requirements,
            total_non_functional_requirements,
            done_requirements,
            total_requirements,
        );

        let StoryShowProjection {
            body,
            checked_criteria,
            total_criteria,
            evidence,
        } = story;
        let _ = (body, checked_criteria, total_criteria, evidence);
    }

    #[test]
    fn planning_show_preserves_existing_sections() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").srs_content(
                r#"# SRS
> Preserve planning section content.

## Scope
In scope:
- Render goal/scope/progress.

Out of scope:
- Lifecycle transition changes.

## Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Keep voyage requirement matrix. | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(TestStory::new("S1").scope("e1/v1").stage(StoryState::Done).body(
                r#"## Acceptance Criteria
- [x] [SRS-01/AC-01] preserve evidence section <!-- verify: cargo test -p keel planning_show_preserves_existing_sections, SRS-01:start:end, proof: ac-1.log -->
"#,
            ))
            .build();

        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Problem Statement
Planning readers need concise summaries without recommendation noise.

## Goals & Objectives
- Keep planning summary and progress sections.

## Verification Strategy
- Keep automated and manual verification rollups.

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| FR-01 | Preserve planning show sections. | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();
        std::fs::write(temp.path().join("stories/S1/EVIDENCE/ac-1.log"), "proof").unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = build_epic_show_projection(&board, board.require_epic("e1").unwrap()).unwrap();
        let voyage =
            build_voyage_show_projection(&board, board.require_voyage("v1").unwrap()).unwrap();
        let story = build_story_show_projection(board.require_story("S1").unwrap()).unwrap();

        assert_eq!(
            epic.doc.problem_statement.as_deref(),
            Some("Planning readers need concise summaries without recommendation noise.")
        );
        assert_eq!(epic.total_voyages, 1);
        assert_eq!(epic.total_stories, 1);
        assert_eq!(epic.done_stories, 1);
        assert_eq!(epic.verification.automated_criteria, 1);

        assert_eq!(
            voyage.goal.as_deref(),
            Some("Preserve planning section content.")
        );
        assert_eq!(voyage.total_stories, 1);
        assert_eq!(voyage.done_stories, 1);
        assert_eq!(voyage.total_requirements, 1);
        assert_eq!(voyage.done_requirements, 1);

        assert_eq!(story.total_criteria, 1);
        assert_eq!(story.checked_criteria, 1);
        assert_eq!(story.evidence.items.len(), 1);
        assert_eq!(story.evidence.linked_proofs, vec!["ac-1.log".to_string()]);
    }
}
