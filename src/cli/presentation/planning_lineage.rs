//! Shared render helpers for planning lineage sections.

use crate::domain::state_machine::invariants::ScopeLineageIssueKind;
use crate::read_model::planning_show::ScopeDriftRow;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::state_machine::invariants::{ScopeLineageIssue, ScopeLineageIssueKind};

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
