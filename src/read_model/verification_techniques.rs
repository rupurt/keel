//! Canonical verification-technique catalog model and built-in entries.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TechniqueModality {
    Command,
    Recording,
    Judge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectStack {
    Rust,
    Browser,
    Cli,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArtifactKind {
    Log,
    Json,
    Video,
    Transcript,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechniqueDefinition {
    /// Stable identifier used by config overrides and rendering.
    pub id: String,
    /// Human-readable label for recommendation output.
    pub label: String,
    /// Summary shown in planning/read surfaces.
    pub description: String,
    /// Technique execution modality.
    pub modality: TechniqueModality,
    /// Canonical command template to adopt.
    pub default_command: String,
    /// Whether this technique is included unless disabled by config.
    pub enabled_by_default: bool,
    /// Ranking anchor for recommendation ordering.
    pub priority: u16,
    /// Tooling/runtime prerequisites.
    pub prerequisites: Vec<String>,
    /// Project stacks where this technique is applicable.
    pub applicable_stacks: Vec<ProjectStack>,
    /// Artifact classes expected as evidence outputs.
    pub expected_artifacts: Vec<ArtifactKind>,
    /// Keywords used by detection/ranking heuristics.
    pub signal_keywords: Vec<String>,
}

impl TechniqueDefinition {
    fn new(
        id: &str,
        label: &str,
        description: &str,
        modality: TechniqueModality,
        default_command: &str,
        enabled_by_default: bool,
        priority: u16,
        prerequisites: Vec<&str>,
        applicable_stacks: Vec<ProjectStack>,
        expected_artifacts: Vec<ArtifactKind>,
        signal_keywords: Vec<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            description: description.to_string(),
            modality,
            default_command: default_command.to_string(),
            enabled_by_default,
            priority,
            prerequisites: prerequisites.into_iter().map(ToOwned::to_owned).collect(),
            applicable_stacks,
            expected_artifacts,
            signal_keywords: signal_keywords.into_iter().map(ToOwned::to_owned).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TechniqueOverrideConfig {
    /// Built-in technique ids forced enabled.
    pub enable_ids: BTreeSet<String>,
    /// Built-in technique ids forced disabled.
    pub disable_ids: BTreeSet<String>,
    /// Field-level overrides for existing techniques.
    pub customize: Vec<TechniqueCustomization>,
    /// Project-local custom techniques.
    pub custom: Vec<TechniqueDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TechniqueCustomization {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub modality: Option<TechniqueModality>,
    pub default_command: Option<String>,
    pub enabled_by_default: Option<bool>,
    pub priority: Option<u16>,
    pub prerequisites: Option<Vec<String>>,
    pub applicable_stacks: Option<Vec<ProjectStack>>,
    pub expected_artifacts: Option<Vec<ArtifactKind>>,
    pub signal_keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechniqueOverrideDiagnostic {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TechniqueOverrideParseResult {
    pub overrides: TechniqueOverrideConfig,
    pub diagnostics: Vec<TechniqueOverrideDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TechniqueCatalogMergeResult {
    pub catalog: Vec<TechniqueDefinition>,
    pub diagnostics: Vec<TechniqueOverrideDiagnostic>,
}

/// Built-in technique bank used as the canonical base catalog.
///
/// The list is sorted by technique id to guarantee deterministic ordering across runs.
pub fn builtin_technique_catalog() -> Vec<TechniqueDefinition> {
    let mut catalog = vec![
        TechniqueDefinition::new(
            "browser-playwright-e2e",
            "Playwright E2E",
            "Browser end-to-end assertions with trace/video evidence.",
            TechniqueModality::Command,
            "npx playwright test --video=on --trace=on",
            true,
            72,
            vec!["node", "playwright"],
            vec![ProjectStack::Browser],
            vec![ArtifactKind::Video, ArtifactKind::Log],
            vec!["playwright", "browser", "ui", "e2e"],
        ),
        TechniqueDefinition::new(
            "llm-judge",
            "LLM-Judge",
            "Semantic assertion pass using llm-judge transcripts.",
            TechniqueModality::Judge,
            "llm-judge",
            true,
            85,
            vec!["llm-judge"],
            vec![ProjectStack::Rust, ProjectStack::Browser, ProjectStack::Cli],
            vec![ArtifactKind::Transcript],
            vec!["manual", "semantic", "acceptance"],
        ),
        TechniqueDefinition::new(
            "rust-unit-tests",
            "Rust Unit/Integration Tests",
            "Fast deterministic command-based verification for Rust code paths.",
            TechniqueModality::Command,
            "cargo test",
            true,
            80,
            vec!["cargo"],
            vec![ProjectStack::Rust, ProjectStack::Cli],
            vec![ArtifactKind::Log],
            vec!["cargo", "rust", "tests"],
        ),
        TechniqueDefinition::new(
            "rust-coverage",
            "Rust Coverage Gate",
            "Coverage regression guard for Rust projects.",
            TechniqueModality::Command,
            "cargo llvm-cov --workspace --lcov --output-path coverage/lcov.info",
            true,
            66,
            vec!["cargo-llvm-cov"],
            vec![ProjectStack::Rust],
            vec![ArtifactKind::Log, ArtifactKind::Json],
            vec!["coverage", "llvm-cov", "regression"],
        ),
        TechniqueDefinition::new(
            "vhs",
            "VHS CLI Recording",
            "Deterministic terminal recordings for acceptance evidence.",
            TechniqueModality::Recording,
            "vhs <tape>.tape",
            true,
            78,
            vec!["vhs", "ffmpeg"],
            vec![ProjectStack::Cli, ProjectStack::Rust],
            vec![ArtifactKind::Video],
            vec!["cli", "recording", "terminal", "demo"],
        ),
    ];

    catalog.sort_by(|a, b| a.id.cmp(&b.id));
    catalog
}

/// Parse `keel.toml` verification-technique override config.
///
/// Canonical schema path:
/// - `[verification.techniques]`
/// - `enable = ["id"]`
/// - `disable = ["id"]`
/// - `[[verification.techniques.customize]]`
/// - `[[verification.techniques.custom]]`
///
/// Parsing is advisory-only: invalid entries emit diagnostics and are ignored.
pub fn parse_technique_overrides_from_keel_toml(content: &str) -> TechniqueOverrideParseResult {
    let mut result = TechniqueOverrideParseResult::default();
    let value: toml::Value = match toml::from_str(content) {
        Ok(value) => value,
        Err(err) => {
            result.diagnostics.push(TechniqueOverrideDiagnostic {
                path: "keel.toml".to_string(),
                message: format!("invalid toml: {err}"),
            });
            return result;
        }
    };

    let Some(section) = value
        .get("verification")
        .and_then(toml::Value::as_table)
        .and_then(|verification| verification.get("techniques"))
        .and_then(toml::Value::as_table)
    else {
        return result;
    };

    validate_allowed_keys(
        section,
        &["enable", "disable", "customize", "custom"],
        "verification.techniques",
        &mut result.diagnostics,
    );

    result.overrides.enable_ids = parse_string_set(
        section.get("enable"),
        "verification.techniques.enable",
        &mut result.diagnostics,
    );
    result.overrides.disable_ids = parse_string_set(
        section.get("disable"),
        "verification.techniques.disable",
        &mut result.diagnostics,
    );

    if let Some(value) = section.get("customize") {
        result.overrides.customize = parse_customize_entries(value, &mut result.diagnostics);
        result.overrides.customize.sort_by(|a, b| a.id.cmp(&b.id));
    }

    if let Some(value) = section.get("custom") {
        result.overrides.custom = parse_custom_entries(value, &mut result.diagnostics);
        result.overrides.custom.sort_by(|a, b| a.id.cmp(&b.id));
    }

    result
}

/// Merge built-in techniques with parsed overrides.
///
/// Deterministic precedence:
/// 1. built-ins
/// 2. explicit disable list
/// 3. explicit enable list
/// 4. per-technique customize entries
/// 5. custom technique additions
pub fn merge_technique_catalog_with_overrides(
    builtin: Vec<TechniqueDefinition>,
    overrides: &TechniqueOverrideConfig,
) -> TechniqueCatalogMergeResult {
    let mut diagnostics = Vec::new();
    let mut by_id: BTreeMap<String, TechniqueDefinition> = builtin
        .into_iter()
        .map(|technique| (technique.id.clone(), technique))
        .collect();

    for id in &overrides.disable_ids {
        if let Some(technique) = by_id.get_mut(id) {
            technique.enabled_by_default = false;
        } else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("verification.techniques.disable[{id}]"),
                message: "unknown technique id".to_string(),
            });
        }
    }

    for id in &overrides.enable_ids {
        if let Some(technique) = by_id.get_mut(id) {
            technique.enabled_by_default = true;
        } else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("verification.techniques.enable[{id}]"),
                message: "unknown technique id".to_string(),
            });
        }
    }

    for customization in &overrides.customize {
        if let Some(technique) = by_id.get_mut(&customization.id) {
            apply_customization(technique, customization);
        } else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("verification.techniques.customize.{}", customization.id),
                message: "customize target id not found in catalog".to_string(),
            });
        }
    }

    for custom in &overrides.custom {
        if by_id.contains_key(&custom.id) {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("verification.techniques.custom.{}", custom.id),
                message: "custom technique id collides with existing catalog id".to_string(),
            });
            continue;
        }

        by_id.insert(custom.id.clone(), custom.clone());
    }

    TechniqueCatalogMergeResult {
        catalog: by_id.into_values().collect(),
        diagnostics,
    }
}

