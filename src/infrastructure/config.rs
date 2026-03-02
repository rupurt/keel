//! Keel configuration system
//!
//! Provides layered configuration with resolution order:
//! 1. `./keel.toml` (project-local)
//! 2. `~/.config/keel.toml` (user global)
//! 3. Built-in defaults

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Source of the resolved configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    /// Project-local keel.toml
    Local(PathBuf),
    /// User global ~/.config/keel.toml
    User(PathBuf),
    /// Built-in defaults
    Defaults,
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigSource::Local(path) => write!(f, "{}", path.display()),
            ConfigSource::User(path) => write!(f, "{}", path.display()),
            ConfigSource::Defaults => write!(f, "(built-in defaults)"),
        }
    }
}

/// Scoring mode weights
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModeWeights {
    pub impact_weight: f64,
    pub confidence_weight: f64,
    pub effort_weight: f64,
    pub risk_weight: f64,
}

impl Default for ModeWeights {
    fn default() -> Self {
        // Constrained mode is the default
        Self::constrained()
    }
}

impl ModeWeights {
    /// Constrained mode: Limited capital, survival focus
    pub fn constrained() -> Self {
        Self {
            impact_weight: 1.0,
            confidence_weight: 1.5,
            effort_weight: 2.0,
            risk_weight: 1.5,
        }
    }

    /// Growth mode: Capital available, scaling
    pub fn growth() -> Self {
        Self {
            impact_weight: 2.0,
            confidence_weight: 1.5,
            effort_weight: 1.0,
            risk_weight: 1.0,
        }
    }

    /// Product mode: Existing product ROI
    pub fn product() -> Self {
        Self {
            impact_weight: 1.5,
            confidence_weight: 1.0,
            effort_weight: 1.5,
            risk_weight: 1.0,
        }
    }
}

/// Scoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScoringConfig {
    /// Current scoring mode
    #[serde(default = "default_mode")]
    pub mode: String,
    /// Custom mode definitions (optional)
    #[serde(default)]
    pub modes: std::collections::HashMap<String, ModeWeights>,
}

fn default_mode() -> String {
    "constrained".to_string()
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            modes: std::collections::HashMap::new(),
        }
    }
}

/// Default board directory
fn default_board_dir() -> String {
    ".keel".to_string()
}

/// Keel configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Board directory path (relative to project root)
    /// Defaults to ".keel" if not specified
    #[serde(default = "default_board_dir")]
    pub board_dir: String,

    /// Scoring configuration
    #[serde(default)]
    pub scoring: ScoringConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            board_dir: default_board_dir(),
            scoring: ScoringConfig::default(),
        }
    }
}

impl Config {
    /// Get the weights for the current mode
    pub fn current_weights(&self) -> ModeWeights {
        // First check custom modes
        if let Some(weights) = self.scoring.modes.get(&self.scoring.mode) {
            return weights.clone();
        }

        // Fall back to built-in modes
        match self.scoring.mode.as_str() {
            "constrained" => ModeWeights::constrained(),
            "growth" => ModeWeights::growth(),
            "product" => ModeWeights::product(),
            _ => ModeWeights::constrained(), // Default to constrained for unknown modes
        }
    }

    /// Get the current mode name
    pub fn mode(&self) -> &str {
        &self.scoring.mode
    }

    /// Set the mode
    pub fn set_mode(&mut self, mode: &str) {
        self.scoring.mode = mode.to_string();
    }

    /// Available built-in modes
    pub fn builtin_modes() -> Vec<&'static str> {
        vec!["constrained", "growth", "product"]
    }

    /// Check if a mode is valid (built-in or custom)
    pub fn is_valid_mode(&self, mode: &str) -> bool {
        Self::builtin_modes().contains(&mode) || self.scoring.modes.contains_key(mode)
    }

    /// Get the configured board directory
    pub fn board_dir(&self) -> &str {
        &self.board_dir
    }
}

/// Find the board directory by searching upwards for the configured path
///
/// Uses the board_dir from config (defaulting to ".keel") and searches
/// upward from the current directory until it finds a matching directory.
pub fn find_board_dir() -> Result<PathBuf> {
    let (config, _source) = load_config();
    let board_dir_name = config.board_dir();

    let mut current = std::env::current_dir().context("Failed to get current directory")?;

    loop {
        let board_dir = current.join(board_dir_name);
        if board_dir.exists() {
            return Ok(board_dir);
        }

        if !current.pop() {
            return Err(anyhow::anyhow!(
                "Could not find '{}' directory. Are you in a keel project?\n\
                 Configure the board directory in keel.toml with: board_dir = \"path/to/board\"",
                board_dir_name
            ));
        }
    }
}

