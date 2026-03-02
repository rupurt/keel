#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use super::super::AC_REQ_RE;
use super::super::types::*;
use crate::domain::model::{Board, StoryState};
use crate::infrastructure::validation::parse_acceptance_criteria;
use crate::infrastructure::validation::structural;

/// Scan story files for structural problems
/// Returns (problems, file_count)
pub fn scan_story_files(board_dir: &Path) -> anyhow::Result<(Vec<Problem>, usize)> {
    structural::scan_story_files(board_dir)
}

/// Extract story ID from file by parsing frontmatter
pub fn extract_story_id_from_file(path: &Path) -> Option<String> {
    structural::extract_story_id_from_file(path)
}

/// Check a single story file for problems
pub fn check_story_file(path: &Path) -> Option<Problem> {
    structural::check_story_file(path)
}

/// Check that filename matches frontmatter (ID and type)
pub fn check_filename_frontmatter_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        let Some(bundle_name) = story
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        else {
            continue;
        };

        // Check ID consistency: frontmatter id must match bundle directory name
        let frontmatter_id = story.id();
        if frontmatter_id != bundle_name {
            let old_path = story.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(
                Problem::warning(
                    story.path.clone(),
                    format!(
                        "frontmatter id '{}' doesn't match bundle directory name '{}'",
                        frontmatter_id, bundle_name
                    ),
                )
                .with_check_id(CheckId::StoryFilenameInconsistent)
                .with_scope(story.frontmatter.scope.clone().unwrap_or_default())
                .with_fix(Fix::RenameFile { old_path, new_path }),
            );
        }
    }

    problems
}

/// Check for deprecated frontmatter fields in stories
pub fn check_deprecated_frontmatter_fields(board_dir: &Path) -> Vec<Problem> {
    // This could also be moved to structural later if needed
    let mut problems = Vec::new();

    let stories_dir = board_dir.join("stories");
    if !stories_dir.exists() {
        return problems;
    }

    for entry in fs::read_dir(&stories_dir).into_iter().flatten().flatten() {
        if entry.path().is_dir() {
            let readme_path = entry.path().join("README.md");
            if readme_path.exists()
                && let Some(problem) = check_story_deprecated_fields(&readme_path)
            {
                problems.push(problem);
            }
        }
    }

    problems
}

/// Check a single story file for deprecated frontmatter fields
pub fn check_story_deprecated_fields(path: &Path) -> Option<Problem> {
    use super::super::fixes::is_date_only_format;
    let content = fs::read_to_string(path).ok()?;

    // Extract just the frontmatter section
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() || lines[0] != "---" {
        return None;
    }

    let end_idx = lines.iter().skip(1).position(|line| *line == "---")?;
    let frontmatter_lines = &lines[1..=end_idx];

    let mut deprecated_fields = Vec::new();

    for line in frontmatter_lines {
        let trimmed = line.trim();

        if trimmed.starts_with("priority:") {
            deprecated_fields.push("priority");
        }
        if trimmed.starts_with("depends:") {
            deprecated_fields.push("depends");
        }
        if trimmed.starts_with("created:") && !trimmed.starts_with("created_at:") {
            deprecated_fields.push("created (should be created_at)");
        }
        if trimmed.starts_with("updated:") && !trimmed.starts_with("updated_at:") {
            deprecated_fields.push("updated (unsupported, must be updated_at)");
        }
        if (trimmed.starts_with("created_at:") || trimmed.starts_with("created:"))
            && is_date_only_format(trimmed)
            && !deprecated_fields.contains(&"created (should be created_at)")
        {
            deprecated_fields.push("created_at uses date format (should be datetime)");
        }
        if trimmed.starts_with("updated_at:") && is_date_only_format(trimmed) {
            deprecated_fields.push("updated_at uses date format (should be datetime)");
        }
    }

    if deprecated_fields.is_empty() {
        return None;
    }

    Some(Problem {
        severity: Severity::Error,
        path: path.to_path_buf(),
        message: format!(
            "unsupported legacy frontmatter: {}",
            deprecated_fields.join(", ")
        ),
        fix: None,
        scope: None,
        category: None,
        check_id: CheckId::StoryDeprecatedFields,
    })
}