fn apply_customization(target: &mut TechniqueDefinition, customization: &TechniqueCustomization) {
    if let Some(label) = &customization.label {
        target.label = label.clone();
    }
    if let Some(description) = &customization.description {
        target.description = description.clone();
    }
    if let Some(modality) = customization.modality {
        target.modality = modality;
    }
    if let Some(default_command) = &customization.default_command {
        target.default_command = default_command.clone();
    }
    if let Some(enabled_by_default) = customization.enabled_by_default {
        target.enabled_by_default = enabled_by_default;
    }
    if let Some(priority) = customization.priority {
        target.priority = priority;
    }
    if let Some(prerequisites) = &customization.prerequisites {
        target.prerequisites = normalize_vec(prerequisites);
    }
    if let Some(applicable_stacks) = &customization.applicable_stacks {
        target.applicable_stacks = normalize_copy_vec(applicable_stacks);
    }
    if let Some(expected_artifacts) = &customization.expected_artifacts {
        target.expected_artifacts = normalize_copy_vec(expected_artifacts);
    }
    if let Some(signal_keywords) = &customization.signal_keywords {
        target.signal_keywords = normalize_vec(signal_keywords);
    }
}

fn parse_customize_entries(
    value: &toml::Value,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Vec<TechniqueCustomization> {
    let Some(entries) = value.as_array() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: "verification.techniques.customize".to_string(),
            message: "expected array of tables".to_string(),
        });
        return Vec::new();
    };

    let mut parsed = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let path = format!("verification.techniques.customize[{index}]");
        let Some(table) = entry.as_table() else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path,
                message: "expected table entry".to_string(),
            });
            continue;
        };

        validate_allowed_keys(
            table,
            &[
                "id",
                "label",
                "description",
                "modality",
                "default_command",
                "enabled",
                "priority",
                "prerequisites",
                "applicable_stacks",
                "expected_artifacts",
                "signal_keywords",
            ],
            &format!("verification.techniques.customize[{index}]"),
            diagnostics,
        );

        let Some(id) = parse_required_string(table.get("id"), &format!("{path}.id"), diagnostics)
        else {
            continue;
        };

        let modality = table
            .get("modality")
            .and_then(|raw| parse_modality(raw, &format!("{path}.modality"), diagnostics));

        let entry = TechniqueCustomization {
            id,
            label: parse_optional_string(table.get("label"), &format!("{path}.label"), diagnostics),
            description: parse_optional_string(
                table.get("description"),
                &format!("{path}.description"),
                diagnostics,
            ),
            modality,
            default_command: parse_optional_string(
                table.get("default_command"),
                &format!("{path}.default_command"),
                diagnostics,
            ),
            enabled_by_default: parse_optional_bool(
                table.get("enabled"),
                &format!("{path}.enabled"),
                diagnostics,
            ),
            priority: parse_optional_u16(
                table.get("priority"),
                &format!("{path}.priority"),
                diagnostics,
            ),
            prerequisites: parse_optional_string_vec(
                table.get("prerequisites"),
                &format!("{path}.prerequisites"),
                diagnostics,
            )
            .map(|values| normalize_vec(&values)),
            applicable_stacks: parse_optional_stack_vec(
                table.get("applicable_stacks"),
                &format!("{path}.applicable_stacks"),
                diagnostics,
            )
            .map(|values| normalize_copy_vec(&values)),
            expected_artifacts: parse_optional_artifact_vec(
                table.get("expected_artifacts"),
                &format!("{path}.expected_artifacts"),
                diagnostics,
            )
            .map(|values| normalize_copy_vec(&values)),
            signal_keywords: parse_optional_string_vec(
                table.get("signal_keywords"),
                &format!("{path}.signal_keywords"),
                diagnostics,
            )
            .map(|values| normalize_vec(&values)),
        };

        parsed.push(entry);
    }

    parsed
}

