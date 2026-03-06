//! Shared render helpers for planning lineage sections.

use crate::domain::state_machine::invariants::{ScopeDisposition, ScopeLineageIssueKind};
use crate::read_model::planning_show::{ScopeDriftRow, ScopeLineageRow};

pub fn format_scope_lineage_row(row: &ScopeLineageRow) -> String {
    let voyage_disposition = scope_disposition_label(row.voyage_disposition);
    match (row.epic_disposition, row.epic_description.as_deref()) {
        (Some(epic_disposition), Some(epic_description)) => {
            let epic_disposition = scope_disposition_label(epic_disposition);
            let epic_detail = if same_scope_description(&row.voyage_description, epic_description) {
                String::new()
            } else {
                format!(": {epic_description}")
            };
            format!(
                "`{}`: {} (voyage {}; epic {}{})",
                row.scope_id,
                row.voyage_description,
                voyage_disposition,
                epic_disposition,
                epic_detail
            )
        }
        _ => format!(
            "`{}`: {} (voyage {}; epic unknown)",
            row.scope_id, row.voyage_description, voyage_disposition
        ),
    }
}

pub fn format_scope_drift_row(row: &ScopeDriftRow) -> String {
    let prefix = row
        .voyage_id
        .as_deref()
        .map(|voyage_id| format!("`{voyage_id}`: "))
        .unwrap_or_default();
    match row.issue.kind {
        ScopeLineageIssueKind::MissingScopeMapping => format!(
            "{}`{}` missing voyage scope mapping for an epic in-scope item",
            prefix,
            row.issue.scope_id.as_deref().unwrap_or("<unknown>")
        ),
        ScopeLineageIssueKind::UnknownScopeRef => format!(
            "{}`{}` references unknown epic scope ID",
            prefix,
            row.issue.scope_id.as_deref().unwrap_or("<unknown>")
        ),
        ScopeLineageIssueKind::OutOfScopeContradiction => format!(
            "{}`{}` marks an epic out-of-scope item as in scope",
            prefix,
            row.issue.scope_id.as_deref().unwrap_or("<unknown>")
        ),
        ScopeLineageIssueKind::LegacyUntaggedScopePath => format!(
            "{}legacy scope bullet: {}",
            prefix,
            row.issue.line.as_deref().unwrap_or("<unknown>")
        ),
    }
}

fn scope_disposition_label(disposition: ScopeDisposition) -> &'static str {
    match disposition {
        ScopeDisposition::In => "in-scope",
        ScopeDisposition::Out => "out-of-scope",
    }
}

fn same_scope_description(left: &str, right: &str) -> bool {
    normalize_scope_description(left) == normalize_scope_description(right)
}

fn normalize_scope_description(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::state_machine::invariants::{
        ScopeDisposition, ScopeLineageIssue, ScopeLineageIssueKind,
    };

    #[test]
    fn scope_lineage_formatter_includes_parent_context_only_when_needed() {
        let row = ScopeLineageRow {
            scope_id: "SCOPE-01".to_string(),
            voyage_description: "Render lineage output.".to_string(),
            voyage_disposition: ScopeDisposition::In,
            epic_description: Some("Render lineage output.".to_string()),
            epic_disposition: Some(ScopeDisposition::In),
        };

        let rendered = format_scope_lineage_row(&row);

        assert!(rendered.contains("voyage in-scope; epic in-scope"));
        assert!(!rendered.contains("epic in-scope:"));
    }

    #[test]
    fn scope_drift_formatter_includes_voyage_prefix_when_present() {
        let row = ScopeDriftRow {
            voyage_id: Some("v1".to_string()),
            issue: ScopeLineageIssue {
                artifact_path: std::path::PathBuf::from("SRS.md"),
                scope_id: Some("SCOPE-02".to_string()),
                line: None,
                kind: ScopeLineageIssueKind::UnknownScopeRef,
            },
        };

        let rendered = format_scope_drift_row(&row);

        assert!(rendered.starts_with("`v1`:"));
        assert!(rendered.contains("unknown epic scope ID"));
    }
}
