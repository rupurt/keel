//! Canonical verification-technique catalog model and built-in entries.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProjectSignalReport {
    /// Confidence by project stack (0.0 - 1.0).
    pub stack_confidence: BTreeMap<ProjectStack, f64>,
    /// Normalized hint tokens detected from files/config.
    pub hints: BTreeSet<String>,
    /// Deterministic list of matched repository artifacts.
    pub detected_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TechniqueRecommendation {
    pub id: String,
    pub label: String,
    pub rationale: String,
    pub score: i64,
    pub matched_stacks: Vec<ProjectStack>,
    pub applicable_stacks: Vec<ProjectStack>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowRecommendation {
    pub id: String,
    pub label: String,
    pub rationale: String,
    pub usage_status: String,
    pub adoption_guidance: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ShowRecommendationReport {
    pub recommendations: Vec<ShowRecommendation>,
    pub diagnostics: Vec<String>,
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

/// Detect project stack signals and confidence from repository artifacts.
pub fn detect_project_signals(project_root: &Path) -> ProjectSignalReport {
    let mut report = ProjectSignalReport::default();

    let cargo_toml = project_root.join("Cargo.toml");
    if cargo_toml.exists() {
        bump_stack_confidence(&mut report, ProjectStack::Rust, 0.75);
        report.hints.insert("cargo".to_string());
        report.hints.insert("rust".to_string());
        report.detected_files.push("Cargo.toml".to_string());
    }

    let src_main = project_root.join("src/main.rs");
    let src_bin = project_root.join("src/bin");
    let justfile = project_root.join("justfile");
    if src_main.exists() || src_bin.exists() || justfile.exists() {
        bump_stack_confidence(&mut report, ProjectStack::Cli, 0.55);
        report.hints.insert("cli".to_string());
        if src_main.exists() {
            report.detected_files.push("src/main.rs".to_string());
        }
        if src_bin.exists() {
            report.detected_files.push("src/bin".to_string());
        }
        if justfile.exists() {
            report.detected_files.push("justfile".to_string());
        }
    }

    let package_json = project_root.join("package.json");
    let package_json_content = fs::read_to_string(&package_json).unwrap_or_default();
    if package_json.exists() {
        bump_stack_confidence(&mut report, ProjectStack::Browser, 0.35);
        report.hints.insert("node".to_string());
        report.detected_files.push("package.json".to_string());
    }

    let playwright_ts = project_root.join("playwright.config.ts");
    let playwright_js = project_root.join("playwright.config.js");
    if playwright_ts.exists()
        || playwright_js.exists()
        || package_json_content.contains("playwright")
    {
        bump_stack_confidence(&mut report, ProjectStack::Browser, 0.55);
        report.hints.insert("playwright".to_string());
        report.hints.insert("browser".to_string());
        report.hints.insert("e2e".to_string());
        if playwright_ts.exists() {
            report
                .detected_files
                .push("playwright.config.ts".to_string());
        }
        if playwright_js.exists() {
            report
                .detected_files
                .push("playwright.config.js".to_string());
        }
    }

    let flake_nix = project_root.join("flake.nix");
    let flake_content = fs::read_to_string(&flake_nix).unwrap_or_default();
    if flake_nix.exists() {
        report.detected_files.push("flake.nix".to_string());
    }
    if flake_content.contains("pkgs.vhs")
        || flake_content.contains(" vhs")
        || flake_content.contains("\tvhs")
    {
        bump_stack_confidence(&mut report, ProjectStack::Cli, 0.20);
        report.hints.insert("vhs".to_string());
        report.hints.insert("recording".to_string());
    }
    if flake_content.contains("pkgs.ffmpeg")
        || flake_content.contains(" ffmpeg")
        || flake_content.contains("\tffmpeg")
    {
        bump_stack_confidence(&mut report, ProjectStack::Cli, 0.10);
        report.hints.insert("ffmpeg".to_string());
        report.hints.insert("video".to_string());
    }

    report.detected_files.sort();
    report.detected_files.dedup();
    report
}

/// Build ranked recommendation output from a technique catalog and project signal report.
pub fn recommend_techniques(
    catalog: &[TechniqueDefinition],
    signals: &ProjectSignalReport,
) -> Vec<TechniqueRecommendation> {
    let mut out = Vec::new();

    for technique in catalog
        .iter()
        .filter(|technique| technique.enabled_by_default)
    {
        let matched_stacks: Vec<ProjectStack> = technique
            .applicable_stacks
            .iter()
            .copied()
            .filter(|stack| {
                signals
                    .stack_confidence
                    .get(stack)
                    .copied()
                    .unwrap_or_default()
                    > 0.0
            })
            .collect();

        let stack_score = matched_stacks
            .iter()
            .filter_map(|stack| signals.stack_confidence.get(stack))
            .copied()
            .fold(0.0_f64, f64::max);

        let keyword_matches = technique
            .signal_keywords
            .iter()
            .filter(|keyword| signals.hints.contains(&keyword.to_ascii_lowercase()))
            .count() as i64;

        let prerequisite_matches = technique
            .prerequisites
            .iter()
            .filter(|requirement| signals.hints.contains(&requirement.to_ascii_lowercase()))
            .count() as i64;

        let score = technique.priority as i64
            + (stack_score * 100.0).round() as i64
            + keyword_matches * 7
            + prerequisite_matches * 5;

        let mut rationale_parts = Vec::new();
        if !matched_stacks.is_empty() {
            let stacks = matched_stacks
                .iter()
                .map(stack_name)
                .collect::<Vec<_>>()
                .join(", ");
            rationale_parts.push(format!(
                "matched stack signals: {stacks} (confidence {:.2})",
                stack_score
            ));
        }
        if keyword_matches > 0 {
            rationale_parts.push(format!("matched {} signal keyword(s)", keyword_matches));
        }
        if prerequisite_matches > 0 {
            rationale_parts.push(format!(
                "matched {} prerequisite token(s)",
                prerequisite_matches
            ));
        }
        if rationale_parts.is_empty() {
            rationale_parts.push("baseline recommendation from built-in priority".to_string());
        }

        out.push(TechniqueRecommendation {
            id: technique.id.clone(),
            label: technique.label.clone(),
            rationale: rationale_parts.join("; "),
            score,
            matched_stacks: normalize_copy_vec(&matched_stacks),
            applicable_stacks: normalize_copy_vec(&technique.applicable_stacks),
        });
    }

    out.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.id.cmp(&right.id))
    });
    out
}