/// Check story date field consistency
pub fn check_story_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        problems.extend(structural::check_date_consistency(
            &story.path,
            CheckId::StoryDateConsistency,
        ));
    }

    problems
}

/// Check that story role fields have valid taxonomy syntax
pub fn check_role_syntax(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if let Some(role) = &story.frontmatter.role
            && let Err(e) = crate::domain::model::taxonomy::parse(role)
        {
            problems.push(
                Problem::error(
                    story.path.clone(),
                    format!("invalid role syntax '{}': {}", role, e),
                )
                .with_check_id(CheckId::StoryInvalidRole)
                .with_scope(story.frontmatter.scope.clone().unwrap_or_default()),
            );
        }
    }

    problems
}

/// Check for orphaned scope references (story refers to non-existent voyage)
pub fn check_orphaned_scope_references(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if let Some(scope) = &story.frontmatter.scope {
            let parts: Vec<&str> = scope.split('/').collect();
            if parts.len() >= 2 {
                let epic_id = parts[0];
                let voyage_id = parts[1];

                if !board.epics.contains_key(epic_id) {
                    problems.push(
                        Problem::warning(
                            story.path.clone(),
                            format!("scope references non-existent epic '{}'", epic_id),
                        )
                        .with_check_id(CheckId::StoryOrphanedScope)
                        .with_scope(scope),
                    );
                    continue;
                }

                let voyage_exists = board
                    .voyages
                    .values()
                    .any(|v| v.epic_id == epic_id && v.id() == voyage_id);

                if !voyage_exists {
                    problems.push(
                        Problem::warning(
                            story.path.clone(),
                            format!(
                                "scope references non-existent voyage '{}' in epic '{}'",
                                voyage_id, epic_id
                            ),
                        )
                        .with_check_id(CheckId::StoryOrphanedScope)
                        .with_scope(scope),
                    );
                }
            }
        }
    }

    problems
}