fn parse_custom_entries(
    value: &toml::Value,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Vec<TechniqueDefinition> {
    let Some(entries) = value.as_array() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: "verification.techniques.custom".to_string(),
            message: "expected array of tables".to_string(),
        });
        return Vec::new();
    };

    let mut parsed = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        let path = format!("verification.techniques.custom[{index}]");
        let Some(table) = entry.as_table() else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path,
                message: "expected table entry".to_string(),
            });
            continue;
        };

        validate_allowed_keys(
            table,
            &[
                "id",
                "label",
                "description",
                "modality",
                "default_command",
                "enabled",
                "priority",
                "prerequisites",
                "applicable_stacks",
                "expected_artifacts",
                "signal_keywords",
            ],
            &format!("verification.techniques.custom[{index}]"),
            diagnostics,
        );

        let Some(id) = parse_required_string(table.get("id"), &format!("{path}.id"), diagnostics)
        else {
            continue;
        };
        let Some(label) =
            parse_required_string(table.get("label"), &format!("{path}.label"), diagnostics)
        else {
            continue;
        };
        let Some(description) = parse_required_string(
            table.get("description"),
            &format!("{path}.description"),
            diagnostics,
        ) else {
            continue;
        };
        let Some(modality) = parse_modality_required(
            table.get("modality"),
            &format!("{path}.modality"),
            diagnostics,
        ) else {
            continue;
        };
        let Some(default_command) = parse_required_string(
            table.get("default_command"),
            &format!("{path}.default_command"),
            diagnostics,
        ) else {
            continue;
        };

        let enabled_by_default = parse_optional_bool(
            table.get("enabled"),
            &format!("{path}.enabled"),
            diagnostics,
        )
        .unwrap_or(true);

        let priority = parse_optional_u16(
            table.get("priority"),
            &format!("{path}.priority"),
            diagnostics,
        )
        .unwrap_or(50);

        let prerequisites = parse_optional_string_vec(
            table.get("prerequisites"),
            &format!("{path}.prerequisites"),
            diagnostics,
        )
        .map(|values| normalize_vec(&values))
        .unwrap_or_default();

        let applicable_stacks = parse_optional_stack_vec(
            table.get("applicable_stacks"),
            &format!("{path}.applicable_stacks"),
            diagnostics,
        )
        .map(|values| normalize_copy_vec(&values))
        .unwrap_or_else(|| vec![ProjectStack::Cli]);

        let expected_artifacts = parse_optional_artifact_vec(
            table.get("expected_artifacts"),
            &format!("{path}.expected_artifacts"),
            diagnostics,
        )
        .map(|values| normalize_copy_vec(&values))
        .unwrap_or_else(|| vec![ArtifactKind::Log]);

        let signal_keywords = parse_optional_string_vec(
            table.get("signal_keywords"),
            &format!("{path}.signal_keywords"),
            diagnostics,
        )
        .map(|values| normalize_vec(&values))
        .unwrap_or_default();

        parsed.push(TechniqueDefinition {
            id,
            label,
            description,
            modality,
            default_command,
            enabled_by_default,
            priority,
            prerequisites,
            applicable_stacks,
            expected_artifacts,
            signal_keywords,
        });
    }

    parsed
}