/// Build show-surface recommendation output from merged catalog and usage signals.
///
/// - Reads optional `keel.toml` override section.
/// - Parses/merges overrides as advisory diagnostics.
/// - Detects project signals and ranks enabled techniques.
/// - Annotates each recommendation as configured-used/configured-unused.
pub fn build_show_recommendation_report(
    project_root: &Path,
    used_techniques: &BTreeSet<String>,
) -> ShowRecommendationReport {
    let keel_toml = project_root.join("keel.toml");
    let keel_toml_content = fs::read_to_string(&keel_toml).unwrap_or_default();

    let parsed = parse_technique_overrides_from_keel_toml(&keel_toml_content);
    let merged =
        merge_technique_catalog_with_overrides(builtin_technique_catalog(), &parsed.overrides);
    let signals = detect_project_signals(project_root);
    let ranked = recommend_techniques(&merged.catalog, &signals);

    let recommendations = ranked
        .into_iter()
        .take(6)
        .map(|recommendation| {
            let usage_status = if technique_used(&recommendation.id, used_techniques) {
                "configured-in-use"
            } else {
                "configured-unused"
            };
            let adoption_guidance = merged
                .catalog
                .iter()
                .find(|technique| technique.id == recommendation.id)
                .map(adoption_guidance_for)
                .unwrap_or_else(|| {
                    "Adopt: add verification annotations for this technique.".to_string()
                });

            ShowRecommendation {
                id: recommendation.id,
                label: recommendation.label,
                rationale: recommendation.rationale,
                usage_status: usage_status.to_string(),
                adoption_guidance,
            }
        })
        .collect();

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

    ShowRecommendationReport {
        recommendations,
        diagnostics,
    }
}

/// Infer known technique ids from a verify command string.
pub fn infer_used_technique_ids(command: &str) -> Vec<String> {
    let command = command.trim();
    let mut ids = Vec::new();

    if command.starts_with("vhs ") || command == "vhs" {
        ids.push("vhs".to_string());
    }
    if command.starts_with("llm-judge") || command == "llm-judge" {
        ids.push("llm-judge".to_string());
    }
    if command.contains("playwright") {
        ids.push("playwright".to_string());
        ids.push("browser-playwright-e2e".to_string());
    }

    ids.sort();
    ids.dedup();
    ids
}

fn bump_stack_confidence(report: &mut ProjectSignalReport, stack: ProjectStack, delta: f64) {
    let entry = report.stack_confidence.entry(stack).or_insert(0.0);
    *entry = (*entry + delta).min(1.0);
}

fn stack_name(stack: &ProjectStack) -> &'static str {
    match stack {
        ProjectStack::Rust => "rust",
        ProjectStack::Browser => "browser",
        ProjectStack::Cli => "cli",
    }
}

fn technique_used(technique_id: &str, used_techniques: &BTreeSet<String>) -> bool {
    if used_techniques.contains(technique_id) {
        return true;
    }

    match technique_id {
        "browser-playwright-e2e" => used_techniques.contains("playwright"),
        _ => false,
    }
}