/// Check for index number violations within scopes (gaps and duplicates)
pub fn check_index_validation(board: &Board) -> Vec<Problem> {
    use std::collections::BTreeMap;

    let mut problems = Vec::new();
    let mut scope_indexes: HashMap<String, Vec<(u32, &str, PathBuf)>> = HashMap::new();

    for story in board.stories.values() {
        if let (Some(scope), Some(seq)) = (story.scope(), story.index()) {
            scope_indexes.entry(scope.to_string()).or_default().push((
                seq,
                story.id(),
                story.path.clone(),
            ));
        }
    }

    for (scope, mut entries) in scope_indexes {
        entries.sort_by_key(|(seq, _, _)| *seq);

        let mut seen: BTreeMap<u32, Vec<(&str, PathBuf)>> = BTreeMap::new();
        for (seq, id, path) in &entries {
            seen.entry(*seq).or_default().push((*id, path.clone()));
        }

        // Check for out-of-order work: Backlog/InProgress story with higher index than an Icebox story
        let mut icebox_indices = Vec::new();
        let mut active_indices = Vec::new();
        for (seq, id, _) in &entries {
            if let Some(story) = board.stories.get(*id) {
                if story.stage == StoryState::Icebox {
                    icebox_indices.push(*seq);
                } else if story.stage == StoryState::Backlog
                    || story.stage == StoryState::InProgress
                {
                    active_indices.push(*seq);
                }
            }
        }

        if !icebox_indices.is_empty() && !active_indices.is_empty() {
            // If any active story has a higher index than any icebox story, it's out of order
            if active_indices
                .iter()
                .any(|&a| icebox_indices.iter().any(|&i| a > i))
            {
                problems.push(
                    Problem::warning(
                        entries[0].2.clone(),
                        format!("{}: stories are being worked on out of order (backlog story has higher index than icebox story)", scope),
                    )
                    .with_check_id(CheckId::StoryIndexGap)
                    .with_scope(&scope),
                );
            }
        }

        for (seq, stories) in &seen {
            if stories.len() > 1 {
                let ids: Vec<&str> = stories.iter().map(|(id, _)| *id).collect();
                problems.push(
                    Problem::warning(
                        stories[0].1.clone(),
                        format!("{}: duplicate index {} ({})", scope, seq, ids.join(", ")),
                    )
                    .with_check_id(CheckId::StoryIndexDuplicate)
                    .with_scope(&scope),
                );
            }
        }

        let indexes: Vec<u32> = entries.iter().map(|(seq, _, _)| *seq).collect();
        if !indexes.is_empty() {
            let min_seq = *indexes.iter().min().unwrap();
            let max_seq = *indexes.iter().max().unwrap();

            let seq_set: HashSet<u32> = indexes.iter().copied().collect();
            let mut gaps = Vec::new();
            for expected in min_seq..=max_seq {
                if !seq_set.contains(&expected) {
                    gaps.push(expected);
                }
            }

            if !gaps.is_empty() {
                let first_path = entries[0].2.clone();
                problems.push(
                    Problem::warning(
                        first_path,
                        format!(
                            "{}: index gap at {} (has {})",
                            scope,
                            gaps.iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(", "),
                            indexes
                                .iter()
                                .map(|n| n.to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )
                    .with_check_id(CheckId::StoryIndexGap)
                    .with_scope(&scope),
                );
            }
        }
    }

    problems
}

/// Check that needs-human-verification and done stories have all acceptance criteria checked
pub fn check_acceptance_criteria_complete(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.stage != StoryState::NeedsHumanVerification && story.stage != StoryState::Done {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let criteria = parse_acceptance_criteria(&content);

        if !criteria.is_complete() {
            let unchecked_list = criteria
                .unchecked
                .iter()
                .map(|c| format!("      - [ ] {}", c))
                .collect::<Vec<_>>()
                .join("\n");

            problems.push(
                Problem::warning(
                    story.path.clone(),
                    format!(
                        "{} unchecked acceptance criteria:\n{}",
                        criteria.unchecked.len(),
                        unchecked_list
                    ),
                )
                .with_check_id(CheckId::StoryIncompleteAcceptance)
                .with_scope(story.scope().unwrap_or_default()),
            );
        }
    }

    problems
}

/// Check story title case
pub fn check_story_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        let title = &story.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(
                Problem::warning(
                    story.path.clone(),
                    format!("title '{}' should use Title Case", title),
                )
                .with_check_id(CheckId::TitleCaseViolation)
                .with_category(GapCategory::Convention)
                .with_scope(story.scope().unwrap_or_default())
                .with_fix(Fix::UpdateTitle {
                    path: story.path.clone(),
                    new_title,
                }),
            );
        }
    }

    problems
}

