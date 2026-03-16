//! Keel configuration system
//!
//! Provides layered configuration with resolution order:
//! 1. `./keel.toml` (project-local)
//! 2. `~/.config/keel.toml` (user global)
//! 3. Built-in defaults

use std::collections::BTreeMap;
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

/// Per-check doctor override.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DoctorCheckOverride {
    /// Disable this doctor check when true.
    #[serde(default)]
    pub disabled: bool,
}

/// Doctor diagnostics configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DoctorConfig {
    /// Per-check disable overrides keyed by stable doctor check id.
    #[serde(default)]
    pub checks: std::collections::HashMap<String, DoctorCheckOverride>,
}

impl DoctorConfig {
    /// Whether a given doctor check is disabled by config.
    pub fn is_disabled(&self, id: &str) -> bool {
        self.checks
            .get(id)
            .is_some_and(|override_| override_.disabled)
    }
}

/// Per-provider research override.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ResearchProviderOverride {
    /// Disable this provider when true.
    #[serde(default)]
    pub disabled: bool,
    /// Relative ranking or weighting used for downstream evidence ordering.
    #[serde(default = "default_research_weight")]
    pub weight: f64,
}

fn default_research_weight() -> f64 {
    1.0
}

/// Research provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ResearchConfig {
    /// Per-provider overrides keyed by stable research provider id.
    #[serde(default)]
    pub providers: std::collections::HashMap<String, ResearchProviderOverride>,
}

/// Workflow defaults controlling the canonical management and delivery examples.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowDefaultsConfig {
    #[serde(default = "default_management_role")]
    pub management_role: String,
    #[serde(default = "default_delivery_role")]
    pub delivery_role: String,
    #[serde(default = "default_management_lane")]
    pub management_lane: String,
    #[serde(default = "default_delivery_lane")]
    pub delivery_lane: String,
}

fn default_management_role() -> String {
    "manager".to_string()
}

fn default_delivery_role() -> String {
    "operator".to_string()
}

fn default_management_lane() -> String {
    "management".to_string()
}

fn default_delivery_lane() -> String {
    "delivery".to_string()
}

impl Default for WorkflowDefaultsConfig {
    fn default() -> Self {
        Self {
            management_role: default_management_role(),
            delivery_role: default_delivery_role(),
            management_lane: default_management_lane(),
            delivery_lane: default_delivery_lane(),
        }
    }
}

pub fn default_max_active_missions() -> usize {
    1
}

pub fn default_open_for_work() -> bool {
    true
}

pub fn default_working_hours_start() -> u8 {
    9
}

pub fn default_working_hours_end() -> u8 {
    17
}

pub fn default_battery_decay_minutes() -> u32 {
    30
}

pub fn default_max_battery_packs() -> usize {
    5
}

/// Workflow-level configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowConfig {
    #[serde(default)]
    pub defaults: WorkflowDefaultsConfig,
    
    /// The maximum number of missions that can be active at the same time.
    /// Represents the Circuit Breaker "max amperage".
    #[serde(default = "default_max_active_missions")]
    pub max_active_missions: usize,
    
    /// The maximum number of ready stories allowed in the backlog before triggering an overload.
    /// Represents plugged-in queue load.
    #[serde(default = "default_max_battery_packs")]
    pub max_battery_packs: usize,
    
    /// The main switch for the engine. If false, the engine does not perform work.
    #[serde(default = "default_open_for_work")]
    pub open_for_work: bool,

    /// Hour of the day (0-23) the system starts working autonomously (default: 9)
    #[serde(default = "default_working_hours_start")]
    pub working_hours_start: u8,

    /// Hour of the day (0-23) the system stops working autonomously (default: 17)
    #[serde(default = "default_working_hours_end")]
    pub working_hours_end: u8,

    /// Number of minutes before the circuit battery decays to zero after the last state transition
    #[serde(default = "default_battery_decay_minutes")]
    pub battery_decay_minutes: u32,

    /// Optional command to execute when human input is required (e.g., 'tmux display-message "..."')
    #[serde(default)]
    pub notification_command: Option<String>,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            defaults: WorkflowDefaultsConfig::default(),
            max_active_missions: default_max_active_missions(),
            max_battery_packs: default_max_battery_packs(),
            open_for_work: default_open_for_work(),
            working_hours_start: default_working_hours_start(),
            working_hours_end: default_working_hours_end(),
            battery_decay_minutes: default_battery_decay_minutes(),
            notification_command: None,
        }
    }
}

