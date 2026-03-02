//! Configuration commands

use anyhow::{Result, bail};
use clap::Subcommand;

use crate::infrastructure::config::{self, Config};

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

    println!("Configuration source: {}", source);
    println!();
    println!("board_dir = \"{}\"", config.board_dir());
    println!();
    println!("[scoring]");
    println!("mode = \"{}\"", config.scoring.mode);
    println!();

    let weights = config.current_weights();
    println!("# Current mode weights:");
    println!("impact_weight = {}", weights.impact_weight);
    println!("confidence_weight = {}", weights.confidence_weight);
    println!("effort_weight = {}", weights.effort_weight);
    println!("risk_weight = {}", weights.risk_weight);

    if !config.scoring.modes.is_empty() {
        println!();
        println!("# Custom modes defined:");
        for name in config.scoring.modes.keys() {
            println!("  - {}", name);
        }
    }

    Ok(())
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
}