/// Check that stories in terminal stages have valid verification manifests
pub fn check_verification_manifests(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        // Manifest is required for terminal stages

        if story.stage != StoryState::NeedsHumanVerification && story.stage != StoryState::Done {
            continue;
        }

        let story_dir = story.path.parent().unwrap();

        let manifest_path = story_dir.join("manifest.yaml");

        if !manifest_path.exists() {
            problems.push(
                Problem::error(
                    story.path.clone(),
                    "missing verification manifest (run `keel verify` to generate)",
                )
                .with_check_id(CheckId::StoryMissingManifest)
                .with_scope(story.scope().unwrap_or_default()),
            );

            continue;
        }

        // Verify manifest integrity

        let manifest_content = match fs::read_to_string(&manifest_path) {
            Ok(c) => c,

            Err(e) => {
                problems.push(
                    Problem::error(manifest_path, format!("cannot read manifest: {}", e))
                        .with_check_id(CheckId::StoryManifestTampered)
                        .with_scope(story.scope().unwrap_or_default()),
                );

                continue;
            }
        };

        let manifest: crate::domain::model::Manifest = match serde_yaml::from_str(&manifest_content)
        {
            Ok(m) => m,

            Err(e) => {
                problems.push(
                    Problem::error(manifest_path, format!("invalid manifest YAML: {}", e))
                        .with_check_id(CheckId::StoryManifestTampered)
                        .with_scope(story.scope().unwrap_or_default()),
                );

                continue;
            }
        };

        // Check evidence hashes

        for (rel_path, expected_hash) in &manifest.evidence {
            let full_path = story_dir.join(rel_path);

            if !full_path.exists() {
                problems.push(
                    Problem::error(
                        manifest_path.clone(),
                        format!("manifested evidence file missing: {}", rel_path),
                    )
                    .with_check_id(CheckId::StoryManifestTampered)
                    .with_scope(story.scope().unwrap_or_default()),
                );

                continue;
            }

            match crate::infrastructure::utils::hash_file(&full_path) {
                Ok(actual_hash) => {
                    if actual_hash != *expected_hash {
                        problems.push(
                            Problem::error(
                                manifest_path.clone(),
                                format!(
                                    "evidence file tampered with (hash mismatch): {}",
                                    rel_path
                                ),
                            )
                            .with_check_id(CheckId::StoryManifestTampered)
                            .with_scope(story.scope().unwrap_or_default()),
                        );
                    }
                }

                Err(e) => {
                    problems.push(
                        Problem::error(full_path, format!("failed to calculate hash: {}", e))
                            .with_check_id(CheckId::StoryManifestTampered)
                            .with_scope(story.scope().unwrap_or_default()),
                    );
                }
            }
        }

        // Check for new evidence files not in manifest

        let evidence_dir = story_dir.join("EVIDENCE");

        if evidence_dir.exists()
            && let Ok(entries) = fs::read_dir(evidence_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();

                    let rel_path = format!("EVIDENCE/{}", filename);

                    if !manifest.evidence.contains_key(&rel_path) {
                        problems.push(

                                Problem::warning(

                                    manifest_path.clone(),

                                    format!("new evidence file not in manifest: {} (run `keel verify` to update)", rel_path),

                                )

                                .with_check_id(CheckId::StoryManifestTampered)

                                .with_scope(story.scope().unwrap_or_default()),

                            );
                    }
                }
            }
        }
    }

    problems
}

pub fn check_verification_annotations(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.stage != StoryState::InProgress
            && story.stage != StoryState::NeedsHumanVerification
            && story.stage != StoryState::Done
        {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut missing_annotations = Vec::new();
        let mut malformed_annotations = Vec::new();

        for line in content.lines() {
            if let Some(start) = line.find("<!--") {
                let rest = &line[start + 4..];
                if let Some(end) = rest.find("-->") {
                    let comment_text = rest[..end].trim();

                    if let Some(verify_start) = comment_text.find("verify:") {
                        let verify_content = comment_text[verify_start + 7..].trim();

                        // New format must contain at least one comma if there are multiple parts.
                        // Actually, even single-part (e.g., manual) is fine without a comma,
                        // but if it looks like "manual SRS-01:start" without a comma, it's old format.

                        let has_comma = verify_content.contains(',');
                        let has_requirement = verify_content.contains("SRS-");
                        let has_proof = verify_content.contains("proof:");

                        // If it has multiple signals but no comma, it's old format.
                        let signals = (if has_requirement { 1 } else { 0 })
                            + (if has_proof { 1 } else { 0 })
                            + 1; // +1 for the command/manual part

                        if signals > 1 && !has_comma {
                            malformed_annotations.push(line.trim().to_string());
                        }
                    }
                }
            } else if line.trim_start().starts_with("- [") {
                // Check for missing annotation on an AC line
                let has_verify = line.contains("verify:") && line.contains("<!--");
                if !has_verify {
                    let text = line.split("<!--").next().unwrap_or(line).trim();
                    if !text.is_empty() && (text.contains("[x]") || text.contains("[ ]")) {
                        missing_annotations.push(text.to_string());
                    }
                }
            }
        }

        if !missing_annotations.is_empty() {
            let list = missing_annotations
                .iter()
                .map(|criterion| format!("      {}", criterion))
                .collect::<Vec<_>>()
                .join("\n");
            problems.push(
                Problem::warning(
                    story.path.clone(),
                    format!(
                        "{} acceptance criteria missing verification annotations:\n{}",
                        missing_annotations.len(),
                        list
                    ),
                )
                .with_check_id(CheckId::StoryMissingVerification)
                .with_scope(story.scope().unwrap_or_default()),
            );
        }

        if !malformed_annotations.is_empty() {
            let list = malformed_annotations
                .iter()
                .map(|line| format!("      {}", line))
                .collect::<Vec<_>>()
                .join("\n");
            problems.push(
                Problem::error(
                    story.path.clone(),
                    format!(
                        "{} verify annotations use legacy space-separated format (must use commas):\n{}",
                        malformed_annotations.len(),
                        list
                    ),
                )
                .with_check_id(CheckId::StoryMalformedVerification)
                .with_scope(story.scope().unwrap_or_default()),
            );
        }
    }

    problems
}

