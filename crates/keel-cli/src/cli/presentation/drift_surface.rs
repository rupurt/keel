//! Shared structural-drift presentation helpers.

use crate::cli::presentation::show::{ShowKeyValues, ShowSection};
use keel::read_model::knowledge_graph::DriftSurfaceSummary;

pub fn render_drift_overview(label: &str, summary: &DriftSurfaceSummary) -> String {
    format!(
        "{label}: {:.2} ({})",
        summary.coefficient,
        summary.severity_label()
    )
}

pub fn render_drift_coverage(summary: &DriftSurfaceSummary) -> String {
    summary
        .facets
        .iter()
        .map(|facet| {
            format!(
                "{} {}/{}",
                facet.kind.short_label(),
                facet.covered,
                facet.total
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

pub fn render_drift_context(summary: &DriftSurfaceSummary, limit: usize) -> Option<String> {
    let hotspots = summary.hotspot_messages(limit);
    if hotspots.is_empty() {
        None
    } else {
        Some(hotspots.join("; "))
    }
}

pub fn render_drift_show_section(summary: &DriftSurfaceSummary) -> ShowSection {
    let mut section = ShowSection::new("Structural Drift");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(12);
    fields.push_row(
        "Coefficient:",
        format!("{:.2} ({})", summary.coefficient, summary.severity_label()),
    );
    fields.push_row("Coverage:", render_drift_coverage(summary));
    if let Some(context) = render_drift_context(summary, 3) {
        fields.push_row("Context:", context);
    }
    section.push_key_values(fields);
    section
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::presentation::show::ShowDocument;
    use keel::read_model::knowledge_graph::{DriftFacetKind, DriftFacetSummary};

    fn sample_summary() -> DriftSurfaceSummary {
        DriftSurfaceSummary {
            coefficient: 0.41,
            facets: vec![
                DriftFacetSummary {
                    kind: DriftFacetKind::EntityArtifacts,
                    covered: 3,
                    total: 5,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::KnowledgeProvenance,
                    covered: 1,
                    total: 2,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::SourceAttachments,
                    covered: 4,
                    total: 7,
                },
                DriftFacetSummary {
                    kind: DriftFacetKind::ProjectDocs,
                    covered: 2,
                    total: 3,
                },
            ],
        }
    }

    #[test]
    fn render_drift_coverage_is_compact_and_deterministic() {
        let rendered = render_drift_coverage(&sample_summary());

        assert_eq!(
            rendered,
            "entities 3/5 | knowledge 1/2 | source 4/7 | docs 2/3"
        );
    }

    #[test]
    fn render_drift_show_section_includes_context() {
        let section = render_drift_show_section(&sample_summary());
        let mut document = ShowDocument::new();
        document.push_sections_spaced([section]);
        let rendered = document.render();

        assert!(rendered.contains("Structural Drift"));
        assert!(rendered.contains("Coefficient:"));
        assert!(rendered.contains("0.41 (elevated)"));
        assert!(rendered.contains("Coverage:"));
        assert!(rendered.contains("Context:"));
        assert!(rendered.contains("3 source files lack graph attachments"));
    }
}
