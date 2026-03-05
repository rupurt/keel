//! Shared duplicate-ID scanner used by commands and diagnostics.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use crate::domain::model::{
    AdrFrontmatter, BearingFrontmatter, EpicFrontmatter, StoryFrontmatter, VoyageFrontmatter,
};
use crate::infrastructure::parser::parse_frontmatter;
use crate::infrastructure::validation::{CheckId, Problem};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuplicateEntity {
    Story,
    Voyage,
    Epic,
    Bearing,
    Adr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateIdGroup {
    pub id: String,
    pub paths: Vec<PathBuf>,
}

impl DuplicateEntity {
    fn singular_label(self) -> &'static str {
        match self {
            Self::Story => "story",
            Self::Voyage => "voyage",
            Self::Epic => "epic",
            Self::Bearing => "bearing",
            Self::Adr => "ADR",
        }
    }

    fn check_id(self) -> CheckId {
        match self {
            Self::Story => CheckId::StoryDuplicateId,
            Self::Voyage => CheckId::VoyageDuplicateId,
            Self::Epic => CheckId::EpicDuplicateId,
            Self::Bearing => CheckId::BearingDuplicateId,
            Self::Adr => CheckId::AdrDuplicateId,
        }
    }

    fn collect_candidate_paths(self, board_dir: &Path) -> Vec<PathBuf> {
        match self {
            Self::Story => collect_story_paths(board_dir),
            Self::Voyage => collect_voyage_paths(board_dir),
            Self::Epic => collect_epic_paths(board_dir),
            Self::Bearing => collect_bearing_paths(board_dir),
            Self::Adr => collect_adr_paths(board_dir),
        }
    }

    fn parse_id(self, content: &str) -> Option<String> {
        match self {
            Self::Story => parse_frontmatter::<StoryFrontmatter>(content)
                .ok()
                .map(|(fm, _)| fm.id),
            Self::Voyage => parse_frontmatter::<VoyageFrontmatter>(content)
                .ok()
                .map(|(fm, _)| fm.id),
            Self::Epic => parse_frontmatter::<EpicFrontmatter>(content)
                .ok()
                .map(|(fm, _)| fm.id),
            Self::Bearing => parse_frontmatter::<BearingFrontmatter>(content)
                .ok()
                .map(|(fm, _)| fm.id),
            Self::Adr => parse_frontmatter::<AdrFrontmatter>(content)
                .ok()
                .map(|(fm, _)| fm.id),
        }
    }
}

pub fn scan_duplicate_ids(board_dir: &Path, entity: DuplicateEntity) -> Vec<DuplicateIdGroup> {
    let mut id_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in entity.collect_candidate_paths(board_dir) {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let Some(id) = entity.parse_id(&content) else {
            continue;
        };
        id_to_paths.entry(id).or_default().push(path);
    }

    let mut duplicates: Vec<DuplicateIdGroup> = id_to_paths
        .into_iter()
        .filter_map(|(id, mut paths)| {
            if paths.len() < 2 {
                return None;
            }
            paths.sort();
            Some(DuplicateIdGroup { id, paths })
        })
        .collect();
    duplicates.sort_by(|left, right| left.id.cmp(&right.id));
    duplicates
}

pub fn duplicate_id_problems(board_dir: &Path, entity: DuplicateEntity) -> Vec<Problem> {
    let mut problems = Vec::new();

    for duplicate in scan_duplicate_ids(board_dir, entity) {
        for path in &duplicate.paths {
            let other_paths: Vec<_> = duplicate
                .paths
                .iter()
                .filter(|candidate| *candidate != path)
                .map(|candidate| candidate.display().to_string())
                .collect();
            problems.push(
                Problem::error(
                    path.clone(),
                    format!(
                        "duplicate {} ID '{}' (also in: {})",
                        entity.singular_label(),
                        duplicate.id,
                        other_paths.join(", ")
                    ),
                )
                .with_check_id(entity.check_id()),
            );
        }
    }

    problems
}

pub fn ensure_unique_ids(board_dir: &Path, entity: DuplicateEntity, command: &str) -> Result<()> {
    let duplicates = scan_duplicate_ids(board_dir, entity);
    if duplicates.is_empty() {
        return Ok(());
    }

    let mut lines = Vec::new();
    for duplicate in duplicates {
        let paths = duplicate
            .paths
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("  - {} => {}", duplicate.id, paths));
    }

    bail!(
        "Cannot run `{}` while duplicate {} IDs exist.\nRun `keel doctor` to fix duplicates first.\n{}",
        command,
        entity.singular_label(),
        lines.join("\n")
    )
}

