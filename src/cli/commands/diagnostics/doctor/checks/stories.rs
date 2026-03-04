#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use super::super::AC_REQ_RE;
use super::super::types::*;
use crate::domain::model::{Board, StoryState, VoyageState};
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
        if story.stage != StoryState::Backlog
            && story.stage != StoryState::InProgress
            && story.stage != StoryState::NeedsHumanVerification
            && story.stage != StoryState::Done
        {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let criteria = parse_acceptance_criteria(&content);
        let missing_refs = crate::infrastructure::validation::missing_srs_references(&criteria);

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

/// Check for cycles in derived implementation dependencies.
///
/// Cycles indicate conflicting AC/SRS traceability where no story can be started
/// without completing another unfinished story in the same cycle.
pub fn check_story_dependency_cycles(board: &Board) -> Vec<Problem> {
    let active_story_ids: HashSet<String> = board
        .stories
        .values()
        .filter(|story| story.stage != StoryState::Done)
        .map(|story| story.id().to_string())
        .collect();

    if active_story_ids.is_empty() {
        return Vec::new();
    }

    let dependencies = crate::read_model::traceability::derive_implementation_dependencies(board);
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for story_id in &active_story_ids {
        let mut active_dependencies: Vec<String> = dependencies
            .get(story_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|dependency_id| active_story_ids.contains(dependency_id))
            .collect();
        active_dependencies.sort();
        active_dependencies.dedup();
        graph.insert(story_id.clone(), active_dependencies);
    }

    let cycles = find_dependency_cycles(&graph);
    let mut problems = Vec::new();

    for cycle in cycles {
        let Some(anchor_story) = board.stories.get(&cycle[0]) else {
            continue;
        };

        let mut scopes: HashSet<String> = HashSet::new();
        for story_id in &cycle {
            if let Some(scope) = board
                .stories
                .get(story_id)
                .and_then(|story| story.scope())
                .map(|scope| scope.to_string())
            {
                scopes.insert(scope);
            }
        }

        let mut scope_list: Vec<String> = scopes.into_iter().collect();
        scope_list.sort();

        let scope_hint = if scope_list.is_empty() {
            String::new()
        } else {
            format!("scope(s): {}. ", scope_list.join(", "))
        };

        problems.push(
            Problem::error(
                anchor_story.path.clone(),
                format!(
                    "implementation dependency cycle detected among stories [{}]. {}Update AC SRS references so dependency order is acyclic.",
                    cycle.join(", "),
                    scope_hint
                ),
            )
            .with_check_id(CheckId::StoryDependencyCycle)
            .with_scope(anchor_story.scope().unwrap_or_default()),
        );
    }

    problems
}

fn find_dependency_cycles(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    struct Tarjan<'a> {
        graph: &'a HashMap<String, Vec<String>>,
        index: usize,
        indices: HashMap<String, usize>,
        low_links: HashMap<String, usize>,
        stack: Vec<String>,
        on_stack: HashSet<String>,
        components: Vec<Vec<String>>,
    }

    impl<'a> Tarjan<'a> {
        fn new(graph: &'a HashMap<String, Vec<String>>) -> Self {
            Self {
                graph,
                index: 0,
                indices: HashMap::new(),
                low_links: HashMap::new(),
                stack: Vec::new(),
                on_stack: HashSet::new(),
                components: Vec::new(),
            }
        }

        fn run(mut self) -> Vec<Vec<String>> {
            let mut nodes: Vec<String> = self.graph.keys().cloned().collect();
            nodes.sort();

            for node in nodes {
                if !self.indices.contains_key(&node) {
                    self.strong_connect(node);
                }
            }

            self.components
        }

        fn strong_connect(&mut self, node: String) {
            self.indices.insert(node.clone(), self.index);
            self.low_links.insert(node.clone(), self.index);
            self.index += 1;

            self.stack.push(node.clone());
            self.on_stack.insert(node.clone());

            let neighbors = self.graph.get(&node).cloned().unwrap_or_default();
            for neighbor in neighbors {
                if !self.indices.contains_key(&neighbor) {
                    self.strong_connect(neighbor.clone());
                    let node_low = *self.low_links.get(&node).unwrap();
                    let neighbor_low = *self.low_links.get(&neighbor).unwrap();
                    self.low_links
                        .insert(node.clone(), std::cmp::min(node_low, neighbor_low));
                } else if self.on_stack.contains(&neighbor) {
                    let node_low = *self.low_links.get(&node).unwrap();
                    let neighbor_index = *self.indices.get(&neighbor).unwrap();
                    self.low_links
                        .insert(node.clone(), std::cmp::min(node_low, neighbor_index));
                }
            }

            let node_low = *self.low_links.get(&node).unwrap();
            let node_index = *self.indices.get(&node).unwrap();
            if node_low == node_index {
                let mut component = Vec::new();
                while let Some(current) = self.stack.pop() {
                    self.on_stack.remove(&current);
                    component.push(current.clone());
                    if current == node {
                        break;
                    }
                }
                self.components.push(component);
            }
        }
    }

    let components = Tarjan::new(graph).run();

    let mut cycles = Vec::new();
    for mut component in components {
        let has_cycle = if component.len() > 1 {
            true
        } else {
            let node = &component[0];
            graph
                .get(node)
                .is_some_and(|dependencies| dependencies.iter().any(|dep| dep == node))
        };

        if has_cycle {
            component.sort();
            cycles.push(component);
        }
    }

    cycles.sort_by(|a, b| a[0].cmp(&b[0]));
    cycles
}

