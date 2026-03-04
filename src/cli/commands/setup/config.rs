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
use crate::read_model::verification_techniques::{self, TechniqueDefinition, TechniqueModality};

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
    summary: TechniqueStatusSummary,
    rows: Vec<TechniqueStatusRow>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TechniqueStatusSummary {
    total: usize,
    detected: usize,
    disabled: usize,
    active: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct TechniqueStatusRow {
    label: String,
    name: String,
    detected: bool,
    disabled: bool,
    active: bool,
    modality: String,
    command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ConfigShowVerificationPayload {
    summary: TechniqueStatusSummary,
    techniques: Vec<TechniqueStatusRow>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ConfigShowPayload {
    source: String,
    project_root: String,
    board_dir: String,
    verification: ConfigShowVerificationPayload,
}

fn build_show_payload(
    config: &Config,
    source: &config::ConfigSource,
    project_root: &Path,
) -> ConfigShowPayload {
    let projection = build_verification_technique_projection(config, project_root);

    ConfigShowPayload {
        source: source.to_string(),
        project_root: project_root.display().to_string(),
        board_dir: config.board_dir().to_string(),
        verification: ConfigShowVerificationPayload {
            summary: projection.summary,
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
    lines.push("[verification.techniques]".to_string());
    lines.push(format!("total = {}", payload.verification.summary.total));
    lines.push(format!(
        "detected = {}",
        payload.verification.summary.detected
    ));
    lines.push(format!(
        "disabled = {}",
        payload.verification.summary.disabled
    ));
    lines.push(format!("active = {}", payload.verification.summary.active));
    lines.push(String::new());

    lines.push("# Technique status:".to_string());
    if payload.verification.techniques.is_empty() {
        lines.push("  (none)".to_string());
    } else {
        for technique in &payload.verification.techniques {
            lines.push(format!("  - label = \"{}\"", technique.label));
            lines.push(format!("    name = \"{}\"", technique.name));
            lines.push(format!("    detected = {}", technique.detected));
            lines.push(format!("    disabled = {}", technique.disabled));
            lines.push(format!("    active = {}", technique.active));
            lines.push(format!("    modality = \"{}\"", technique.modality));
            lines.push(format!("    command = \"{}\"", technique.command));
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
    let keel_toml_path = project_root.join("keel.toml");
    let keel_toml_content = fs::read_to_string(&keel_toml_path).unwrap_or_default();
    let parsed =
        verification_techniques::parse_technique_overrides_from_keel_toml(&keel_toml_content);
    let merged = verification_techniques::merge_technique_catalog_with_overrides(
        verification_techniques::builtin_technique_catalog(),
        &parsed.overrides,
    );
    let signals = verification_techniques::detect_project_signals(project_root);
    let used_techniques = collect_used_techniques(config, project_root);
    let mut disabled: Vec<&TechniqueDefinition> = merged
        .catalog
        .iter()
        .filter(|technique| !technique.enabled_by_default)
        .collect();
    disabled.sort_by(|left, right| left.id.cmp(&right.id));

    let mut rows: Vec<TechniqueStatusRow> = merged
        .catalog
        .iter()
        .map(|technique| {
            let detected = technique_is_detected(technique, &signals, &used_techniques);
            let disabled = !technique.enabled_by_default;
            let active = detected && !disabled;
            TechniqueStatusRow {
                label: technique.id.clone(),
                name: technique.label.clone(),
                detected,
                disabled,
                active,
                modality: modality_name(technique.modality).to_string(),
                command: technique.default_command.clone(),
            }
        })
        .collect();
    rows.sort_by(|left, right| left.label.cmp(&right.label));

    let summary = TechniqueStatusSummary {
        total: rows.len(),
        detected: rows.iter().filter(|row| row.detected).count(),
        disabled: disabled.len(),
        active: rows.iter().filter(|row| row.active).count(),
    };

    let mut diagnostics = Vec::new();
    diagnostics.extend(
        parsed
            .diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message)),
    );
    diagnostics.extend(
        merged
            .diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message)),
    );
    diagnostics.sort();
    diagnostics.dedup();

    TechniqueStatusProjection {
        summary,
        rows,
        diagnostics,
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

fn technique_is_detected(
    technique: &TechniqueDefinition,
    signals: &verification_techniques::ProjectSignalReport,
    used_techniques: &BTreeSet<String>,
) -> bool {
    if used_techniques.contains(&technique.id) {
        return true;
    }

    if technique.applicable_stacks.iter().any(|stack| {
        signals
            .stack_confidence
            .get(stack)
            .copied()
            .unwrap_or_default()
            > 0.0
    }) {
        return true;
    }

    if technique
        .signal_keywords
        .iter()
        .any(|keyword| signals.hints.contains(&keyword.to_ascii_lowercase()))
    {
        return true;
    }

    technique
        .prerequisites
        .iter()
        .any(|requirement| signals.hints.contains(&requirement.to_ascii_lowercase()))
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

fn modality_name(modality: TechniqueModality) -> &'static str {
    match modality {
        TechniqueModality::Command => "command",
        TechniqueModality::Recording => "recording",
        TechniqueModality::Judge => "judge",
    }
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

        assert!(rendered.contains("[verification.techniques]"));
        assert!(!rendered.contains("[scoring]"));
        assert!(!rendered.contains("impact_weight"));
        assert!(rendered.contains("# Technique status:"));
        assert!(rendered.contains("label = \"rust-unit-tests\""));
        assert!(rendered.contains("label = \"vhs\""));
        assert!(rendered.contains("label = \"rust-coverage\""));
        assert!(rendered.contains("disabled = true"));
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
        assert!(json["verification"]["summary"]["total"].as_u64().unwrap() >= 1);
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
    }
}