impl WorkflowConfig {
    fn is_default(&self) -> bool {
        self == &Self::default()
    }

    /// Get the effective notification command, defaulting to tmux display-message if in tmux.
    pub fn effective_notification_command(&self) -> Option<String> {
        if let Some(cmd) = &self.notification_command {
            return Some(cmd.clone());
        }

        if std::env::var("TMUX").is_ok() {
            return Some(
                "tmux display-message \"Keel: Human input required (Capacitors full)\"".to_string(),
            );
        }

        None
    }
}

/// Configured role family.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoleFamilyConfig {
    pub default_lane: String,
    pub operational_contract: String,
}

/// Configured lane definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneConfig {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub parallel: bool,
    #[serde(default)]
    pub manual_accept: bool,
    #[serde(default)]
    pub priority: i32,
}

/// Exact taxonomy override for operational contract selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoleOverrideConfig {
    pub operational_contract: String,
}

/// Default board directory
fn default_board_dir() -> String {
    ".keel".to_string()
}

/// Supported storage backends.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
    #[default]
    Filesystem,
    Server,
}

/// Storage-level configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct StorageConfig {
    #[serde(default)]
    pub backend: StorageBackend,
}

/// Keel configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Board directory path (relative to project root)
    /// Defaults to ".keel" if not specified
    #[serde(default = "default_board_dir")]
    pub board_dir: String,

    /// Storage configuration.
    #[serde(default)]
    pub storage: StorageConfig,

    /// Workflow defaults and topology controls.
    #[serde(default, skip_serializing_if = "WorkflowConfig::is_default")]
    pub workflow: WorkflowConfig,

    /// Configured role families keyed by base role.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub roles: BTreeMap<String, RoleFamilyConfig>,

    /// Configured lanes keyed by lane id.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lanes: BTreeMap<String, LaneConfig>,

    /// Exact taxonomy overrides keyed by the full role taxonomy string.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub role_overrides: BTreeMap<String, RoleOverrideConfig>,

    /// Scoring configuration
    #[serde(default)]
    pub scoring: ScoringConfig,

    /// Doctor diagnostics configuration.
    #[serde(default)]
    pub doctor: DoctorConfig,

    /// Research provider configuration.
    #[serde(default)]
    pub research: ResearchConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            board_dir: default_board_dir(),
            storage: StorageConfig::default(),
            workflow: WorkflowConfig::default(),
            roles: BTreeMap::new(),
            lanes: BTreeMap::new(),
            role_overrides: BTreeMap::new(),
            scoring: ScoringConfig::default(),
            doctor: DoctorConfig::default(),
            research: ResearchConfig::default(),
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

/// Load configuration with layered resolution, starting from a specific directory
pub fn load_config_from(dir: &Path) -> (Config, ConfigSource) {
    // Try project-local first in the specified directory
    let local_path = dir.join("keel.toml");
    let (mut config, source) = if let Some(config) = local_path
        .exists()
        .then(|| load_from_file(&local_path).ok())
        .flatten()
    {
        (config, ConfigSource::Local(local_path))
    } else if let Some(config_dir) = dirs::config_dir() {
        // Try user global
        let user_path = config_dir.join("keel.toml");
        if let Some(config) = user_path
            .exists()
            .then(|| load_from_file(&user_path).ok())
            .flatten()
        {
            (config, ConfigSource::User(user_path))
        } else {
            (Config::default(), ConfigSource::Defaults)
        }
    } else {
        (Config::default(), ConfigSource::Defaults)
    };

    // Layer 4: Environment Overrides
    if let Ok(backend_str) = std::env::var("KEEL_STORAGE_BACKEND") {
        match backend_str.to_lowercase().as_str() {
            "filesystem" => config.storage.backend = StorageBackend::Filesystem,
            "server" => config.storage.backend = StorageBackend::Server,
            _ => {} // Ignore invalid env var values
        }
    }

    (config, source)
}