fn validate_allowed_keys(
    table: &toml::map::Map<String, toml::Value>,
    allowed: &[&str],
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) {
    let allowed: BTreeSet<&str> = allowed.iter().copied().collect();
    for key in table.keys() {
        if !allowed.contains(key.as_str()) {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("{path}.{key}"),
                message: "unknown field".to_string(),
            });
        }
    }
}

fn parse_string_set(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> BTreeSet<String> {
    let Some(value) = value else {
        return BTreeSet::new();
    };

    let Some(items) = value.as_array() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected array of strings".to_string(),
        });
        return BTreeSet::new();
    };

    let mut out = BTreeSet::new();
    for (index, item) in items.iter().enumerate() {
        match item
            .as_str()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            Some(id) => {
                out.insert(id.to_string());
            }
            None => diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("{path}[{index}]"),
                message: "expected non-empty string".to_string(),
            }),
        }
    }

    out
}

fn parse_required_string(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<String> {
    let Some(value) = value else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "missing required field".to_string(),
        });
        return None;
    };

    let Some(text) = value
        .as_str()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected non-empty string".to_string(),
        });
        return None;
    };

    Some(text.to_string())
}

fn parse_optional_string(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<String> {
    let Some(value) = value else {
        return None;
    };

    let Some(text) = value
        .as_str()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected non-empty string".to_string(),
        });
        return None;
    };

    Some(text.to_string())
}

