use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::model::Story;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::utils::{hash_file, slugify};

use super::parser::{RequirementPhase, parse_verify_annotations};

const JUDGE_BUNDLE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JudgeBundle {
    pub schema_version: u32,
    pub story: JudgeBundleStory,
    pub criterion: JudgeBundleCriterion,
    pub evidence: Vec<JudgeBundleEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JudgeBundleStory {
    pub id: String,
    pub title: String,
    pub scope: Option<String>,
    pub readme_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JudgeBundleCriterion {
    pub text: String,
    pub command: Option<String>,
    pub proof: Option<String>,
    pub srs_requirement: Option<String>,
    pub requirement_phase: Option<String>,
    pub acceptance_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JudgeBundleEvidence {
    pub rel_path: String,
    pub role: String,
    pub media_type: String,
    pub exists: bool,
    pub sha256: Option<String>,
    pub size_bytes: Option<u64>,
}

pub fn build_judge_bundle(
    board_dir: &Path,
    story_id: &str,
    criterion: &str,
) -> Result<JudgeBundle> {
    let board = load_board(board_dir)?;
    let story = board
        .stories
        .get(story_id)
        .with_context(|| format!("Story not found: {story_id}"))?;
    let story_content = fs::read_to_string(&story.path)?;
    let annotation = parse_verify_annotations(&story_content)
        .into_iter()
        .find(|ann| ann.criterion == criterion)
        .with_context(|| {
            format!(
                "Acceptance criterion not found for story {}: {}",
                story.id(),
                criterion
            )
        })?;

    let story_dir = story
        .path
        .parent()
        .with_context(|| format!("Story path has no parent: {}", story.path.display()))?;
    let proof_ref = annotation.proof.as_deref().map(normalize_proof_rel_path);
    let evidence = collect_evidence(story_dir, proof_ref.as_deref())?;

    Ok(JudgeBundle {
        schema_version: JUDGE_BUNDLE_SCHEMA_VERSION,
        story: build_story_context(board_dir, story),
        criterion: JudgeBundleCriterion {
            text: annotation.criterion,
            command: annotation.command,
            proof: proof_ref,
            srs_requirement: annotation.requirement.as_ref().map(|req| req.id.clone()),
            requirement_phase: annotation
                .requirement
                .as_ref()
                .map(|req| requirement_phase_name(&req.phase).to_string()),
            acceptance_ref: annotation
                .ac_ref
                .as_ref()
                .map(|reference| format!("{}/AC-{:02}", reference.srs_id, reference.ac_num)),
        },
        evidence,
    })
}

pub fn materialize_judge_bundle(
    board_dir: &Path,
    story_id: &str,
    criterion: &str,
) -> Result<PathBuf> {
    let bundle = build_judge_bundle(board_dir, story_id, criterion)?;
    let evidence_dir = board_dir.join("stories").join(story_id).join("EVIDENCE");
    fs::create_dir_all(&evidence_dir)?;

    let bundle_path = evidence_dir.join(format!("judge-bundle-{}.json", slugify(criterion)));
    fs::write(&bundle_path, serde_json::to_vec_pretty(&bundle)?)?;
    Ok(bundle_path)
}

fn build_story_context(board_dir: &Path, story: &Story) -> JudgeBundleStory {
    let readme_path = story
        .path
        .strip_prefix(board_dir)
        .unwrap_or(&story.path)
        .display()
        .to_string();

    JudgeBundleStory {
        id: story.id().to_string(),
        title: story.title().to_string(),
        scope: story.scope().map(str::to_string),
        readme_path,
    }
}

fn collect_evidence(story_dir: &Path, proof_ref: Option<&str>) -> Result<Vec<JudgeBundleEvidence>> {
    let evidence_dir = story_dir.join("EVIDENCE");
    let mut rel_paths = Vec::new();

    if let Some(proof_ref) = proof_ref {
        rel_paths.push(proof_ref.to_string());
    }

    if evidence_dir.exists() {
        for entry in fs::read_dir(&evidence_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .with_context(|| format!("Invalid evidence filename: {}", path.display()))?;
            rel_paths.push(format!("EVIDENCE/{file_name}"));
        }
    }

    rel_paths.sort();
    rel_paths.dedup();

    rel_paths
        .into_iter()
        .map(|rel_path| build_evidence_item(story_dir, &rel_path, proof_ref))
        .collect()
}

fn build_evidence_item(
    story_dir: &Path,
    rel_path: &str,
    proof_ref: Option<&str>,
) -> Result<JudgeBundleEvidence> {
    let full_path = story_dir.join(rel_path);
    let metadata = fs::metadata(&full_path).ok();
    let exists = metadata.is_some();

    let sha256 = if exists {
        Some(hash_file(&full_path)?)
    } else {
        None
    };

    let size_bytes = metadata.map(|meta| meta.len());
    let role = if proof_ref == Some(rel_path) {
        "criterion-proof"
    } else {
        "story-evidence"
    };

    Ok(JudgeBundleEvidence {
        rel_path: rel_path.to_string(),
        role: role.to_string(),
        media_type: media_type_for_path(Path::new(rel_path)).to_string(),
        exists,
        sha256,
        size_bytes,
    })
}

fn normalize_proof_rel_path(proof: &str) -> String {
    if proof.starts_with("EVIDENCE/") {
        proof.to_string()
    } else {
        format!("EVIDENCE/{proof}")
    }
}

fn requirement_phase_name(phase: &RequirementPhase) -> &'static str {
    match phase {
        RequirementPhase::Start => "start",
        RequirementPhase::Continues => "continues",
        RequirementPhase::End => "end",
        RequirementPhase::StartEnd => "start:end",
    }
}

fn media_type_for_path(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "gif" => "image/gif",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        "json" => "application/json",
        "yaml" | "yml" => "application/yaml",
        "log" | "txt" | "md" | "tape" => "text/plain",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::board_init::init_board;
    use crate::infrastructure::config::Config;
    use tempfile::tempdir;

    #[test]
    fn artifact_judge_bundle_schema_captures_story_context() {
        let temp = tempdir().unwrap();
        init_board(temp.path(), &Config::default()).unwrap();

        let story_dir = temp.path().join(".keel/stories/S1");
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();
        fs::write(
            story_dir.join("README.md"),
            r#"---
id: S1
title: Judge Story
type: feat
status: in-progress
scope: EPIC1/VOY1
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

# Judge Story

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Judge the dogfood artifacts. <!-- verify: llm-judge, SRS-01:start:end, proof: ac-1.log -->
"#,
        )
        .unwrap();
        fs::write(story_dir.join("EVIDENCE/ac-1.log"), "proof\n").unwrap();
        fs::write(story_dir.join("EVIDENCE/epic-flow.gif"), "gif\n").unwrap();
        fs::write(
            story_dir.join("EVIDENCE/epic-flow.transcript.txt"),
            "transcript\n",
        )
        .unwrap();

        let left = build_judge_bundle(
            temp.path().join(".keel").as_path(),
            "S1",
            "Judge the dogfood artifacts.",
        )
        .unwrap();
        let right = build_judge_bundle(
            temp.path().join(".keel").as_path(),
            "S1",
            "Judge the dogfood artifacts.",
        )
        .unwrap();

        assert_eq!(left.schema_version, 1);
        assert_eq!(left.story.id, "S1");
        assert_eq!(left.story.title, "Judge Story");
        assert_eq!(left.story.scope.as_deref(), Some("EPIC1/VOY1"));
        assert_eq!(left.story.readme_path, "stories/S1/README.md");
        assert_eq!(left.criterion.text, "Judge the dogfood artifacts.");
        assert_eq!(left.criterion.command.as_deref(), Some("llm-judge"));
        assert_eq!(left.criterion.proof.as_deref(), Some("EVIDENCE/ac-1.log"));
        assert_eq!(left.criterion.srs_requirement.as_deref(), Some("SRS-01"));
        assert_eq!(
            left.criterion.requirement_phase.as_deref(),
            Some("start:end")
        );
        assert_eq!(
            left.criterion.acceptance_ref.as_deref(),
            Some("SRS-01/AC-01")
        );
        assert_eq!(
            left.evidence
                .iter()
                .map(|item| item.rel_path.as_str())
                .collect::<Vec<_>>(),
            vec![
                "EVIDENCE/ac-1.log",
                "EVIDENCE/epic-flow.gif",
                "EVIDENCE/epic-flow.transcript.txt",
            ]
        );
        assert_eq!(left.evidence[0].role, "criterion-proof");
        assert_eq!(left.evidence[0].media_type, "text/plain");
        assert!(left.evidence.iter().all(|item| item.exists));
        assert_eq!(
            serde_json::to_string(&left).unwrap(),
            serde_json::to_string(&right).unwrap()
        );
    }

    #[test]
    fn materialize_judge_bundle_writes_stable_bundle_json() {
        let temp = tempdir().unwrap();
        init_board(temp.path(), &Config::default()).unwrap();

        let story_dir = temp.path().join(".keel/stories/S1");
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();
        fs::write(
            story_dir.join("README.md"),
            r#"---
id: S1
title: Judge Story
type: feat
status: in-progress
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

# Judge Story

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Judge the dogfood artifacts. <!-- verify: llm-judge, SRS-01:start:end -->
"#,
        )
        .unwrap();

        let board_dir = temp.path().join(".keel");
        let left =
            materialize_judge_bundle(&board_dir, "S1", "Judge the dogfood artifacts.").unwrap();
        let right =
            materialize_judge_bundle(&board_dir, "S1", "Judge the dogfood artifacts.").unwrap();

        assert_eq!(left, right);
        assert!(left.exists());
    }
}