fn collect_story_paths(board_dir: &Path) -> Vec<PathBuf> {
    let stories_dir = board_dir.join("stories");
    if !stories_dir.exists() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(&stories_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let readme = path.join("README.md");
                if readme.exists() {
                    paths.push(readme);
                }
            }
        }
    }
    paths.sort();
    paths
}

fn collect_epic_paths(board_dir: &Path) -> Vec<PathBuf> {
    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(&epics_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let readme = path.join("README.md");
                if readme.exists() {
                    paths.push(readme);
                }
            }
        }
    }
    paths.sort();
    paths
}

fn collect_voyage_paths(board_dir: &Path) -> Vec<PathBuf> {
    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    if let Ok(epic_entries) = fs::read_dir(&epics_dir) {
        for epic_entry in epic_entries.flatten() {
            let epic_path = epic_entry.path();
            if !epic_path.is_dir() {
                continue;
            }

            let voyages_dir = epic_path.join("voyages");
            if !voyages_dir.exists() {
                continue;
            }

            if let Ok(voyage_entries) = fs::read_dir(voyages_dir) {
                for voyage_entry in voyage_entries.flatten() {
                    let voyage_path = voyage_entry.path();
                    if !voyage_path.is_dir() {
                        continue;
                    }
                    let readme = voyage_path.join("README.md");
                    if readme.exists() {
                        paths.push(readme);
                    }
                }
            }
        }
    }
    paths.sort();
    paths
}

fn collect_bearing_paths(board_dir: &Path) -> Vec<PathBuf> {
    let bearings_dir = board_dir.join("bearings");
    if !bearings_dir.exists() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(&bearings_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let readme = path.join("README.md");
                if readme.exists() {
                    paths.push(readme);
                }
            }
        }
    }
    paths.sort();
    paths
}

fn collect_adr_paths(board_dir: &Path) -> Vec<PathBuf> {
    let adrs_dir = board_dir.join("adrs");
    if !adrs_dir.exists() {
        return Vec::new();
    }

    let mut paths = Vec::new();
    if let Ok(entries) = fs::read_dir(&adrs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|extension| extension == "md") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn scan_duplicate_story_ids_detects_collisions() {
        let temp = TempDir::new().unwrap();
        let stories_dir = temp.path().join("stories");
        fs::create_dir_all(stories_dir.join("A")).unwrap();
        fs::create_dir_all(stories_dir.join("B")).unwrap();
        fs::write(
            stories_dir.join("A/README.md"),
            "---\nid: SAME\ntitle: One\nstatus: backlog\ntype: feat\ncreated_at: 2026-01-01T00:00:00\nupdated_at: 2026-01-01T00:00:00\n---\n",
        )
        .unwrap();
        fs::write(
            stories_dir.join("B/README.md"),
            "---\nid: SAME\ntitle: Two\nstatus: backlog\ntype: feat\ncreated_at: 2026-01-01T00:00:00\nupdated_at: 2026-01-01T00:00:00\n---\n",
        )
        .unwrap();

        let duplicates = scan_duplicate_ids(temp.path(), DuplicateEntity::Story);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].id, "SAME");
        assert_eq!(duplicates[0].paths.len(), 2);
    }

    #[test]
    fn ensure_unique_ids_returns_actionable_error() {
        let temp = TempDir::new().unwrap();
        let epics_dir = temp.path().join("epics");
        fs::create_dir_all(epics_dir.join("E1")).unwrap();
        fs::create_dir_all(epics_dir.join("E2")).unwrap();
        fs::write(
            epics_dir.join("E1/README.md"),
            "---\nid: DUP\ntitle: One\ncreated_at: 2026-01-01T00:00:00\n---\n",
        )
        .unwrap();
        fs::write(
            epics_dir.join("E2/README.md"),
            "---\nid: DUP\ntitle: Two\ncreated_at: 2026-01-01T00:00:00\n---\n",
        )
        .unwrap();

        let err = ensure_unique_ids(temp.path(), DuplicateEntity::Epic, "keel epic new")
            .unwrap_err()
            .to_string();
        assert!(err.contains("Cannot run `keel epic new`"));
        assert!(err.contains("duplicate epic IDs"));
        assert!(err.contains("Run `keel doctor`"));
    }
}