fn parse_optional_bool(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<bool> {
    let Some(value) = value else {
        return None;
    };

    let Some(boolean) = value.as_bool() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected boolean".to_string(),
        });
        return None;
    };

    Some(boolean)
}

fn parse_optional_u16(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<u16> {
    let Some(value) = value else {
        return None;
    };

    let Some(raw) = value.as_integer() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected integer".to_string(),
        });
        return None;
    };

    if !(0..=u16::MAX as i64).contains(&raw) {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "priority must be between 0 and 65535".to_string(),
        });
        return None;
    }

    Some(raw as u16)
}

fn parse_optional_string_vec(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<Vec<String>> {
    let Some(value) = value else {
        return None;
    };

    let Some(items) = value.as_array() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected array of strings".to_string(),
        });
        return None;
    };

    let mut out = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let Some(text) = item.as_str().map(str::trim).filter(|text| !text.is_empty()) else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("{path}[{index}]"),
                message: "expected non-empty string".to_string(),
            });
            continue;
        };
        out.push(text.to_string());
    }

    Some(out)
}

fn parse_modality(
    value: &toml::Value,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<TechniqueModality> {
    let Some(raw) = value.as_str().map(str::trim) else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected modality string".to_string(),
        });
        return None;
    };

    match raw {
        "command" => Some(TechniqueModality::Command),
        "recording" => Some(TechniqueModality::Recording),
        "judge" => Some(TechniqueModality::Judge),
        _ => {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: path.to_string(),
                message: format!("unknown modality '{raw}' (expected: command, recording, judge)"),
            });
            None
        }
    }
}

fn parse_modality_required(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<TechniqueModality> {
    let Some(value) = value else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "missing required field".to_string(),
        });
        return None;
    };

    parse_modality(value, path, diagnostics)
}

fn parse_optional_stack_vec(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<Vec<ProjectStack>> {
    let Some(value) = value else {
        return None;
    };

    let Some(items) = value.as_array() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected array of stack strings".to_string(),
        });
        return None;
    };

    let mut out = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let Some(raw) = item.as_str().map(str::trim) else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("{path}[{index}]"),
                message: "expected stack string".to_string(),
            });
            continue;
        };

        let parsed = match raw {
            "rust" => Some(ProjectStack::Rust),
            "browser" => Some(ProjectStack::Browser),
            "cli" => Some(ProjectStack::Cli),
            _ => {
                diagnostics.push(TechniqueOverrideDiagnostic {
                    path: format!("{path}[{index}]"),
                    message: format!("unknown stack '{raw}' (expected: rust, browser, cli)"),
                });
                None
            }
        };

        if let Some(parsed) = parsed {
            out.push(parsed);
        }
    }

    Some(out)
}

