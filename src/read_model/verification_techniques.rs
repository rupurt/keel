//! Canonical verification-technique catalog model and built-in entries.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TechniqueModality {
    Command,
    Recording,
    Judge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectStack {
    Rust,
    Browser,
    Cli,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        let ids: Vec<&str> = catalog.iter().map(|t| t.id.as_str()).collect();

        assert!(ids.contains(&"vhs"));
        assert!(ids.contains(&"llm-judge"));
        assert!(catalog.iter().any(|t| {
            t.modality == TechniqueModality::Command
                && t.applicable_stacks.contains(&ProjectStack::Rust)
        }));
        assert!(catalog.iter().any(|t| {
            t.modality == TechniqueModality::Command
                && t.applicable_stacks.contains(&ProjectStack::Browser)
        }));
    }

    #[test]
    fn builtin_technique_catalog_deterministic() {
        let first = builtin_technique_catalog();
        let second = builtin_technique_catalog();

        assert_eq!(first, second);

        let ids: Vec<&str> = first.iter().map(|t| t.id.as_str()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        assert_eq!(ids, sorted);
    }
}