fn adoption_guidance_for(technique: &TechniqueDefinition) -> String {
    match technique.id.as_str() {
        "vhs" => {
            "Adopt: add `<!-- verify: vhs demo.tape, SRS-XX:start -->` and attach the generated recording.".to_string()
        }
        "llm-judge" => {
            "Adopt: add `<!-- verify: llm-judge, SRS-XX:start -->` for semantic acceptance checks.".to_string()
        }
        _ => format!(
            "Adopt: use `{}` in verify annotations where this technique applies.",
            technique.default_command
        ),
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
    use std::fs;
    use tempfile::TempDir;

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

    #[test]
    fn technique_project_signal_detection() {
        let temp = TempDir::new().unwrap();
        fs::create_dir_all(temp.path().join("src")).unwrap();
        fs::write(temp.path().join("Cargo.toml"), "[package]\nname=\"demo\"\n").unwrap();
        fs::write(temp.path().join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(
            temp.path().join("package.json"),
            r#"{ "devDependencies": { "playwright": "^1.0.0" } }"#,
        )
        .unwrap();
        fs::write(
            temp.path().join("playwright.config.ts"),
            "export default {};",
        )
        .unwrap();
        fs::write(
            temp.path().join("flake.nix"),
            "buildInputs = [ pkgs.vhs pkgs.ffmpeg ];",
        )
        .unwrap();

        let report = detect_project_signals(temp.path());

        assert!(
            report
                .stack_confidence
                .get(&ProjectStack::Rust)
                .copied()
                .unwrap_or_default()
                > 0.0
        );
        assert!(
            report
                .stack_confidence
                .get(&ProjectStack::Browser)
                .copied()
                .unwrap_or_default()
                > 0.0
        );
        assert!(
            report
                .stack_confidence
                .get(&ProjectStack::Cli)
                .copied()
                .unwrap_or_default()
                > 0.0
        );

        assert!(report.hints.contains("cargo"));
        assert!(report.hints.contains("playwright"));
        assert!(report.hints.contains("vhs"));
        assert!(
            report
                .detected_files
                .iter()
                .any(|entry| entry == "Cargo.toml")
        );
        assert!(
            report
                .detected_files
                .iter()
                .any(|entry| entry == "playwright.config.ts")
        );
    }

    #[test]
    fn technique_recommendation_ranking() {
        let catalog = vec![
            TechniqueDefinition::new(
                "alpha-rust",
                "Alpha Rust",
                "Rust checks",
                TechniqueModality::Command,
                "cargo test",
                true,
                40,
                vec!["cargo"],
                vec![ProjectStack::Rust],
                vec![ArtifactKind::Log],
                vec!["rust", "cargo"],
            ),
            TechniqueDefinition::new(
                "beta-browser",
                "Beta Browser",
                "Browser checks",
                TechniqueModality::Command,
                "npx playwright test",
                true,
                55,
                vec!["playwright"],
                vec![ProjectStack::Browser],
                vec![ArtifactKind::Video],
                vec!["playwright", "browser"],
            ),
        ];

        let mut signals = ProjectSignalReport::default();
        signals.stack_confidence.insert(ProjectStack::Rust, 0.95);
        signals.stack_confidence.insert(ProjectStack::Browser, 0.20);
        signals.hints.insert("cargo".to_string());
        signals.hints.insert("rust".to_string());

        let recommendations = recommend_techniques(&catalog, &signals);
        assert_eq!(recommendations.len(), 2);
        assert_eq!(recommendations[0].id, "alpha-rust");
        assert!(recommendations[0].score > recommendations[1].score);
        assert!(
            recommendations[0]
                .rationale
                .contains("matched stack signals")
        );
        assert!(recommendations[0].rationale.contains("keyword"));
        assert!(
            recommendations[0]
                .matched_stacks
                .contains(&ProjectStack::Rust)
        );
    }

    #[test]
    fn technique_recommendation_deterministic() {
        let first_catalog = vec![
            TechniqueDefinition::new(
                "zeta",
                "Zeta",
                "Z",
                TechniqueModality::Command,
                "zeta",
                true,
                30,
                vec!["zeta"],
                vec![ProjectStack::Cli],
                vec![ArtifactKind::Log],
                vec!["zeta"],
            ),
            TechniqueDefinition::new(
                "alpha",
                "Alpha",
                "A",
                TechniqueModality::Command,
                "alpha",
                true,
                30,
                vec!["alpha"],
                vec![ProjectStack::Cli],
                vec![ArtifactKind::Log],
                vec!["alpha"],
            ),
        ];
        let second_catalog = vec![first_catalog[1].clone(), first_catalog[0].clone()];

        let mut signals = ProjectSignalReport::default();
        signals.stack_confidence.insert(ProjectStack::Cli, 0.8);
        signals.hints.insert("alpha".to_string());
        signals.hints.insert("zeta".to_string());

        let first = recommend_techniques(&first_catalog, &signals);
        let second = recommend_techniques(&second_catalog, &signals);

        assert_eq!(first, second);
        assert_eq!(first[0].id, "alpha");
        assert_eq!(first[1].id, "zeta");
    }
}
