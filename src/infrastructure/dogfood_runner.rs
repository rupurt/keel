use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use crate::infrastructure::dogfood_workspace::{self, reset_workspace};
use crate::infrastructure::vhs;

pub const DOGFOOD_SCENARIO_ROOT: &str = "testdata/dogfood/scenarios";
pub const DOGFOOD_OUTPUT_ROOT: &str = "testdata/dogfood/output";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DogfoodRunReport {
    pub scenario: String,
    pub workspace_root: PathBuf,
    pub tape_path: PathBuf,
    pub gif_path: PathBuf,
    pub log_path: PathBuf,
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
    let output_dir = output_root(repo_root).join(scenario);
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }
    fs::create_dir_all(&output_dir)?;

    let gif_path = output_dir.join(format!("{scenario}.gif"));
    let log_path = output_dir.join(format!("{scenario}.log"));
    let result = vhs::run_tape(&workspace_root, &tape_path, std::slice::from_ref(&gif_path))?;

    let log_body = format!(
        "scenario: {scenario}\nworkspace: {}\ntape: {}\noutput: {}\nexit_code: {}\n\n--- stdout ---\n{}\n\n--- stderr ---\n{}\n",
        workspace_root.display(),
        tape_path.display(),
        gif_path.display(),
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

    Ok(DogfoodRunReport {
        scenario: scenario.to_string(),
        workspace_root,
        tape_path,
        gif_path,
        log_path,
    })
}

pub fn scenario_root(repo_root: &Path) -> PathBuf {
    repo_root.join(DOGFOOD_SCENARIO_ROOT)
}

pub fn scenario_path(repo_root: &Path, scenario: &str) -> PathBuf {
    scenario_root(repo_root).join(format!("{scenario}.tape"))
}

pub fn output_root(repo_root: &Path) -> PathBuf {
    repo_root.join(DOGFOOD_OUTPUT_ROOT)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::board_init::init_board;
    use crate::infrastructure::config::Config;
    use tempfile::tempdir;

    #[test]
    fn dogfood_runner_executes_named_scenarios() {
        let temp = tempdir().unwrap();
        init_board(temp.path(), &Config::default()).unwrap();
        dogfood_workspace::ensure_workspace(temp.path()).unwrap();
        fs::create_dir_all(scenario_root(temp.path())).unwrap();
        fs::write(
            scenario_path(temp.path(), "smoke-flow"),
            "Require bash\nSet Shell \"bash\"\nSet Width 1200\nSet Height 600\nType \"pwd\"\nEnter\nSleep 200ms\nType \"ls .keel\"\nEnter\nSleep 200ms\n",
        )
        .unwrap();

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
        assert!(report.gif_path.exists());
        assert!(report.log_path.exists());
        let log = fs::read_to_string(&report.log_path).unwrap();
        assert!(log.contains("scenario: smoke-flow"));
        assert!(log.contains("exit_code: 0"));

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
}
