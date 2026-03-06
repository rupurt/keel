use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::infrastructure::config::Config;
use crate::infrastructure::dogfood_workspace::{self, reset_workspace};
use crate::infrastructure::verification::executor::generate_manifest;
use crate::infrastructure::vhs;

pub const DOGFOOD_SCENARIO_ROOT: &str = "testdata/dogfood/scenarios";
pub const DOGFOOD_ARTIFACT_BOARD_ROOT: &str = "testdata/dogfood/board";

#[derive(Debug, Clone, PartialEq, Eq)]
struct DogfoodOwnerStory {
    id: String,
    readme_path: PathBuf,
    story_dir: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DogfoodRunReport {
    pub scenario: String,
    pub workspace_root: PathBuf,
    pub artifact_board_root: PathBuf,
    pub owner_story_id: String,
    pub tape_path: PathBuf,
    pub gif_path: PathBuf,
    pub transcript_path: PathBuf,
    pub log_path: PathBuf,
    pub manifest_path: PathBuf,
}

pub fn list_scenarios(repo_root: &Path) -> Result<Vec<String>> {
    let scenario_root = scenario_root(repo_root);
    if !scenario_root.exists() {
        return Ok(Vec::new());
    }

    let mut scenarios = Vec::new();
    for entry in fs::read_dir(&scenario_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("tape") {
            continue;
        }
        if let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) {
            scenarios.push(stem.to_string());
        }
    }
    scenarios.sort();
    Ok(scenarios)
}

pub fn run_named_scenario(repo_root: &Path, scenario: &str) -> Result<DogfoodRunReport> {
    let tape_path = scenario_path(repo_root, scenario);
    if !tape_path.exists() {
        let available = list_scenarios(repo_root)?;
        let suggestions = if available.is_empty() {
            "No dogfood scenarios are authored yet.".to_string()
        } else {
            format!("Available scenarios: {}", available.join(", "))
        };
        bail!("Unknown dogfood scenario '{scenario}'. {suggestions}");
    }

    reset_workspace(repo_root)?;

    let workspace_root = dogfood_workspace::workspace_root(repo_root);
    let artifact_board_root = artifact_board_root(repo_root);
    let artifact_board_dir = artifact_board_dir(repo_root);
    let owner_story = resolve_owner_story(&artifact_board_dir, scenario)?;
    reset_story_evidence(&owner_story.story_dir)?;

    let evidence_dir = owner_story.story_dir.join("EVIDENCE");
    fs::create_dir_all(&evidence_dir)?;

    let gif_path = evidence_dir.join(format!("{scenario}.gif"));
    let transcript_path = evidence_dir.join(format!("{scenario}.transcript.txt"));
    let log_path = evidence_dir.join(format!("{scenario}.log"));
    let result = vhs::run_tape(&workspace_root, &tape_path, std::slice::from_ref(&gif_path))?;
    let transcript_body = build_transcript(scenario, &tape_path, &owner_story, repo_root)?;
    fs::write(&transcript_path, transcript_body)?;

    let log_body = format!(
        "scenario: {scenario}\nworkspace: {}\nartifact_board: {}\nowner_story: {}\ntape: {}\noutput: {}\ntranscript: {}\nexit_code: {}\n\n--- stdout ---\n{}\n\n--- stderr ---\n{}\n",
        workspace_root.display(),
        artifact_board_root.display(),
        owner_story.id,
        tape_path.display(),
        gif_path.display(),
        transcript_path.display(),
        result.exit_code,
        result.stdout,
        result.stderr,
    );
    fs::write(&log_path, log_body)?;

    if result.exit_code != 0 {
        bail!(
            "Dogfood scenario '{scenario}' failed.\nWorkspace: {}\nTape: {}\nLog: {}\n{}",
            workspace_root.display(),
            tape_path.display(),
            log_path.display(),
            result.stderr.trim()
        );
    }

    generate_manifest(&artifact_board_dir, &owner_story.id)?;
    let manifest_path = owner_story.story_dir.join("manifest.yaml");

    Ok(DogfoodRunReport {
        scenario: scenario.to_string(),
        workspace_root,
        artifact_board_root,
        owner_story_id: owner_story.id,
        tape_path,
        gif_path,
        transcript_path,
        log_path,
        manifest_path,
    })
}

