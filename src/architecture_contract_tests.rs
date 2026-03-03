//! Architecture contract tests for enforceable layer boundaries.
//!
//! These checks fail fast when forbidden dependency edges are introduced
//! between core DDD layers.

use std::fs;
use std::path::{Path, PathBuf};

fn repo_file(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn read_source(path: &str) -> String {
    fs::read_to_string(repo_file(path)).expect("source file should be readable")
}

fn read_production_source(path: &str) -> String {
    let source = read_source(path);
    source
        .split("\n#[cfg(test)]")
        .next()
        .map(str::to_string)
        .unwrap_or(source)
}

fn forbidden_patterns(content: &str, patterns: &[&str]) -> Vec<String> {
    patterns
        .iter()
        .filter(|pattern| content.contains(**pattern))
        .map(|pattern| pattern.to_string())
        .collect()
}

fn assert_no_forbidden_edges_in_production(path: &str, patterns: &[&str]) {
    let content = read_production_source(path);
    let violations = forbidden_patterns(&content, patterns);
    assert!(
        violations.is_empty(),
        "{path} has forbidden dependency edges in production code: {:?}",
        violations
    );
}

#[test]
fn main_dispatch_adapter_avoids_domain_and_infrastructure_internals() {
    assert_no_forbidden_edges_in_production(
        "src/main.rs",
        &[
            "keel::domain::",
            "keel::infrastructure::",
            "keel::application::",
            "keel::read_model::",
        ],
    );
}

#[test]
fn main_bootstrap_depends_on_cli_surface() {
    let main_source = read_source("src/main.rs");
    assert!(
        main_source.contains("keel::cli::run()"),
        "main bootstrap should delegate to the cli runtime surface"
    );
    for forbidden in [
        "keel::domain::",
        "keel::infrastructure::",
        "keel::application::",
        "keel::read_model::",
    ] {
        assert!(
            !main_source.contains(forbidden),
            "main bootstrap should not depend on {forbidden} directly"
        );
    }
}

#[test]
fn lib_declarations_expose_normalized_layer_roots() {
    let lib_source = read_source("src/lib.rs");
    for required in [
        "pub mod cli;",
        "pub mod application;",
        "pub mod domain;",
        "pub mod infrastructure;",
        "pub mod read_model;",
    ] {
        assert!(
            lib_source.contains(required),
            "lib module declarations should include {required}"
        );
    }

    for forbidden in [
        "mod commands;",
        "mod flow;",
        "mod next;",
        "mod model;",
        "mod policy;",
        "mod state_machine;",
        "mod transitions;",
        "mod loader;",
        "mod parser;",
        "mod templates;",
        "mod generate;",
        "mod verification;",
        "mod config;",
        "mod evidence;",
        "mod invariants;",
        "mod scoring;",
        "mod story_id;",
        "mod style;",
        "mod table;",
        "mod taxonomy;",
        "mod traceability;",
        "mod utils;",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "lib module declarations should not include legacy {forbidden}"
        );
    }
}

#[test]
fn source_tree_removes_legacy_root_module_families() {
    for missing_path in [
        "src/commands",
        "src/flow",
        "src/next",
        "src/cli/commands/adr",
        "src/cli/commands/bearing",
        "src/cli/commands/epic",
        "src/cli/commands/knowledge",
        "src/cli/commands/story",
        "src/cli/commands/voyage",
        "src/cli/commands/next.rs",
        "src/cli/commands/play.rs",
        "src/cli/commands/verify.rs",
        "src/cli/next",
        "src/cli/flow",
        "src/model",
        "src/policy",
        "src/state_machine",
        "src/transitions",
        "src/generate",
        "src/verification",
        "src/loader.rs",
        "src/parser.rs",
        "src/templates.rs",
        "src/config.rs",
        "src/evidence.rs",
        "src/invariants.rs",
        "src/scoring.rs",
        "src/story_id.rs",
        "src/style.rs",
        "src/table.rs",
        "src/taxonomy.rs",
        "src/traceability.rs",
        "src/utils.rs",
    ] {
        assert!(
            !repo_file(missing_path).exists(),
            "legacy root path should be removed: {missing_path}"
        );
    }

    for expected_path in [
        "src/lib.rs",
        "src/cli",
        "src/cli/commands/management",
        "src/cli/presentation/flow",
        "src/application",
        "src/domain",
        "src/infrastructure",
        "src/read_model",
        "src/infrastructure/config.rs",
        "src/read_model/evidence.rs",
        "src/domain/state_machine/invariants.rs",
        "src/infrastructure/scoring.rs",
        "src/infrastructure/story_id.rs",
        "src/cli/style.rs",
        "src/cli/table.rs",
        "src/domain/model/taxonomy.rs",
        "src/read_model/traceability.rs",
        "src/infrastructure/utils.rs",
    ] {
        assert!(
            repo_file(expected_path).exists(),
            "normalized layer root should exist: {expected_path}"
        );
    }
}