/// Load configuration with layered resolution
pub fn load_config() -> (Config, ConfigSource) {
    // Try project-local first
    let local_path = PathBuf::from("keel.toml");
    if let Some(config) = local_path
        .exists()
        .then(|| load_from_file(&local_path).ok())
        .flatten()
    {
        return (config, ConfigSource::Local(local_path));
    }

    // Try user global
    if let Some(config_dir) = dirs::config_dir() {
        let user_path = config_dir.join("keel.toml");
        if let Some(config) = user_path
            .exists()
            .then(|| load_from_file(&user_path).ok())
            .flatten()
        {
            return (config, ConfigSource::User(user_path));
        }
    }

    // Fall back to defaults
    (Config::default(), ConfigSource::Defaults)
}

/// Load configuration from a specific file
pub fn load_from_file(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))
}

/// Save configuration to project-local keel.toml
pub fn save_config(config: &Config) -> Result<PathBuf> {
    let path = PathBuf::from("keel.toml");
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;

    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn default_config_has_constrained_mode() {
        let config = Config::default();
        assert_eq!(config.scoring.mode, "constrained");
    }

    #[test]
    fn current_weights_returns_constrained_by_default() {
        let config = Config::default();
        let weights = config.current_weights();
        assert_eq!(weights.effort_weight, 2.0);
        assert_eq!(weights.risk_weight, 1.5);
    }

    #[test]
    fn current_weights_returns_growth_mode() {
        let mut config = Config::default();
        config.set_mode("growth");
        let weights = config.current_weights();
        assert_eq!(weights.impact_weight, 2.0);
        assert_eq!(weights.effort_weight, 1.0);
    }

    #[test]
    fn current_weights_returns_product_mode() {
        let mut config = Config::default();
        config.set_mode("product");
        let weights = config.current_weights();
        assert_eq!(weights.impact_weight, 1.5);
        assert_eq!(weights.effort_weight, 1.5);
    }

    #[test]
    fn current_weights_falls_back_to_constrained_for_unknown_mode() {
        let mut config = Config::default();
        config.set_mode("unknown");
        let weights = config.current_weights();
        assert_eq!(weights, ModeWeights::constrained());
    }

    #[test]
    fn config_serialization_roundtrip() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn load_config_returns_defaults_when_no_file() {
        // Change to temp dir where no config exists
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(&temp).unwrap();

        let (config, source) = load_config();
        assert_eq!(config.scoring.mode, "constrained");
        assert_eq!(source, ConfigSource::Defaults);
    }

    #[test]
    fn load_from_file_parses_valid_toml() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[scoring]
mode = "growth"
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert_eq!(config.scoring.mode, "growth");
    }

    #[test]
    fn load_from_file_parses_custom_modes() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[scoring]
mode = "custom"

[scoring.modes.custom]
impact_weight = 3.0
confidence_weight = 2.0
effort_weight = 1.0
risk_weight = 0.5
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert_eq!(config.scoring.mode, "custom");
        let weights = config.current_weights();
        assert_eq!(weights.impact_weight, 3.0);
        assert_eq!(weights.risk_weight, 0.5);
    }

    #[test]
    fn is_valid_mode_checks_builtin_and_custom() {
        let mut config = Config::default();
        assert!(config.is_valid_mode("constrained"));
        assert!(config.is_valid_mode("growth"));
        assert!(config.is_valid_mode("product"));
        assert!(!config.is_valid_mode("custom"));

        // Add custom mode
        config
            .scoring
            .modes
            .insert("custom".to_string(), ModeWeights::constrained());
        assert!(config.is_valid_mode("custom"));
    }

    #[test]
    fn save_config_creates_file() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(&temp).unwrap();

        let mut config = Config::default();
        config.set_mode("growth");

        let path = save_config(&config).unwrap();
        assert!(path.exists());

        let loaded = load_from_file(&path).unwrap();
        assert_eq!(loaded.scoring.mode, "growth");
    }

    #[test]
    fn default_config_has_keel_board_dir() {
        let config = Config::default();
        assert_eq!(config.board_dir(), ".keel");
    }

    #[test]
    fn load_from_file_parses_board_dir() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
board_dir = "docs/board"
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert_eq!(config.board_dir(), "docs/board");
    }

    #[test]
    fn load_from_file_uses_default_board_dir_when_not_specified() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[scoring]
mode = "growth"
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert_eq!(config.board_dir(), ".keel");
        assert_eq!(config.scoring.mode, "growth");
    }

    // Note: find_board_dir() is tested via integration tests since it
    // relies on current directory state which is global and causes flaky
    // tests when run in parallel. The core logic is covered by:
    // - load_from_file_parses_board_dir (config parsing)
    // - load_from_file_uses_default_board_dir_when_not_specified (defaults)
}