/// Check terminal stories for unresolved scaffold/default text in README and REFLECT artifacts.
pub fn check_terminal_story_coherence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.stage != StoryState::NeedsHumanVerification && story.stage != StoryState::Done {
            continue;
        }

        let scope = story.scope().unwrap_or_default().to_string();

        if let Ok(content) = fs::read_to_string(&story.path)
            && let Some(pattern) = structural::first_unfilled_placeholder_pattern(&content)
        {
            problems.push(
                Problem::error(
                    story.path.clone(),
                    format!(
                        "README has unresolved scaffold/default text (pattern: {})",
                        pattern
                    ),
                )
                .with_check_id(CheckId::StoryTerminalScaffold)
                .with_scope(scope.clone())
                .with_fix(Fix::ClearPlaceholder {
                    path: story.path.clone(),
                    pattern,
                }),
            );
        }

        let reflect_path = story.path.parent().unwrap().join("REFLECT.md");
        if reflect_path.exists()
            && let Ok(content) = fs::read_to_string(&reflect_path)
        {
            if let Some(pattern) = structural::first_unfilled_placeholder_pattern(&content) {
                problems.push(
                    Problem::error(
                        reflect_path.clone(),
                        format!(
                            "REFLECT has unresolved scaffold/default text (pattern: {})",
                            pattern
                        ),
                    )
                    .with_check_id(CheckId::StoryTerminalScaffold)
                    .with_scope(scope.clone())
                    .with_fix(Fix::ClearPlaceholder {
                        path: reflect_path.clone(),
                        pattern,
                    }),
                );
            }

            for issue in crate::read_model::knowledge::scanner::validate_knowledge_content(&content)
            {
                problems.push(
                    Problem::error(
                        reflect_path.clone(),
                        format!(
                            "REFLECT has invalid knowledge unit {}: {}",
                            issue.id, issue.reason
                        ),
                    )
                    .with_check_id(CheckId::StoryTerminalScaffold)
                    .with_scope(scope.clone()),
                );
            }
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

/// Check stories in active voyage scopes for unresolved scaffold/default text.
///
/// Stories in planned/in-progress voyages are actionable queue items, so their
/// README artifacts must be fully authored before execution begins.
pub fn check_active_story_coherence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for story in board.stories.values() {
        let Some(scope) = story.scope() else {
            continue;
        };

        let Some(voyage) = board
            .voyages
            .values()
            .find(|voyage| voyage.scope_path() == scope)
        else {
            continue;
        };

        if voyage.status() != VoyageState::Planned && voyage.status() != VoyageState::InProgress {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        let criteria = parse_acceptance_criteria(&content);
        let criteria_count = criteria.checked.len() + criteria.unchecked.len();
        if !criteria.has_section {
            problems.push(
                Problem::error(
                    story.path.clone(),
                    format!("Story {} has no acceptance criteria section", story.id()),
                )
                .with_check_id(CheckId::StoryIncompleteAcceptance)
                .with_scope(scope),
            );
        } else if criteria_count == 0 {
            problems.push(
                Problem::error(
                    story.path.clone(),
                    format!(
                        "Story {} has no acceptance criteria checklist items",
                        story.id()
                    ),
                )
                .with_check_id(CheckId::StoryIncompleteAcceptance)
                .with_scope(scope),
            );
        }

        if let Some(pattern) = structural::first_unfilled_placeholder_pattern(&content) {
            problems.push(
                Problem::error(
                    story.path.clone(),
                    format!(
                        "README has unresolved scaffold/default text (pattern: {})",
                        pattern
                    ),
                )
                .with_check_id(CheckId::StoryPlanningScaffold)
                .with_scope(scope),
            );
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

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

    #[test]
    fn test_check_story_dependency_cycles_detects_cycle() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(
                TestVoyage::new("01-test", "keel")
                    .status("planned")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1\n- [ ] [SRS-02/AC-01] req2"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-02] req1b\n- [ ] [SRS-02/AC-02] req2b"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_story_dependency_cycles(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryDependencyCycle);
        assert!(problems[0].message.contains("S1"));
        assert!(problems[0].message.contains("S2"));
    }

    #[test]
    fn test_check_story_dependency_cycles_allows_acyclic_ordering() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(
                TestVoyage::new("01-test", "keel")
                    .status("planned")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_story_dependency_cycles(&board);

        assert!(problems.is_empty());
    }

    #[test]
    fn terminal_coherence_fails_story_readme_scaffold_in_needs_human_verification() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::NeedsHumanVerification)
                    .body("## Summary\n\nTODO: Describe the story\n\n## Acceptance Criteria\n\n- [x] [SRS-02/AC-01] done <!-- verify: manual, SRS-02:start:end -->"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_terminal_story_coherence(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryTerminalScaffold);
        assert!(
            problems[0]
                .message
                .contains("README has unresolved scaffold/default text")
        );
    }

    #[test]
    fn terminal_coherence_fails_reflect_scaffold_in_done_stage() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Done)
                    .body("## Summary\n\nImplemented.\n\n## Acceptance Criteria\n\n- [x] [SRS-03/AC-01] done <!-- verify: manual, SRS-03:start:end -->"),
            )
            .build();

        let reflect_path = temp.path().join("stories").join("S1").join("REFLECT.md");
        fs::write(
            &reflect_path,
            "# Reflection - Test\n\n## Observations\n\nTODO: What went well?",
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_terminal_story_coherence(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryTerminalScaffold);
        assert_eq!(problems[0].path, reflect_path);
        assert!(
            problems[0]
                .message
                .contains("REFLECT has unresolved scaffold/default text")
        );
    }

    #[test]
    fn terminal_coherence_fails_invalid_knowledge_entries_in_reflect() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Done)
                    .body("## Summary\n\nImplemented.\n\n## Acceptance Criteria\n\n- [x] [SRS-03/AC-01] done <!-- verify: manual, SRS-03:start:end -->"),
            )
            .build();

        let reflect_path = temp.path().join("stories").join("S1").join("REFLECT.md");
        fs::write(
            &reflect_path,
            "# Reflection - Test\n\n## Knowledge\n\n### L001: Implementation Insight\n\n| Field | Value |\n|-------|-------|\n| **Insight** | |\n| **Suggested Action** | |\n",
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_terminal_story_coherence(&board);

        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("invalid knowledge unit")),
            "expected invalid knowledge unit error for placeholder/empty knowledge block"
        );
    }

    #[test]
    fn terminal_coherence_allows_reflect_without_knowledge_entries() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Done)
                    .body("## Summary\n\nImplemented.\n\n## Acceptance Criteria\n\n- [x] [SRS-03/AC-01] done <!-- verify: manual, SRS-03:start:end -->"),
            )
            .build();

        let reflect_path = temp.path().join("stories").join("S1").join("REFLECT.md");
        fs::write(
            &reflect_path,
            "# Reflection - Test\n\n## Knowledge\n\n## Observations\n\nNo reusable insight captured.",
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_terminal_story_coherence(&board);

        assert!(
            problems
                .iter()
                .all(|p| !p.message.contains("invalid knowledge unit")),
            "empty knowledge sections should be allowed"
        );
    }

    #[test]
    fn terminal_coherence_skips_non_terminal_stories() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::InProgress)
                    .body("## Summary\n\nTODO: Describe the story\n\n## Acceptance Criteria\n\n- [ ] [SRS-02/AC-02] todo <!-- verify: manual, SRS-02:start:end -->"),
            )
            .build();

        let reflect_path = temp.path().join("stories").join("S1").join("REFLECT.md");
        fs::write(
            reflect_path,
            "# Reflection - Test\n\n## Observations\n\nTODO: What went well?",
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_terminal_story_coherence(&board);

        assert!(
            problems.is_empty(),
            "non-terminal stories should not be checked for terminal scaffold coherence"
        );
    }

    #[test]
    fn active_story_coherence_flags_scaffold_in_planned_voyage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .stage(StoryState::Backlog)
                    .body(
                        "## Summary\n\nTODO: Describe the story\n\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Define acceptance criteria for this slice",
                    ),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_active_story_coherence(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryPlanningScaffold);
        assert!(
            problems[0]
                .message
                .contains("README has unresolved scaffold/default text")
        );
    }

    #[test]
    fn active_story_coherence_requires_acceptance_criteria_items() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("in-progress"))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .stage(StoryState::Backlog)
                    .body("## Summary\n\nReady.\n\n## Acceptance Criteria\n\nTBD"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_active_story_coherence(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::StoryIncompleteAcceptance);
        assert!(
            problems[0]
                .message
                .contains("has no acceptance criteria checklist items")
        );
    }

    #[test]
    fn active_story_coherence_skips_draft_voyage_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("draft"))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .stage(StoryState::Icebox)
                    .body(
                        "## Summary\n\nTODO: Describe the story\n\n## Acceptance Criteria\n\n- [ ] TODO: Add criteria",
                    ),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_active_story_coherence(&board);

        assert!(
            problems.is_empty(),
            "draft voyage stories should be skipped"
        );
    }

    #[test]
    fn srs_traceability_flags_backlog_story_missing_refs() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] Missing traceability marker"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_srs_traceability(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].check_id, CheckId::StoryMissingSrsRef);
        assert!(problems[0].message.contains("missing SRS refs"));
    }

    #[test]
    fn srs_traceability_skips_icebox_story_missing_refs() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Icebox)
                    .body("## Acceptance Criteria\n\n- [ ] Missing traceability marker"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_srs_traceability(&board);

        assert!(problems.is_empty());
    }
}