#[test]
fn next_algorithm_avoids_interface_or_transition_edges() {
    assert_no_forbidden_edges_in_production(
        "src/cli/commands/management/next_support/algorithm.rs",
        &[
            "crate::cli::commands::",
            "crate::domain::transitions::",
            "crate::read_model::flow_metrics::calculate_metrics",
        ],
    );
}

#[test]
fn flow_and_status_adapters_use_canonical_projection_service() {
    let flow = read_production_source("src/cli/commands/diagnostics/flow.rs");
    let status = read_production_source("src/cli/commands/diagnostics/status.rs");

    assert!(
        flow.contains("flow_status::project"),
        "flow adapter should use canonical projection service"
    );
    assert!(
        status.contains("flow_status::project"),
        "status adapter should use canonical projection service"
    );
    assert!(
        !flow.contains("flow::metrics::calculate_metrics"),
        "flow adapter should not use legacy flow metrics path directly"
    );
    assert!(
        !status.contains("flow::metrics::calculate_metrics"),
        "status adapter should not use legacy flow metrics path directly"
    );
    assert!(
        !flow.contains("read_model::flow_metrics::calculate_metrics"),
        "flow adapter should use canonical projection service instead of direct flow metrics"
    );
    assert!(
        !status.contains("read_model::flow_metrics::calculate_metrics"),
        "status adapter should use canonical projection service instead of direct flow metrics"
    );
}

#[test]
fn capacity_diagnostics_adapter_delegates_to_shared_capacity_interface() {
    let capacity = read_production_source("src/cli/commands/diagnostics/capacity.rs");

    assert!(
        capacity.contains("flow::capacity"),
        "capacity diagnostics adapter should delegate to shared flow capacity interface"
    );
    assert!(
        !capacity.contains("read_model::capacity"),
        "capacity diagnostics adapter should not wire read model directly"
    );
    assert!(
        !capacity.contains("load_board"),
        "capacity diagnostics adapter should not duplicate board loading logic"
    );
}

#[test]
fn throughput_diagnostics_adapter_uses_throughput_projection_and_store() {
    let throughput = read_production_source("src/cli/commands/diagnostics/throughput.rs");

    assert!(
        throughput.contains("read_model::throughput_history::project_default"),
        "throughput diagnostics adapter should consume throughput projection via read model"
    );
    assert!(
        throughput.contains("infrastructure::throughput_history_store::save_if_changed"),
        "throughput diagnostics adapter should persist through the infrastructure store adapter"
    );
    assert!(
        !throughput.contains("flow::metrics::calculate_metrics"),
        "throughput diagnostics adapter should not depend on legacy flow metrics path"
    );
    assert!(
        !throughput.contains("read_model::flow_metrics::calculate_metrics"),
        "throughput diagnostics adapter should not bypass throughput projection with direct flow metrics"
    );
}

#[test]
fn queue_policy_consumers_use_shared_read_model_api() {
    for path in [
        "src/cli/commands/management/next_support/algorithm.rs",
        "src/cli/presentation/flow/bottleneck.rs",
        "src/domain/state_machine/flow.rs",
    ] {
        let content = read_production_source(path);
        assert!(
            content.contains("read_model::queue_policy"),
            "{path} should consume queue policy via shared read-model API"
        );
    }
}