fn parse_optional_artifact_vec(
    value: Option<&toml::Value>,
    path: &str,
    diagnostics: &mut Vec<TechniqueOverrideDiagnostic>,
) -> Option<Vec<ArtifactKind>> {
    let Some(value) = value else {
        return None;
    };

    let Some(items) = value.as_array() else {
        diagnostics.push(TechniqueOverrideDiagnostic {
            path: path.to_string(),
            message: "expected array of artifact strings".to_string(),
        });
        return None;
    };

    let mut out = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let Some(raw) = item.as_str().map(str::trim) else {
            diagnostics.push(TechniqueOverrideDiagnostic {
                path: format!("{path}[{index}]"),
                message: "expected artifact string".to_string(),
            });
            continue;
        };

        let parsed = match raw {
            "log" => Some(ArtifactKind::Log),
            "json" => Some(ArtifactKind::Json),
            "video" => Some(ArtifactKind::Video),
            "transcript" => Some(ArtifactKind::Transcript),
            _ => {
                diagnostics.push(TechniqueOverrideDiagnostic {
                    path: format!("{path}[{index}]"),
                    message: format!(
                        "unknown artifact '{raw}' (expected: log, json, video, transcript)"
                    ),
                });
                None
            }
        };

        if let Some(parsed) = parsed {
            out.push(parsed);
        }
    }

    Some(out)
}

