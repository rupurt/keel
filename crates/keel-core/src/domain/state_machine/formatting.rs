//! Shared formatting helpers for transition and gate errors.

use crate::infrastructure::validation::Problem;

/// Format a non-empty set of transition/gate problems into a command error string.
pub fn format_transition_error(entity: &str, transition: &str, problems: &[Problem]) -> String {
    let detail_lines = problems
        .iter()
        .map(|problem| format!("- {}", problem.message))
        .collect::<Vec<_>>()
        .join("\n");

    format!("Cannot {transition} {entity}:\n{detail_lines}")
}