#[test]
fn diagnostics_adapters_avoid_cross_context_orchestration_edges() {
    let forbidden = [
        "crate::cli::commands::management::story::",
        "crate::cli::commands::management::voyage::",
        "crate::cli::commands::management::epic::",
        "crate::cli::commands::management::bearing::",
        "crate::cli::commands::management::adr::",
        "crate::domain::transitions::",
        "derive_implementation_dependencies(",
    ];

    for path in [
        "src/cli/commands/diagnostics/flow.rs",
        "src/cli/commands/diagnostics/status.rs",
        "src/cli/commands/diagnostics/capacity.rs",
    ] {
        assert_no_forbidden_edges_in_production(path, &forbidden);
    }
}

#[test]
fn creation_paths_use_shared_template_rendering_service() {
    for path in [
        "src/cli/commands/management/story/new.rs",
        "src/cli/commands/management/epic/new.rs",
        "src/cli/commands/management/voyage/new.rs",
        "src/cli/commands/management/bearing/new.rs",
    ] {
        let content = read_production_source(path);
        assert!(
            content.contains("template_rendering::render"),
            "{path} should call shared template rendering service"
        );
        assert!(
            !content.contains("story::new::render_template"),
            "{path} should not depend on story::new::render_template"
        );
    }
}

#[test]
fn lifecycle_commands_delegate_to_application_services() {
    for path in [
        "src/cli/commands/management/story/start.rs",
        "src/cli/commands/management/story/submit.rs",
        "src/cli/commands/management/story/accept.rs",
        "src/cli/commands/management/story/reject.rs",
        "src/cli/commands/management/story/ice.rs",
        "src/cli/commands/management/story/thaw.rs",
    ] {
        let content = read_source(path);
        assert!(
            content.contains("StoryLifecycleService::"),
            "{path} should delegate to StoryLifecycleService"
        );
        assert!(
            !content.contains("crate::domain::transitions::"),
            "{path} should not call transitions directly"
        );
    }

    for path in [
        "src/cli/commands/management/voyage/start.rs",
        "src/cli/commands/management/voyage/done.rs",
    ] {
        let content = read_source(path);
        assert!(
            content.contains("VoyageEpicLifecycleService::"),
            "{path} should delegate to VoyageEpicLifecycleService"
        );
        assert!(
            !content.contains("crate::domain::transitions::"),
            "{path} should not call transitions directly"
        );
    }
}

#[test]
fn read_model_projection_layer_avoids_command_and_transition_edges() {
    let read_model_dir = repo_file("src/read_model");
    let entries = fs::read_dir(read_model_dir).expect("read_model directory should exist");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let content = fs::read_to_string(&path).expect("read_model source should be readable");
        let violations = forbidden_patterns(
            &content,
            &[
                "crate::cli::commands::",
                "crate::cli::presentation::",
                "crate::domain::transitions::",
            ],
        );
        assert!(
            violations.is_empty(),
            "{} has forbidden dependency edges: {:?}",
            path.display(),
            violations
        );
    }
}

#[test]
fn forbidden_edge_detector_reports_fixture_violations() {
    let fixture = "use crate::cli::commands::management::story::start;\nuse crate::domain::transitions::execute;\n";
    let violations = forbidden_patterns(
        fixture,
        &["crate::cli::commands::", "crate::domain::transitions::"],
    );
    assert_eq!(
        violations,
        vec![
            "crate::cli::commands::".to_string(),
            "crate::domain::transitions::".to_string()
        ]
    );
}

#[test]
fn forbidden_edge_detector_reports_cross_context_fixture_violations() {
    let fixture = "use crate::cli::commands::management::story::new::run;\nuse crate::domain::transitions::execute;\n";
    let violations = forbidden_patterns(
        fixture,
        &[
            "crate::cli::commands::management::story::",
            "crate::domain::transitions::",
        ],
    );
    assert_eq!(
        violations,
        vec![
            "crate::cli::commands::management::story::".to_string(),
            "crate::domain::transitions::".to_string()
        ]
    );
}