/// Load configuration with layered resolution
pub fn load_config() -> (Config, ConfigSource) {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    load_config_from(&cwd)
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
    fn load_from_file_parses_doctor_check_overrides() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[doctor.checks.voyage-scope-authored-content]
disabled = true
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert!(config.doctor.is_disabled("voyage-scope-authored-content"));
        assert!(!config.doctor.is_disabled("story-id-uniqueness"));
    }

    #[test]
    fn load_from_file_parses_research_provider_overrides() {
        let temp = tempfile::TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");
        fs::write(
            &path,
            r#"
[research.providers.manual]
disabled = true
weight = 0.5

[research.providers.academic]
weight = 1.4
"#,
        )
        .unwrap();

        let config = load_from_file(&path).unwrap();

        assert!(
            config
                .research
                .providers
                .get("manual")
                .is_some_and(|provider| provider.disabled && provider.weight == 0.5)
        );
        assert!(
            config
                .research
                .providers
                .get("academic")
                .is_some_and(|provider| !provider.disabled && provider.weight == 1.4)
        );
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

    #[test]
    fn config_storage_env_override() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        // Config has filesystem
        let content = r#"
[storage]
backend = "filesystem"
"#;
        fs::write(&path, content).unwrap();

        // Env overrides to server
        unsafe { std::env::set_var("KEEL_STORAGE_BACKEND", "server") };
        let (config, _) = load_config_from(temp.path());
        unsafe { std::env::remove_var("KEEL_STORAGE_BACKEND") };

        assert_eq!(config.storage.backend, StorageBackend::Server);
    }

    #[test]
    fn config_storage_validation() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[storage]
backend = "unknown-backend"
"#;
        fs::write(&path, content).unwrap();

        let result = load_from_file(&path);
        assert!(result.is_err());
        let err_msg = format!("{:?}", result.unwrap_err());
        assert!(err_msg.contains("unknown variant `unknown-backend`"));
    }

    #[test]
    fn load_from_file_parses_storage_section() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[storage]
backend = "server"
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert_eq!(config.storage.backend, StorageBackend::Server);

        // Default case
        let config_default = Config::default();
        assert_eq!(config_default.storage.backend, StorageBackend::Filesystem);
    }

    #[test]
    fn workflow_topology_config_uses_seed_defaults_when_omitted() {
        let config = Config::default();

        assert_eq!(config.workflow.defaults.management_role, "manager");
        assert_eq!(config.workflow.defaults.delivery_role, "operator");
        assert_eq!(config.workflow.defaults.management_lane, "management");
        assert_eq!(config.workflow.defaults.delivery_lane, "delivery");
        assert!(config.roles.is_empty());
        assert!(config.lanes.is_empty());
        assert!(config.role_overrides.is_empty());
    }

    #[test]
    fn workflow_topology_config_parses_sections() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("keel.toml");

        let content = r#"
[workflow.defaults]
management_role = "director"
delivery_role = "operator"
management_lane = "review"
delivery_lane = "delivery"

[roles.director]
default_lane = "review"
operational_contract = "director-core"

[roles.operator]
default_lane = "delivery"
operational_contract = "operator-core"

[lanes.review]
description = "Review lane"
include = ["story.needs-human-verification", "voyage.draft"]
exclude = []
parallel = false
manual_accept = true
priority = 100

[lanes.delivery]
description = "Delivery lane"
include = ["story.*"]
exclude = ["story.done"]
parallel = true
manual_accept = false
priority = 50

[role_overrides."operator/software"]
operational_contract = "software-operator-core"
"#;
        fs::write(&path, content).unwrap();

        let config = load_from_file(&path).unwrap();
        assert_eq!(config.workflow.defaults.management_role, "director");
        assert_eq!(config.workflow.defaults.management_lane, "review");
        assert_eq!(
            config.roles["director"].operational_contract,
            "director-core"
        );
        assert_eq!(config.lanes["review"].priority, 100);
        assert!(config.lanes["review"].manual_accept);
        assert_eq!(
            config.role_overrides["operator/software"].operational_contract,
            "software-operator-core"
        );
    }

    // Note: find_board_dir() is tested via integration tests since it
    // relies on current directory state which is global and causes flaky
    // tests when run in parallel. The core logic is covered by:
    // - load_from_file_parses_board_dir (config parsing)
    // - load_from_file_uses_default_board_dir_when_not_specified (defaults)
}
