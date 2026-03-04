//! Configuration commands

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use clap::Subcommand;

use crate::infrastructure::config::{self, Config};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification::parser::parse_verify_annotations;
use crate::read_model::verification_techniques::{
    self, ProjectStack, TechniqueDefinition, TechniqueModality,
};

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Show resolved configuration and source
    Show,
    /// Show or change scoring mode
    Mode {
        /// Mode to switch to (omit to show current mode)
        name: Option<String>,
    },
}

/// Show resolved configuration and source
pub fn run_show() -> Result<()> {
    let (config, source) = config::load_config();
    let project_root = resolve_project_root(&config);
    for line in build_show_lines(&config, &source, &project_root) {
        println!("{}", line);
    }

    Ok(())
}

fn build_show_lines(
    config: &Config,
    source: &config::ConfigSource,
    project_root: &Path,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("Configuration source: {}", source));
    lines.push(format!("project_root = \"{}\"", project_root.display()));
    lines.push(String::new());
    lines.push(format!("board_dir = \"{}\"", config.board_dir()));
    lines.push(String::new());
    lines.push("[scoring]".to_string());
    lines.push(format!("mode = \"{}\"", config.scoring.mode));
    lines.push(String::new());

    let weights = config.current_weights();
    lines.push("# Current mode weights:".to_string());
    lines.push(format!("impact_weight = {}", weights.impact_weight));
    lines.push(format!("confidence_weight = {}", weights.confidence_weight));
    lines.push(format!("effort_weight = {}", weights.effort_weight));
    lines.push(format!("risk_weight = {}", weights.risk_weight));

    if !config.scoring.modes.is_empty() {
        lines.push(String::new());
        lines.push("# Custom modes defined:".to_string());
        for name in config.scoring.modes.keys() {
            lines.push(format!("  - {}", name));
        }
    }

    lines.extend(build_verification_technique_lines(config, project_root));
    lines
}

fn build_verification_technique_lines(config: &Config, project_root: &Path) -> Vec<String> {
    let mut lines = Vec::new();
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
    let found_report =
        verification_techniques::build_show_recommendation_report(project_root, &used_techniques);

    let mut active: Vec<&TechniqueDefinition> = merged
        .catalog
        .iter()
        .filter(|technique| technique.enabled_by_default)
        .collect();
    active.sort_by(|left, right| left.id.cmp(&right.id));

    let mut disabled: Vec<&TechniqueDefinition> = merged
        .catalog
        .iter()
        .filter(|technique| !technique.enabled_by_default)
        .collect();
    disabled.sort_by(|left, right| left.id.cmp(&right.id));

    lines.push(String::new());
    lines.push("[verification.techniques]".to_string());
    lines.push(format!("available = {}", merged.catalog.len()));
    lines.push(format!("active = {}", active.len()));
    lines.push(format!("disabled = {}", disabled.len()));

    lines.push(String::new());
    lines.push("# Active configured options:".to_string());
    if active.is_empty() {
        lines.push("  (none)".to_string());
    } else {
        for technique in active {
            lines.push(format!(
                "  - {} [{}] ({})",
                technique.label,
                technique.id,
                modality_name(technique.modality)
            ));
            lines.push(format!("    Command: {}", technique.default_command));
        }
    }

    if !disabled.is_empty() {
        lines.push(String::new());
        lines.push("# Disabled options:".to_string());
        for technique in disabled {
            lines.push(format!("  - {} [{}]", technique.label, technique.id));
        }
    }

    lines.push(String::new());
    lines.push("# Found project signals:".to_string());
    if signals.stack_confidence.is_empty()
        && signals.detected_files.is_empty()
        && signals.hints.is_empty()
    {
        lines.push("  (none detected)".to_string());
    } else {
        if !signals.stack_confidence.is_empty() {
            let mut stack_confidences: Vec<(String, f64)> = signals
                .stack_confidence
                .iter()
                .map(|(stack, confidence)| (stack_name(*stack).to_string(), *confidence))
                .collect();
            stack_confidences.sort_by(|left, right| left.0.cmp(&right.0));
            for (stack, confidence) in stack_confidences {
                lines.push(format!("  - stack:{stack} confidence:{confidence:.2}"));
            }
        }
        if !signals.detected_files.is_empty() {
            lines.push(format!("  - files: {}", signals.detected_files.join(", ")));
        }
        if !signals.hints.is_empty() {
            let hints = signals.hints.iter().cloned().collect::<Vec<_>>().join(", ");
            lines.push(format!("  - hints: {}", hints));
        }
    }

    lines.push(String::new());
    lines.push("# Found options for this project (ranked):".to_string());
    if found_report.recommendations.is_empty() {
        lines.push("  (no recommendations available)".to_string());
    } else {
        for recommendation in &found_report.recommendations {
            lines.push(format!(
                "  - {} [{}] ({})",
                recommendation.label, recommendation.id, recommendation.usage_status
            ));
            lines.push(format!("    Rationale: {}", recommendation.rationale));
            lines.push(format!("    {}", recommendation.adoption_guidance));
        }
    }

    if !found_report.diagnostics.is_empty() {
        lines.push(String::new());
        lines.push("# Config diagnostics:".to_string());
        for diagnostic in &found_report.diagnostics {
            lines.push(format!("  - {}", diagnostic));
        }
    }

    lines
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

fn stack_name(stack: ProjectStack) -> &'static str {
    match stack {
        ProjectStack::Rust => "rust",
        ProjectStack::Browser => "browser",
        ProjectStack::Cli => "cli",
    }
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
    fn config_show_surfaces_active_and_found_verification_options() {
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
        let lines = build_show_lines(&config, &config::ConfigSource::Defaults, temp.path());
        let rendered = lines.join("\n");

        assert!(rendered.contains("[verification.techniques]"));
        assert!(rendered.contains("# Active configured options:"));
        assert!(rendered.contains("# Found options for this project (ranked):"));
        assert!(rendered.contains("Rust Unit/Integration Tests [rust-unit-tests]"));
        assert!(rendered.contains("VHS CLI Recording [vhs]"));
        assert!(rendered.contains("Rust Coverage Gate [rust-coverage]"));
    }

    #[test]
    fn config_show_marks_configured_in_use_techniques() {
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
        let lines = build_show_lines(&config, &config::ConfigSource::Defaults, temp.path());
        let rendered = lines.join("\n");

        assert!(rendered.contains("LLM-Judge [llm-judge] (configured-in-use)"));
    }
}
