//! Configuration commands

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use clap::Subcommand;
use serde::Serialize;

use crate::infrastructure::config::{self, Config};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification::parser::parse_verify_annotations;
use crate::read_model::verification_techniques;

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show resolved configuration and source
    Show {
        /// Output as JSON for scripting
        json: bool,
    },
    /// Show or change scoring mode
    Mode {
        /// Mode to switch to (omit to show current mode)
        name: Option<String>,
    },
}

/// Show resolved configuration and source
pub fn run_show(json: bool) -> Result<()> {
    let (config, source) = config::load_config();
    let project_root = resolve_project_root(&config);
    let payload = build_show_payload(&config, &source, &project_root);

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        for line in render_show_payload(&payload) {
            println!("{}", line);
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TechniqueStatusProjection {
    rows: Vec<TechniqueStatusRow>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TechniqueStatusRow {
    label: String,
    detected: bool,
    disabled: bool,
    active: bool,
    command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ConfigShowVerificationPayload {
    techniques: Vec<TechniqueStatusRow>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ConfigShowScoringPayload {
    mode: String,
    impact_weight: f64,
    confidence_weight: f64,
    effort_weight: f64,
    risk_weight: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ConfigShowPayload {
    source: String,
    project_root: String,
    board_dir: String,
    scoring: ConfigShowScoringPayload,
    verification: ConfigShowVerificationPayload,
}

fn build_show_payload(
    config: &Config,
    source: &config::ConfigSource,
    project_root: &Path,
) -> ConfigShowPayload {
    let projection = build_verification_technique_projection(config, project_root);
    let weights = config.current_weights();

    ConfigShowPayload {
        source: source.to_string(),
        project_root: project_root.display().to_string(),
        board_dir: config.board_dir().to_string(),
        scoring: ConfigShowScoringPayload {
            mode: config.mode().to_string(),
            impact_weight: weights.impact_weight,
            confidence_weight: weights.confidence_weight,
            effort_weight: weights.effort_weight,
            risk_weight: weights.risk_weight,
        },
        verification: ConfigShowVerificationPayload {
            techniques: projection.rows,
            diagnostics: projection.diagnostics,
        },
    }
}

fn render_show_payload(payload: &ConfigShowPayload) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Configuration source: {}", payload.source));
    lines.push(format!("project_root = \"{}\"", payload.project_root));
    lines.push(String::new());
    lines.push(format!("board_dir = \"{}\"", payload.board_dir));
    lines.push(String::new());
    lines.push("[scoring]".to_string());
    lines.push(format!("mode = \"{}\"", payload.scoring.mode));
    lines.push(format!("impact_weight = {}", payload.scoring.impact_weight));
    lines.push(format!(
        "confidence_weight = {}",
        payload.scoring.confidence_weight
    ));
    lines.push(format!("effort_weight = {}", payload.scoring.effort_weight));
    lines.push(format!("risk_weight = {}", payload.scoring.risk_weight));
    lines.push(String::new());

    lines.push(
        "# Verification technique status (`disabled` and `command` are configurable):".to_string(),
    );
    if payload.verification.techniques.is_empty() {
        lines.push("  (none)".to_string());
    } else {
        for technique in &payload.verification.techniques {
            lines.push(format!("[verification.{}]", technique.label));
            lines.push(format!("detected = {}", technique.detected));
            lines.push(format!("disabled = {}", technique.disabled));
            lines.push(format!("active = {}", technique.active));
            lines.push(format!("command = \"{}\"", technique.command));
            lines.push(String::new());
        }
    }

    if !payload.verification.diagnostics.is_empty() {
        lines.push(String::new());
        lines.push("# Config diagnostics:".to_string());
        for diagnostic in &payload.verification.diagnostics {
            lines.push(format!("  - {}", diagnostic));
        }
    }

    lines
}

fn build_verification_technique_projection(
    config: &Config,
    project_root: &Path,
) -> TechniqueStatusProjection {
    let used_techniques = collect_used_techniques(config, project_root);
    let report =
        verification_techniques::resolve_technique_status_report(project_root, &used_techniques);

    let rows: Vec<TechniqueStatusRow> = report
        .techniques
        .iter()
        .map(|technique| TechniqueStatusRow {
            label: technique.id.clone(),
            detected: technique.detected,
            disabled: technique.disabled,
            active: technique.active,
            command: technique.default_command.clone(),
        })
        .collect();

    TechniqueStatusProjection {
        rows,
        diagnostics: report.diagnostics,
    }
}

fn collect_used_techniques(config: &Config, project_root: &Path) -> BTreeSet<String> {
    let mut used = BTreeSet::new();
    let board_dir = project_root.join(config.board_dir());
    if !board_dir.exists() {
        return used;
    }

    let Ok(board) = load_board(&board_dir) else {
        return used;
    };

    for story in board.stories.values() {
        let Ok(content) = fs::read_to_string(&story.path) else {
            continue;
        };
        for annotation in parse_verify_annotations(&content) {
            if let Some(command) = annotation.command {
                for technique_id in verification_techniques::infer_used_technique_ids(&command) {
                    used.insert(technique_id);
                }
            }
        }
    }

    used
}

fn resolve_project_root(config: &Config) -> PathBuf {
    if let Ok(board_dir) = config::find_board_dir() {
        if config.board_dir() == "." {
            return board_dir;
        }
        return board_dir.parent().unwrap_or(&board_dir).to_path_buf();
    }

    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Show or change scoring mode
pub fn run_mode(name: Option<String>) -> Result<()> {
    let (mut config, source) = config::load_config();

    match name {
        None => {
            // Show current mode
            println!("Current mode: {}", config.scoring.mode);
            println!("Source: {}", source);
            println!();
            println!("Available modes:");
            for mode in Config::builtin_modes() {
                let marker = if mode == config.scoring.mode {
                    " (active)"
                } else {
                    ""
                };
                println!("  - {}{}", mode, marker);
            }
            // Show custom modes if any
            for mode in config.scoring.modes.keys() {
                let marker = if mode == &config.scoring.mode {
                    " (active)"
                } else {
                    ""
                };
                println!("  - {}{} [custom]", mode, marker);
            }
        }
        Some(new_mode) => {
            // Validate mode
            if !config.is_valid_mode(&new_mode) {
                let available: Vec<_> = Config::builtin_modes()
                    .into_iter()
                    .chain(config.scoring.modes.keys().map(|s| s.as_str()))
                    .collect();
                bail!(
                    "Unknown mode '{}'. Available modes: {}",
                    new_mode,
                    available.join(", ")
                );
            }

            let old_mode = config.scoring.mode.clone();
            config.set_mode(&new_mode);

            // Save to local keel.toml
            let path = config::save_config(&config)?;
            println!("Mode changed: {} → {}", old_mode, new_mode);
            println!("Saved to: {}", path.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use std::fs;
    use tempfile::TempDir;

    // Note: Tests that change current directory are tested via integration tests
    // to avoid parallel test conflicts. Here we test the logic without side effects.

    #[test]
    fn builtin_modes_are_valid() {
        let config = Config::default();
        assert!(config.is_valid_mode("constrained"));
        assert!(config.is_valid_mode("growth"));
        assert!(config.is_valid_mode("product"));
        assert!(!config.is_valid_mode("invalid"));
    }

    #[test]
    fn mode_validation_includes_custom() {
        let mut config = Config::default();
        config
            .scoring
            .modes
            .insert("custom".to_string(), config::ModeWeights::constrained());
        assert!(config.is_valid_mode("custom"));
    }

    #[test]
    fn config_show_renders_technique_flag_matrix() {
        let temp = TempDir::new().unwrap();
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(
            temp.path().join("keel.toml"),
            r#"[verification.techniques]
disable = ["rust-coverage"]
"#,
        )
        .unwrap();
        fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname=\"demo\"\nversion=\"0.1.0\"\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("flake.nix"),
            "buildInputs = [ pkgs.vhs pkgs.ffmpeg ];",
        )
        .unwrap();

        let config = Config::default();
        let payload = build_show_payload(&config, &config::ConfigSource::Defaults, temp.path());
        let lines = render_show_payload(&payload);
        let rendered = lines.join("\n");

        assert!(rendered.contains("[scoring]"));
        assert!(rendered.contains("mode = \"constrained\""));
        assert!(rendered.contains("impact_weight = 1"));
        assert!(rendered.contains("confidence_weight = 1.5"));
        assert!(rendered.contains("effort_weight = 2"));
        assert!(rendered.contains("risk_weight = 1.5"));
        assert!(!rendered.contains("total = "));
        assert!(!rendered.contains("[verification.techniques]"));
        assert!(rendered.contains(
            "# Verification technique status (`disabled` and `command` are configurable):"
        ));
        assert!(rendered.contains("[verification.rust-unit-tests]"));
        assert!(rendered.contains("[verification.vhs]"));
        assert!(rendered.contains("[verification.rust-coverage]"));
        assert!(rendered.contains("command = \"cargo test\""));
        assert!(rendered.contains("disabled = true"));
        assert!(!rendered.contains("label = \""));
        assert!(!rendered.contains("name = \""));
        assert!(!rendered.contains("modality = \""));
    }

    #[test]
    fn config_show_lists_all_techniques_deterministically() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Done)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] judge <!-- verify: llm-judge, SRS-01:start:end -->",
                    ),
            )
            .build();
        fs::write(
            temp.path().join("keel.toml"),
            r#"board_dir = "."

[verification.techniques]
enable = ["llm-judge"]
"#,
        )
        .unwrap();

        let config = Config {
            board_dir: ".".to_string(),
            ..Config::default()
        };
        let payload = build_show_payload(&config, &config::ConfigSource::Defaults, temp.path());
        let labels: Vec<&str> = payload
            .verification
            .techniques
            .iter()
            .map(|row| row.label.as_str())
            .collect();
        let mut sorted = labels.clone();
        sorted.sort_unstable();
        assert_eq!(labels, sorted);

        assert!(
            payload
                .verification
                .techniques
                .iter()
                .any(|row| row.label == "llm-judge" && row.detected && row.active)
        );
        assert!(
            payload
                .verification
                .techniques
                .iter()
                .any(|row| row.label == "browser-playwright-e2e" && !row.detected)
        );
    }

    #[test]
    fn config_show_json_contract_contains_required_flags() {
        let temp = TempDir::new().unwrap();
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname=\"demo\"\nversion=\"0.1.0\"\n",
        )
        .unwrap();
        let config = Config::default();
        let payload = build_show_payload(&config, &config::ConfigSource::Defaults, temp.path());
        let json = serde_json::to_value(payload).unwrap();

        assert_eq!(
            json["source"],
            serde_json::Value::String(config::ConfigSource::Defaults.to_string())
        );
        assert_eq!(
            json["scoring"]["mode"],
            serde_json::Value::String("constrained".to_string())
        );
        assert!(json["scoring"]["impact_weight"].as_f64().unwrap() > 0.0);
        assert!(json["verification"]["summary"].is_null());
        let techniques = json["verification"]["techniques"]
            .as_array()
            .unwrap()
            .clone();
        assert!(!techniques.is_empty());
        let first = &techniques[0];
        assert!(first.get("label").is_some());
        assert!(first.get("detected").is_some());
        assert!(first.get("disabled").is_some());
        assert!(first.get("active").is_some());
        assert!(first.get("command").is_some());
        assert!(first.get("name").is_none());
        assert!(first.get("modality").is_none());
    }
}