/// Check that acceptance criteria have SRS traceability references.
pub fn check_srs_traceability(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.stage != StoryState::InProgress
            && story.stage != StoryState::NeedsHumanVerification
            && story.stage != StoryState::Done
        {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut missing_refs = Vec::new();

        let criteria = parse_acceptance_criteria(&content);
        for criterion in criteria.checked.into_iter().chain(criteria.unchecked) {
            let text = criterion.split("<!--").next().unwrap_or(&criterion).trim();

            if text.is_empty() {
                continue;
            }

            if !AC_REQ_RE.is_match(text) {
                missing_refs.push(text.to_string());
            }
        }

        if !missing_refs.is_empty() {
            let list = missing_refs
                .iter()
                .map(|criterion| format!("      - {}", criterion))
                .collect::<Vec<_>>()
                .join("\n");
            problems.push(
                Problem::warning(
                    story.path.clone(),
                    format!(
                        "{} acceptance criteria missing SRS refs:\n{}",
                        missing_refs.len(),
                        list
                    ),
                )
                .with_check_id(CheckId::StoryMissingSrsRef)
                .with_scope(story.scope().unwrap_or_default()),
            );
        }
    }

    problems
}

/// Check that stories with a scope have at least one SRS reference in their AC.
pub fn check_scoped_story_evidence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.scope().is_none() {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if !AC_REQ_RE.is_match(&content) {
            problems.push(
                Problem::warning(
                    story.path.clone(),
                    "scoped story has no SRS references in acceptance criteria",
                )
                .with_check_id(CheckId::StoryMissingSrsRef)
                .with_scope(story.scope().unwrap_or_default()),
            );
        }
    }

    problems
}

/// Check that REFLECT.md is only present for active or terminal stories.
/// Stories in backlog or icebox should not have reflections.
pub fn check_reflection_coherence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.stage != StoryState::Backlog && story.stage != StoryState::Icebox {
            continue;
        }

        let reflect_path = story.path.parent().unwrap().join("REFLECT.md");
        if reflect_path.exists() {
            problems.push(
                Problem::warning(
                    reflect_path.clone(),
                    format!("story is in {} stage but has a REFLECT.md", story.stage),
                )
                .with_check_id(CheckId::StoryUnexpectedReflection)
                .with_scope(story.scope().unwrap_or_default())
                .with_fix(Fix::RemoveFile { path: reflect_path }),
            );
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn test_check_verification_annotations_detects_malformed() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::InProgress)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] legacy <!-- verify: manual SRS-01:start -->")
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_verification_annotations(&board);

        assert_eq!(problems.len(), 1);
        assert!(
            problems[0]
                .message
                .contains("legacy space-separated format")
        );
        assert_eq!(problems[0].check_id, CheckId::StoryMalformedVerification);
    }

    #[test]

    fn test_check_verification_annotations_accepts_valid() {
        let temp = TestBoardBuilder::new()

                .story(

                    TestStory::new("S1")

                        .stage(StoryState::InProgress)

                        .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] valid <!-- verify: manual, SRS-01:start -->")

                )

                .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let problems = check_verification_annotations(&board);

        assert_eq!(problems.len(), 0);
    }

    #[test]

    fn test_check_index_order_detects_out_of_order_work() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .scope("epic/v1")
                    .index(1)
                    .stage(StoryState::Icebox),
            )
            .story(
                TestStory::new("S2")
                    .scope("epic/v1")
                    .index(2)
                    .stage(StoryState::Backlog),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        // This function needs to be implemented or updated to check for this

        let problems = check_index_validation(&board);

        let has_out_of_order = problems.iter().any(|p| p.message.contains("out of order"));

        assert!(has_out_of_order);
    }
}