pub fn scenario_root(repo_root: &Path) -> PathBuf {
    repo_root.join(DOGFOOD_SCENARIO_ROOT)
}

pub fn scenario_path(repo_root: &Path, scenario: &str) -> PathBuf {
    scenario_root(repo_root).join(format!("{scenario}.tape"))
}

pub fn artifact_board_root(repo_root: &Path) -> PathBuf {
    repo_root.join(DOGFOOD_ARTIFACT_BOARD_ROOT)
}

pub fn artifact_board_dir(repo_root: &Path) -> PathBuf {
    artifact_board_root(repo_root).join(Config::default().board_dir())
}

pub fn find_repo_root(start: &Path) -> Result<PathBuf> {
    let mut current = start
        .canonicalize()
        .with_context(|| format!("failed to resolve {}", start.display()))?;

    loop {
        if current.join("Cargo.toml").exists() && current.join("justfile").exists() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }

    bail!(
        "Could not find repository root from {} (expected Cargo.toml and justfile)",
        start.display()
    )
}

fn resolve_owner_story(artifact_board_dir: &Path, scenario: &str) -> Result<DogfoodOwnerStory> {
    let stories_dir = artifact_board_dir.join("stories");
    if !stories_dir.is_dir() {
        bail!(
            "Dogfood artifact board is missing stories at {}",
            stories_dir.display()
        );
    }

    let scenario_ref = format!("testdata/dogfood/scenarios/{scenario}.tape");
    let mut matches = Vec::new();

    for entry in fs::read_dir(&stories_dir)? {
        let entry = entry?;
        let story_dir = entry.path();
        if !story_dir.is_dir() {
            continue;
        }

        let readme_path = story_dir.join("README.md");
        if !readme_path.is_file() {
            continue;
        }

        let content = fs::read_to_string(&readme_path)?;
        if content.contains(&scenario_ref) {
            let id = story_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_string();
            matches.push(DogfoodOwnerStory {
                id,
                readme_path,
                story_dir,
            });
        }
    }

    match matches.len() {
        0 => bail!(
            "Dogfood scenario '{scenario}' has no owner story in {}. Add a story README that references {}.",
            stories_dir.display(),
            scenario_ref
        ),
        1 => Ok(matches.remove(0)),
        _ => {
            let owners = matches
                .iter()
                .map(|story| story.id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            bail!(
                "Dogfood scenario '{scenario}' is referenced by multiple owner stories in {}: {}",
                stories_dir.display(),
                owners
            )
        }
    }
}

fn reset_story_evidence(story_dir: &Path) -> Result<()> {
    let evidence_dir = story_dir.join("EVIDENCE");
    if evidence_dir.exists() {
        fs::remove_dir_all(&evidence_dir)?;
    }

    let manifest_path = story_dir.join("manifest.yaml");
    if manifest_path.exists() {
        fs::remove_file(manifest_path)?;
    }

    Ok(())
}

fn build_transcript(
    scenario: &str,
    tape_path: &Path,
    owner_story: &DogfoodOwnerStory,
    repo_root: &Path,
) -> Result<String> {
    let tape_body = fs::read_to_string(tape_path)?;
    let tape_rel = tape_path
        .strip_prefix(repo_root)
        .unwrap_or(tape_path)
        .display()
        .to_string();
    let readme_rel = owner_story
        .readme_path
        .strip_prefix(repo_root)
        .unwrap_or(&owner_story.readme_path)
        .display()
        .to_string();

    Ok(format!(
        "scenario: {scenario}\nowner_story: {}\nowner_readme: {readme_rel}\ntape: {tape_rel}\n\n--- tape ---\n{tape_body}",
        owner_story.id
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::Manifest;
    use crate::infrastructure::board_init::init_board;
    use crate::infrastructure::config::Config;
    use std::process::Command;
    use tempfile::tempdir;

    #[test]
    fn dogfood_vhs_evidence_enters_manifest() {
        let temp = tempdir().unwrap();
        init_board(temp.path(), &Config::default()).unwrap();
        dogfood_workspace::ensure_workspace(temp.path()).unwrap();
        fs::create_dir_all(scenario_root(temp.path())).unwrap();
        fs::write(
            scenario_path(temp.path(), "smoke-flow"),
            "Require bash\nSet Shell \"bash\"\nSet Width 1200\nSet Height 600\nType \"pwd\"\nEnter\nSleep 200ms\nType \"ls .keel\"\nEnter\nSleep 200ms\n",
        )
        .unwrap();
        write_owner_story(temp.path(), "DGFSMOKE01", "smoke-flow").unwrap();
        init_git_repo(temp.path());

        let primary_sentinel = temp.path().join(".keel/PRIMARY_SENTINEL.txt");
        fs::write(&primary_sentinel, "primary board untouched\n").unwrap();
        let primary_before = fs::read_to_string(&primary_sentinel).unwrap();
        fs::write(
            dogfood_workspace::board_dir(temp.path()).join("README.md"),
            "dirty workspace\n",
        )
        .unwrap();

        unsafe {
            std::env::set_var("KEEL_MOCK_VHS", "1");
        }
        let report = run_named_scenario(temp.path(), "smoke-flow").unwrap();
        unsafe {
            std::env::remove_var("KEEL_MOCK_VHS");
        }

        assert_eq!(report.scenario, "smoke-flow");
        assert_eq!(report.owner_story_id, "DGFSMOKE01");
        assert!(report.gif_path.exists());
        assert!(report.transcript_path.exists());
        assert!(report.log_path.exists());
        assert!(report.manifest_path.exists());

        let transcript = fs::read_to_string(&report.transcript_path).unwrap();
        assert!(transcript.contains("scenario: smoke-flow"));
        assert!(transcript.contains("testdata/dogfood/scenarios/smoke-flow.tape"));

        let log = fs::read_to_string(&report.log_path).unwrap();
        assert!(log.contains("scenario: smoke-flow"));
        assert!(log.contains("owner_story: DGFSMOKE01"));
        assert!(log.contains("exit_code: 0"));

        let manifest: Manifest =
            serde_yaml::from_str(&fs::read_to_string(&report.manifest_path).unwrap()).unwrap();
        assert_eq!(manifest.id, "DGFSMOKE01");
        assert!(manifest.evidence.contains_key("EVIDENCE/smoke-flow.gif"));
        assert!(
            manifest
                .evidence
                .contains_key("EVIDENCE/smoke-flow.transcript.txt")
        );
        assert!(manifest.evidence.contains_key("EVIDENCE/smoke-flow.log"));

        let workspace_readme =
            fs::read_to_string(dogfood_workspace::board_dir(temp.path()).join("README.md"))
                .unwrap();
        assert_ne!(workspace_readme, "dirty workspace\n");
        assert!(!workspace_readme.trim().is_empty());

        let primary_after = fs::read_to_string(&primary_sentinel).unwrap();
        assert_eq!(primary_before, primary_after);
    }

    #[test]
    fn dogfood_runner_reports_unknown_scenarios() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(scenario_root(temp.path())).unwrap();
        fs::write(scenario_path(temp.path(), "smoke-flow"), "Require bash\n").unwrap();

        let err = run_named_scenario(temp.path(), "missing-flow")
            .unwrap_err()
            .to_string();

        assert!(err.contains("Unknown dogfood scenario"));
        assert!(err.contains("smoke-flow"));
    }

    #[test]
    fn dogfood_runner_reports_missing_owner_story() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(scenario_root(temp.path())).unwrap();
        fs::write(scenario_path(temp.path(), "smoke-flow"), "Require bash\n").unwrap();
        init_board(&artifact_board_root(temp.path()), &Config::default()).unwrap();

        let err = run_named_scenario(temp.path(), "smoke-flow")
            .unwrap_err()
            .to_string();

        assert!(err.contains("has no owner story"));
        assert!(err.contains("testdata/dogfood/scenarios/smoke-flow.tape"));
    }

    #[test]
    fn dogfood_runner_is_opt_in_and_not_wired_into_default_checks() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let justfile = fs::read_to_string(repo_root.join("justfile")).unwrap();
        let ci_workflow = fs::read_to_string(repo_root.join(".github/workflows/ci.yml")).unwrap();

        assert!(justfile.contains("e2e-vhs"));
        assert!(justfile.contains("pre-commit: quality test"));
        assert!(!ci_workflow.contains("e2e-vhs"));
    }

    #[test]
    fn checked_in_dogfood_scenarios_include_epic_flow() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let scenarios = list_scenarios(&repo_root).unwrap();

        assert!(scenarios.contains(&"bearing-flow".to_string()));
        assert!(scenarios.contains(&"epic-flow".to_string()));
        assert!(scenarios.contains(&"smoke-flow".to_string()));
    }

    #[test]
    fn checked_in_dogfood_artifact_board_owns_all_scenarios() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        for scenario in ["bearing-flow", "epic-flow", "smoke-flow"] {
            let owner = resolve_owner_story(&artifact_board_dir(&repo_root), scenario).unwrap();
            assert!(
                owner.readme_path.exists(),
                "expected checked-in owner story for {scenario}"
            );
        }
    }

    #[test]
    fn epic_flow_tape_covers_creation_and_decomposition() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tape = fs::read_to_string(scenario_path(&repo_root, "epic-flow")).unwrap();

        for snippet in [
            "keel epic new",
            "keel epic show $EPIC_ID",
            "keel voyage new",
            "keel story new",
            "keel story link",
            "keel voyage plan",
            "keel voyage show $VOYAGE_ID",
        ] {
            assert!(
                tape.contains(snippet),
                "expected epic-flow tape to contain {snippet}"
            );
        }
    }

    #[test]
    fn epic_flow_tape_surfaces_next_and_flow() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tape = fs::read_to_string(scenario_path(&repo_root, "epic-flow")).unwrap();

        assert!(tape.contains("keel next --agent"));
        assert!(tape.contains("keel flow"));
        assert!(tape.contains("SRS-03"));
    }

    #[test]
    fn epic_flow_tape_avoids_fixed_entity_ids() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tape = fs::read_to_string(scenario_path(&repo_root, "epic-flow")).unwrap();

        assert!(tape.contains("latest_id .keel/epics"));
        assert!(tape.contains("latest_id .keel/epics/$EPIC_ID/voyages"));
        assert!(tape.contains("latest_id .keel/stories"));
        assert!(tape.contains("sleep 1"));
        assert!(!tape.contains("1vyWLl000"));
    }

    #[test]
    fn bearing_flow_tape_covers_research_lifecycle() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tape = fs::read_to_string(scenario_path(&repo_root, "bearing-flow")).unwrap();

        for snippet in [
            "keel bearing new",
            "keel bearing survey $BEARING_ID",
            "keel bearing assess $BEARING_ID",
            "keel bearing lay $BEARING_ID",
        ] {
            assert!(
                tape.contains(snippet),
                "expected bearing-flow tape to contain {snippet}"
            );
        }
    }

    #[test]
    fn bearing_flow_tape_avoids_fixed_entity_ids() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let tape = fs::read_to_string(scenario_path(&repo_root, "bearing-flow")).unwrap();

        assert!(tape.contains("latest_id .keel/bearings"));
        assert!(tape.contains("sleep 1"));
        assert!(!tape.contains("1vyWLl000"));
    }

    fn write_owner_story(repo_root: &Path, story_id: &str, scenario: &str) -> Result<()> {
        init_board(&artifact_board_root(repo_root), &Config::default())?;
        let story_dir = artifact_board_dir(repo_root).join("stories").join(story_id);
        fs::create_dir_all(&story_dir)?;
        fs::write(
            story_dir.join("README.md"),
            format!(
                r#"---
id: {story_id}
title: Capture {scenario} Evidence
type: chore
status: backlog
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

# Capture {scenario} Evidence

Tape source: `testdata/dogfood/scenarios/{scenario}.tape`
"#
            ),
        )?;
        Ok(())
    }

    fn init_git_repo(repo_root: &Path) {
        assert!(
            Command::new("git")
                .args(["init"])
                .current_dir(repo_root)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            Command::new("git")
                .args(["config", "user.email", "dogfood@example.com"])
                .current_dir(repo_root)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            Command::new("git")
                .args(["config", "user.name", "Dogfood Runner"])
                .current_dir(repo_root)
                .status()
                .unwrap()
                .success()
        );
        assert!(
            Command::new("git")
                .args(["commit", "--allow-empty", "-m", "init"])
                .current_dir(repo_root)
                .status()
                .unwrap()
                .success()
        );
    }
}