fn normalize_vec(values: &[String]) -> Vec<String> {
    let mut values = values
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

fn normalize_copy_vec<T>(values: &[T]) -> Vec<T>
where
    T: Copy + Ord,
{
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn technique_definition_model_has_required_fields() {
        let model = TechniqueDefinition::new(
            "sample",
            "Sample",
            "Sample description",
            TechniqueModality::Command,
            "echo ok",
            true,
            50,
            vec!["tool-a"],
            vec![ProjectStack::Cli],
            vec![ArtifactKind::Log],
            vec!["cli"],
        );

        assert_eq!(model.id, "sample");
        assert_eq!(model.label, "Sample");
        assert_eq!(model.modality, TechniqueModality::Command);
        assert_eq!(model.default_command, "echo ok");
        assert!(model.enabled_by_default);
        assert_eq!(model.priority, 50);
        assert_eq!(model.prerequisites, vec!["tool-a".to_string()]);
        assert_eq!(model.applicable_stacks, vec![ProjectStack::Cli]);
        assert_eq!(model.expected_artifacts, vec![ArtifactKind::Log]);
        assert_eq!(model.signal_keywords, vec!["cli".to_string()]);
    }

    #[test]
    fn builtin_technique_catalog_contains_vhs_llm_judge() {
        let catalog = builtin_technique_catalog();
        let ids: Vec<&str> = catalog
            .iter()
            .map(|technique| technique.id.as_str())
            .collect();

        assert!(ids.contains(&"vhs"));
        assert!(ids.contains(&"llm-judge"));
        assert!(catalog.iter().any(|technique| {
            technique.modality == TechniqueModality::Command
                && technique.applicable_stacks.contains(&ProjectStack::Rust)
        }));
        assert!(catalog.iter().any(|technique| {
            technique.modality == TechniqueModality::Command
                && technique.applicable_stacks.contains(&ProjectStack::Browser)
        }));
    }

    #[test]
    fn builtin_technique_catalog_deterministic() {
        let first = builtin_technique_catalog();
        let second = builtin_technique_catalog();

        assert_eq!(first, second);

        let ids: Vec<&str> = first
            .iter()
            .map(|technique| technique.id.as_str())
            .collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }

    #[test]
    fn technique_override_config_parse() {
        let content = r#"
[verification.techniques]
enable = ["vhs", "llm-judge"]
disable = ["rust-coverage"]

[[verification.techniques.customize]]
id = "vhs"
enabled = true
default_command = "vhs demos/main.tape"
priority = 91
signal_keywords = ["terminal", "demo"]

[[verification.techniques.custom]]
id = "shellcheck"
label = "Shellcheck"
description = "Shell lint gate"
modality = "command"
default_command = "shellcheck scripts/*.sh"
enabled = true
priority = 61
prerequisites = ["shellcheck"]
applicable_stacks = ["cli"]
expected_artifacts = ["log"]
signal_keywords = ["shell", "lint"]
"#;

        let parsed = parse_technique_overrides_from_keel_toml(content);

        assert!(parsed.diagnostics.is_empty());
        assert!(parsed.overrides.enable_ids.contains("vhs"));
        assert!(parsed.overrides.enable_ids.contains("llm-judge"));
        assert!(parsed.overrides.disable_ids.contains("rust-coverage"));
        assert_eq!(parsed.overrides.customize.len(), 1);
        assert_eq!(parsed.overrides.customize[0].id, "vhs");
        assert_eq!(
            parsed.overrides.customize[0].default_command.as_deref(),
            Some("vhs demos/main.tape")
        );
        assert_eq!(parsed.overrides.custom.len(), 1);
        assert_eq!(parsed.overrides.custom[0].id, "shellcheck");
    }

    #[test]
    fn technique_override_merge_precedence() {
        let mut overrides = TechniqueOverrideConfig::default();
        overrides.disable_ids.insert("vhs".to_string());
        overrides.enable_ids.insert("vhs".to_string());
        overrides.customize.push(TechniqueCustomization {
            id: "vhs".to_string(),
            enabled_by_default: Some(false),
            priority: Some(95),
            default_command: Some("vhs demos/override.tape".to_string()),
            ..TechniqueCustomization::default()
        });
        overrides.custom.push(TechniqueDefinition::new(
            "z-custom",
            "Custom",
            "Custom technique",
            TechniqueModality::Command,
            "custom --run",
            true,
            40,
            vec!["custom"],
            vec![ProjectStack::Cli],
            vec![ArtifactKind::Log],
            vec!["custom"],
        ));

        let merged =
            merge_technique_catalog_with_overrides(builtin_technique_catalog(), &overrides);
        assert!(merged.diagnostics.is_empty());

        let mut ids: Vec<&str> = merged
            .catalog
            .iter()
            .map(|technique| technique.id.as_str())
            .collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);

        let vhs = merged
            .catalog
            .iter()
            .find(|technique| technique.id == "vhs")
            .unwrap();
        assert!(!vhs.enabled_by_default);
        assert_eq!(vhs.priority, 95);
        assert_eq!(vhs.default_command, "vhs demos/override.tape");

        ids.sort_unstable();
        assert!(ids.contains(&"z-custom"));
    }

    #[test]
    fn technique_override_invalid_is_advisory_only() {
        let content = r#"
[verification.techniques]
enable = "vhs"
disable = ["missing-technique"]

[[verification.techniques.customize]]
id = "missing-technique"
priority = -1

[[verification.techniques.custom]]
id = "broken"
label = ""
description = "Broken"
modality = "unknown"
default_command = ""
"#;

        let parsed = parse_technique_overrides_from_keel_toml(content);
        assert!(!parsed.diagnostics.is_empty());

        let builtin = builtin_technique_catalog();
        let merged = merge_technique_catalog_with_overrides(builtin.clone(), &parsed.overrides);

        assert!(!merged.diagnostics.is_empty());

        let builtin_ids: Vec<&str> = builtin
            .iter()
            .map(|technique| technique.id.as_str())
            .collect();
        let merged_ids: Vec<&str> = merged
            .catalog
            .iter()
            .map(|technique| technique.id.as_str())
            .collect();
        assert_eq!(builtin_ids, merged_ids);

        assert!(
            !merged
                .catalog
                .iter()
                .any(|technique| technique.id == "broken")
        );
    }
}
