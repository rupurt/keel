use std::collections::BTreeSet;

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StatusFilter {
    statuses: BTreeSet<String>,
}

impl StatusFilter {
    pub(crate) fn contains(&self, status: &str) -> bool {
        self.statuses.contains(status)
    }

    pub(crate) fn display(&self) -> String {
        if self.statuses.is_empty() {
            "(none)".to_string()
        } else {
            self.statuses.iter().cloned().collect::<Vec<_>>().join(", ")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EmptyListState {
    pub(crate) headline: String,
    pub(crate) detail: Option<String>,
    pub(crate) tip: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusDirective {
    Replace,
    Add,
    Remove,
}

pub(crate) fn resolve_status_filter(
    args: &[String],
    default_statuses: &[&str],
    allowed_statuses: &[&str],
) -> Result<StatusFilter> {
    let mut statuses: BTreeSet<String> =
        default_statuses.iter().map(|s| (*s).to_string()).collect();
    let mut saw_replace = false;

    for raw in args {
        let (directive, status) = split_status_arg(raw);
        if !allowed_statuses.contains(&status) {
            return Err(anyhow!(
                "invalid status '{}'; expected one of: {}",
                raw,
                allowed_statuses.join(", ")
            ));
        }

        match directive {
            StatusDirective::Replace => {
                if !saw_replace {
                    statuses.clear();
                    saw_replace = true;
                }
                statuses.insert(status.to_string());
            }
            StatusDirective::Add => {
                statuses.insert(status.to_string());
            }
            StatusDirective::Remove => {
                statuses.remove(status);
            }
        }
    }

    Ok(StatusFilter { statuses })
}

pub(crate) fn validate_status_arg(
    value: &str,
    allowed_statuses: &[&str],
) -> Result<String, String> {
    let (_, status) = split_status_arg(value);
    if allowed_statuses.contains(&status) {
        Ok(value.to_string())
    } else {
        Err(format!(
            "invalid status '{}'; expected one of: {}",
            value,
            allowed_statuses.join(", ")
        ))
    }
}

pub(crate) fn build_empty_list_state(
    entity_plural: &str,
    has_any_entities: bool,
    status_filter: &StatusFilter,
    tip: Option<String>,
) -> EmptyListState {
    if has_any_entities {
        EmptyListState {
            headline: format!("No {entity_plural} matched the current filters."),
            detail: Some(format!("Status filter: {}", status_filter.display())),
            tip,
        }
    } else {
        EmptyListState {
            headline: format!("No {entity_plural} found."),
            detail: None,
            tip,
        }
    }
}

pub(crate) fn print_empty_list_state(state: &EmptyListState) {
    println!("{}", state.headline.yellow());
    if let Some(detail) = &state.detail {
        println!("{}", detail.dimmed());
    }
    if let Some(tip) = &state.tip {
        println!("{}", format!("Tip: {tip}").dimmed());
    }
}

pub(crate) fn build_append_status_suggestion(
    append_statuses: &[&str],
    description: &str,
) -> Option<String> {
    if append_statuses.is_empty() {
        None
    } else {
        let flags = append_statuses
            .iter()
            .map(|status| format!("`--status +{status}`"))
            .collect::<Vec<_>>()
            .join(" ");
        Some(format!("Try {flags} to include {description}."))
    }
}

fn split_status_arg(value: &str) -> (StatusDirective, &str) {
    match value.as_bytes().first().copied() {
        Some(b'+') => (StatusDirective::Add, &value[1..]),
        Some(b'-') => (StatusDirective::Remove, &value[1..]),
        _ => (StatusDirective::Replace, value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULTS: &[&str] = &["draft", "active"];
    const ALLOWED: &[&str] = &["draft", "active", "done"];

    #[test]
    fn resolve_status_filter_uses_defaults_when_empty() {
        let filter = resolve_status_filter(&[], DEFAULTS, ALLOWED).unwrap();
        assert!(filter.contains("draft"));
        assert!(filter.contains("active"));
        assert!(!filter.contains("done"));
    }

    #[test]
    fn resolve_status_filter_replaces_defaults_with_plain_values() {
        let filter = resolve_status_filter(
            &["done".to_string(), "draft".to_string()],
            DEFAULTS,
            ALLOWED,
        )
        .unwrap();
        assert!(filter.contains("done"));
        assert!(filter.contains("draft"));
        assert!(!filter.contains("active"));
    }

    #[test]
    fn resolve_status_filter_supports_append_and_remove() {
        let filter = resolve_status_filter(
            &["+done".to_string(), "-active".to_string()],
            DEFAULTS,
            ALLOWED,
        )
        .unwrap();
        assert!(filter.contains("draft"));
        assert!(filter.contains("done"));
        assert!(!filter.contains("active"));
    }

    #[test]
    fn validate_status_arg_accepts_prefix_notation() {
        assert_eq!(
            validate_status_arg("+done", ALLOWED).unwrap(),
            "+done".to_string()
        );
        assert_eq!(
            validate_status_arg("-draft", ALLOWED).unwrap(),
            "-draft".to_string()
        );
    }

    #[test]
    fn resolve_status_filter_can_remove_every_default_status() {
        let filter = resolve_status_filter(
            &["-draft".to_string(), "-active".to_string()],
            DEFAULTS,
            ALLOWED,
        )
        .unwrap();
        assert_eq!(filter.display(), "(none)");
        assert!(!filter.contains("draft"));
        assert!(!filter.contains("active"));
    }

    #[test]
    fn build_empty_list_state_reports_filters_when_entities_exist() {
        let filter = resolve_status_filter(&[], DEFAULTS, ALLOWED).unwrap();
        let state = build_empty_list_state(
            "epics",
            true,
            &filter,
            Some("Try `--status +done` to include completed items.".to_string()),
        );

        assert_eq!(state.headline, "No epics matched the current filters.");
        assert_eq!(
            state.detail,
            Some("Status filter: active, draft".to_string())
        );
        assert_eq!(
            state.tip,
            Some("Try `--status +done` to include completed items.".to_string())
        );
    }

    #[test]
    fn build_empty_list_state_omits_filter_details_when_board_is_empty() {
        let filter = resolve_status_filter(&[], DEFAULTS, ALLOWED).unwrap();
        let state = build_empty_list_state("stories", false, &filter, None);

        assert_eq!(state.headline, "No stories found.");
        assert_eq!(state.detail, None);
        assert_eq!(state.tip, None);
    }

    #[test]
    fn build_append_status_suggestion_renders_repeatable_flags() {
        let suggestion = build_append_status_suggestion(&["done"], "completed items").unwrap();
        assert_eq!(
            suggestion,
            "Try `--status +done` to include completed items."
        );

        let multi =
            build_append_status_suggestion(&["laid", "parked"], "terminal bearings").unwrap();
        assert_eq!(
            multi,
            "Try `--status +laid` `--status +parked` to include terminal bearings."
        );
    }
}
